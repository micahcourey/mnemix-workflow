# UX Spec: Interactive Tui Mode

## Summary

The interactive mode should feel like a lightweight workflow cockpit: users can
scan statuses, move through workstreams and patches, and inspect core
artifacts with a small set of keyboard commands. V1 should feel focused and
calm rather than trying to become a full terminal agent environment
immediately.

## Users And Context

- Primary persona: maintainer, planner, or contributor working in a terminal inside a repository using Mnemix Workflow
- Context of use: reviewing and managing active workflow artifacts without leaving the terminal
- Preconditions: the repository is initialized for Mnemix Workflow and contains one or more tracked workflow items

## User Goals

- See all workstreams and patches grouped or filtered by status quickly
- Inspect the key planning files for a workstream or patch without opening an editor
- Move through the interface entirely from the keyboard
- Understand that richer agent-assisted planning can come later without being forced into it now

## Experience Principles

- Browse first, edit selectively
- Make repository state legible at a glance
- Use a small number of memorable keyboard interactions
- Keep the v1 interaction model stable enough to host future agent actions later

## Primary Journey

1. The user runs `mxw ui` in a repository using Mnemix Workflow.
2. The TUI opens with a status-oriented view of tracked workflow items.
3. The user selects a status bucket such as `open` or `completed`.
4. The user moves through the tracked item list and selects one workstream or patch.
5. The user switches between `STATUS.md`, `spec.md`, `ux.md`, `plan.md`, and `tasks.md` for a workstream, or previews the patch file directly for a patch.
6. The user reviews the artifact content in a readable terminal preview.
7. The user optionally performs a lightweight workflow action or exits back to the normal CLI.

## Alternate Flows

### Flow: Empty Repository

- Trigger: the repository has no workstreams or patches yet
- Path: the TUI should show a clear empty state and point to `mxw new` and `mxw patch new`
- Expected outcome: the user understands the next step immediately

### Flow: Small Terminal

- Trigger: the terminal window is too small for the standard layout
- Path: the TUI should degrade gracefully with a compact layout or a clear warning
- Expected outcome: the UI remains usable or fails clearly rather than rendering garbage

### Flow: Missing Artifact

- Trigger: a tracked item is missing one of the expected Markdown files
- Path: the preview pane should show a clear missing-file message
- Expected outcome: the user understands what is missing without crashing the UI

## Surfaces

### Surface: Status Pane

- Purpose: choose which workflow state to browse
- Key information: status categories and counts
- Available actions: move selection, activate filter
- Navigation expectations: should feel immediate and easy to scan

### Surface: Workstream List Pane

- Purpose: show workstreams and patches for the current status or filter
- Key information: item type, id, title/slug, summary, and optionally PR linkage
- Available actions: move selection, open artifact preview
- Navigation expectations: should support quick up/down movement and obvious selection state

### Surface: Artifact Preview Pane

- Purpose: preview the selected tracked item artifact
- Key information: readable Markdown content from `STATUS.md`, `spec.md`, `ux.md`, `plan.md`, or `tasks.md` for workstreams, or the patch file itself for patches
- Available actions: switch artifact tabs, scroll
- Navigation expectations: should prioritize readability over fancy formatting in v1

### Surface: Footer / Help Bar

- Purpose: teach the keybindings and available actions
- Key information: current key hints and mode
- Available actions: none beyond orientation
- Navigation expectations: should reduce the need to memorize controls

## States

### Loading

- Startup and navigation should feel fast enough that explicit loading states are minimal

### Empty

- Empty-state messaging should explain that no workstreams or patches exist yet and point to `mxw new` or `mxw patch new`

### Success

- The user can reliably browse statuses, select workstreams, and read artifact content

### Error

- Errors should be clear, local to the affected pane when possible, and non-destructive

## Interaction Details

- Input behavior: support arrow keys and `j/k` style movement where sensible
- Feedback: keep selection, focus, and current artifact obvious at all times
- Keyboard behavior: support fully keyboard-driven navigation with a visible quit action such as `q`
- Responsive behavior: prefer graceful pane compression over breaking the layout entirely

## Content And Tone

- Labels should be short and operational, such as `Open`, `Completed`, `Workstream`, `Patch`, `Spec`, `UX`, `Plan`, `Tasks`
- Messages should be clear, quiet, and practical rather than chatty

## Accessibility Requirements

- The full v1 flow must work without a mouse
- Focus/selection should never rely on color alone
- Content should remain readable in common terminal screen reader setups

## Acceptance Scenarios

```gherkin
Scenario: Browse open workstreams in the TUI
  Given a repository contains multiple workstreams and patches with different statuses
  When the user launches mxw ui
  Then the user should be able to select the open status bucket
  And the tracked item list should update to show matching items

Scenario: Preview a workstream artifact
  Given a user has selected a workstream in the TUI
  When the user switches to the plan artifact
  Then the preview pane should display plan.md content in a readable terminal view

Scenario: Preview a patch artifact
  Given a user has selected a patch in the TUI
  When the user opens the preview pane
  Then the preview pane should display the patch file content in a readable terminal view

Scenario: Handle an empty workflow repo
  Given the repository contains no workstreams or patches
  When the user launches mxw ui
  Then the TUI should show an empty state
  And it should point the user toward creating a workstream or patch
```

## References

- `README.md`
- `workflow/workstreams/004-status-metadata-and-cli-support/ux.md`
- `https://ratatui.rs/`
