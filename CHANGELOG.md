# Changelog

All notable changes to `mnemix-workflow` are documented here.

## v0.2.0 - 2026-03-30

### Added

- Optional repo-canonical GitHub issue mirroring with `mxw github init` and
  `mxw github sync`
- Workstream mirroring to one parent issue plus sub-issues for `spec.md`,
  `ux.md`, `plan.md`, and `tasks.md`
- Patch mirroring to a single GitHub issue
- Filtered and automation-oriented sync modes including `--all`, `--status`,
  `--changed`, and `--dry-run`
- Optional generated GitHub Action scaffold for changed-item auto-sync

### Changed

- Pull request CI now runs standard Rust, shell, and Python package checks
- Release prep now stages `Cargo.lock`
- Release runbook now reflects the scripted release-prep and publish flow

### References

- PR #18: GitHub issue support
- PR #17: pull request CI checks
- PR #16: release script and release notes polish
- PR #19: release checklist alignment

## v0.1.1 - 2026-03-29

### Added

- First packaged public release flow for `mnemix-workflow`
- PyPI distribution support for `pip` and `pipx`
- Release scripts, publish workflow, and maintainer runbook
- Root release notes file for GitHub release publishing

### Included Product Surface

- Repo-native workstreams, patches, and status metadata
- `mxw` CLI for scaffolding, status, hooks, validation, and contract helpers
- `mnx` interactive TUI for browsing tracked work
- OpenAPI, AsyncAPI, and JSON Schema scaffold-and-validate support

### References

- PR #13: packaging and release prep
- PR #14: release preparation for `v0.1.1`
- PR #15: Cargo.lock update for the `v0.1.1` release
