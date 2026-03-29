use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, anyhow, bail};

const PRE_COMMIT_HOOK: &str = include_str!("../resources/hooks/pre-commit-status-updated");
const PRE_PUSH_HOOK: &str = include_str!("../resources/hooks/pre-push-status-reminder");

pub(crate) struct InstalledHook {
    pub(crate) path: PathBuf,
    pub(crate) installed: bool,
}

pub(crate) fn install_hooks(repo_root: &Path, force: bool) -> Result<Vec<InstalledHook>> {
    let hooks_dir = git_hooks_dir(repo_root)?;
    fs::create_dir_all(&hooks_dir)
        .with_context(|| format!("Failed to create {}", hooks_dir.display()))?;

    Ok(vec![
        install_hook(&hooks_dir.join("pre-commit"), PRE_COMMIT_HOOK, force)?,
        install_hook(&hooks_dir.join("pre-push"), PRE_PUSH_HOOK, force)?,
    ])
}

fn git_hooks_dir(repo_root: &Path) -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-path", "hooks"])
        .current_dir(repo_root)
        .output()
        .with_context(|| format!("Failed to run git from {}", repo_root.display()))?;

    if !output.status.success() {
        bail!(
            "Failed to locate the git hooks directory: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    let hooks_path = String::from_utf8(output.stdout)
        .map_err(|_| anyhow!("Git returned a non-UTF-8 hooks path."))?;
    let hooks_path = hooks_path.trim();
    let hooks_path = PathBuf::from(hooks_path);
    if hooks_path.is_absolute() {
        Ok(hooks_path)
    } else {
        Ok(repo_root.join(hooks_path))
    }
}

fn install_hook(destination: &Path, content: &str, force: bool) -> Result<InstalledHook> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }

    if destination.exists() {
        let existing = fs::read_to_string(destination)
            .with_context(|| format!("Failed to read {}", destination.display()))?;
        if existing == content {
            return Ok(InstalledHook {
                path: destination.to_path_buf(),
                installed: false,
            });
        }
        if !force {
            bail!(
                "Hook already exists and differs: {}. Re-run with `--force` to overwrite it.",
                destination.display()
            );
        }
    }

    fs::write(destination, content)
        .with_context(|| format!("Failed to write {}", destination.display()))?;
    set_executable(destination)?;
    Ok(InstalledHook {
        path: destination.to_path_buf(),
        installed: true,
    })
}

#[cfg(unix)]
fn set_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let metadata =
        fs::metadata(path).with_context(|| format!("Failed to stat {}", path.display()))?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)
        .with_context(|| format!("Failed to chmod {}", path.display()))
}

#[cfg(not(unix))]
fn set_executable(_path: &Path) -> Result<()> {
    Ok(())
}
