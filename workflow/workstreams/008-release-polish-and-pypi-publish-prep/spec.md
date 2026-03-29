# Feature Spec: Release Polish And Pypi Publish Prep

## Summary

Prepare `mnemix-workflow` for its first PyPI release by tightening the CLI
surface, defining the packaging and install path, and adding a release pipeline
that mirrors the proven `mnemix` release flow where it makes sense.

## Problem

The core CLI is now feature-complete enough for a first public release, but the
project still lacks the release-facing polish and packaging path needed to ship
it confidently through PyPI and `pipx`. Without release scripts, publish
documentation, install smoke tests, and a packaged command story for
`mnemix-workflow`, `mxw`, and `mnx`, the first release would rely on ad hoc
steps and local tribal knowledge.

## Users

- Primary persona: Maintainer cutting the first public `mnemix-workflow` release
- Secondary persona: New user installing `mnemix-workflow` through `pipx`

## Goals

- Define a release process that mirrors the core `mnemix` release pipeline where practical
- Make `pipx install mnemix-workflow` a real, tested installation path
- Ensure the packaged install exposes `mnemix-workflow`, `mxw`, and `mnx`
- Close the remaining CLI polish gaps that matter for a first public release

## Resolved Decisions

- Use the same release packaging strategy as `mnemix`: a Python package
  published to PyPI that bundles the Rust CLI binaries inside the package
- Support both `pip install mnemix-workflow` and
  `pipx install mnemix-workflow`
- Expose all three console entrypoints from the packaged distribution:
  `mnemix-workflow`, `mxw`, and `mnx`

## Non-Goals

- Build a full package-manager abstraction layer beyond the first PyPI release
- Add major new product features unrelated to packaging, release, or near-term CLI polish
- Redesign the TUI beyond minor help and discoverability improvements

## User Value

This workstream turns `mnemix-workflow` from a promising local tool into a
releasable product. Maintainers get a repeatable publish path, and users get a
clear install story instead of needing a Rust checkout and local `cargo run`.

## Functional Requirements

- The repo must define a packaging path that installs `mnemix-workflow`, `mxw`,
  and `mnx` through PyPI / `pipx`
- The CLI should provide a first-class hook installation path, likely via
  `mxw hooks install`
- The CLI should provide an umbrella validation path, likely via `mxw validate`
- Help output and release-facing docs should make the command surfaces easier to
  discover for first-time users
- The repo should include preflight and publish automation modeled on the
  `mnemix` release flow
- The release process should include a clean-environment install smoke test for
  the packaged distribution

## Constraints

- Keep the release pipeline conceptually aligned with `mnemix/docs/release-checklist.md`
- Reuse `mnemix/scripts/` patterns where they fit instead of inventing a totally different release story
- Preserve the existing repo-native workflow model; packaging should distribute the product, not change the methodology
- Keep the first release process maintainable for a single maintainer

## Success Criteria

- A maintainer can follow a documented preflight and publish process without
  relying on chat history
- A clean `pipx install mnemix-workflow` produces working `mnemix-workflow`,
  `mxw`, and `mnx` commands
- The repo has release scripts and/or workflows that mirror the `mnemix`
  release pattern closely enough to feel familiar
- The CLI help and packaging docs are clear enough for a first public audience

## Risks

- Packaging the Rust binaries for PyPI may require more distribution glue than expected
- The first release pipeline may overfit to local maintainer assumptions
- Release automation may drift from `mnemix` if the mirrored approach is not captured clearly
- CLI polish scope may expand beyond the minimum needed for the first release

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
