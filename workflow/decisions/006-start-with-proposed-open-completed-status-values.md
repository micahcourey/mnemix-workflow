# ADR 006: Start With `proposed`, `open`, And `completed` Status Values

## Status

Accepted

## Context

The first status-aware version of `mnemix-workflow` needs a small lifecycle model that is easy to understand and easy to support in the CLI and future Studio views.

The status system should be expressive enough to separate:

- not-yet-started work
- active work
- finished work

But it should avoid expanding into a full workflow taxonomy before real usage proves the need.

## Decision

Start with three supported status values:

- `proposed`
- `open`
- `completed`

Treat richer states such as `blocked`, `archived`, or reporting-oriented completion fields as later additions if usage demands them.

## Consequences

- The first CLI support stays small and predictable
- Studio can build useful status filters without a complex lifecycle model
- Teams may still rely on prose or other artifacts for more nuanced workflow meaning until the model evolves
