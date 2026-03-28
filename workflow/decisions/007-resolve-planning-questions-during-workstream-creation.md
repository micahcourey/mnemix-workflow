# ADR 007: Resolve Planning Questions During Workstream Creation

## Status

Accepted

## Context

Early workstream templates included generic `Open Questions` sections and plan phases devoted to deciding things that should already have been resolved during planning.

That shape made workstreams feel less implementation-ready and encouraged agents to defer important choices into the artifacts instead of resolving them with the user while the workstream was being created.

## Decision

Treat workstream creation as the point where material planning questions are surfaced and, whenever possible, resolved with the human creating the workstream.

As a result:

- default workstream templates should not include generic `Open Questions` sections
- `plan.md` should focus on implementation approach and execution slices rather than a separate decision phase by default
- agents should ask necessary clarifying questions while creating the workstream so the resulting artifacts are ready to execute unless the user requests changes
- if the human explicitly leaves a meaningful question unresolved, the workstream may add a focused `Open Questions` section or a decision-oriented plan slice for that specific gap

## Consequences

- New workstreams usually start in a more execution-ready state
- Planning artifacts stay lighter and more decisive in the common case
- Truly unresolved dependencies can still be captured intentionally when the human does not want to decide yet
