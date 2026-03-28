# Plan: Interactive Tui Mode

## Summary

Build a narrow v1 terminal UI on top of the existing Rust CLI and workflow
data model. The first release should focus on browsing workstreams and patches
by status and previewing artifacts, with a small number of lightweight
actions. The internal architecture should leave room for future workstreams to add direct
agent-assisted planning without forcing that complexity into v1.

## Scope Analysis

### Affected Areas

| Area | Changes Required |
|------|-----------------|
| CLI surface | Add an interactive entrypoint such as `mxw ui` |
| TUI runtime | Add terminal event loop, layout, and state management |
| Workflow data access | Reuse status and tracked-item loading in a browsable model |
| Markdown preview | Render core workstream files and patch files readably in a terminal pane |
| Documentation | Explain the new interactive mode and how it complements static commands |

### Affected Layers

- [x] Documentation
- [ ] Workflow artifacts
- [ ] Scripts
- [x] CLI implementation

## Technical Design

### Proposed Additions

```text
src/
  tui/
    mod.rs
    app.rs
    state.rs
    render.rs
    events.rs
    data.rs
  commands/
    ui.rs
docs/
  methodology/
tests/
  cli.rs
```

### Runtime Approach

- Use `ratatui` for layout and widgets
- Use `crossterm` for terminal setup and keyboard event handling
- Reuse the existing workstream, patch, and status loading logic instead of building a separate storage path

### V1 Layout

- left pane: status buckets
- center pane: tracked item list
- right pane: artifact preview
- footer: current key hints and mode

### V1 Actions

- browse statuses
- browse tracked items
- preview core artifacts and patch files
- optionally trigger a small workflow action such as a status change or artifact switch

### Long-Term Vision

Future workstreams can extend this architecture with:

- guided workstream creation
- status and PR updates inline
- direct agent-assisted planning inside the TUI
- structured planning prompts that draft `spec.md`, `ux.md`, `plan.md`, and `tasks.md`
- deeper handoff between the TUI and future Mnemix Studio views

## Design Constraints

- Keep v1 browse-first and avoid turning the TUI into a full agent shell
- Keep the implementation modular so future agent integration can plug into the action layer
- Preserve the existing static CLI for scripting and automation
- Prefer a readable terminal preview over a perfect Markdown renderer in v1

## Implementation Slices

### Slice 1: Add The Interactive Entry Point

- Add a new CLI command such as `mxw ui`
- Set up terminal initialization, teardown, and event loop boundaries
- Define the core app state and selection model

### Slice 2: Build The Browse-First TUI

- Render status buckets, workstream list, and artifact preview panes
- Load and display workstream and patch data from the current repository
- Support artifact switching and scrolling

### Slice 3: Add Lightweight Workflow Actions

- Add one or two small actions that prove the TUI can become operational, not just read-only
- Keep semantic changes explicit and aligned with existing CLI behavior

### Slice 4: Document The TUI And Future Direction

- Update README with the interactive mode
- Document keyboard usage and the v1 scope
- Document the long-term direction for future workstreams, including direct agent-assisted planning

## Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| The TUI grows too ambitious | High | Medium | Keep v1 focused on browse and inspect flows |
| Terminal rendering complexity slows delivery | Medium | Medium | Use straightforward pane rendering and simple Markdown preview rules |
| The architecture blocks future agent integration | High | Low | Separate app state, data access, rendering, and action layers from the start |
| The TUI duplicates static commands awkwardly | Medium | Medium | Treat static commands as the automation layer and the TUI as the interactive layer |

## References

- `docs/prd.md`
- `README.md`
- `workflow/workstreams/004-status-metadata-and-cli-support/plan.md`
- `https://ratatui.rs/`
- `https://docs.rs/ratatui/`
- `https://docs.rs/crossterm/`
