# Tasks: Interactive Tui Mode

## Workstream Goal

Plan and then implement a browse-first interactive terminal UI for
`mnemix-workflow` that makes tracked-item navigation and artifact inspection
much faster than the current static commands, while preserving a future path
toward direct agent-assisted planning.

## Execution Slices

### Slice 1: Define The V1 TUI Scope

- [x] Define the v1 scope as browse-first rather than full agent interaction
- [x] Define the core panes, interactions, and artifact preview flow
- [x] Define the long-term direction for later agent-assisted planning workstreams

### Slice 2: Implement The Interactive Entry Point

- [ ] Add a new interactive CLI command such as `mxw ui`
- [ ] Add terminal setup, teardown, and event handling
- [ ] Add a central TUI app state model

### Slice 3: Build The Browse Experience

- [ ] Render status buckets and tracked-item lists
- [ ] Render artifact preview for workstream artifacts and patch files
- [ ] Add keyboard navigation and scrolling

### Slice 4: Add Lightweight Actions

- [ ] Add one or two small workflow actions to prove the TUI can evolve beyond read-only browsing
- [ ] Keep those actions aligned with existing CLI behavior

### Slice 5: Verify And Document

- [ ] Add tests for TUI entry and core state behavior where practical
- [ ] Update the README with the interactive mode
- [ ] Document keyboard usage and the v1 scope clearly

## Validation Checklist

- [ ] A user can launch an interactive mode from `mxw`
- [ ] A user can browse workstreams and patches by status
- [ ] A user can preview the main workstream artifacts and patch files inside the terminal
- [ ] The architecture leaves a clear path for later direct agent-assisted planning
- [ ] The v1 scope remains intentionally smaller than a full terminal agent shell

## Notes

- This workstream should establish the TUI foundation, not finish the entire long-term terminal product vision.
- Later workstreams can build on this with richer actions, guided creation flows, and direct agent-assisted planning.
