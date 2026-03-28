use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow, bail};
use chrono::Utc;
use serde_yaml::{Mapping, Number, Value};

const FRONTMATTER_DELIMITER: &str = "---\n";
const FRONTMATTER_END: &str = "\n---\n";
const VALID_STATUSES: &[&str] = &["proposed", "open", "completed"];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TrackedKind {
    Workstream,
    Patch,
}

impl TrackedKind {
    pub(crate) fn singular(self) -> &'static str {
        match self {
            Self::Workstream => "workstream",
            Self::Patch => "patch",
        }
    }

    pub(crate) fn plural(self) -> &'static str {
        match self {
            Self::Workstream => "workstreams",
            Self::Patch => "patches",
        }
    }

    fn root(self, repo_root: &Path) -> PathBuf {
        repo_root.join("workflow").join(self.plural())
    }

    pub(crate) fn status_path(self, item_path: &Path) -> PathBuf {
        match self {
            Self::Workstream => item_path.join("STATUS.md"),
            Self::Patch => item_path.to_path_buf(),
        }
    }

    pub(crate) fn display_name(self, item_path: &Path) -> String {
        let raw_name = item_path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or(match self {
                Self::Workstream => "<unknown-workstream>",
                Self::Patch => "<unknown-patch>",
            });
        match self {
            Self::Workstream => raw_name.to_owned(),
            Self::Patch => raw_name.trim_end_matches(".md").to_owned(),
        }
    }
}

pub(crate) fn today_string() -> String {
    Utc::now().date_naive().to_string()
}

pub(crate) fn resolve_tracked_path(
    repo_root: &Path,
    tracked_ref: &str,
    kind: TrackedKind,
) -> Result<PathBuf> {
    let items_dir = kind.root(repo_root);
    if !items_dir.exists() {
        bail!(
            "{} not found: {tracked_ref}. Use the numeric id or full name from workflow/{}/.",
            capitalize(kind.singular()),
            kind.plural(),
        );
    }

    let direct = items_dir.join(tracked_ref);
    match kind {
        TrackedKind::Workstream if direct.is_dir() => return Ok(direct),
        TrackedKind::Patch if direct.is_file() => return Ok(direct),
        _ => {}
    }

    if kind == TrackedKind::Patch && direct.with_extension("md").is_file() {
        return Ok(direct.with_extension("md"));
    }

    let mut matches = Vec::new();
    for entry in fs::read_dir(&items_dir)
        .with_context(|| format!("Failed to read {}", items_dir.display()))?
    {
        let entry = entry.with_context(|| format!("Failed to inspect {}", items_dir.display()))?;
        if !entry
            .file_type()
            .with_context(|| format!("Failed to inspect {}", entry.path().display()))?
            .is_dir()
            && !entry
                .file_type()
                .with_context(|| format!("Failed to inspect {}", entry.path().display()))?
                .is_file()
        {
            continue;
        }

        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        let canonical_name = match kind {
            TrackedKind::Workstream => file_name.as_ref(),
            TrackedKind::Patch => file_name.trim_end_matches(".md"),
        };
        if canonical_name == tracked_ref
            || file_name
                .split_once('-')
                .is_some_and(|(prefix, _)| prefix == tracked_ref)
        {
            matches.push(entry.path());
        }
    }

    match matches.len() {
        1 => Ok(matches.remove(0)),
        0 => bail!(
            "{} not found: {tracked_ref}. Use the numeric id or full name from workflow/{}/.",
            capitalize(kind.singular()),
            kind.plural(),
        ),
        _ => bail!(
            "{} reference is ambiguous: {tracked_ref}",
            capitalize(kind.singular())
        ),
    }
}

pub(crate) fn list_tracked_items(repo_root: &Path, kind: TrackedKind) -> Result<Vec<PathBuf>> {
    let items_dir = kind.root(repo_root);
    let mut entries = Vec::new();

    if !items_dir.exists() {
        return Ok(entries);
    }

    for entry in fs::read_dir(&items_dir)
        .with_context(|| format!("Failed to read {}", items_dir.display()))?
    {
        let entry = entry.with_context(|| format!("Failed to inspect {}", items_dir.display()))?;
        let file_type = entry
            .file_type()
            .with_context(|| format!("Failed to inspect {}", entry.path().display()))?;
        match kind {
            TrackedKind::Workstream if file_type.is_dir() => entries.push(entry.path()),
            TrackedKind::Patch if file_type.is_file() => entries.push(entry.path()),
            _ => {}
        }
    }

    entries.sort_by_key(|path| {
        let name = path
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or_default();
        let prefix = name
            .split_once('-')
            .and_then(|(prefix, _)| prefix.parse::<u32>().ok())
            .unwrap_or(u32::MAX);
        (prefix, name.to_owned())
    });

    Ok(entries)
}

pub(crate) struct StatusFile {
    status: String,
    summary: String,
    updated: String,
    prs: Option<Vec<u64>>,
    extra: BTreeMap<String, Value>,
    body: String,
}

impl StatusFile {
    pub(crate) fn read(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let (frontmatter, body) = split_frontmatter(&content)?;
        let mapping: Mapping = serde_yaml::from_str(frontmatter)
            .with_context(|| format!("Invalid frontmatter in {}", path.display()))?;

        let mut status = None;
        let mut summary = None;
        let mut updated = None;
        let mut prs = None;
        let mut extra = BTreeMap::new();

        for (key, value) in mapping {
            let key = key
                .as_str()
                .ok_or_else(|| anyhow!("Frontmatter keys must be strings in {}", path.display()))?
                .to_owned();
            match key.as_str() {
                "status" => {
                    let next = value.as_str().ok_or_else(|| {
                        anyhow!("`status` must be a string in {}", path.display())
                    })?;
                    validate_status(next)?;
                    status = Some(next.to_owned());
                }
                "summary" => {
                    let next = value.as_str().ok_or_else(|| {
                        anyhow!("`summary` must be a string in {}", path.display())
                    })?;
                    summary = Some(next.to_owned());
                }
                "updated" => {
                    let next = value.as_str().ok_or_else(|| {
                        anyhow!("`updated` must be a string in {}", path.display())
                    })?;
                    updated = Some(next.to_owned());
                }
                "prs" => {
                    let sequence = value
                        .as_sequence()
                        .ok_or_else(|| anyhow!("`prs` must be a list in {}", path.display()))?;
                    let mut numbers = Vec::new();
                    for value in sequence {
                        numbers.push(value.as_u64().ok_or_else(|| {
                            anyhow!("`prs` entries must be numbers in {}", path.display())
                        })?);
                    }
                    prs = Some(numbers);
                }
                _ => {
                    extra.insert(key, value);
                }
            }
        }

        Ok(Self {
            status: status
                .ok_or_else(|| anyhow!("Missing required `status` field in {}", path.display()))?,
            summary: summary
                .ok_or_else(|| anyhow!("Missing required `summary` field in {}", path.display()))?,
            updated: updated
                .ok_or_else(|| anyhow!("Missing required `updated` field in {}", path.display()))?,
            prs,
            extra,
            body: body.to_owned(),
        })
    }

    pub(crate) fn write(&self, path: &Path) -> Result<()> {
        fs::write(path, self.render()?)
            .with_context(|| format!("Failed to write {}", path.display()))
    }

    pub(crate) fn status(&self) -> &str {
        &self.status
    }

    pub(crate) fn summary(&self) -> &str {
        &self.summary
    }

    pub(crate) fn updated(&self) -> &str {
        &self.updated
    }

    pub(crate) fn prs(&self) -> Option<&[u64]> {
        self.prs.as_deref()
    }

    pub(crate) fn set_status(&mut self, value: &str) -> Result<()> {
        validate_status(value)?;
        self.status = value.to_owned();
        Ok(())
    }

    pub(crate) fn set_summary(&mut self, value: &str) {
        self.summary = value.to_owned();
    }

    pub(crate) fn touch_updated(&mut self) {
        self.updated = today_string();
    }

    pub(crate) fn set_prs(&mut self, value: Vec<u64>) {
        self.prs = Some(value);
    }

    pub(crate) fn clear_prs(&mut self) {
        self.prs = None;
    }

    fn render(&self) -> Result<String> {
        let mut mapping = Mapping::new();
        mapping.insert(
            Value::String("status".to_owned()),
            Value::String(self.status.clone()),
        );
        mapping.insert(
            Value::String("summary".to_owned()),
            Value::String(self.summary.clone()),
        );
        mapping.insert(
            Value::String("updated".to_owned()),
            Value::String(self.updated.clone()),
        );
        if let Some(prs) = &self.prs {
            let sequence = prs
                .iter()
                .map(|value| Value::Number(Number::from(*value)))
                .collect::<Vec<_>>();
            mapping.insert(Value::String("prs".to_owned()), Value::Sequence(sequence));
        }
        for (key, value) in &self.extra {
            mapping.insert(Value::String(key.clone()), value.clone());
        }

        let yaml = serde_yaml::to_string(&mapping)
            .context("Failed to render status frontmatter")?
            .trim_start_matches(FRONTMATTER_DELIMITER)
            .to_owned();

        let body = self.body.trim_start_matches('\n');
        let mut content = format!("{FRONTMATTER_DELIMITER}{yaml}{FRONTMATTER_END}");
        if !body.is_empty() {
            content.push('\n');
            content.push_str(body);
            if !body.ends_with('\n') {
                content.push('\n');
            }
        }
        Ok(content)
    }
}

fn split_frontmatter(content: &str) -> Result<(&str, &str)> {
    let remainder = content
        .strip_prefix(FRONTMATTER_DELIMITER)
        .ok_or_else(|| anyhow!("Status file must start with frontmatter"))?;
    let end = remainder
        .find(FRONTMATTER_END)
        .ok_or_else(|| anyhow!("Status file frontmatter must end with `---`"))?;
    let frontmatter = &remainder[..end];
    let body = &remainder[end + FRONTMATTER_END.len()..];
    Ok((frontmatter, body))
}

pub(crate) fn validate_status(value: &str) -> Result<()> {
    if VALID_STATUSES.contains(&value) {
        return Ok(());
    }

    bail!(
        "Unsupported status `{value}`. Valid values are: {}",
        VALID_STATUSES.join(", ")
    );
}

fn capitalize(value: &str) -> String {
    let mut characters = value.chars();
    match characters.next() {
        Some(first) => first.to_uppercase().chain(characters).collect(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::{StatusFile, split_frontmatter, today_string, validate_status};

    #[test]
    fn split_frontmatter_reads_basic_shape() {
        let content = "---\nstatus: open\nsummary: Example\nupdated: 2026-03-28\n---\n\n# Status\n";
        let (frontmatter, body) = split_frontmatter(content).expect("frontmatter");
        assert!(frontmatter.contains("status: open"));
        assert!(body.contains("# Status"));
    }

    #[test]
    fn status_round_trip_preserves_prs_and_body() {
        let original = "---\nstatus: open\nsummary: Example\nupdated: 2026-03-28\nprs:\n  - 3\n---\n\n# Status\n\nBody.\n";
        let path = tempfile::NamedTempFile::new().expect("tempfile");
        std::fs::write(path.path(), original).expect("write");

        let mut status = StatusFile::read(path.path()).expect("read");
        status.touch_updated();
        status.write(path.path()).expect("write");

        let rendered = std::fs::read_to_string(path.path()).expect("read");
        assert!(rendered.contains("status: open"));
        assert!(rendered.contains("prs:"));
        assert!(rendered.contains("- 3"));
        assert!(rendered.contains("# Status"));
        assert!(rendered.contains(&today_string()));
    }

    #[test]
    fn validate_status_accepts_supported_values() {
        assert!(validate_status("open").is_ok());
        assert!(validate_status("completed").is_ok());
        assert!(validate_status("done").is_err());
    }
}
