# ADR 010: Use `workflow/patches/` For Lightweight Tracked Work

## Status

Accepted

## Context

`mnemix-workflow` currently treats the full workstream as the primary planning
unit. That is appropriate for larger changes, but it adds too much ceremony for
narrow bug fixes, chores, and small enhancements. At the same time, the
framework should keep the rule that every pull request is tracked in Mnemix
Workflow.

We considered more branded names for the lightweight lane, including
`currents`, but chose a clearer and more immediately understandable term.

## Decision

Use `workflow/patches/` as the lightweight tracked lane for small changes.

Each patch will:

- be represented by a single Markdown file
- use a zero-padded 4-digit numeric prefix
- reuse the existing status metadata model
- remain lighter than a full workstream

Every pull request should map to either:

- a workstream in `workflow/workstreams/`, or
- a patch in `workflow/patches/`

## Consequences

Positive:

- small changes can remain planned and tracked without full workstream overhead
- the naming is clear to new users
- future CLI, TUI, and Studio views can include patches without inventing a new lifecycle model

Tradeoffs:

- the framework now has two tracked planning lanes instead of one
- guidance is needed so teams choose consistently between a patch and a full workstream
- patch templates and CLI support must be added to make the lane truly ergonomic
