# Plan: Release Polish And Pypi Publish Prep

## Summary

Implement the first release-prep slice by mirroring the successful `mnemix`
release pipeline shape: define packaging metadata and install behavior, add
release preflight and publish scripts, add clean-environment smoke tests, and
close the remaining CLI polish gaps that matter for a first public PyPI release.

## Scope Analysis

### Affected Areas

| Area | Changes Required |
|------|-----------------|
| Packaging metadata | Define the PyPI build/distribution path for the Rust CLI binaries |
| Release scripts | Add release-prep and publish helpers modeled on `mnemix/scripts/` |
| CI / GitHub workflows | Add release automation and publish workflow wiring |
| CLI help and polish | Tighten help output and release-facing command discoverability |
| Hook installation | Add an install path for the bundled git hooks |
| Validation | Add an umbrella `mxw validate` command over existing checks |
| Documentation | Add a release checklist / runbook and installation verification guidance |

### Affected Layers

- [x] Documentation
- [x] Workflow artifacts
- [x] Scripts
- [x] CLI implementation
- [x] Packaging / distribution
- [x] CI / release automation

## Technical Design

### Proposed Additions

```text
scripts/
  check-python-package.sh
  check-linux-release-build.sh
  release.sh
  publish-release.sh
docs/
  release-checklist.md
.github/workflows/
  publish-python.yml
python/ or packaging shim/
  packaging metadata for pipx / PyPI distribution
src/commands/
  hooks.rs
  validate.rs
```

### Design Constraints

- Mirror the `mnemix` release flow where practical:
  release-prep script -> merged release-prep PR -> publish script -> GitHub
  Release -> PyPI verification
- Keep the release pipeline understandable to a solo maintainer
- Preserve the existing command mental model instead of introducing separate
  release tooling with unrelated conventions
- Ensure the packaged install exposes all three intended commands:
  `mnemix-workflow`, `mxw`, and `mnx`

## Implementation Slices

### Slice 1: Packaging And Install Story

- Mirror the `mnemix` Python-package-plus-bundled-binary strategy for
  `mnemix-workflow`
- Ensure `pipx install mnemix-workflow` exposes `mnemix-workflow`, `mxw`, and `mnx`
- Add a clean-environment install smoke test

### Slice 1 Decisions Locked In

- Package name: `mnemix-workflow`
- Distribution channels: `pip` and `pipx`
- Packaging model: Python package that bundles the Rust release binaries, the
  same general strategy used in `mnemix`
- Binary entrypoints to expose:
  - `mnemix-workflow`
  - `mxw`
  - `mnx`
- Reference files for implementation:
  - `mnemix/python/pyproject.toml`
  - `mnemix/scripts/build-python-wheel-with-cli.sh`
  - `mnemix/.github/workflows/publish-python.yml`

### Slice 2: Release Scripts And Workflow

- Add release-prep and publish scripts modeled on `mnemix/scripts/release.sh`
  and `mnemix/scripts/publish-release.sh`
- Add preflight checks modeled on `mnemix/scripts/check-python-package.sh`
  and `mnemix/scripts/check-linux-release-build.sh`
- Add or wire the GitHub publish workflow and release trigger path

### Slice 3: CLI Polish

- Improve help output where needed, especially installed-user discoverability
- Add `mxw hooks install`
- Add `mxw validate` as an umbrella command over the existing workflow and
  contract checks

### Slice 4: Release Docs

- Add a `mnemix-workflow` release checklist/runbook modeled on
  `mnemix/docs/release-checklist.md`
- Update the README and install docs to match the packaged path and release flow

## Human Configuration Checklist

### PyPI

- [ ] Create or confirm the `mnemix-workflow` project on PyPI
- [ ] Add yourself as a project owner/maintainer on PyPI
- [ ] Configure trusted publishing for the GitHub repo
- [ ] Confirm the trusted publisher points at the intended GitHub workflow and environment
- [ ] Confirm the project name `mnemix-workflow` is available and matches the package metadata

### GitHub Repository Settings

- [ ] Create or confirm a `pypi` environment in GitHub Actions
- [ ] Allow the publish workflow to use OIDC trusted publishing for that environment
- [ ] Confirm the repo has permission to create releases and run publish workflows
- [ ] Confirm release creation from tags is acceptable for the repo’s protection settings

### GitHub Release Process

- [ ] Decide whether the first publish will be triggered only from a GitHub Release or also allow `workflow_dispatch`
- [ ] Confirm who will cut the first release and approve the release-prep PR
- [ ] Confirm the desired version number for the first public release

### Post-Publish Verification

- [ ] Verify the PyPI project page shows the expected metadata and install instructions
- [ ] Verify `pip install mnemix-workflow` works in a clean environment
- [ ] Verify `pipx install mnemix-workflow` exposes `mnemix-workflow`, `mxw`, and `mnx`

## Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Shipping binaries through PyPI is more complex than expected | High | Medium | Mirror the `mnemix` pattern closely and validate with clean install smoke tests |
| Release scripts drift from the proven `mnemix` flow | Medium | Medium | Reference the core repo scripts directly while designing the new ones |
| Scope expands into general product polish | Medium | High | Keep this workstream focused on release-critical polish and packaging |
| `mxw validate` grows too broad | Medium | Medium | Start with a narrow umbrella over existing checks rather than inventing a new framework |

## References

- `mnemix/docs/release-checklist.md`
- `mnemix/python/pyproject.toml`
- `mnemix/scripts/check-python-package.sh`
- `mnemix/scripts/check-linux-release-build.sh`
- `mnemix/scripts/build-python-wheel-with-cli.sh`
- `mnemix/scripts/release.sh`
- `mnemix/scripts/publish-release.sh`
- `mnemix/.github/workflows/publish-python.yml`
- `README.md`
- `docs/prd.md`
