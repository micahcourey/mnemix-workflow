# Tasks: CLI Bootstrap

## Workstream Goal

Introduce the first real Rust CLI slice for `mnemix-workflow` by shipping `init` for repository bootstrap and `new` for workstream creation, while establishing the initial CLI project structure.

## Execution Slices

### Slice 1: Define The First CLI Surface

- [x] Confirm the canonical binary name and the `mxw` shorthand strategy
- [x] Decide the minimal Rust crate layout for future commands
- [x] Decide how the CLI discovers the repository root and distinguishes `init` from `new`
- [x] Decide the first-release installation and packaging strategy for `pipx install mnemix-workflow`

### Slice 2: Bootstrap The Rust Project

- [x] Add `Cargo.toml`
- [x] Add the initial CLI entrypoints
- [x] Add the command and scaffolding modules needed for `init` and `new`

### Slice 3: Implement Repository Bootstrap

- [x] Implement `init` to create the minimum workflow structure in a consuming repository
- [x] Decide whether `init` creates README stubs, starter decision files, or only directories
- [x] Emit clear success and re-run behavior for already-initialized repositories

### Slice 4: Implement Workstream Creation

- [x] Port slugify, title, numbering, and path generation behavior from the Python script
- [x] Reuse the canonical templates under `resources/skills/mnemix-workflow/assets/workstream/`
- [x] Emit clear success and error messages

### Slice 5: Verify And Document

- [x] Add tests for init success, invalid names, and id selection
- [x] Add tests for running `new` before initialization
- [x] Update the root README to document the CLI path
- [x] Document the planned installation paths for `pipx`, GitHub Releases, and optional `cargo install`
- [x] Clarify the transition plan between the Python script and the Rust CLI

## Validation Checklist

- [x] Running `mxw init` creates the minimum workflow structure in a consuming repository
- [x] Running `mnemix-workflow new "<workstream name>"` creates the expected workstream folder shape
- [x] Running `mxw new "<workstream name>"` produces the same result as the canonical command
- [x] Running `new` before `init` fails clearly with a corrective next step
- [x] The planned user install flow is clear and starts with `pipx install mnemix-workflow`
- [x] The CLI uses the same numbering convention as the Python scaffold script
- [x] The CLI works from a reasonable repository location and fails clearly outside it
- [x] The implementation remains small enough to understand quickly

## Notes

- This workstream should stay intentionally narrow; validation, export, and standards-specific commands belong in later workstreams.
