use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow, bail};

use crate::status::today_string;

const DECISIONS_README: &str = include_str!("../workflow/decisions/README.md");
const WORKSTREAM_DECISIONS_README: &str =
    include_str!("../resources/skills/mnemix-workflow/assets/workstream/decisions/README.md");
const STATUS_TEMPLATE: &str =
    include_str!("../resources/skills/mnemix-workflow/assets/workstream/STATUS.md");
const PATCH_TEMPLATE: &str = include_str!("../resources/skills/mnemix-workflow/assets/patch.md");
const SPEC_TEMPLATE: &str =
    include_str!("../resources/skills/mnemix-workflow/assets/workstream/spec.md");
const UX_TEMPLATE: &str =
    include_str!("../resources/skills/mnemix-workflow/assets/workstream/ux.md");
const PLAN_TEMPLATE: &str =
    include_str!("../resources/skills/mnemix-workflow/assets/workstream/plan.md");
const TASKS_TEMPLATE: &str =
    include_str!("../resources/skills/mnemix-workflow/assets/workstream/tasks.md");

pub(crate) fn find_repo_root(start: &Path) -> Result<PathBuf> {
    for candidate in std::iter::once(start).chain(start.ancestors().skip(1)) {
        if candidate.join(".git").exists() {
            return Ok(candidate.to_path_buf());
        }
    }

    bail!(
        "Repository root not found from the current working directory. Run this command from inside a git repository or worktree."
    );
}

pub(crate) fn ensure_initialized(repo_root: &Path, program: &str) -> Result<()> {
    let workstreams_dir = repo_root.join("workflow").join("workstreams");
    let decisions_readme = repo_root
        .join("workflow")
        .join("decisions")
        .join("README.md");

    if workstreams_dir.is_dir() && decisions_readme.is_file() {
        return Ok(());
    }

    bail!("Repository is not initialized for Mnemix Workflow. Run `{program} init` first.");
}

pub(crate) fn init_repository(repo_root: &Path) -> Result<bool> {
    let workflow_dir = repo_root.join("workflow");
    let decisions_dir = workflow_dir.join("decisions");
    let workstreams_dir = workflow_dir.join("workstreams");
    let patches_dir = workflow_dir.join("patches");
    let decisions_readme = decisions_dir.join("README.md");
    let workflow_exists = workflow_dir.exists();
    let decisions_dir_exists = decisions_dir.exists();
    let workstreams_dir_exists = workstreams_dir.exists();
    let patches_dir_exists = patches_dir.exists();
    let decisions_readme_exists = decisions_readme.exists();

    fs::create_dir_all(&decisions_dir)
        .with_context(|| format!("Failed to create {}", decisions_dir.display()))?;
    fs::create_dir_all(&workstreams_dir)
        .with_context(|| format!("Failed to create {}", workstreams_dir.display()))?;
    fs::create_dir_all(&patches_dir)
        .with_context(|| format!("Failed to create {}", patches_dir.display()))?;

    let mut created = false;
    if !decisions_readme_exists {
        fs::write(&decisions_readme, DECISIONS_README)
            .with_context(|| format!("Failed to write {}", decisions_readme.display()))?;
        created = true;
    }

    Ok(created
        || !workflow_exists
        || !decisions_dir_exists
        || !workstreams_dir_exists
        || !patches_dir_exists)
}

pub(crate) fn create_workstream(repo_root: &Path, name: &str) -> Result<PathBuf> {
    let slug = slugify(name);
    if slug.is_empty() {
        bail!("Name must contain at least one letter or digit.");
    }

    let workstreams_dir = repo_root.join("workflow").join("workstreams");
    let numeric_id = next_id(&workstreams_dir)?;
    let formatted_id = format_id(numeric_id, 3);
    let title = titleize(name);
    let folder_name = format!("{formatted_id}-{slug}");
    let destination = workstreams_dir.join(folder_name);
    let today = today_string();

    if destination.exists() {
        bail!("Workstream already exists: {}", destination.display());
    }

    let substitutions = [
        ("{{WORKSTREAM_ID}}", formatted_id.as_str()),
        ("{{WORKSTREAM_SLUG}}", slug.as_str()),
        ("{{WORKSTREAM_TITLE}}", title.as_str()),
        ("{{TODAY}}", today.as_str()),
    ];

    fs::create_dir_all(destination.join("decisions"))
        .with_context(|| format!("Failed to create {}", destination.display()))?;

    write_template(
        &destination.join("STATUS.md"),
        STATUS_TEMPLATE,
        &substitutions,
    )?;
    write_template(&destination.join("spec.md"), SPEC_TEMPLATE, &substitutions)?;
    write_template(&destination.join("ux.md"), UX_TEMPLATE, &substitutions)?;
    write_template(&destination.join("plan.md"), PLAN_TEMPLATE, &substitutions)?;
    write_template(
        &destination.join("tasks.md"),
        TASKS_TEMPLATE,
        &substitutions,
    )?;
    write_template(
        &destination.join("decisions").join("README.md"),
        WORKSTREAM_DECISIONS_README,
        &substitutions,
    )?;

    destination
        .strip_prefix(repo_root)
        .map(PathBuf::from)
        .map_err(|_| anyhow!("Failed to build a repository-relative workstream path"))
}

pub(crate) fn create_patch(repo_root: &Path, name: &str) -> Result<PathBuf> {
    let slug = slugify(name);
    if slug.is_empty() {
        bail!("Name must contain at least one letter or digit.");
    }

    let patches_dir = repo_root.join("workflow").join("patches");
    fs::create_dir_all(&patches_dir)
        .with_context(|| format!("Failed to create {}", patches_dir.display()))?;

    let numeric_id = next_id(&patches_dir)?;
    let formatted_id = format_id(numeric_id, 4);
    let title = titleize(name);
    let file_name = format!("{formatted_id}-{slug}.md");
    let destination = patches_dir.join(file_name);
    let today = today_string();

    if destination.exists() {
        bail!("Patch already exists: {}", destination.display());
    }

    let substitutions = [
        ("{{PATCH_ID}}", formatted_id.as_str()),
        ("{{PATCH_SLUG}}", slug.as_str()),
        ("{{PATCH_TITLE}}", title.as_str()),
        ("{{TODAY}}", today.as_str()),
    ];

    write_template(&destination, PATCH_TEMPLATE, &substitutions)?;

    destination
        .strip_prefix(repo_root)
        .map(PathBuf::from)
        .map_err(|_| anyhow!("Failed to build a repository-relative patch path"))
}

fn write_template(path: &Path, template: &str, substitutions: &[(&str, &str)]) -> Result<()> {
    let mut content = template.to_owned();
    for (key, value) in substitutions {
        content = content.replace(key, value);
    }

    fs::write(path, content).with_context(|| format!("Failed to write {}", path.display()))
}

fn next_id(items_dir: &Path) -> Result<u32> {
    let mut highest = 0;

    if !items_dir.exists() {
        return Ok(1);
    }

    for entry in fs::read_dir(items_dir)
        .with_context(|| format!("Failed to read {}", items_dir.display()))?
    {
        let entry = entry.with_context(|| format!("Failed to inspect {}", items_dir.display()))?;
        let file_type = entry.file_type().with_context(|| {
            format!("Failed to inspect file type for {}", entry.path().display())
        })?;
        if !file_type.is_dir() && !file_type.is_file() {
            continue;
        }

        if let Some(prefix) = parse_prefix(&entry.file_name().to_string_lossy()) {
            highest = highest.max(prefix);
        }
    }

    Ok(highest + 1)
}

fn parse_prefix(name: &str) -> Option<u32> {
    let prefix = name.split_once('-')?.0;
    prefix.parse().ok()
}

fn format_id(value: u32, width: usize) -> String {
    let max_padded = 10u32.saturating_pow(width as u32).saturating_sub(1);
    if value <= max_padded {
        format!("{value:0width$}")
    } else {
        value.to_string()
    }
}

pub(crate) fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;

    for character in value.trim().chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            slug.push(character);
            last_was_dash = false;
        } else if !slug.is_empty() && !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    slug.trim_matches('-').to_owned()
}

fn titleize(value: &str) -> String {
    value
        .split(|character: char| character == '-' || character == '_' || character.is_whitespace())
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut characters = segment.chars();
            match characters.next() {
                Some(first) => first.to_uppercase().chain(characters).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::{format_id, parse_prefix, slugify, titleize};

    #[test]
    fn slugify_normalizes_names() {
        assert_eq!(slugify("User Profile Redesign"), "user-profile-redesign");
        assert_eq!(slugify("!!!"), "");
    }

    #[test]
    fn titleize_preserves_words() {
        assert_eq!(titleize("user-profile_redesign"), "User Profile Redesign");
    }

    #[test]
    fn parse_prefix_reads_numeric_prefixes() {
        assert_eq!(parse_prefix("001-first-workstream"), Some(1));
        assert_eq!(parse_prefix("abc-first-workstream"), None);
    }

    #[test]
    fn format_id_zero_pads_using_requested_width() {
        assert_eq!(format_id(1, 3), "001");
        assert_eq!(format_id(999, 3), "999");
        assert_eq!(format_id(1000, 3), "1000");
        assert_eq!(format_id(1, 4), "0001");
        assert_eq!(format_id(9999, 4), "9999");
        assert_eq!(format_id(10000, 4), "10000");
    }
}
