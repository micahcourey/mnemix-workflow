use std::path::Path;

use anyhow::{Result, bail};

use crate::{
    cli::ValidateArgs,
    contracts::{validate_asyncapi, validate_openapi, validate_schema},
    scaffold::{ensure_initialized, find_repo_root},
    status::{StatusFile, TrackedKind, list_tracked_items, resolve_tracked_path},
};

pub(crate) fn run(cwd: &Path, program: &str, args: ValidateArgs) -> Result<Vec<String>> {
    let repo_root = find_repo_root(cwd)?;
    ensure_initialized(&repo_root, program)?;

    match args.target {
        Some(target) => validate_target(&repo_root, &target),
        None => validate_repository(&repo_root),
    }
}

fn validate_repository(repo_root: &Path) -> Result<Vec<String>> {
    let workstreams = list_tracked_items(repo_root, TrackedKind::Workstream)?;
    let patches = list_tracked_items(repo_root, TrackedKind::Patch)?;
    let mut contract_count = 0usize;

    for workstream in &workstreams {
        validate_workstream(repo_root, workstream, &mut contract_count)?;
    }
    for patch in &patches {
        validate_patch(patch)?;
    }

    Ok(vec![
        "Repository validation passed.".to_owned(),
        format!("Workstreams checked: {}", workstreams.len()),
        format!("Patches checked: {}", patches.len()),
        format!("Contracts checked: {contract_count}"),
    ])
}

fn validate_target(repo_root: &Path, target: &str) -> Result<Vec<String>> {
    if let Ok(workstream) = resolve_tracked_path(repo_root, target, TrackedKind::Workstream) {
        let mut contract_count = 0usize;
        validate_workstream(repo_root, &workstream, &mut contract_count)?;
        return Ok(vec![
            format!(
                "Validated workstream: {}",
                TrackedKind::Workstream.display_name(&workstream)
            ),
            "Status metadata: OK".to_owned(),
            format!("Contracts checked: {contract_count}"),
        ]);
    }

    if let Ok(patch) = resolve_tracked_path(repo_root, target, TrackedKind::Patch) {
        validate_patch(&patch)?;
        return Ok(vec![
            format!(
                "Validated patch: {}",
                TrackedKind::Patch.display_name(&patch)
            ),
            "Status metadata: OK".to_owned(),
        ]);
    }

    bail!(
        "Tracked item not found: {target}. Use a workstream id/name or patch id/name from workflow/workstreams or workflow/patches."
    );
}

fn validate_workstream(
    repo_root: &Path,
    workstream_path: &Path,
    contract_count: &mut usize,
) -> Result<()> {
    let status_path = TrackedKind::Workstream.status_path(workstream_path);
    StatusFile::read(&status_path)?;

    let openapi_path = workstream_path.join("contracts").join("openapi.yaml");
    if openapi_path.is_file() {
        validate_openapi(repo_root, &path_arg(repo_root, &openapi_path))?;
        *contract_count += 1;
    }

    let asyncapi_path = workstream_path.join("contracts").join("asyncapi.yaml");
    if asyncapi_path.is_file() {
        validate_asyncapi(repo_root, &path_arg(repo_root, &asyncapi_path))?;
        *contract_count += 1;
    }

    let schemas_dir = workstream_path.join("contracts").join("schemas");
    if schemas_dir.is_dir() {
        let validated = validate_schema(repo_root, &path_arg(repo_root, &schemas_dir))?;
        *contract_count += validated.len();
    }

    Ok(())
}

fn validate_patch(patch_path: &Path) -> Result<()> {
    StatusFile::read(patch_path)?;
    Ok(())
}

fn path_arg(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}
