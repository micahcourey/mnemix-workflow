use std::path::Path;

use anyhow::{Result, bail};

use crate::{
    cli::{StatusAction, StatusArgs, StatusListArgs},
    scaffold::{ensure_initialized, find_repo_root},
    status::{StatusFile, list_workstreams, resolve_workstream_path, validate_status},
};

pub(crate) fn run(cwd: &Path, program: &str, args: StatusArgs) -> Result<Vec<String>> {
    let repo_root = find_repo_root(cwd)?;
    ensure_initialized(&repo_root, program)?;

    match (args.workstream, args.action) {
        (Some(workstream), None) => show_status(&repo_root, &workstream),
        (None, Some(StatusAction::Set(args))) => set_status(&repo_root, &args.workstream, &args.status, args.summary.as_deref(), &args.prs, args.clear_prs),
        (None, Some(StatusAction::List(args))) => list_status(&repo_root, args),
        _ => bail!(
            "Usage:\n  {program} status <workstream>\n  {program} status set <workstream> <status> [--summary <text>] [--pr <number>]... [--clear-prs]\n  {program} status list [--status <value>]"
        ),
    }
}

fn list_status(repo_root: &Path, args: StatusListArgs) -> Result<Vec<String>> {
    if let Some(status) = args.status.as_deref() {
        validate_status(status)?;
    }

    let mut lines = Vec::new();
    for workstream_dir in list_workstreams(repo_root)? {
        let status_path = workstream_dir.join("STATUS.md");
        if !status_path.is_file() {
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

        let workstream_name = workstream_dir
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("<unknown>");
        let mut line = format!(
            "{workstream_name}: {} | {} | {}",
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
            return Ok(vec![format!("No workstreams found with status `{status}`.")]);
        }
        return Ok(vec!["No workstreams found.".to_owned()]);
    }

    Ok(lines)
}

fn show_status(repo_root: &Path, workstream_ref: &str) -> Result<Vec<String>> {
    let workstream_dir = resolve_workstream_path(repo_root, workstream_ref)?;
    let status_path = workstream_dir.join("STATUS.md");
    let status = StatusFile::read(&status_path)?;
    let workstream_name = workstream_dir
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or(workstream_ref);

    let mut lines = vec![
        format!("Workstream: {workstream_name}"),
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
    workstream_ref: &str,
    next_status: &str,
    summary: Option<&str>,
    prs: &[u64],
    clear_prs: bool,
) -> Result<Vec<String>> {
    let workstream_dir = resolve_workstream_path(repo_root, workstream_ref)?;
    let status_path = workstream_dir.join("STATUS.md");
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

    let workstream_name = workstream_dir
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or(workstream_ref);

    let mut lines = vec![
        format!("Updated status for workstream: {workstream_name}"),
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
