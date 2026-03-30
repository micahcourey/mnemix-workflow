# Release Notes

Mnemix Workflow `v0.2.0` adds optional repo-canonical GitHub issue support so
teams can mirror tracked workstreams and patches into GitHub without moving the
source of truth out of the repository. This release keeps the repo-native
spec-driven workflow intact while extending the CLI with a practical execution
surface for GitHub-centric teams.

---

## Release Schedule

| Field | Value |
|-------|-------|
| **Release Date** | 2026-03-30 |
| **Version** | `v0.2.0` |
| **Release Type** | Minor |
| **Release Focus** | GitHub issue mirroring and release-flow polish |

## Release Scope

This release expands the shipped workflow surface with optional GitHub issue
mirroring and rounds out the release process with stronger CI and clearer
maintainer documentation.

| Area | Summary | Status |
|------|---------|--------|
| GitHub issue support | `mxw github init` and `mxw github sync` can mirror workstreams and patches into GitHub Issues while keeping repo artifacts canonical | Done |
| Filtered and automation-friendly sync | Sync supports single-target, `--all`, `--status`, `--changed`, and `--dry-run` flows, plus optional generated auto-sync workflow scaffolding | Done |
| Pull request CI | Standard PR checks now run formatting, clippy, tests, shell validation, and Python package preflight on PRs and `main` pushes | Done |
| Release maintenance polish | Release prep now stages `Cargo.lock`, the runbook matches the scripted flow, and the repo includes maintained release notes/changelog artifacts | Done |

## User Acceptance Checks

This release focuses on the new GitHub workflow surface plus the release and CI
polish needed to support it confidently.

| Test Scenario | Result |
|---------------|--------|
| Run Rust verification with `cargo fmt --all --check` | Pass |
| Run Rust test suite with `cargo test` | Pass |
| Run linting with `cargo clippy --all-targets -- -D warnings` | Pass |
| Verify `mxw github --help` and `mxw github sync --help` | Pass |
| Validate PR CI workflow coverage on normal pull requests | Pass |

## Known Follow-Ups

| Item | Status |
|------|--------|
| Deeper GitHub integration such as Projects support | Future workstream |
| GitHub issue editing guidance and permissions remain operator-controlled rather than enforced by the CLI | Ongoing maintainer guidance |

## Deployment Checklist

- [x] Release branch prepared for `v0.2.0`
- [x] Release notes updated for `v0.2.0`
- [x] Changelog updated with prior and current releases
- [ ] Release tag created
- [ ] GitHub Release published
- [ ] PyPI publish workflow completed successfully
- [ ] Clean `pip install mnemix-workflow` verification completed against the live package
- [ ] Clean `pipx install mnemix-workflow` verification completed against the live package

## Notes

Use this file as the GitHub Release notes source when needed:

```bash
gh release edit v0.2.0 --notes-file RELEASE_NOTES.md
```

Updating release notes does not rebuild or republish package artifacts.
