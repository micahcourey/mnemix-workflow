# ADR 005: Use Frontmatter As The Canonical Status Metadata Format

## Status

Accepted

## Context

Once `STATUS.md` was chosen as the artifact for workstream state, the next design question was how tools should parse it.

Options discussed included:

- plain Markdown with lines like `Status: Open`
- frontmatter plus human-readable prose
- standalone JSON or YAML files
- duplicating status and summary in both machine-readable and human-readable sections

Performance was not the deciding factor. These files are tiny, so parsing speed differences are negligible compared with filesystem access. The more important concerns are clarity, extensibility, and avoiding drift between what humans read and what tools parse.

## Decision

Use frontmatter in `STATUS.md` as the canonical machine-readable source of truth.

The initial required fields should be:

- `status`
- `summary`
- `updated`

The body of `STATUS.md` should remain human-readable prose and should not duplicate the frontmatter as a second source of truth.

## Consequences

- CLI and Studio get a stable, explicit metadata contract
- The file remains readable in git and plain text editors
- The status artifact can grow gradually without switching formats later
- Validation becomes important because malformed frontmatter is now a real failure mode
