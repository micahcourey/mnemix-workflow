use std::path::Path;

use anyhow::{Result, bail};

use crate::{
    cli::{StatusAction, StatusArgs, StatusListArgs},
    scaffold::{ensure_initialized, find_repo_root},
    status::{StatusFile, TrackedKind, list_tracked_items, resolve_tracked_path, validate_status},
};

pub(crate) fn run(cwd: &Path, program: &str, args: StatusArgs) -> Result<Vec<String>> {
    run_for_kind(
        cwd,
        program,
        args.workstream,
        args.action,
        TrackedKind::Workstream,
    )
}

pub(crate) fn run_for_kind(
    cwd: &Path,
    program: &str,
    item_ref: Option<String>,
    action: Option<StatusAction>,
    kind: TrackedKind,
) -> Result<Vec<String>> {
    let repo_root = find_repo_root(cwd)?;
    ensure_initialized(&repo_root, program)?;

    match (item_ref, action) {
        (Some(item), None) => show_status(&repo_root, &item, kind),
        (None, Some(StatusAction::Set(args))) => set_status(
            &repo_root,
            &args.workstream,
            &args.status,
            args.summary.as_deref(),
            &args.prs,
            args.clear_prs,
            kind,
        ),
        (None, Some(StatusAction::List(args))) => list_status(&repo_root, args, kind),
        _ => bail!(
            "Usage:\n  {program} {}status <{}>\n  {program} {}status set <{}> <status> [--summary <text>] [--pr <number>]... [--clear-prs]\n  {program} {}status list [--status <value>]",
            kind_prefix(kind),
            kind.singular(),
            kind_prefix(kind),
            kind.singular(),
            kind_prefix(kind),
        ),
    }
}

fn list_status(repo_root: &Path, args: StatusListArgs, kind: TrackedKind) -> Result<Vec<String>> {
    if let Some(status) = args.status.as_deref() {
        validate_status(status)?;
    }

    let mut lines = Vec::new();
    for item_path in list_tracked_items(repo_root, kind)? {
        let status_path = kind.status_path(&item_path);
        if !status_path.exists() {
            continue;
        }
        let status = StatusFile::read(&status_path)?;
        if args
            .status
            .as_deref()
            .is_some_and(|value| status.status() != value)
        {
            continue;
        }

        let item_name = kind.display_name(&item_path);
        let mut line = format!(
            "{item_name}: {} | {} | {}",
            status.status(),
            status.updated(),
            status.summary()
        );
        if let Some(prs) = status.prs() {
            if !prs.is_empty() {
                line.push_str(" | PRs: ");
                line.push_str(
                    &prs.iter()
                        .map(u64::to_string)
                        .collect::<Vec<_>>()
                        .join(", "),
                );
            }
        }
        lines.push(line);
    }

    if lines.is_empty() {
        if let Some(status) = args.status {
            return Ok(vec![format!(
                "No {} found with status `{status}`.",
                kind.plural()
            )]);
        }
        return Ok(vec![format!("No {} found.", kind.plural())]);
    }

    Ok(lines)
}

fn show_status(repo_root: &Path, item_ref: &str, kind: TrackedKind) -> Result<Vec<String>> {
    let item_path = resolve_tracked_path(repo_root, item_ref, kind)?;
    let status_path = kind.status_path(&item_path);
    let status = StatusFile::read(&status_path)?;
    let item_name = kind.display_name(&item_path);

    let mut lines = vec![
        format!("{}: {item_name}", capitalize(kind.singular())),
        format!("Status: {}", status.status()),
        format!("Summary: {}", status.summary()),
        format!("Updated: {}", status.updated()),
    ];

    if let Some(prs) = status.prs() {
        if !prs.is_empty() {
            lines.push(format!(
                "PRs: {}",
                prs.iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }

    Ok(lines)
}

fn set_status(
    repo_root: &Path,
    item_ref: &str,
    next_status: &str,
    summary: Option<&str>,
    prs: &[u64],
    clear_prs: bool,
    kind: TrackedKind,
) -> Result<Vec<String>> {
    let item_path = resolve_tracked_path(repo_root, item_ref, kind)?;
    let status_path = kind.status_path(&item_path);
    let mut status = StatusFile::read(&status_path)?;

    status.set_status(next_status)?;
    if let Some(summary) = summary {
        status.set_summary(summary);
    }
    if clear_prs {
        status.clear_prs();
    } else if !prs.is_empty() {
        status.set_prs(prs.to_vec());
    }
    status.touch_updated();
    status.write(&status_path)?;

    let item_name = kind.display_name(&item_path);

    let mut lines = vec![
        format!("Updated status for {}: {item_name}", kind.singular()),
        format!("Status: {}", status.status()),
        format!("Summary: {}", status.summary()),
        format!("Updated: {}", status.updated()),
    ];

    if let Some(prs) = status.prs() {
        if !prs.is_empty() {
            lines.push(format!(
                "PRs: {}",
                prs.iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }

    Ok(lines)
}

fn kind_prefix(kind: TrackedKind) -> &'static str {
    match kind {
        TrackedKind::Workstream => "",
        TrackedKind::Patch => "patch ",
    }
}

fn capitalize(value: &str) -> String {
    let mut characters = value.chars();
    match characters.next() {
        Some(first) => first.to_uppercase().chain(characters).collect(),
        None => String::new(),
    }
}
