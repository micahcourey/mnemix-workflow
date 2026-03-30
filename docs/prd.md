# Product Requirements Document (PRD)

| Field | Value |
|-------|-------|
| **Title** | Mnemix Workflow |
| **Author** | Codex |
| **Date** | 2026-03-26 |
| **Status** | Draft |
| **Version** | 0.1 |
| **Ticket** | strategy/workflow/mnemix-workflow |

---

## 1. Overview

`mnemix-workflow` is a lightweight, repo-native feature planning framework for human planning with AI-assisted implementation. It provides a versioned, structured path from intent to execution using `spec.md`, `ux.md`, `plan.md`, and `tasks.md` as the core planning artifacts, plus a lightweight `patches/` lane for smaller tracked changes.

This document is the living product document for the project. It replaces the earlier dated plan file and should be updated as the framework, CLI, and methodology evolve.

The current repo state now includes:

- workstream scaffolding
- patch scaffolding
- status metadata and status commands
- an initial browse-first interactive TUI
- optional repo-canonical GitHub issue mirroring for workstreams and patches
- packaged Python distribution support for `pip` and `pipx`
- optional git hook installation
- umbrella workflow validation

## 2. Problem Statement

### Current State

AI coding assistants are powerful, but teams often rely on chat history, loose tickets, or ad hoc docs to explain what should be built. That makes implementation quality inconsistent and forces humans and agents to repeatedly reconstruct context. Existing spec-driven frameworks are useful, but they do not cleanly fit the Mnemix ecosystem's combination of repo-native guidance, progressive disclosure, optional memory, and future visual planning.

### Impact

Without a lightweight, teachable workflow layer:

- product intent drifts from implementation details
- UX expectations are easy to under-specify
- AI agents must infer too much from incomplete prompts
- future sessions lose clarity and narrative continuity
- the Mnemix ecosystem lacks a dedicated layer between project guidance and implementation execution

## 3. Goals & Objectives

| Goal | Success Metric | Target |
|------|---------------|--------|
| Create a lightweight planning workflow | Core workflow works with the four primary artifacts | `spec.md`, `ux.md`, `plan.md`, `tasks.md` are sufficient for the common case |
| Keep every PR tracked without overloading tiny fixes | Small work has a lighter tracked lane | Every PR maps to either a workstream or a patch |
| Make UX first-class | UX artifact exists and is used in active workstreams | Every user-facing workstream can define `ux.md` with narrative plus Gherkin scenarios |
| Keep the system repo-native and AI-operable | Workstreams and decisions live in normal versioned files | No required hidden metadata system for the core workflow |
| Keep the workflow skill aligned with the shipped CLI | Agents have a concise, standards-compliant guide to the current workflow | The skill reinforces `mxw`, `mnx`, and repo conventions without stale alternate paths |
| Add optional GitHub execution visibility without moving source of truth | Teams can mirror tracked work into GitHub when useful | Repo artifacts stay canonical while GitHub issues remain synced mirrors |
| Preserve ecosystem fit | Integration boundaries stay clear | `mnemix-context` remains canonical for repo rules and `mnemix` memory remains optional |

### Non-Goals

- Build a heavyweight project-management suite
- Replace `AGENTS.md` or repo-level instructions as the source of project policy
- Require memory integration for basic workflow usage
- Force every feature to use every optional standard or artifact type
- Finalize all future CLI, validation, export, and Studio integration behavior in v0

## 4. User Personas

| Persona | Role | Need | Pain Point |
|---------|------|------|------------|
| Maintainer | Framework author / project lead | A teachable methodology and clear repo structure | Existing workflow ideas are easy to discuss but harder to codify consistently |
| AI Implementation Agent | Primary implementation worker | Clear, versioned intent and execution artifacts | Chat-only instructions are ambiguous and easy to lose |
| Contributor | Future collaborator | A quick way to understand the project and start work | Framework repos are often heavy on philosophy and light on usable entrypoints |

## 5. Functional Requirements

### FR1: Core Workstream Artifacts
- **Description**: The framework must define a simple, repo-native core artifact set.
- **User Story**: As a maintainer or AI agent, I want a predictable workstream structure, so that I can plan and execute work consistently.
- **Acceptance Criteria**:
  - Given a workstream, When it is scaffolded, Then it contains `spec.md`, `ux.md`, `plan.md`, `tasks.md`, and `decisions/`
  - Given the common case, When a feature is planned, Then the four core files are sufficient without requiring extra layers
- **Priority**: Must Have

### FR2: First-Class UX Artifact
- **Description**: The framework must define `ux.md` as a first-party artifact for user or developer experience planning.
- **User Story**: As a planner, I want to define journeys, states, and acceptance scenarios clearly, so that AI implementation remains aligned with intended behavior.
- **Acceptance Criteria**:
  - Given a user-facing workstream, When UX is specified, Then `ux.md` can capture narrative behavior and embedded Gherkin scenarios
  - Given an AI agent, When it reads `ux.md`, Then it can derive user-visible behavior without relying only on the spec or plan
- **Priority**: Must Have

### FR3: Repo-Native Workstream Domain
- **Description**: Active planning artifacts must live in a dedicated workflow domain inside the repository.
- **User Story**: As a contributor, I want the repo shape to explain itself, so that I can distinguish active workstreams from methodology docs and reusable resources.
- **Acceptance Criteria**:
  - Given the repo root, When a user browses it, Then `workflow/` contains active work artifacts
  - Given durable framework decisions, When they are recorded, Then they live under `workflow/decisions/`
- **Priority**: Must Have

### FR3b: Lightweight Patch Lane
- **Description**: The framework must support a lightweight tracked lane for narrow fixes and minor enhancements.
- **User Story**: As a maintainer or AI agent, I want a smaller tracked unit than a full workstream, so that every PR can stay planned without unnecessary ceremony.
- **Acceptance Criteria**:
  - Given a narrow fix or minor enhancement, When it is tracked, Then it can live as a single Markdown file under `workflow/patches/`
  - Given a patch, When it is created, Then it uses frontmatter status metadata consistent with workstreams
  - Given the framework rule, When a PR is opened, Then it maps to either a workstream or a patch
- **Priority**: Must Have

### FR4: Standards-Compliant Workflow Skill
- **Description**: The repo must include a real Agent Skills Open Standard skill that teaches the current CLI-first workflow.
- **User Story**: As an AI agent, I want a standards-compliant workflow skill, so that I can create and maintain tracked work consistently using the same conventions the framework teaches.
- **Acceptance Criteria**:
  - Given the repository, When an agent inspects `resources/skills/mnemix-workflow/`, Then it finds `SKILL.md`, `assets/`, and `references/`
  - Given the current workflow, When an agent reads the skill, Then it can follow `mxw`, `mnx`, status, and contract commands without stale fallback instructions
- **Priority**: Must Have

### FR5: Numbering And Naming Conventions
- **Description**: Workstreams must use a documented numeric-prefix convention that scales beyond 999 entries.
- **User Story**: As a maintainer, I want predictable ordering and naming, so that the repository remains navigable over time.
- **Acceptance Criteria**:
  - Given workstream ids from `001` through `999`, When they are created, Then they are zero-padded to 3 digits
  - Given more than `999` workstreams, When a new one is created, Then numbering continues at `1000+`
  - Given tooling, When it determines the next id, Then it sorts numerically rather than lexicographically
- **Priority**: Must Have

### FR6: Interactive Terminal Workflow View
- **Description**: The CLI should provide a browse-first interactive mode for viewing tracked workflow items.
- **User Story**: As a maintainer or contributor, I want a terminal-native workflow cockpit, so that I can scan statuses and inspect artifacts faster than command-by-command shell usage.
- **Acceptance Criteria**:
  - Given an initialized repository, When a user runs `mxw ui`, Then they can browse workstreams and patches by status
  - Given a selected item, When a user opens the preview pane, Then they can inspect the main workstream artifacts or patch file inside the terminal
  - Given the v1 interactive mode, When a user wants a lightweight operational action, Then they can change the selected item's status without leaving the TUI
- **Priority**: Should Have

### FR7: Optional Repo-Canonical GitHub Issue Mirroring
- **Description**: The CLI should optionally mirror repo-tracked work into GitHub Issues without moving source of truth out of the repository.
- **User Story**: As a maintainer, I want GitHub Issues to reflect tracked workstreams and patches, so that teams who operate in GitHub can see and manage execution without editing planning content there.
- **Acceptance Criteria**:
  - Given a repo using GitHub mirroring, When a maintainer runs `mxw github sync 009`, Then the workstream is mirrored as one parent issue plus sub-issues for `spec.md`, `ux.md`, `plan.md`, and `tasks.md`
  - Given a patch, When a maintainer runs `mxw github sync 0005`, Then it is mirrored as a single GitHub issue
  - Given repo-canonical sync, When issue titles or bodies are refreshed, Then manual GitHub body edits may be overwritten from repo artifacts
  - Given automation, When the optional GitHub Action runs `mxw github sync --changed`, Then only already-linked changed items are refreshed
- **Priority**: Should Have

## 6. Non-Functional Requirements

| Category | Requirement | Target |
|----------|-------------|--------|
| Simplicity | The common-case workflow should stay small | Four core artifacts plus optional `decisions/` |
| Clarity | File and folder names should be easy to understand | New contributors can navigate the repo without deep onboarding |
| Tool Neutrality | The framework should not require one editor or vendor environment | Repo artifacts remain normal Markdown and scripts |
| Interoperability | Use open standards selectively where they help | OpenAPI, AsyncAPI, and JSON Schema are the supported contract standards |
| Maintainability | The bootstrap path should be replaceable by the CLI later | Bootstrap skill mirrors the intended future CLI mental model |

## 7. User Experience

### User Flow
1. A maintainer or AI agent opens the repository and reads the root README.
2. They understand the methodology, repository shape, and active workstreams.
3. They inspect the current workstream or scaffold a new one using `mnx`, `mxw`, and the bundled workflow skill guidance.
4. They choose either a full workstream or a lightweight patch based on the scope of the change.
5. They fill in the relevant artifacts and implement or refine the work using them as the shared source of intent.

### Wireframes / Mockups

Not applicable for the initial repository-first experience. The primary experience surface is the repository structure, Markdown artifacts, the CLI, and the bundled workflow skill.

## 8. Technical Considerations

### Dependencies
- Agent Skills Open Standard skill shape for the bundled workflow guidance
- Rust CLI and TUI implementation in the Mnemix ecosystem
- GitHub CLI and GitHub Issues APIs for optional mirrored issue support

### Constraints
- `mnemix-context` remains the canonical source of repo-level operating guidance
- `mnemix` memory integration remains optional, not required
- The framework should not introduce hidden metadata as a dependency for the core flow

### Data Requirements
- No database requirements for v0
- Workflows, patches, decisions, and plans are stored as normal versioned repository files
- Optional GitHub issue linkage is stored back into repo metadata for mirrored items

## 9. Release Criteria

- [ ] Root README clearly explains the product and quickstart
- [ ] `workflow/` contains the active workstream domain and decision area
- [ ] `workflow/patches/` is defined as the lightweight tracked lane
- [ ] The workflow skill exists under `resources/skills/mnemix-workflow/`
- [ ] The workflow skill reflects the shipped CLI-first workflow accurately
- [ ] `001` and `002` workstreams clearly document the initial methodology and bootstrap path
- [ ] The CLI can scaffold the primary tracked units the methodology teaches
- [ ] The CLI can optionally mirror tracked work into GitHub Issues while keeping repo artifacts canonical
- [ ] The packaged install exposes `mnemix-workflow`, `mxw`, and `mnx`
- [ ] The repo includes a maintainer-facing release checklist and publish workflow

## 9.1 Contract Standards Scope

The framework's supported standards scope is intentionally narrow:

- `OpenAPI` for HTTP API contracts
- `AsyncAPI` for event-driven contracts
- `JSON Schema` for shared data shapes and reusable payload models

These standards are the contract-focused standards layer in the current
product. Decisions, architecture, and UX remain repo-native artifacts unless a
later workstream introduces more formal integrations.

## 10. Risks & Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| The framework becomes too heavy | High | Medium | Keep the common case centered on four files and make other layers additive |
| UX specification becomes too vague or too heavy | Medium | Medium | Keep `ux.md` narrative-first with selective Gherkin scenarios |
| The workflow skill drifts away from the shipped CLI behavior | Medium | Medium | Keep the skill concise and update it whenever the CLI surface changes |
| Skill scope grows too large | Medium | Medium | Start with one skill, then split only when real complexity appears |
| Repo structure becomes confusing | Medium | Low | Keep clear boundaries between `docs/`, `resources/`, and `workflow/`, and explain workstreams versus patches clearly |
| GitHub mirror state drifts from repo state | Medium | Medium | Keep repo artifacts canonical, overwrite issue bodies on sync, and limit auto-sync to `--changed` updates for already-linked items |

## 11. Timeline

| Milestone | Target Date | Owner |
|-----------|------------|-------|
| Repository bootstrap and methodology docs | Completed in current repo state | Micah / Codex |
| Workflow skill and CLI-aligned templates | Completed in current repo state | Micah / Codex |
| Dedicated CLI surface | Completed in current repo state | Micah / Codex |
| Lightweight patch lane | Completed in current repo state | Micah / Codex |
| Interactive TUI mode | Completed as initial browse-first slice in current repo state | Micah / Codex |
| Contract standards support | Completed as initial scaffold-and-validate slice in current repo state | Micah / Codex |
| GitHub issue mirroring | Completed as initial repo-canonical sync slice in current repo state | Micah / Codex |

## 12. Open Questions

- [ ] When should validation and export helpers split into their own skills, if ever?
- [ ] When should the workflow skill grow beyond conventions and command guidance into richer agent-specific workflows?
- [ ] What should the next interactive slice be after browse-first TUI: richer inline actions, guided creation, or agent-assisted planning?
- [ ] How much GitHub Projects support should be layered on top of issue mirroring, if any?

## Appendix

### Related Documents
- [Methodology Naming System](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/docs/methodology/naming-system.md)
- [Bootstrap Workstream 001](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/workflow/workstreams/001-bootstrap-mnemix-workflow/spec.md)
- [Workflow Skill Bootstrap Workstream 002](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/workflow/workstreams/002-workflow-skill-bootstrap/spec.md)
- [Bootstrap Skill](/Users/micah/Projects/mnemix-workspace/mnemix-workflow/resources/skills/mnemix-workflow/SKILL.md)

### Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1 | 2026-03-26 | Codex | Replaced the dated framework plan with a living PRD |
