# Tasks: Release Polish And Pypi Publish Prep

## Workstream Goal

Prepare `mnemix-workflow` for its first PyPI release by completing the
packaging, release automation, install verification, and release-facing CLI
polish needed for a confident public launch.

## Execution Slices

### Slice 1: Packaging And Install Path

- [x] Decide the package build/distribution strategy for shipping the Rust CLI through PyPI
- [x] Align the strategy with `mnemix`'s bundled Python-package release model
- [x] Implement the package layout so installed environments expose `mnemix-workflow`, `mxw`, and `mnx`
- [x] Add a clean-environment install smoke test for the packaged release

### Slice 2: Release Scripts And CI

- [x] Add release-prep and publish scripts modeled on the `mnemix` release flow
- [x] Add release preflight checks analogous to `mnemix`'s local package and Linux build checks
- [x] Add or wire the GitHub publish workflow and release trigger path

### Slice 3: Release-Facing CLI Polish

- [x] Improve help and install-facing command discoverability where needed
- [x] Add `mxw hooks install` for the bundled git hooks
- [x] Add `mxw validate` as an umbrella validation command

### Slice 4: Documentation

- [x] Add a `mnemix-workflow` release checklist / runbook modeled on `mnemix/docs/release-checklist.md`
- [x] Update README and release-facing docs to match the packaged install and publish flow
- [x] Add a maintainer checklist for required PyPI and GitHub UI configuration

## Validation Checklist

- [x] A maintainer can follow the release-prep and publish path from docs and scripts alone
- [x] A clean install exposes `mnemix-workflow`, `mxw`, and `mnx`
- [x] Local preflight catches release issues before tagging/publishing
- [x] The release flow clearly mirrors the `mnemix` runbook and script model where practical

## Notes

- Reference implementation:
  - `mnemix/docs/release-checklist.md`
  - `mnemix/python/pyproject.toml`
  - `mnemix/scripts/check-python-package.sh`
  - `mnemix/scripts/check-linux-release-build.sh`
  - `mnemix/scripts/build-python-wheel-with-cli.sh`
  - `mnemix/scripts/release.sh`
  - `mnemix/scripts/publish-release.sh`
  - `mnemix/.github/workflows/publish-python.yml`
