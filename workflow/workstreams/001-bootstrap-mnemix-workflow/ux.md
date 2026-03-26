# UX Spec: Bootstrap Mnemix Workflow

## Summary

The first user experience of `mnemix-workflow` is not a GUI, but a repository and CLI experience. The experience should feel simple, teachable, and legible to both humans and AI agents.

## Users And Context

- Primary persona: maintainer bootstrapping the framework
- Secondary persona: contributor opening the repository for the first time
- Context of use: local repository browsing, Markdown-based planning, early CLI design
- Preconditions: the user has the repository open and wants to understand or extend the methodology

## User Goals

- Understand the workflow quickly.
- Find the active workstream easily.
- Recognize the difference between repo-level guidance and workstream artifacts.
- See how the methodology is meant to be used in practice.

## Experience Principles

- The repo should teach by example.
- The default path should feel lightweight, not bureaucratic.
- Naming should be memorable and still self-explanatory.
- A contributor should be able to move from overview to active workstream with minimal hunting.

## Primary Journey

1. The user opens the repository.
2. They read the README and learn the basic methodology.
3. They open the naming-system document and understand the vocabulary.
4. They open the first workstream and see the actual artifacts in use.
5. They can infer the next implementation steps from the workstream plan and tasks.

## Alternate Flows

### Flow: AI implementation handoff

- Trigger: an AI agent is asked to implement the next phase of the framework
- Path: the agent reads `spec.md`, `ux.md`, `plan.md`, and `tasks.md`
- Expected outcome: the agent can act without relying on ambiguous chat history

### Flow: New contributor exploring the repo

- Trigger: a contributor opens the repository for the first time
- Path: they read `README.md`, then `docs/methodology/naming-system.md`, then the active workstream
- Expected outcome: they understand the methodology and current direction quickly

## Surfaces

### Surface: Repository root

- Purpose: explain what `mnemix-workflow` is
- Key information: one-line description, repo shape, starting point
- Available actions: open docs, open active workstream
- Navigation expectations: clear path from root to workstreams

### Surface: Methodology docs

- Purpose: define naming and teaching vocabulary
- Key information: workflow, workstream, spec, UX, plan, tasks, decisions
- Available actions: read, align terminology, make future naming decisions
- Navigation expectations: documentation should complement, not replace, the active workstream

### Surface: Active workstream folder

- Purpose: show the methodology in use
- Key information: problem, experience goals, implementation plan, tasks
- Available actions: refine artifacts, implement next steps, record decisions
- Navigation expectations: the workstream should be self-contained enough to guide action

## States

### Loading

- Not applicable in a UI sense, but initial repository comprehension should still feel immediate

### Empty

- An empty repo should quickly become understandable once the starter docs and first workstream are added

### Success

- The user understands what the framework is and where to go next

### Error

- If naming or folder structure is confusing, the repo experience has failed and should be simplified

### Permission Denied

- Not applicable for the initial local repository experience

## Interaction Details

- The repo should privilege obvious entrypoints over deep nesting.
- File names should be self-explanatory.
- The active workstream should be easy to find from the root.
- The methodology should be understandable without reading every document.

## Content And Tone

- The language should be clear, confident, and unpretentious.
- The framework should sound lightweight and practical rather than academic.
- Naming should support teaching without becoming gimmicky.

## Accessibility Requirements

- Documents should use readable Markdown structure.
- Heading hierarchy should be clear.
- The methodology should not rely on visual diagrams alone for comprehension.

## Acceptance Scenarios

```gherkin
Scenario: New contributor understands the repo shape quickly
  Given a contributor opens the repository for the first time
  When they read the README
  Then they should understand what mnemix-workflow is
  And they should know that active feature work lives under workflow/workstreams/

Scenario: AI agent can find the active planning artifacts
  Given an AI agent is asked to continue building the framework
  When it opens workflow/workstreams/001-bootstrap-mnemix-workflow/
  Then it should find spec.md, ux.md, plan.md, and tasks.md
  And it should have enough context to continue implementation planning
```

## Open Questions

- Should the README eventually include a visual lifecycle diagram, or stay text-first?
- How much CLI detail should appear in repository docs versus workstream plans?

## References

- `README.md`
- `docs/methodology/naming-system.md`
- `workflow/workstreams/001-bootstrap-mnemix-workflow/spec.md`
