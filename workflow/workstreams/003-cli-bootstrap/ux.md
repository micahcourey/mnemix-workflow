# UX Spec: CLI Bootstrap

## Summary

The first CLI experience should feel direct, trustworthy, and unsurprising: a user can bootstrap a repository with `init`, then create workstreams with `new`, and always know what to do next.

## Users And Context

- Primary persona: AI implementation agent working in a local repository
- Context of use: terminal-driven planning inside a repository that adopts Mnemix Workflow conventions
- Preconditions: the target repository contains the expected `workflow/workstreams/` structure and the installed CLI can access its canonical workstream templates

## User Goals

- Bootstrap a compatible repository without manually recreating folder structure
- Create a correctly numbered workstream without thinking about folder structure
- Trust that the generated workstream matches project conventions
- Understand the next action immediately after scaffolding

## Experience Principles

- Keep the happy path to one obvious command
- Prefer deterministic behavior over clever configuration
- Make failures easy to understand and recover from
- Preserve parity with the existing skill-based bootstrap path

## Primary Journey

1. The user runs `mxw init` inside a repository that does not yet use Mnemix Workflow conventions.
2. The CLI creates the minimum workflow domain, including `workflow/`, `workflow/decisions/`, and `workflow/workstreams/`.
3. The user then runs `mxw new "<workstream name>"` from somewhere inside that repository.
4. The CLI determines the target repo root, loads its canonical templates, and calculates the next numeric workstream id.
5. The CLI creates a new numbered workstream folder at `workflow/workstreams/<id>-<slug>/` with `spec.md`, `ux.md`, `plan.md`, `tasks.md`, and `decisions/README.md`.
6. The CLI prints the created path and tells the user to fill in the core artifacts next.

## Alternate Flows

### Flow: Invalid Name

- Trigger: the user passes a name that slugifies to nothing
- Path: the CLI rejects the input before creating any files
- Expected outcome: stderr explains that the name must contain at least one letter or digit

### Flow: Existing Destination

- Trigger: the computed workstream folder already exists
- Path: the CLI stops without modifying any files
- Expected outcome: stderr names the conflicting path clearly

### Flow: Missing Templates Or Repo Structure

- Trigger: the command is run outside a repository using Mnemix Workflow conventions or required templates are unavailable
- Path: the CLI fails fast before scaffolding
- Expected outcome: stderr explains which expected path could not be found

### Flow: Repository Not Initialized Yet

- Trigger: the user runs `new` before running `init`
- Path: the CLI rejects the command without creating any files
- Expected outcome: stderr explains that the repository must be initialized first and suggests `mxw init`

## Surfaces

### Surface: Terminal Command

- Purpose: the primary interface for workstream creation
- Key information: command help, bootstrap path, success path, error messages, and next-step guidance
- Available actions: `init`, `new`, `--help`, and argument validation for the workstream name through either `mnemix-workflow` or `mxw`
- Navigation expectations: subcommand help should be discoverable without reading implementation code

### Surface: Generated Workstream Folder

- Purpose: the immediate handoff from command execution to planning work
- Key information: the four core artifacts and local decisions folder
- Available actions: open and fill the generated files
- Navigation expectations: the folder shape should match what the README and methodology docs already teach

## States

### Loading

- The command should complete quickly enough that no explicit progress UI is needed for v0

### Empty

- `init` should create the minimum workflow structure cleanly in a repository that has no prior workflow artifacts
- If no prior workstreams exist after initialization, `new` should still create `001-...` cleanly without special user input

### Success

- Stdout prints the created relative path and a short next-step instruction

### Error

- Stderr explains what failed and points at the relevant path or argument without dumping noisy stack traces

## Interaction Details

- Input behavior: accept one quoted human-readable workstream name
- Feedback: keep success output to two short lines
- Keyboard behavior: standard terminal invocation and `--help` usage only
- Responsive behavior: not applicable beyond normal terminal width wrapping

## Content And Tone

- Important messages should use direct language like `Created workstream:` and `Next step: fill in spec.md, ux.md, plan.md, and tasks.md`
- Errors should be calm and specific rather than verbose or accusatory

## Accessibility Requirements

- The full flow must be keyboard-only
- Help and error output should be readable in standard terminal screen readers
- Success and error messages should avoid relying on color alone

## Acceptance Scenarios

```gherkin
Scenario: Initialize a repository for Mnemix Workflow
  Given the user is inside a repository that does not yet contain workflow/workstreams
  When the user runs `mxw init`
  Then the CLI should create the minimum workflow structure
  And the repository should contain workflow/decisions and workflow/workstreams

Scenario: Create a first workstream from the CLI
  Given the repository has already been initialized for Mnemix Workflow
  And the installed CLI can access the canonical workstream templates
  When the user runs `mxw new "user profile redesign"`
  Then the CLI should create workflow/workstreams/001-user-profile-redesign
  And the new folder should include spec.md, ux.md, plan.md, tasks.md, and decisions/README.md
  And stdout should show the created path and the next suggested action

Scenario: Reject an invalid workstream name
  Given the user is inside a repository initialized for Mnemix Workflow
  When the user runs `mxw new "!!!"`
  Then the CLI should exit with a non-zero status
  And stderr should explain that the name must contain at least one letter or digit
```

## Open Questions

- What is the minimum safe structure that `mxw init` should create in a consuming repository?
- Should the first CLI version support being run from any nested directory inside a target repository, or only from that repository's root?

## References

- `README.md`
- `docs/prd.md`
- `resources/skills/mnemix-workflow/SKILL.md`
- `resources/skills/mnemix-workflow/scripts/new-workstream.py`
