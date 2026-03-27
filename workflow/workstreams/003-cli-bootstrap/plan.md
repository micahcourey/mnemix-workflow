# Plan: CLI Bootstrap

## Summary

Initialize the first Rust crate for `mnemix-workflow` and implement a narrow two-command CLI slice with `init` for repository bootstrap and `new` for workstream creation, while establishing both the canonical `mnemix-workflow` binary and the shorthand `mxw` entrypoint future commands will build on.

## Scope Analysis

### Affected Areas

| Area | Changes Required |
|------|-----------------|
| Rust project bootstrap | Add the initial Cargo manifest and CLI entrypoint |
| CLI parsing | Define the first command surface for repository bootstrap and workstream creation |
| Repository bootstrap | Create the minimum workflow domain in consuming projects |
| Workstream scaffolding | Port slug, numbering, and template copy behavior into Rust |
| Documentation | Explain the new CLI path and its relationship to the bootstrap skill |
| Temporary script parity | Keep the Python script behavior aligned until the CLI can take over fully |

### Affected Layers

- [x] Documentation
- [x] Workflow artifacts
- [x] Scripts
- [x] CLI implementation

## Technical Design

### Proposed Additions

```text
Cargo.toml
src/
  main.rs
  cli.rs
  commands/
    mod.rs
    new.rs
  scaffold.rs
tests/
  new_workstream.rs
```

### CLI Shape

- Start with a standalone binary named `mnemix-workflow`
- Plan for `mxw` as the shorthand alias for the same CLI surface
- Implement `mnemix-workflow init` and `mxw init` for repository bootstrap
- Implement `mnemix-workflow new <name>` and `mxw new <name>` as equivalent entrypoints for workstream creation
- Keep the mental model aligned with the future delegated surface `mnemix workflow new`

### Distribution Shape

- Treat `pipx install mnemix-workflow` as the primary installation path for users
- Plan to expose both `mnemix-workflow` and `mxw` after installation
- Support GitHub Releases as a secondary path for prebuilt binaries
- Keep `cargo install mnemix-workflow` as an optional Rust-native install path, not the primary onboarding flow

### Behavior Parity

- Keep `init` intentionally minimal: create only the workflow domain needed to start using the framework in a consuming repository
- Reuse the canonical templates under `resources/skills/mnemix-workflow/assets/workstream/`
- Preserve the existing slugging, numeric id selection, and formatting rules from the Python bootstrap script
- Print the same style of success guidance after workstream creation

### Repo Discovery

- For `init`, prefer discovering the git repo root by walking upward from the current working directory
- For `new`, discover the git repo root and then require the initialized workflow structure to exist
- Fail fast with a clear message if the command is run outside a git repository or if `new` is used before initialization

### Design Constraints

- Keep dependencies minimal and ergonomic; `clap` is reasonable for command parsing if it keeps help output clean
- Avoid introducing configuration files or alternate template registries in this first slice
- Treat the Python script as the reference behavior during porting, not as a second evolving implementation

## Implementation Phases

### Phase 1: Bootstrap The Rust CLI

- Add `Cargo.toml` and the initial `src/` structure
- Define the first command surface, help text, and shorthand alias strategy
- Define the first-release installation and packaging strategy for `pipx`, alias exposure, and release artifacts
- Decide the minimal crate layout for future commands without overengineering it

### Phase 2: Implement Repository Bootstrap

- Implement `init` to create the minimum workflow directory structure
- Decide which starter files should exist at initialization time versus first workstream creation
- Return clear success and non-destructive error messages for re-running `init`

### Phase 3: Port Scaffolding Behavior

- Implement repo-root discovery
- Port slugify, title formatting, numeric sorting, and id formatting logic
- Port template-copy behavior using the existing skill templates
- Return clear success and error messages

### Phase 4: Verify And Document

- Add tests for init success, init idempotence or guarded reruns, invalid names, and successful workstream creation
- Update the root README quickstart to show the CLI path and planned install story alongside the temporary script
- Clarify whether the Python script remains a fallback or starts delegating once the CLI is stable

## Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| The first CLI slice grows into a full framework rewrite | High | Medium | Limit scope to `init` and `new` only |
| Rust implementation diverges from the Python script | Medium | Medium | Treat script behavior as the reference during porting |
| Repo discovery adds complexity and edge cases | Medium | Medium | Keep discovery narrow and fail clearly when invariants are missing |
| The crate layout becomes too abstract too early | Medium | Medium | Add only the modules needed for `new` and one shared scaffold layer |
| `init` creates too much project policy by default | Medium | Medium | Keep initialization to the minimum workflow domain and avoid forcing unrelated repo structure |

## Open Questions

- Should `mxw` be exposed as an actual second binary, a symlink, or an install-time alias in the first release?
- Should the PyPI package wrap a compiled Rust binary directly, or should the project use a Python shim that installs bundled executables?
- Should `init` create only directories and README stubs, or should it also seed starter decision files and templates?
- Should the first Rust implementation keep the Python script in place unchanged, or update the script later to call into the CLI for parity?
- Do we want a `workstream new` alias soon after `new`, or should that wait until more commands exist?

## References

- `docs/prd.md`
- `README.md`
- `workflow/workstreams/002-workflow-skill-bootstrap/plan.md`
- `resources/skills/mnemix-workflow/scripts/new-workstream.py`
