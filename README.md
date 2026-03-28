# Mnemix Workflow

`mnemix-workflow` is a lightweight, repo-native feature planning framework for human planning with AI-assisted implementation.

It gives teams a clear, versioned path from intent to execution using:

- `spec.md` for product intent
- `ux.md` for user or developer experience intent
- `plan.md` for technical strategy
- `tasks.md` for execution slices

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
- `workflow/decisions/`
  - durable repo-level decisions
- `workflow/workstreams/<id>/decisions/`
  - decisions local to one workstream

### Lightweight By Default

The common case is intentionally small:

```text
workflow/workstreams/001-some-workstream/
  spec.md
  ux.md
  plan.md
  tasks.md
  decisions/
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
  skills/
    mnemix-workflow/
workflow/
  decisions/
  workstreams/
    001-bootstrap-mnemix-workflow/
    002-workflow-skill-bootstrap/
```

### Folder Roles

- `docs/`
  - explanatory documents about the methodology and project direction
- `resources/skills/`
  - reusable operational assets such as skills
- `workflow/`
  - active workflow artifacts
- `workflow/workstreams/`
  - individual units of planned work
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
  spec.md
  ux.md
  plan.md
  tasks.md
  decisions/
    README.md
```

### 4. Fill In The Core Artifacts

After scaffolding a workstream:

1. Write `spec.md` to define the problem and goals.
2. Write `ux.md` to define the experience and acceptance scenarios.
3. Write `plan.md` to define the implementation approach.
4. Write `tasks.md` to break the work into execution slices.

### 5. Use The Skill Directly

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
- a working Rust CLI with `init` and `new` commands plus the `mxw` shorthand alias

The next major step is packaging and releasing the CLI cleanly so the intended install flow, `pipx install mnemix-workflow`, becomes real for consuming projects.
