# ADR 008: Create `STATUS.md` When A Workstream Is Created

## Status

Accepted

## Context

Once `STATUS.md` was chosen as the durable artifact for workstream state, the next question was when that file should appear.

Options considered included:

- creating `STATUS.md` only when a later status command is used
- creating `STATUS.md` during repository initialization
- creating `STATUS.md` when each new workstream is created

Deferring creation until later would make status support feel optional and inconsistent, and it would force the CLI or Studio to handle more missing-file edge cases in the common path.

## Decision

Create `STATUS.md` by default whenever a new workstream is created.

New workstreams should start with:

- `status: open`
- `summary`
- `updated`

`mxw new` may later allow an explicit initial status override, but a status file should exist from the start either way.

## Consequences

- Every workstream has a consistent status artifact from day one
- CLI and Studio can rely on `STATUS.md` existing in the normal case
- The framework dogfoods status metadata as a first-class part of the workstream shape
