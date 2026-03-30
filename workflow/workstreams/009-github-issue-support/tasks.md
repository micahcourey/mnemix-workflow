# Tasks: Github Issue Support

## Workstream Goal

Add optional, repo-canonical GitHub issue mirroring for workstreams and patches
so repositories can project planned work into GitHub without moving the source
of truth out of repo artifacts.

## Execution Slices

### Slice 1

- [x] Define the GitHub config shape in `workflow/github.yml`
- [x] Define repo metadata fields for linked parent issues, sub-issues, and patch issues
- [x] Define the issue body templates for parent issues, file sub-issues, and patches
- [x] Decide and document the exact status-to-issue-state mapping
- [x] Decide and document `--all` backfill and `--status` filtered sync behavior

### Slice 2

- [x] Implement `mxw github init`
- [x] Implement `mxw github sync <target>` for workstreams and patches
- [x] Implement `mxw github sync --all` for backfill
- [x] Implement filtered sync such as `mxw github sync --status open`
- [x] Create or update one parent issue plus `spec`, `ux`, `plan`, and `tasks` sub-issues for workstreams
- [x] Create or update one mirrored issue for patches
- [x] Persist GitHub issue linkage back into repo metadata after sync
- [x] Keep first mirror creation as an explicit manual sync path in v1

### Slice 3

- [x] Implement `mxw github sync --changed`
- [x] Add dry-run or preview output so users can review intended issue changes
- [x] Add optional GitHub Actions auto-sync on push to `main` using `mxw github sync --changed`
- [x] Update README, PRD, skill docs, and conventions docs with the repo-canonical issue model
- [x] Add guidance recommending restricted GitHub issue edit/create permissions where practical

## Validation Checklist

- [x] A workstream can be mirrored into one parent issue and four sub-issues
- [x] A patch can be mirrored into one GitHub issue
- [x] Re-running sync updates existing issues instead of duplicating them
- [x] `mxw github sync --all` can backfill previously-created tracked items including completed items
- [x] Completed items are mirrored as closed GitHub issues
- [x] Manual GitHub body edits are overwritten on sync from the repo
- [x] `mxw github sync --changed` updates already-linked changed items without creating unrelated new mirrors
- [x] Filtered sync updates only the requested subset of tracked items
- [x] Auto-sync remains optional and can be disabled without breaking repo-native workflow
- [x] Documentation clearly states that repo artifacts remain the source of truth

## Notes

- Future `mnx` and Studio views should be able to surface linked GitHub issues without changing the core repo model.
