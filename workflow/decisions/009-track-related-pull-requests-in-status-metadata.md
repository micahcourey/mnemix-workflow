# ADR 009: Track Related Pull Requests In Status Metadata

## Status

Accepted

## Context

Workstreams often map to one or more implementation PRs, and maintainers need a lightweight way to connect planning artifacts with the code review history.

That linkage should be easy for humans to read, easy for tooling to parse, and flexible enough to support more than one PR for a single workstream.

## Decision

Support an optional `prs` field in `STATUS.md` frontmatter.

The field should hold a list of related pull request numbers for the current repository, for example:

```yaml
prs: [3, 7]
```

The initial required fields remain:

- `status`
- `summary`
- `updated`

`prs` is optional metadata, not a required field for every workstream.

## Consequences

- Workstreams can point cleanly at one or more implementation PRs
- CLI and future Studio views can surface planning-to-code linkage without scraping prose
- The metadata contract stays lightweight because PR linkage is additive rather than required
