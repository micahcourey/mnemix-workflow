# Tasks: Status Metadata And Cli Support

## Workstream Goal

Introduce a frontmatter-based `STATUS.md` contract for workstreams and the first CLI support for reading and updating that status without changing workstream folder paths.

## Execution Slices

### Slice 1: Define The Metadata Contract

- [x] Define the initial `STATUS.md` frontmatter fields
- [x] Decide the first allowed status values
- [x] Decide when `STATUS.md` is created for a workstream
- [x] Decide how PR linkage should be represented in status metadata

### Slice 2: Add Workstream Template Support

- [x] Add a `STATUS.md` template or generation path for new workstreams
- [x] Ensure the template keeps frontmatter canonical and prose non-duplicative
- [x] Ensure new workstreams receive `STATUS.md` by default

### Slice 3: Add CLI Status Support

- [x] Implement status parsing and validation
- [x] Implement the first CLI read command for workstream status
- [x] Implement the first CLI update command for workstream status
- [x] Preserve and surface optional `prs` linkage in status reads and writes
- [x] Implement status-aware workstream listing or filtering

### Slice 4: Add Local Hook Support

- [x] Add an optional low-risk hook that refreshes `updated` for touched workstreams
- [x] Add an optional pre-push reminder hook for status review
- [x] Make the reminder message explicitly say that a PR completing the workstream should update `STATUS.md` to `completed`

### Slice 5: Verify And Document

- [x] Add tests for valid status reads and writes
- [x] Add tests for malformed or missing status metadata
- [x] Add tests for hook behavior or at least script-level validation
- [x] Update the root README with the status workflow rules
- [x] Update the `mnemix-workflow` skill and references with the status workflow rules
- [x] Update methodology docs with the stable-path status model

## Validation Checklist

- [x] Workstreams can carry a `STATUS.md` file with frontmatter
- [x] New workstreams receive a `STATUS.md` file by default
- [x] The CLI can read a workstream status without inferring from prose
- [x] The CLI can update status without changing the workstream path
- [x] The status model can link workstreams to one or more PR numbers
- [x] Optional local hooks can refresh `updated` and remind the user about status responsibilities
- [x] The status model is clear enough for future Studio ingestion

## Notes

- This workstream should keep the first status support small and composable; richer workflow dashboards belong in later CLI and Studio work.
- Repo-level rationale for the status model lives in `workflow/decisions/004-use-status-files-for-workstream-state.md`, `workflow/decisions/005-use-frontmatter-as-canonical-status-metadata.md`, and `workflow/decisions/006-start-with-proposed-open-completed-status-values.md`.
- Workstream creation should resolve material planning questions up front rather than leaving a generic `Open Questions` section in the artifacts.
- Optional features that the human has not decided yet can stay in a focused `Open Questions` section until they are resolved.
