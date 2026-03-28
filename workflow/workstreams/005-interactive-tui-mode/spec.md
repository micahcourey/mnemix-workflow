# Feature Spec: Interactive Tui Mode

## Summary

Add a browse-first interactive terminal mode to `mxw` so users can navigate
tracked workflow items by status, inspect workstream and patch artifacts
directly in the terminal, and perform a few lightweight workflow actions
without leaving the CLI.

## Problem

`mnemix-workflow` now has useful static commands, but the user experience is
still command-by-command and file-by-file. That makes it harder to scan the
current state of a repository, compare workstreams across statuses, and inspect
artifact content quickly. A richer terminal interface would make the workflow
feel more approachable and more product-like while still staying repo-native.

## Users

- Primary persona: maintainer or planner managing multiple tracked workflow items in a repository
- Secondary persona: AI operator or contributor who wants a fast terminal-native overview of workflow state

## Goals

- Add a full-screen interactive mode such as `mxw ui` for browsing workstreams and patches
- Make status-based navigation and artifact inspection much faster than one-off static commands
- Keep v1 scope intentionally narrow and read-focused, with only lightweight workflow actions
- Preserve a clean architecture for future agent-assisted planning inside the TUI

## Non-Goals

- Build a full terminal chat agent in v1
- Replace the existing static CLI commands
- Support inline editing of every Markdown artifact in the first release
- Finalize all future agent-integration details in this workstream

## User Value

Users get a much clearer operational view of workflow state directly in the
terminal: they can see what is proposed, open, or completed, jump into a
specific workstream or patch, and inspect the relevant planning artifacts
without shelling out to other tools.

## Functional Requirements

- The CLI should expose an interactive mode such as `mxw ui`
- The TUI should show workstreams and patches grouped or filtered by status
- The TUI should let the user select a tracked item and preview its core artifacts
- The TUI should support at least `STATUS.md`, `spec.md`, `ux.md`, `plan.md`, and `tasks.md` for workstreams, plus patch-file previews for patches
- The TUI should provide lightweight keyboard-driven navigation between status views, tracked-item lists, and artifact previews
- The TUI should allow at least one narrow workflow action in v1, such as changing status or opening an artifact selection
- The TUI should reuse the existing repo scanning and status metadata logic rather than inventing a parallel data path
- The implementation should preserve a future extension point for direct agent-assisted planning in later workstreams

## V1 Scope

- Full-screen terminal UI with:
  - status view
  - tracked item list
  - artifact preview pane
  - lightweight action/help footer
- Read-first workflow with one or two small actions
- Terminal Markdown preview that is readable, even if not fully rich-rendered

## Future Vision

Later workstreams can extend the TUI toward:

- guided workstream creation
- status and PR updates inline
- direct agent-assisted planning inside the TUI
- structured planning prompts that draft `spec.md`, `ux.md`, `plan.md`, and `tasks.md`
- deeper Studio-aligned workflow orchestration

## Constraints

- Keep the v1 scope browse-first so the first TUI remains understandable and shippable
- Reuse the current Rust CLI foundation rather than building a separate app
- Keep the artifact model repo-native and file-backed
- Design the state and action layers so future agent integration is possible without rewriting the UI core

## Success Criteria

- A user can launch an interactive CLI mode and browse workstreams and patches by status
- A user can inspect the content of a selected workstream or patch artifact inside the terminal
- The TUI feels clearly more useful for navigation than the static commands alone
- The implementation remains small enough that future agent integration can build on it rather than replace it

## Risks

- The TUI could become too ambitious and delay a useful v1
- Terminal Markdown rendering could become a sink for polish work if not kept simple
- A too-chat-like design could blur the line between workflow browsing and future agent integration prematurely

## References

- `docs/prd.md`
- `README.md`
- `workflow/workstreams/004-status-metadata-and-cli-support/spec.md`
- `https://ratatui.rs/`
- `https://docs.rs/ratatui/`
- `https://docs.rs/crossterm/`
