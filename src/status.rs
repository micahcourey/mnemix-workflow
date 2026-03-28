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

pub(crate) fn today_string() -> String {
    Utc::now().date_naive().to_string()
}

pub(crate) fn resolve_workstream_path(repo_root: &Path, workstream_ref: &str) -> Result<PathBuf> {
    let workstreams_dir = repo_root.join("workflow").join("workstreams");
    let direct = workstreams_dir.join(workstream_ref);
    if direct.is_dir() {
        return Ok(direct);
    }

    let mut matches = Vec::new();
    for entry in fs::read_dir(&workstreams_dir)
        .with_context(|| format!("Failed to read {}", workstreams_dir.display()))?
    {
        let entry =
            entry.with_context(|| format!("Failed to inspect {}", workstreams_dir.display()))?;
        if !entry
            .file_type()
            .with_context(|| format!("Failed to inspect {}", entry.path().display()))?
            .is_dir()
        {
            continue;
        }

        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if file_name == workstream_ref
            || file_name
                .split_once('-')
                .is_some_and(|(prefix, _)| prefix == workstream_ref)
        {
            matches.push(entry.path());
        }
    }

    match matches.len() {
        1 => Ok(matches.remove(0)),
        0 => bail!(
            "Workstream not found: {workstream_ref}. Use the numeric id or full folder name from workflow/workstreams/."
        ),
        _ => bail!("Workstream reference is ambiguous: {workstream_ref}"),
    }
}

pub(crate) fn list_workstreams(repo_root: &Path) -> Result<Vec<PathBuf>> {
    let workstreams_dir = repo_root.join("workflow").join("workstreams");
    let mut entries = Vec::new();

    for entry in fs::read_dir(&workstreams_dir)
        .with_context(|| format!("Failed to read {}", workstreams_dir.display()))?
    {
        let entry =
            entry.with_context(|| format!("Failed to inspect {}", workstreams_dir.display()))?;
        if entry
            .file_type()
            .with_context(|| format!("Failed to inspect {}", entry.path().display()))?
            .is_dir()
        {
            entries.push(entry.path());
        }
    }

    entries.sort_by_key(|path| {
        let name = path.file_name().and_then(|v| v.to_str()).unwrap_or_default();
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
        let content =
            fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
        let (frontmatter, body) = split_frontmatter(&content)?;
        let mapping: Mapping =
            serde_yaml::from_str(frontmatter).with_context(|| format!("Invalid frontmatter in {}", path.display()))?;

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
                    let next = value
                        .as_str()
                        .ok_or_else(|| anyhow!("`status` must be a string in {}", path.display()))?;
                    validate_status(next)?;
                    status = Some(next.to_owned());
                }
                "summary" => {
                    let next = value
                        .as_str()
                        .ok_or_else(|| anyhow!("`summary` must be a string in {}", path.display()))?;
                    summary = Some(next.to_owned());
                }
                "updated" => {
                    let next = value
                        .as_str()
                        .ok_or_else(|| anyhow!("`updated` must be a string in {}", path.display()))?;
                    updated = Some(next.to_owned());
                }
                "prs" => {
                    let sequence = value
                        .as_sequence()
                        .ok_or_else(|| anyhow!("`prs` must be a list in {}", path.display()))?;
                    let mut numbers = Vec::new();
                    for value in sequence {
                        numbers.push(
                            value
                                .as_u64()
                                .ok_or_else(|| anyhow!("`prs` entries must be numbers in {}", path.display()))?,
                        );
                    }
                    prs = Some(numbers);
                }
                _ => {
                    extra.insert(key, value);
                }
            }
        }

        Ok(Self {
            status: status.ok_or_else(|| anyhow!("Missing required `status` field in {}", path.display()))?,
            summary: summary.ok_or_else(|| anyhow!("Missing required `summary` field in {}", path.display()))?,
            updated: updated.ok_or_else(|| anyhow!("Missing required `updated` field in {}", path.display()))?,
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
        mapping.insert(Value::String("status".to_owned()), Value::String(self.status.clone()));
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
        .ok_or_else(|| anyhow!("STATUS.md must start with frontmatter"))?;
    let end = remainder
        .find(FRONTMATTER_END)
        .ok_or_else(|| anyhow!("STATUS.md frontmatter must end with `---`"))?;
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
