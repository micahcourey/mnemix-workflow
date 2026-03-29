# Release Notes

Mnemix Workflow `v0.1.1` prepares the first public packaged release flow for
the project. This release brings together the repo-native spec-driven planning
methodology, the Rust CLI and TUI surfaces, the patch lane, contract helpers,
and the bundled Python/PyPI packaging path so the product can be installed with
`pip` and `pipx` instead of only local Cargo workflows.

---

## Release Schedule

| Field | Value |
|-------|-------|
| **Release Date** | 2026-03-29 |
| **Version** | `v0.1.1` |
| **Release Type** | Patch |
| **Release Focus** | First public release packaging and release-process readiness |

## Release Scope

This release packages the existing `mnemix-workflow` product surface for
distribution and captures the maintainer workflows needed to cut and publish
future releases cleanly.

| Area | Summary | Status |
|------|---------|--------|
| Core CLI and TUI | `mnemix-workflow`, `mxw`, and `mnx` provide scaffolding, status tracking, contract helpers, and a browse-first TUI | Done |
| Patch lane | Lightweight tracked patches exist under `workflow/patches/` | Done |
| Contract standards | OpenAPI, AsyncAPI, and JSON Schema scaffolding/validation are shipped | Done |
| Packaging and release flow | Bundled Python distribution, release scripts, and GitHub publish workflow are in place | Done |

## User Acceptance Checks

This release focuses on installability, packaging correctness, and release
readiness rather than a new user-facing feature slice.

| Test Scenario | Result |
|---------------|--------|
| Run Rust test suite with `cargo test` | Pass |
| Run Python package tests | Pass |
| Build sdist and validate metadata with `twine check --strict` | Pass |
| Build and install bundled wheel in a clean virtual environment | Pass |
| Verify packaged entrypoints `mnemix-workflow`, `mxw`, and `mnx` | Pass |

## Known Follow-Ups

| Item | Status |
|------|--------|
| Add normal pull-request CI checks so release-prep PRs show GitHub checks | Planned in `workflow/patches/0005-add-pull-request-ci-checks.md` |
| Keep release notes updated for subsequent releases | Ongoing maintainer task |

## Deployment Checklist

- [x] Package metadata aligned for `Cargo.toml` and `python/mnemix_workflow/_version.py`
- [x] Publish workflow configured for the `pypi` environment
- [x] Local package preflight passes
- [ ] Release tag created
- [ ] GitHub Release published
- [ ] PyPI publish workflow completed successfully
- [ ] Clean `pip install mnemix-workflow` verification completed against the live package
- [ ] Clean `pipx install mnemix-workflow` verification completed against the live package

## Notes

Use this file as the GitHub Release notes source when needed:

```bash
gh release edit v0.1.1 --notes-file RELEASE_NOTES.md
```

Updating release notes does not rebuild or republish package artifacts.
