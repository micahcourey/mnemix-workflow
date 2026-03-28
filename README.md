# Mnemix Workflow

`mnemix-workflow` is a lightweight, repo-native feature planning framework for human planning with AI-assisted implementation.

It gives teams a clear, versioned path from intent to execution using:

- `spec.md` for product intent
- `ux.md` for user or developer experience intent
- `plan.md` for technical strategy
- `tasks.md` for execution slices
- `STATUS.md` for lightweight machine-readable workstream state
- `patches/` for lightweight tracked fixes and minor enhancements

The framework is designed to stay small in the common case, make UX first-class, and use open standards only where they add real value.

## Project Goals

- Keep the default workflow lightweight and teachable
- Make planning artifacts readable to humans and operable by AI agents
- Preserve a clear narrative from problem to implementation
- Make UX a first-class planning artifact instead of burying it inside generic specs
- Use open standards selectively for decisions, contracts, and architecture
- Stay repo-native and tool-neutral
- Provide a smooth path from bootstrap tooling to a future dedicated CLI

## Methodology

The core idea is simple:

> A workflow is made of workstreams. Each workstream moves from spec to UX to plan to tasks, with decisions recorded when they become durable.

Every pull request should map to either:

- a full `workstream`
- a lightweight `patch`

### Core Concepts

- `workflow`
  - the overall methodology and the artifact root in the repository
- `workstream`
  - one unit of planned work
- `spec.md`
  - the problem, users, goals, scope, and success criteria
- `ux.md`
  - journeys, states, interaction expectations, and Gherkin acceptance scenarios
- `plan.md`
  - technical approach and design strategy
- `tasks.md`
  - execution slices and validation checkpoints
- `STATUS.md`
  - lightweight machine-readable state and PR linkage for a workstream
- `workflow/decisions/`
  - durable repo-level decisions
- `workflow/workstreams/<id>/decisions/`
  - decisions local to one workstream
- `workflow/patches/`
  - single-file tracked fixes, chores, and narrow enhancements

### Lightweight By Default

The common case is intentionally small:

```text
workflow/workstreams/001-some-workstream/
  STATUS.md
  spec.md
  ux.md
  plan.md
  tasks.md
  decisions/

workflow/patches/0001-fix-status-copy.md
```

You do not need every possible layer for every feature. The four core files are the center of gravity. Everything else is additive.

## Open Standards

`mnemix-workflow` does not try to replace standards that already work well. It uses them by layer when they are helpful:

- `MADR` for durable decisions
- `OpenAPI` for HTTP interfaces
- `AsyncAPI` for event-driven interfaces
- `JSON Schema` for shared data shapes
- `Structurizr DSL` for architecture when needed

There is no strong repo-native open standard for feature-level UX specifications, which is why `ux.md` is a first-party Mnemix artifact.

## Repository Layout

```text
README.md
Cargo.toml
docs/
  methodology/
src/
tests/
resources/
  hooks/
  skills/
    mnemix-workflow/
workflow/
  decisions/
  patches/
  workstreams/
    001-bootstrap-mnemix-workflow/
    002-workflow-skill-bootstrap/
```

### Folder Roles

- `docs/`
  - explanatory documents about the methodology and project direction
- `resources/skills/`
  - reusable operational assets such as skills
- `resources/hooks/`
  - optional local git hook scripts for status nudges
- `workflow/`
  - active workflow artifacts
- `workflow/workstreams/`
  - individual units of planned work
- `workflow/patches/`
  - lightweight tracked changes represented by a single file
- `workflow/decisions/`
  - durable framework decisions

## Quickstart

### 1. Read The Method

Start with:

- [Methodology Naming System](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/docs/methodology/naming-system.md)
- [Product Requirements Document](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/docs/prd.md)

Then look at the existing workstreams:

- [001 Bootstrap](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/workflow/workstreams/001-bootstrap-mnemix-workflow/spec.md)
- [002 Workflow Skill Bootstrap](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/workflow/workstreams/002-workflow-skill-bootstrap/spec.md)
- [003 CLI Bootstrap](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/workflow/workstreams/003-cli-bootstrap/spec.md)

### 2. Initialize A Repository

During local development, run the CLI directly:

```bash
cargo run --bin mxw -- init
```

After packaging lands, the intended installed flow is:

```bash
mxw init
```

This creates the minimum workflow domain:

```text
workflow/
  decisions/
    README.md
  patches/
  workstreams/
```

### 3. Create A New Workstream

During local development:

```bash
cargo run --bin mxw -- new "user profile redesign"
```

Once installed:

```bash
mxw new "user profile redesign"
```

This creates:

```text
workflow/workstreams/<id>-user-profile-redesign/
  STATUS.md
  spec.md
  ux.md
  plan.md
  tasks.md
  decisions/
    README.md
```

`STATUS.md` is created by default for each new workstream. Its frontmatter uses:

- required fields: `status`, `summary`, `updated`
- optional fields: `prs`

### 4. Fill In The Core Artifacts

After scaffolding a workstream:

1. Write `spec.md` to define the problem and goals.
2. Write `ux.md` to define the experience and acceptance scenarios.
3. Write `plan.md` to define the implementation approach.
4. Write `tasks.md` to break the work into execution slices.
5. Keep `STATUS.md` current so CLI and future Studio views can track state and linked PRs.

### 5. Use A Patch For Smaller Tracked Work

For a narrow fix, chore, or minor enhancement, use a patch instead of a full
workstream:

```bash
mxw patch new "fix status copy"
```

This creates:

```text
workflow/patches/0001-fix-status-copy.md
```

Patch files are single-file mini-specs. They carry frontmatter status metadata
directly in the file using:

- required fields: `status`, `summary`, `updated`
- optional fields: `prs`

Use a patch when one file can clearly capture the intent, scope, implementation
notes, and validation. Use a full workstream when the work needs first-class
spec, UX, plan, and task separation.

### 6. Read Or Update Status

Show status:

```bash
mxw status 004
```

List only completed workstreams:

```bash
mxw status list --status completed
```

Set status and link a PR:

```bash
mxw status set 004 completed --pr 12
```

This keeps the workstream path stable while updating frontmatter in `STATUS.md`.

Patches use the same lifecycle metadata:

```bash
mxw patch status 0001
mxw patch status set 0001 completed --pr 12
mxw patch status list --status open
```

### 7. Browse Interactively

Launch the browse-first terminal UI:

```bash
mxw ui
```

V1 of the TUI gives you:

- status buckets for `proposed`, `open`, and `completed`
- a unified tracked-item list for workstreams and patches
- artifact preview for `STATUS.md`, `spec.md`, `ux.md`, `plan.md`, and `tasks.md`
- direct patch-file preview for patches
- one lightweight operational action: press `s` to cycle the selected item's status

Key controls:

- `Tab` / `Shift+Tab` to move focus
- `j` / `k` or arrow keys to move selection
- `h` / `l` in preview to switch artifacts
- `s` to cycle the selected item's status
- `q` or `Esc` to quit

### 8. Optional Local Hooks

The repository includes optional local hook scripts under `resources/hooks/`:

- `pre-commit-status-updated`
  - refreshes `updated` when workstream files change
- `pre-push-status-reminder`
  - warns that `STATUS.md` may need review before push
  - reminds you that if the resulting PR completes the workstream, `STATUS.md` should be set to `completed`

These hooks are local nudges, not the source of truth. Final semantic status should still land through the normal PR flow.

### 7. Use The Skill Directly

The repository includes a real Agent Skills Open Standard skill at:

- [resources/skills/mnemix-workflow/SKILL.md](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/resources/skills/mnemix-workflow/SKILL.md)

It bundles:

- workstream templates in `assets/`
- a scaffold script in `scripts/`
- focused conventions in `references/`

The skill remains a useful fallback while the Rust CLI is being packaged for release.

## Numbering Convention

Workstreams use numeric prefixes:

- `001` through `999` use zero-padded 3-digit ids
- after `999`, ids continue as `1000`, `1001`, and so on

Tooling should always sort by the numeric prefix, not plain string order.

## Current Status

The repository is being bootstrapped by dogfooding the framework on itself.

The current implementation includes:

- a methodology and naming system
- an initial framework plan
- the first bootstrap workstream
- a real bootstrap skill for creating new workstreams before the dedicated CLI exists
- an implementation-focused workstream for the first Rust CLI slice
- a working Rust CLI with `init`, `new`, and `status` commands plus the `mxw` shorthand alias
- optional local hook scripts for `updated` refreshes and push-time reminders

The next major step is packaging and releasing the CLI cleanly so the intended install flow, `pipx install mnemix-workflow`, becomes real for consuming projects.
