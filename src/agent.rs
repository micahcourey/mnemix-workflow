use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};

use crate::cli::AssistantTool;

struct CommandTemplate {
    name: &'static str,
    content: &'static str,
}

const COMMAND_TEMPLATES: &[CommandTemplate] = &[
    CommandTemplate {
        name: "explore",
        content: include_str!("../resources/commands/explore.md"),
    },
    CommandTemplate {
        name: "track",
        content: include_str!("../resources/commands/track.md"),
    },
    CommandTemplate {
        name: "implement",
        content: include_str!("../resources/commands/implement.md"),
    },
    CommandTemplate {
        name: "close",
        content: include_str!("../resources/commands/close.md"),
    },
    CommandTemplate {
        name: "sync",
        content: include_str!("../resources/commands/sync.md"),
    },
    CommandTemplate {
        name: "status",
        content: include_str!("../resources/commands/status.md"),
    },
];

pub(crate) fn install(
    repo_root: &Path,
    selected_tools: &[AssistantTool],
    overwrite: bool,
) -> Result<Vec<String>> {
    let tools = normalized_tools(selected_tools);
    let planned_writes = planned_writes(repo_root, &tools)?;

    if !overwrite {
        let conflicts = planned_writes
            .iter()
            .filter(|write| write.path.exists() && existing_differs(write))
            .map(|write| repo_relative(repo_root, &write.path))
            .collect::<Vec<_>>();
        if !conflicts.is_empty() {
            bail!(
                "Refusing to overwrite existing assistant command files:\n{}\nRun `mxw agent update` to refresh them.",
                conflicts
                    .into_iter()
                    .map(|path| format!("  {path}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    }

    let mut lines = Vec::new();
    lines.push(if overwrite {
        "Updated assistant slash commands.".to_owned()
    } else {
        "Installed assistant slash commands.".to_owned()
    });

    for tool in &tools {
        let commands_dir = tool.commands_dir(repo_root);
        fs::create_dir_all(&commands_dir)
            .with_context(|| format!("Failed to create {}", commands_dir.display()))?;
        lines.push(format!(
            "{} commands: {}",
            tool.display_name(),
            repo_relative(repo_root, &commands_dir)
        ));

        for template in COMMAND_TEMPLATES {
            let path = commands_dir.join(format!("{}.md", template.name));
            let action = if path.exists() {
                if existing_matches(&path, template.content) {
                    "Unchanged"
                } else {
                    "Updated"
                }
            } else {
                "Created"
            };

            fs::write(&path, template.content)
                .with_context(|| format!("Failed to write {}", path.display()))?;
            lines.push(format!(
                "  {action}: {}",
                repo_relative(repo_root, &path)
            ));
        }
    }

    lines.push(format!(
        "Available slash commands: {}",
        COMMAND_TEMPLATES
            .iter()
            .map(|template| format!("/mxw:{}", template.name))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    lines.push(
        "These integrations are repo-local and keep `mxw` as the authoritative workflow engine."
            .to_owned(),
    );

    Ok(lines)
}

pub(crate) fn list_supported_tools(repo_root: &Path) -> Result<Vec<String>> {
    let tools = normalized_tools(&[]);
    let mut lines = vec!["Supported assistant integrations:".to_owned()];

    for tool in tools {
        lines.push(format!(
            "- {} (`{}`): {}",
            tool.display_name(),
            tool.as_str(),
            repo_relative(repo_root, &tool.commands_dir(repo_root))
        ));
    }

    lines.push(
        "Each tool receives the same namespaced command set: /mxw:explore, /mxw:track, /mxw:implement, /mxw:close, /mxw:sync, /mxw:status."
            .to_owned(),
    );

    Ok(lines)
}

struct PlannedWrite {
    path: PathBuf,
    content: &'static str,
}

fn planned_writes(repo_root: &Path, tools: &[AssistantTool]) -> Result<Vec<PlannedWrite>> {
    let mut writes = Vec::new();
    for tool in tools {
        let commands_dir = tool.commands_dir(repo_root);
        for template in COMMAND_TEMPLATES {
            writes.push(PlannedWrite {
                path: commands_dir.join(format!("{}.md", template.name)),
                content: template.content,
            });
        }
    }

    Ok(writes)
}

fn normalized_tools(selected_tools: &[AssistantTool]) -> Vec<AssistantTool> {
    let tools = if selected_tools.is_empty() {
        vec![AssistantTool::Claude, AssistantTool::Cursor]
    } else {
        selected_tools.to_vec()
    };

    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();
    for tool in tools {
        if seen.insert(tool.as_str()) {
            deduped.push(tool);
        }
    }
    deduped
}

fn existing_matches(path: &Path, content: &str) -> bool {
    fs::read_to_string(path).is_ok_and(|existing| existing == content)
}

fn existing_differs(write: &PlannedWrite) -> bool {
    !existing_matches(&write.path, write.content)
}

fn repo_relative(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .map(|value| value.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}

impl AssistantTool {
    fn commands_dir(self, repo_root: &Path) -> PathBuf {
        match self {
            Self::Claude => repo_root.join(".claude").join("commands").join("mxw"),
            Self::Cursor => repo_root.join(".cursor").join("commands").join("mxw"),
        }
    }
}
