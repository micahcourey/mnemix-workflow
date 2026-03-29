# Mnemix Workflow

`mnemix-workflow` is a repo-native spec-driven development methodology and CLI
for teams that want a clear path from intent to implementation, with AI as the
primary implementation engine and humans staying in control of the plan.

It combines:

- a lightweight spec-driven planning method built around versioned Markdown artifacts
- a command-oriented CLI for scaffolding and status management
- an interactive terminal UI for browsing workstreams and patches quickly
- selective support for machine-readable contract standards where they add real value

The result is a spec-driven workflow that stays readable to humans, operable by
agents, and grounded in normal repository files instead of hidden metadata.

At the methodology level, it gives teams a lightweight path from:

- `spec.md` for problem and intent
- `ux.md` for experience and behavior
- `plan.md` for implementation approach
- `tasks.md` for execution slices
- `STATUS.md` for current state and PR linkage

It also gives teams a simple planning rule:

- use a `workstream` for larger, multi-artifact work
- use a `patch` for smaller tracked changes that still need repo-visible planning
- record durable repo-shaping choices in `workflow/decisions/`

For the full method, terminology, and repository shape, see the
[Methodology Naming System](docs/methodology/naming-system.md).

## Why It Exists

AI coding gets much more reliable when requirements, UX intent, execution
slices, and status are stored in the repo instead of living only in chat
history.

`mnemix-workflow` gives you a lightweight structure for that:

- `workstreams` for larger planned work
- `patches` for smaller tracked fixes and enhancements
- `STATUS.md` and frontmatter metadata for lightweight machine-readable state
- `mnx` for fast interactive browsing
- `mxw` for explicit scaffolding, status, and contract commands

## Install

Today the most reliable local-development path is:

```bash
cargo run --bin mxw -- --help
```

Shortcut into the interactive UI during local development:

```bash
cargo run --bin mnx --
```

The intended packaged install experience is:

```bash
pipx install mnemix-workflow
```

That packaged distribution is not the default local-dev path yet, so the README
below uses `cargo run` for development examples and `mxw` / `mnx` for the
installed mental model.

## Quickstart

### 1. Initialize A Repository

Local development:

```bash
cargo run --bin mxw -- init
```

Installed usage:

```bash
mxw init
```

This creates:

```text
workflow/
  decisions/
    README.md
  patches/
  workstreams/
```

### 2. Create Planned Work

For a full workstream:

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

For a smaller tracked change:

```bash
mxw patch new "fix status copy"
```

This creates:

```text
workflow/patches/0001-fix-status-copy.md
```

### 3. Track Status

Show status:

```bash
mxw status 004
mxw patch status 0001
```

Update status and link a PR:

```bash
mxw status set 004 completed --pr 12
mxw patch status set 0001 completed --pr 12
```

List tracked items by state:

```bash
mxw status list --status completed
mxw patch status list --status open
```

### 4. Launch The Interactive UI

The fastest way into the product experience is:

```bash
mnx
```

You can also launch it explicitly through the main CLI:

```bash
mxw ui
```

`mnx` is the shortcut app-like entrypoint. `mxw` is the explicit command
surface for scripting, scaffolding, and operational actions.

## The `mnx` UI

`mnx` is the browse-first terminal experience for Mnemix Workflow.

Today it gives you:

- status buckets for `proposed`, `open`, and `completed`
- a unified tracked-item list for workstreams and patches
- artifact preview for `STATUS.md`, `spec.md`, `ux.md`, `plan.md`, and `tasks.md`
- basic Markdown formatting in the preview pane for headings, lists,
  blockquotes, code fences, and simple emphasis
- direct patch-file preview for patches
- one lightweight operational action: press `s` to cycle the selected item's status

Key controls:

- `Tab` / `Shift+Tab` to move focus
- `j` / `k` or arrow keys to move selection
- `h` / `l` in preview to switch artifacts
- `s` to cycle the selected item's status
- `q` or `Esc` to quit

## Method Overview

The core idea is simple:

> A workflow is made of workstreams. Each workstream moves from spec to UX to
> plan to tasks, with decisions recorded when they become durable.

That methodology is intentionally trying to solve a specific problem:

- humans need a clear narrative of intent, scope, and tradeoffs
- AI agents need explicit artifacts they can read, update, and validate
- teams need progress and status to live in the repo instead of only in chat or PR threads

Every pull request should map to either:

- a full `workstream`
- a lightweight `patch`

In practice, the methodology is meant to help humans and agents align before
implementation starts:

- `spec.md` captures what should be built and why
- `ux.md` captures how it should feel and behave
- `plan.md` captures how the implementation will be approached
- `tasks.md` captures the execution slices and verification steps
- `STATUS.md` captures whether the work is proposed, open, or completed

### Planning Lanes

- `workflow/workstreams/` holds larger planned initiatives with the full artifact set
- `workflow/patches/` holds lightweight tracked changes in a single file
- `workflow/decisions/` holds durable decisions that outlive any one workstream or patch

This keeps the framework lightweight by default without allowing untracked PRs.

For a fuller explanation of the method, terminology, repository shape, and
teaching vocabulary, see the
[Methodology Naming System](docs/methodology/naming-system.md).

### Core Artifacts

- `spec.md`
  - problem, users, goals, scope, and success criteria
- `ux.md`
  - journeys, states, interaction expectations, and Gherkin acceptance scenarios
- `plan.md`
  - technical approach and implementation strategy
- `tasks.md`
  - execution slices and validation checkpoints
- `STATUS.md`
  - machine-readable workstream state and linked PR metadata

### Lightweight By Default

The common case stays intentionally small:

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

## Contract Standards

`mnemix-workflow` uses open standards selectively where machine-readable
contracts are especially useful:

- `OpenAPI` for synchronous HTTP interfaces
- `AsyncAPI` for event-driven interfaces
- `JSON Schema` for reusable data shapes

Examples:

```bash
mxw openapi init 007
mxw openapi validate 007

mxw asyncapi init 007
mxw asyncapi validate 007

mxw schema new 007 "repository event"
mxw schema validate 007
```

These artifacts live under a workstream's `contracts/` folder when needed.

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
```

### Folder Roles

- `docs/`
  - explanatory docs about the method and product direction
- `resources/skills/`
  - reusable agent-facing assets and templates
- `resources/hooks/`
  - optional local git hook scripts for status nudges
- `workflow/`
  - active workflow artifacts
- `workflow/workstreams/`
  - larger planned work
- `workflow/patches/`
  - lightweight tracked fixes and enhancements
- `workflow/decisions/`
  - durable framework decisions

## Learn More

Start with:

- [Methodology Naming System](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/docs/methodology/naming-system.md)
- [Product Requirements Document](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/docs/prd.md)
- [Bootstrap Workstream 001](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/workflow/workstreams/001-bootstrap-mnemix-workflow/spec.md)
- [CLI Bootstrap Workstream 003](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/workflow/workstreams/003-cli-bootstrap/spec.md)

The repo also includes a real Agent Skills Open Standard skill at:

- [resources/skills/mnemix-workflow/SKILL.md](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/resources/skills/mnemix-workflow/SKILL.md)

## Current Status

The repository is dogfooding the framework on itself.

The current implementation includes:

- repo initialization and scaffolding via `mxw`
- workstream and patch tracking
- status metadata and PR linkage
- browse-first TUI access via `mnx`
- contract scaffolding and validation for `OpenAPI`, `AsyncAPI`, and `JSON Schema`

## Numbering Convention

Workstreams use numeric prefixes:

- `001` through `999` use zero-padded 3-digit ids
- after `999`, ids continue as `1000`, `1001`, and so on

Patches use zero-padded 4-digit prefixes:

- `0001`, `0002`, `0003`, and so on
