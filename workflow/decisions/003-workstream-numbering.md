# ADR 003: Use 3-Digit Workstream IDs With 1000+ Overflow

## Status

Accepted

## Context

`mnemix-workflow` needs a numbering convention that is friendly in the common case but still scales past long-lived project growth.

## Decision

Use:

- `001` through `999` as zero-padded 3-digit workstream ids
- `1000+` as the overflow format after `999`

All tooling must sort and increment workstream ids numerically, not lexicographically.

## Consequences

- The default case stays familiar and compact
- The convention scales without renumbering old workstreams
- Tooling must parse numeric prefixes rather than assuming a fixed width forever
