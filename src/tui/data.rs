use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::status::{StatusFile, TrackedKind, list_tracked_items};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ArtifactKind {
    Status,
    Spec,
    Ux,
    Plan,
    Tasks,
    Patch,
}

impl ArtifactKind {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Status => "STATUS",
            Self::Spec => "SPEC",
            Self::Ux => "UX",
            Self::Plan => "PLAN",
            Self::Tasks => "TASKS",
            Self::Patch => "PATCH",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct TrackedItem {
    pub(crate) kind: TrackedKind,
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) status: String,
    pub(crate) summary: String,
    pub(crate) updated: String,
    pub(crate) prs: Vec<u64>,
}

impl TrackedItem {
    pub(crate) fn kind_label(&self) -> &'static str {
        match self.kind {
            TrackedKind::Workstream => "W",
            TrackedKind::Patch => "P",
        }
    }

    pub(crate) fn artifact_kinds(&self) -> Vec<ArtifactKind> {
        match self.kind {
            TrackedKind::Workstream => vec![
                ArtifactKind::Status,
                ArtifactKind::Spec,
                ArtifactKind::Ux,
                ArtifactKind::Plan,
                ArtifactKind::Tasks,
            ],
            TrackedKind::Patch => vec![ArtifactKind::Patch],
        }
    }

    pub(crate) fn artifact_path(&self, artifact: ArtifactKind) -> PathBuf {
        match (self.kind, artifact) {
            (TrackedKind::Workstream, ArtifactKind::Status) => self.path.join("STATUS.md"),
            (TrackedKind::Workstream, ArtifactKind::Spec) => self.path.join("spec.md"),
            (TrackedKind::Workstream, ArtifactKind::Ux) => self.path.join("ux.md"),
            (TrackedKind::Workstream, ArtifactKind::Plan) => self.path.join("plan.md"),
            (TrackedKind::Workstream, ArtifactKind::Tasks) => self.path.join("tasks.md"),
            (TrackedKind::Patch, ArtifactKind::Patch) => self.path.clone(),
            _ => self.path.clone(),
        }
    }
}

pub(crate) fn load_items(repo_root: &Path) -> Result<Vec<TrackedItem>> {
    let mut items = Vec::new();
    for kind in [TrackedKind::Workstream, TrackedKind::Patch] {
        for item_path in list_tracked_items(repo_root, kind)? {
            let status_path = kind.status_path(&item_path);
            if !status_path.exists() {
                continue;
            }
            let status = StatusFile::read(&status_path)?;
            items.push(TrackedItem {
                kind,
                name: kind.display_name(&item_path),
                path: item_path,
                status: status.status().to_owned(),
                summary: status.summary().to_owned(),
                updated: status.updated().to_owned(),
                prs: status.prs().unwrap_or(&[]).to_vec(),
            });
        }
    }
    items.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(items)
}

pub(crate) fn read_artifact(item: &TrackedItem, artifact: ArtifactKind) -> String {
    let path = item.artifact_path(artifact);
    match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) => format!(
            "Missing artifact: {}\n\n{}",
            path.display(),
            format_read_error(&error).trim()
        ),
    }
}

fn format_read_error(error: &std::io::Error) -> String {
    format!("Failed to read artifact: {error}")
}

pub(crate) fn preview_title(item: &TrackedItem, artifact: ArtifactKind) -> String {
    match item.kind {
        TrackedKind::Workstream => {
            let labels = item
                .artifact_kinds()
                .into_iter()
                .map(|candidate| {
                    if candidate == artifact {
                        format!("[{}]", candidate.label())
                    } else {
                        candidate.label().to_owned()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            format!("{} · {}", item.name, labels)
        }
        TrackedKind::Patch => format!("{} · PATCH", item.name),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::scaffold::{create_patch, create_workstream, init_repository};
    use crate::status::TrackedKind;

    use super::{ArtifactKind, load_items, read_artifact};

    #[test]
    fn load_items_reads_workstreams_and_patches() {
        let temp = tempfile::TempDir::new().expect("tempdir");
        fs::create_dir(temp.path().join(".git")).expect("git dir");
        init_repository(temp.path()).expect("init repo");
        create_workstream(temp.path(), "user profile redesign").expect("workstream");
        create_patch(temp.path(), "fix status copy").expect("patch");

        let items = load_items(temp.path()).expect("load items");
        assert_eq!(items.len(), 2);
        assert!(
            items
                .iter()
                .any(|item| item.kind == TrackedKind::Workstream)
        );
        assert!(items.iter().any(|item| item.kind == TrackedKind::Patch));
    }

    #[test]
    fn read_artifact_returns_patch_contents() {
        let temp = tempfile::TempDir::new().expect("tempdir");
        fs::create_dir(temp.path().join(".git")).expect("git dir");
        init_repository(temp.path()).expect("init repo");
        let patch_path = create_patch(temp.path(), "fix status copy").expect("patch");
        let items = load_items(temp.path()).expect("load items");
        let patch = items
            .iter()
            .find(|item| item.path.ends_with(patch_path.as_path()))
            .expect("patch item");

        let content = read_artifact(patch, ArtifactKind::Patch);
        assert!(content.contains("# Patch: Fix Status Copy"));
    }
}
