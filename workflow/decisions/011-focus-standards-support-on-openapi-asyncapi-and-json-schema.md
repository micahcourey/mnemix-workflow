# ADR 011: Focus Standards Support On OpenAPI, AsyncAPI, And JSON Schema

## Status

Accepted

## Context

`mnemix-workflow` originally described a broader open-standards story that
included `MADR` and `Structurizr DSL` alongside `OpenAPI`, `AsyncAPI`, and
`JSON Schema`.

That framing was too broad for the product we are actually building. The
strongest fit for first-class standards support is machine-readable API and
data contracts. Decisions, architecture, and UX are still important, but they
do not yet need dedicated standards-specific CLI support to make the framework
useful.

## Decision

Narrow the planned standards support in `mnemix-workflow` to:

- `OpenAPI`
- `AsyncAPI`
- `JSON Schema`

Treat these as the supported contract standards for future CLI, validation, and
examples work.

Keep decisions and architecture repo-native by default:

- durable decisions continue to live in `workflow/decisions/`
- architecture remains optional documentation unless a later workstream adds a
  clearer integration need

## Consequences

Positive:

- the standards story is clearer and easier to teach
- implementation effort stays focused on the standards with the strongest
  machine-readable value
- documentation no longer overstates support that the CLI does not yet provide

Tradeoffs:

- teams wanting first-class ADR or architecture tooling will rely on repo-native
  conventions for now
- future architecture or decision standards support would need a new deliberate
  workstream rather than being implied by the README
