# Feature Spec: CLI Bootstrap

## Summary

Define the first real Rust CLI slice for `mnemix-workflow` by replacing the ad hoc bootstrap experience with a standalone command surface that supports both repository bootstrap and workstream creation through `init` and `new`, with shorthand `mxw` aliases alongside the canonical `mnemix-workflow` binary.

## Problem

`mnemix-workflow` now has a documented methodology and a temporary skill-based bootstrap path, but it still lacks an actual product surface. Relying only on a Python script inside a skill keeps the framework usable for agents, but it does not yet deliver the clear CLI experience that the project promises. It also leaves a bootstrap gap for consuming projects: a repository cannot use `new` until the expected workflow structure already exists.

## Users

- Primary persona: AI implementation agent creating the next unit of work
- Secondary persona: maintainer or contributor bootstrapping work from the terminal

## Goals

- Establish the first Rust CLI structure in the repository
- Implement an `init` command that bootstraps a repository for Mnemix Workflow
- Implement a `new` command that scaffolds a workstream from canonical templates
- Mirror the current numbering, slugging, and output conventions from the Python bootstrap path
- Keep the first CLI slice narrow enough to ship quickly and teach clearly
- Preserve a clean path toward a future `mnemix workflow new` delegated command in the main `mnemix` CLI

## Non-Goals

- Build validation, export, or standards-adapter commands
- Replace the bootstrap skill immediately
- Finalize all future CLI subcommands or plugin boundaries
- Add hidden metadata, config files, or a stateful project database

## User Value

Agents and humans get a clearer, more product-shaped entrypoint for the framework, while the repository starts proving that `mnemix-workflow` is more than a set of docs and templates.

## Functional Requirements

- The repository should define an initial Rust CLI binary for `mnemix-workflow`
- The CLI should support `mnemix-workflow init` as the canonical repository bootstrap command
- The CLI should support `mxw init` as the shorthand alias for the same behavior
- The CLI should support `mnemix-workflow new <name>` as the canonical first user-facing command
- The CLI should also support `mxw new <name>` as the shorthand alias for the same behavior
- The planned first-release installation flow should start with `pipx install mnemix-workflow`
- The packaging plan should expose both `mnemix-workflow` and `mxw` after installation
- The `init` command should create the minimum workflow structure a consuming project needs in order to use `new`
- The `init` command should be safe to run in an existing repository and should not overwrite meaningful existing workflow artifacts silently
- The command should create `workflow/workstreams/<id>-<slug>/` using the same starter artifact set as the current skill templates
- The command should preserve the existing numbering convention: `001` through `999`, then `1000+`
- The command should sort existing workstreams numerically rather than lexicographically when selecting the next id
- The command should print a clear success message with the created path and next suggested action
- The command should emit actionable error messages for invalid names, duplicate folders, and missing templates or repo structure
- The initial CLI implementation should reuse the canonical templates under `resources/skills/mnemix-workflow/assets/workstream/` rather than introducing a second template source

## Constraints

- The first CLI slice must stay lightweight and avoid overdesign
- The repository currently has no Rust project structure, so this workstream must define the initial crate shape
- The bootstrap skill should remain usable while the CLI is introduced
- The CLI should feel consistent with the future delegated `mnemix workflow ...` model, even if that integration is not implemented yet
- Distribution should remain friendly to users who do not have Rust tooling installed

## Success Criteria

- A contributor can run a single Rust CLI command to scaffold a new workstream successfully
- A contributor can run `mnemix-workflow init` or `mxw init` in a consuming repository and get the minimum required workflow structure
- A clear user install story exists, with `pipx install mnemix-workflow` as the default path
- The CLI-generated workstream matches the structure produced by the bootstrap skill
- The repository has a clear first Rust code path that future CLI work can build on
- The first CLI slice is small enough to understand without reopening methodology debates

## Risks

- The first CLI slice may grow too broad if it tries to solve validation and export at the same time
- The CLI may drift from the existing skill behavior if template sourcing or numbering rules diverge
- Repository bootstrap work may get bogged down in architecture choices before the first command ships
- Packaging and alias exposure may get deferred unless the install story is designed alongside the first CLI slice
- `init` may accidentally hardcode too much repo policy if it bootstraps more than the minimal workflow domain

## Open Questions

- Should `mxw` ship as a second binary alias immediately, or should it start as a documented shell alias until packaging is in place?
- Should the PyPI distribution package the Rust executable directly or use a small Python wrapper to install bundled binaries?
- What is the exact minimum artifact set that `init` should create for a consuming project?
- When the Rust CLI is stable, should the Python bootstrap script become a thin compatibility wrapper or remain as an agent-only fallback?

## References

- `docs/prd.md`
- `README.md`
- `workflow/workstreams/002-workflow-skill-bootstrap/spec.md`
- `resources/skills/mnemix-workflow/SKILL.md`
- `resources/skills/mnemix-workflow/scripts/new-workstream.py`
