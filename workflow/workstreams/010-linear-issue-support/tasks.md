# Tasks: Linear Issue Support

## Workstream Goal

Add optional, repo-canonical Linear issue mirroring for workstreams and patches
using an issue-first model that keeps Projects out of the first slice.

## Execution Slices

### Slice 1

- [x] Research Linear's official issue, sub-issue, project, GraphQL, auth, and webhook docs
- [x] Decide that v1 will use issues and child issues rather than Projects
- [ ] Define the Linear config shape in `workflow/linear.yml`
- [ ] Define repo metadata fields for linked parent issues, child issues, and patch issues
- [ ] Define the issue body templates for parent issues, child issues, and patches
- [ ] Decide and document the status-to-Linear-state mapping
- [ ] Decide and document the required team mapping model
- [ ] Decide the first auth mode for v1 and document later OAuth expansion

### Slice 2

- [ ] Implement `mxw linear init`
- [ ] Implement `mxw linear sync <target>` for workstreams and patches
- [ ] Implement `mxw linear sync --all` for backfill
- [ ] Implement filtered sync such as `mxw linear sync --status open`
- [ ] Create or update one parent issue plus child issues for `spec.md`, `ux.md`, `plan.md`, and `tasks.md`
- [ ] Create or update one mirrored issue for patches
- [ ] Persist Linear issue linkage back into repo metadata after sync

### Slice 3

- [ ] Implement `mxw linear sync --changed`
- [ ] Add dry-run or preview output so users can review intended Linear changes
- [ ] Add optional automation support for changed-only Linear sync
- [ ] Update README, PRD, skill docs, and conventions docs with the repo-canonical Linear model
- [ ] Document why Projects are intentionally deferred from the first slice

## Validation Checklist

- [ ] A workstream can be mirrored into one parent Linear issue and four child issues
- [ ] A patch can be mirrored into one Linear issue
- [ ] Re-running sync updates existing Linear issues instead of duplicating them
- [ ] `mxw linear sync --all` can backfill previously-created tracked items including completed items
- [ ] Filtered sync updates only the requested subset of tracked items
- [ ] `mxw linear sync --changed` updates already-linked changed items without creating unrelated new mirrors
- [ ] Documentation clearly states that repo artifacts remain the source of truth
- [ ] Documentation clearly states that Projects are out of scope for v1

## Notes

- Future work can add a Projects mode if teams need promoted, larger planning containers, but the first slice should stay issue-first.
