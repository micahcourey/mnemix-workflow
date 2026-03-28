# Feature Spec: Status Metadata And Cli Support

## Summary

Define a stable `STATUS.md` artifact for workstreams using frontmatter as the machine-readable source of truth, and add the first CLI support for reading and updating workstream status so both `mxw` and future Studio views can rely on the same metadata contract.

## Problem

`mnemix-workflow` can create workstreams, but it does not yet have a first-class way to represent whether a workstream is proposed, open, or completed. Folder-based status would create path churn and weak alignment with the long-term product vision, where status should be surfaced through CLI commands and eventually Mnemix Studio rather than by moving files around in the repository.

## Users

- Primary persona: maintainer or AI implementation agent tracking the state of workstreams
- Secondary persona: future Mnemix Studio user needing reliable machine-readable status data

## Goals

- Define a durable `STATUS.md` format that is easy for humans to read and easy for tools to parse
- Keep the workstream path stable regardless of status changes
- Add initial CLI support for creating, reading, and updating workstream status metadata
- Establish a metadata shape that Studio can consume later without guessing from prose

## Non-Goals

- Build the full Studio workstream management experience
- Add every possible workflow field in the first version
- Introduce folder-based status movement such as `open/` or `completed/`
- Replace `tasks.md` as the execution checklist artifact

## User Value

Maintainers and agents get a clear, explicit way to know whether a workstream is proposed, open, or completed without relying on path conventions or incomplete checklists, and future Studio views gain a structured source of truth they can build on directly.

## Functional Requirements

- Each workstream should be able to contain a `STATUS.md` file
- `STATUS.md` should use frontmatter as the canonical machine-readable source of truth
- The frontmatter should require `status`, `summary`, and `updated`
- The frontmatter should support an optional `prs` field for related pull request numbers
- The body of `STATUS.md` should remain human-readable without duplicating the frontmatter as a second source of truth
- The CLI should create a default `STATUS.md` when a workstream is created
- The CLI should support reading workstream status in a structured, deterministic way
- The CLI should support updating status values such as `proposed`, `open`, and `completed`
- The CLI should default newly created workstreams to `open` unless the user provides an explicit initial status
- The CLI should support listing or filtering workstreams by status using the frontmatter value
- The CLI should preserve and eventually support updating optional PR linkage metadata
- The framework should support an optional local git hook that refreshes `updated` when workstream artifacts change
- The framework should support an optional local reminder hook that warns when workstream files changed and `STATUS.md` may need review before push
- The framework docs should clearly state that if a PR completes a workstream, the user should update `STATUS.md` to `completed`

## Status Model

- `proposed`
- `open`
- `completed`

## Constraints

- The status format should stay lightweight and repo-native
- The CLI should not infer status from checked boxes or prose
- The body text in `STATUS.md` should not duplicate canonical machine fields
- The metadata format should remain friendly to future Studio integration

## Success Criteria

- A workstream can be marked `proposed`, `open`, or `completed` without changing its folder path
- The CLI can read and update status deterministically
- Optional local hooks can keep `updated` fresher and remind the user about status responsibilities without auto-changing semantic status fields
- The status contract is clear enough that Studio could consume it later without additional repository changes
- The solution stays lightweight enough to fit the `mnemix-workflow` philosophy

## Risks

- The metadata format may be overdesigned if too many fields are added up front
- CLI status operations may blur the line between planning data and execution metadata if they become too heavy
- If the frontmatter contract is unclear, Studio and CLI may diverge later

## Open Questions

- Should `completed_at` and `completed_by` land in v1, or stay optional until the human decides there is enough reporting value?

## References

- `docs/prd.md`
- `workflow/workstreams/003-cli-bootstrap/spec.md`
- `workflow/workstreams/003-cli-bootstrap/plan.md`
- `workflow/decisions/004-use-status-files-for-workstream-state.md`
- `workflow/decisions/005-use-frontmatter-as-canonical-status-metadata.md`
- `workflow/decisions/006-start-with-proposed-open-completed-status-values.md`
- `workflow/decisions/007-resolve-planning-questions-during-workstream-creation.md`
- `workflow/decisions/008-create-status-file-when-a-workstream-is-created.md`
- `workflow/decisions/009-track-related-pull-requests-in-status-metadata.md`
- `README.md`
