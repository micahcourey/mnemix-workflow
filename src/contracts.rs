use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow, bail};
use serde_json::Value as JsonValue;
use serde_yaml::{Mapping, Value};

use crate::{
    scaffold::slugify,
    status::{TrackedKind, resolve_tracked_path},
};

const OPENAPI_TEMPLATE: &str =
    include_str!("../resources/skills/mnemix-workflow/assets/openapi.yaml");
const ASYNCAPI_TEMPLATE: &str =
    include_str!("../resources/skills/mnemix-workflow/assets/asyncapi.yaml");
const JSON_SCHEMA_TEMPLATE: &str =
    include_str!("../resources/skills/mnemix-workflow/assets/schema.json");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ContractStandard {
    OpenApi,
    AsyncApi,
}

impl ContractStandard {
    pub(crate) fn display_name(self) -> &'static str {
        match self {
            Self::OpenApi => "OpenAPI",
            Self::AsyncApi => "AsyncAPI",
        }
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::OpenApi => "openapi.yaml",
            Self::AsyncApi => "asyncapi.yaml",
        }
    }
}

pub(crate) fn scaffold_openapi(repo_root: &Path, workstream_ref: &str) -> Result<PathBuf> {
    let workstream_path = resolve_tracked_path(repo_root, workstream_ref, TrackedKind::Workstream)?;
    let title = workstream_title(&workstream_path);
    let slug = workstream_slug(&workstream_path);
    let destination = workstream_path.join("contracts").join("openapi.yaml");
    write_contract_template(
        repo_root,
        &destination,
        OPENAPI_TEMPLATE,
        &[
            ("{{WORKSTREAM_TITLE}}", title.as_str()),
            ("{{WORKSTREAM_SLUG}}", slug.as_str()),
        ],
    )
}

pub(crate) fn scaffold_asyncapi(repo_root: &Path, workstream_ref: &str) -> Result<PathBuf> {
    let workstream_path = resolve_tracked_path(repo_root, workstream_ref, TrackedKind::Workstream)?;
    let title = workstream_title(&workstream_path);
    let slug = workstream_slug(&workstream_path);
    let destination = workstream_path.join("contracts").join("asyncapi.yaml");
    write_contract_template(
        repo_root,
        &destination,
        ASYNCAPI_TEMPLATE,
        &[
            ("{{WORKSTREAM_TITLE}}", title.as_str()),
            ("{{WORKSTREAM_SLUG}}", slug.as_str()),
        ],
    )
}

pub(crate) fn scaffold_schema(
    repo_root: &Path,
    workstream_ref: &str,
    schema_name: &str,
) -> Result<PathBuf> {
    let workstream_path = resolve_tracked_path(repo_root, workstream_ref, TrackedKind::Workstream)?;
    let title = humanize_name(schema_name);
    let slug = slugify(schema_name);
    let workstream_slug = workstream_slug(&workstream_path);
    if slug.is_empty() {
        bail!("Schema name must contain at least one letter or digit.");
    }
    let destination = workstream_path
        .join("contracts")
        .join("schemas")
        .join(format!("{slug}.schema.json"));
    write_contract_template(
        repo_root,
        &destination,
        JSON_SCHEMA_TEMPLATE,
        &[
            ("{{SCHEMA_TITLE}}", title.as_str()),
            ("{{SCHEMA_SLUG}}", slug.as_str()),
            ("{{WORKSTREAM_SLUG}}", workstream_slug.as_str()),
        ],
    )
}

pub(crate) fn validate_openapi(repo_root: &Path, target: &str) -> Result<Vec<PathBuf>> {
    let path = resolve_contract_target(repo_root, target, ContractStandard::OpenApi)?;
    validate_openapi_file(&path)?;
    Ok(vec![relative_or_absolute(repo_root, &path)])
}

pub(crate) fn validate_asyncapi(repo_root: &Path, target: &str) -> Result<Vec<PathBuf>> {
    let path = resolve_contract_target(repo_root, target, ContractStandard::AsyncApi)?;
    validate_asyncapi_file(&path)?;
    Ok(vec![relative_or_absolute(repo_root, &path)])
}

pub(crate) fn validate_schema(repo_root: &Path, target: &str) -> Result<Vec<PathBuf>> {
    let explicit = resolve_existing_path(repo_root, target);
    let mut paths = if let Some(path) = explicit {
        if path.is_dir() {
            schema_paths_from_dir(&path)?
        } else {
            vec![path]
        }
    } else {
        let workstream_path = resolve_tracked_path(repo_root, target, TrackedKind::Workstream)?;
        schema_paths_from_dir(&workstream_path.join("contracts").join("schemas"))?
    };

    if paths.is_empty() {
        bail!("No JSON Schema files found for `{target}`.");
    }

    paths.sort();
    for path in &paths {
        validate_schema_file(path)?;
    }

    Ok(paths
        .into_iter()
        .map(|path| relative_or_absolute(repo_root, &path))
        .collect())
}

fn resolve_contract_target(
    repo_root: &Path,
    target: &str,
    standard: ContractStandard,
) -> Result<PathBuf> {
    if let Some(path) = resolve_existing_path(repo_root, target) {
        return Ok(path);
    }

    let workstream_path = resolve_tracked_path(repo_root, target, TrackedKind::Workstream)?;
    let contract_path = workstream_path.join("contracts").join(standard.file_name());
    if contract_path.is_file() {
        Ok(contract_path)
    } else {
        bail!(
            "{} contract not found for workstream `{target}`. Run `mxw {} init {target}` first.",
            standard.display_name(),
            standard_name(standard),
        )
    }
}

fn resolve_existing_path(repo_root: &Path, target: &str) -> Option<PathBuf> {
    let direct = Path::new(target);
    if direct.exists() {
        return Some(direct.to_path_buf());
    }

    let repo_relative = repo_root.join(target);
    if repo_relative.exists() {
        return Some(repo_relative);
    }

    None
}

fn schema_paths_from_dir(dir: &Path) -> Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut paths = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry.with_context(|| format!("Failed to inspect {}", dir.display()))?;
        if entry
            .file_type()
            .with_context(|| format!("Failed to inspect {}", entry.path().display()))?
            .is_file()
        {
            let path = entry.path();
            if path.extension().and_then(|v| v.to_str()) == Some("json") {
                paths.push(path);
            }
        }
    }

    Ok(paths)
}

fn validate_openapi_file(path: &Path) -> Result<()> {
    let document = read_yaml_mapping(path)?;
    let version = require_string(&document, "openapi", path)?;
    let info = require_mapping(&document, "info", path)?;
    require_string(info, "title", path)?;
    require_string(info, "version", path)?;
    require_mapping(&document, "paths", path)?;

    if !version.starts_with('3') {
        bail!(
            "Expected an OpenAPI 3.x document in {}, found `{version}`.",
            path.display()
        );
    }

    Ok(())
}

fn validate_asyncapi_file(path: &Path) -> Result<()> {
    let document = read_yaml_mapping(path)?;
    let version = require_string(&document, "asyncapi", path)?;
    let info = require_mapping(&document, "info", path)?;
    require_string(info, "title", path)?;
    require_string(info, "version", path)?;

    let has_channels = mapping_value(&document, "channels").is_some_and(Value::is_mapping);
    let has_operations = mapping_value(&document, "operations").is_some_and(Value::is_mapping);

    if !has_channels && !has_operations {
        bail!(
            "AsyncAPI document {} must include at least one of `channels` or `operations`.",
            path.display()
        );
    }

    if !version.starts_with('2') && !version.starts_with('3') {
        bail!(
            "Expected an AsyncAPI 2.x or 3.x document in {}, found `{version}`.",
            path.display()
        );
    }

    Ok(())
}

fn validate_schema_file(path: &Path) -> Result<()> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let document: JsonValue = serde_json::from_str(&content)
        .with_context(|| format!("Invalid JSON in {}", path.display()))?;

    match document {
        JsonValue::Bool(_) => Ok(()),
        JsonValue::Object(object) => {
            if let Some(schema) = object.get("$schema") {
                if !schema.is_string() {
                    bail!("`$schema` must be a string in {}.", path.display());
                }
            }
            if let Some(title) = object.get("title") {
                if !title.is_string() {
                    bail!("`title` must be a string in {}.", path.display());
                }
            }
            if let Some(ty) = object.get("type") {
                if !(ty.is_string() || ty.is_array()) {
                    bail!("`type` must be a string or array in {}.", path.display());
                }
            }
            Ok(())
        }
        _ => bail!(
            "JSON Schema documents must be a boolean or object at the top level in {}.",
            path.display()
        ),
    }
}

fn read_yaml_mapping(path: &Path) -> Result<Mapping> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let value: Value = serde_yaml::from_str(&content)
        .with_context(|| format!("Invalid YAML in {}", path.display()))?;
    value.as_mapping().cloned().ok_or_else(|| {
        anyhow!(
            "Expected a YAML mapping at the top level in {}",
            path.display()
        )
    })
}

fn require_string<'a>(mapping: &'a Mapping, key: &str, path: &Path) -> Result<&'a str> {
    mapping_value(mapping, key)
        .and_then(Value::as_str)
        .ok_or_else(|| {
            anyhow!(
                "Missing required string field `{key}` in {}",
                path.display()
            )
        })
}

fn require_mapping<'a>(mapping: &'a Mapping, key: &str, path: &Path) -> Result<&'a Mapping> {
    mapping_value(mapping, key)
        .and_then(Value::as_mapping)
        .ok_or_else(|| {
            anyhow!(
                "Missing required mapping field `{key}` in {}",
                path.display()
            )
        })
}

fn mapping_value<'a>(mapping: &'a Mapping, key: &str) -> Option<&'a Value> {
    mapping.get(Value::String(key.to_owned()))
}

fn write_contract_template(
    repo_root: &Path,
    destination: &Path,
    template: &str,
    substitutions: &[(&str, &str)],
) -> Result<PathBuf> {
    if destination.exists() {
        bail!("Contract already exists: {}", destination.display());
    }

    let parent = destination
        .parent()
        .ok_or_else(|| anyhow!("Missing parent directory for {}", destination.display()))?;
    fs::create_dir_all(parent).with_context(|| format!("Failed to create {}", parent.display()))?;

    let mut content = template.to_owned();
    for (key, value) in substitutions {
        content = content.replace(key, value);
    }

    fs::write(destination, content)
        .with_context(|| format!("Failed to write {}", destination.display()))?;
    Ok(relative_or_absolute(repo_root, destination))
}

fn workstream_title(path: &Path) -> String {
    humanize_name(&workstream_slug(path))
}

fn workstream_slug(path: &Path) -> String {
    path.file_name()
        .and_then(|value| value.to_str())
        .and_then(|value| value.split_once('-').map(|(_, slug)| slug.to_owned()))
        .unwrap_or_else(|| "workstream".to_owned())
}

fn humanize_name(value: &str) -> String {
    value
        .split(|character: char| character == '-' || character == '_' || character.is_whitespace())
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut characters = segment.chars();
            match characters.next() {
                Some(first) => first
                    .to_uppercase()
                    .chain(characters.flat_map(char::to_lowercase))
                    .collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn standard_name(standard: ContractStandard) -> &'static str {
    match standard {
        ContractStandard::OpenApi => "openapi",
        ContractStandard::AsyncApi => "asyncapi",
    }
}

fn relative_or_absolute(repo_root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(repo_root)
        .map(PathBuf::from)
        .unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::scaffold::{create_workstream, init_repository};

    use super::{
        ContractStandard, resolve_contract_target, scaffold_asyncapi, scaffold_openapi,
        scaffold_schema, validate_asyncapi, validate_asyncapi_file, validate_openapi,
        validate_openapi_file, validate_schema, validate_schema_file,
    };

    #[test]
    fn scaffold_openapi_creates_contract_file() {
        let temp = tempfile::TempDir::new().expect("tempdir");
        fs::create_dir(temp.path().join(".git")).expect("git dir");
        init_repository(temp.path()).expect("init repo");
        create_workstream(temp.path(), "api contracts").expect("workstream");

        let path = scaffold_openapi(temp.path(), "001").expect("openapi");
        assert_eq!(
            path,
            std::path::PathBuf::from(
                "workflow/workstreams/001-api-contracts/contracts/openapi.yaml"
            )
        );
    }

    #[test]
    fn validate_generated_contracts() {
        let temp = tempfile::TempDir::new().expect("tempdir");
        fs::create_dir(temp.path().join(".git")).expect("git dir");
        init_repository(temp.path()).expect("init repo");
        create_workstream(temp.path(), "api contracts").expect("workstream");

        scaffold_openapi(temp.path(), "001").expect("openapi");
        scaffold_asyncapi(temp.path(), "001").expect("asyncapi");
        scaffold_schema(temp.path(), "001", "repository event").expect("schema");

        assert_eq!(
            validate_openapi(temp.path(), "001")
                .expect("validate")
                .len(),
            1
        );
        assert_eq!(
            validate_asyncapi(temp.path(), "001")
                .expect("validate")
                .len(),
            1
        );
        assert_eq!(
            validate_schema(temp.path(), "001").expect("validate").len(),
            1
        );
    }

    #[test]
    fn validate_schema_accepts_explicit_directory() {
        let temp = tempfile::TempDir::new().expect("tempdir");
        fs::create_dir(temp.path().join(".git")).expect("git dir");
        init_repository(temp.path()).expect("init repo");
        create_workstream(temp.path(), "api contracts").expect("workstream");
        let path = scaffold_schema(temp.path(), "001", "repository event").expect("schema");

        let full_path = temp.path().join(path);
        let schemas_dir = full_path.parent().expect("parent");
        assert_eq!(
            validate_schema(temp.path(), schemas_dir.to_str().expect("dir"))
                .expect("validate")
                .len(),
            1
        );
    }

    #[test]
    fn resolve_contract_target_accepts_explicit_file_path() {
        let temp = tempfile::TempDir::new().expect("tempdir");
        fs::create_dir(temp.path().join(".git")).expect("git dir");
        init_repository(temp.path()).expect("init repo");
        create_workstream(temp.path(), "api contracts").expect("workstream");
        let path = scaffold_openapi(temp.path(), "001").expect("openapi");
        let full_path = temp.path().join(path);

        let resolved = resolve_contract_target(
            temp.path(),
            full_path.to_str().expect("path"),
            ContractStandard::OpenApi,
        )
        .expect("resolved");
        assert_eq!(resolved, full_path);
    }

    #[test]
    fn invalid_openapi_fails_validation() {
        let temp = tempfile::TempDir::new().expect("tempdir");
        let path = temp.path().join("openapi.yaml");
        fs::write(
            &path,
            "openapi: 3.1.1\ninfo:\n  title: Missing paths\n  version: 0.1.0\n",
        )
        .expect("write");

        let error = validate_openapi_file(&path).expect_err("should fail");
        assert!(error.to_string().contains("paths"));
    }

    #[test]
    fn invalid_asyncapi_fails_validation() {
        let temp = tempfile::TempDir::new().expect("tempdir");
        let path = temp.path().join("asyncapi.yaml");
        fs::write(
            &path,
            "asyncapi: 3.0.0\ninfo:\n  title: Missing channels\n  version: 0.1.0\n",
        )
        .expect("write");

        let error = validate_asyncapi_file(&path).expect_err("should fail");
        assert!(error.to_string().contains("channels"));
    }

    #[test]
    fn invalid_schema_fails_validation() {
        let temp = tempfile::TempDir::new().expect("tempdir");
        let path = temp.path().join("broken.schema.json");
        fs::write(&path, "{ invalid json").expect("write");

        let error = validate_schema_file(&path).expect_err("should fail");
        assert!(error.to_string().contains("Invalid JSON"));
    }
}
