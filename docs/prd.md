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

`mnemix-workflow` is a lightweight, repo-native feature planning framework for human planning with AI-assisted implementation. It provides a versioned, structured path from intent to execution using `spec.md`, `ux.md`, `plan.md`, and `tasks.md` as the core planning artifacts.

This document is the living product document for the project. It replaces the earlier dated plan file and should be updated as the framework, CLI, and methodology evolve.

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
| Make UX first-class | UX artifact exists and is used in active workstreams | Every user-facing workstream can define `ux.md` with narrative plus Gherkin scenarios |
| Keep the system repo-native and AI-operable | Workstreams and decisions live in normal versioned files | No required hidden metadata system for the core workflow |
| Provide a bootstrap path before the CLI exists | Reusable bootstrap tool exists in the repo | A standards-compliant skill can scaffold new workstreams |
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

### FR4: Standards-Compliant Bootstrap Skill
- **Description**: The repo must include a real Agent Skills Open Standard skill for workstream bootstrapping before the dedicated CLI exists.
- **User Story**: As an AI agent, I want a standards-compliant bootstrap skill, so that I can create new workstreams consistently using the same conventions the framework teaches.
- **Acceptance Criteria**:
  - Given the repository, When an agent inspects `resources/skills/mnemix-workflow/`, Then it finds `SKILL.md`, `assets/`, `scripts/`, and `references/`
  - Given the bootstrap script, When it runs successfully, Then it creates a new numbered workstream folder with the standard artifact set
- **Priority**: Must Have

### FR5: Numbering And Naming Conventions
- **Description**: Workstreams must use a documented numeric-prefix convention that scales beyond 999 entries.
- **User Story**: As a maintainer, I want predictable ordering and naming, so that the repository remains navigable over time.
- **Acceptance Criteria**:
  - Given workstream ids from `001` through `999`, When they are created, Then they are zero-padded to 3 digits
  - Given more than `999` workstreams, When a new one is created, Then numbering continues at `1000+`
  - Given tooling, When it determines the next id, Then it sorts numerically rather than lexicographically
- **Priority**: Must Have

## 6. Non-Functional Requirements

| Category | Requirement | Target |
|----------|-------------|--------|
| Simplicity | The common-case workflow should stay small | Four core artifacts plus optional `decisions/` |
| Clarity | File and folder names should be easy to understand | New contributors can navigate the repo without deep onboarding |
| Tool Neutrality | The framework should not require one editor or vendor environment | Repo artifacts remain normal Markdown and scripts |
| Interoperability | Use open standards selectively where they help | MADR, OpenAPI, AsyncAPI, JSON Schema, Structurizr DSL are optional by layer |
| Maintainability | The bootstrap path should be replaceable by the CLI later | Bootstrap skill mirrors the intended future CLI mental model |

## 7. User Experience

### User Flow
1. A maintainer or AI agent opens the repository and reads the root README.
2. They understand the methodology, repository shape, and active workstreams.
3. They inspect the current workstream or scaffold a new one using the bootstrap skill.
4. They fill in `spec.md`, `ux.md`, `plan.md`, and `tasks.md`.
5. They implement or refine the work using those artifacts as the shared source of intent.

### Wireframes / Mockups

Not applicable for the initial repository-first experience. The primary experience surface is the repository structure, Markdown artifacts, and the bootstrap skill.

## 8. Technical Considerations

### Dependencies
- Agent Skills Open Standard skill shape for the bootstrap implementation
- Python 3 for the temporary scaffold script
- Future Rust CLI work in the Mnemix ecosystem

### Constraints
- `mnemix-context` remains the canonical source of repo-level operating guidance
- `mnemix` memory integration remains optional, not required
- The framework should not introduce hidden metadata as a dependency for the core flow

### Data Requirements
- No database requirements for v0
- Workflows, decisions, and plans are stored as normal versioned repository files

## 9. Release Criteria

- [ ] Root README clearly explains the product and quickstart
- [ ] `workflow/` contains the active workstream domain and decision area
- [ ] The bootstrap skill exists under `resources/skills/mnemix-workflow/`
- [ ] The bootstrap script can scaffold a valid workstream
- [ ] `001` and `002` workstreams clearly document the initial methodology and bootstrap path
- [ ] The next implementation-focused workstream is ready to be created

## 10. Risks & Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| The framework becomes too heavy | High | Medium | Keep the common case centered on four files and make other layers additive |
| UX specification becomes too vague or too heavy | Medium | Medium | Keep `ux.md` narrative-first with selective Gherkin scenarios |
| The bootstrap skill becomes a permanent substitute for the CLI | Medium | Medium | Keep the script intentionally narrow and document it as transitional |
| Skill scope grows too large | Medium | Medium | Start with one skill, then split only when real complexity appears |
| Repo structure becomes confusing | Medium | Low | Keep clear boundaries between `docs/`, `resources/`, and `workflow/` |

## 11. Timeline

| Milestone | Target Date | Owner |
|-----------|------------|-------|
| Repository bootstrap and methodology docs | Completed in current repo state | Micah / Codex |
| Bootstrap skill and temporary scaffold script | Completed in current repo state | Micah / Codex |
| Next workstream for CLI implementation | Next major milestone | Micah / Codex |
| Dedicated CLI surface | Future phase | Micah / Codex |

## 12. Open Questions

- [ ] When should validation and export helpers split into their own skills, if ever?
- [ ] How much of the eventual CLI should mirror the bootstrap skill exactly?
- [ ] What is the first implementation-focused workstream after the bootstrap phase?

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
