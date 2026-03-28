# ADR 004: Use `STATUS.md` Files For Workstream State

## Status

Accepted

## Context

`mnemix-workflow` needs a clear way to represent whether a workstream is proposed, open, or completed.

Two main approaches were considered:

- encode state in folder placement such as `workstreams/open/` and `workstreams/completed/`
- keep stable workstream paths and store state inside each workstream

The long-term product vision favors CLI commands and Mnemix Studio as the primary surfaces for viewing and managing workstream state. In that model, the repository is the durable storage layer rather than the main dashboard. Folder-based status movement would create path churn, fragile links, and unnecessary git noise.

## Decision

Keep workstream paths stable under:

- `workflow/workstreams/<id>-<slug>/`

Represent lifecycle state inside each workstream with a `STATUS.md` file instead of encoding status in folder layout.

## Consequences

- Workstream paths stay stable over time
- CLI and Studio can become the main operational views over status
- Repository history avoids noisy folder moves when status changes
- Visual status in raw folder browsing is weaker, so tooling must provide the main status views
