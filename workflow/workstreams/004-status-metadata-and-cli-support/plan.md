# Plan: Status Metadata And Cli Support

## Summary

Add a dedicated `STATUS.md` artifact with frontmatter for workstreams and extend the CLI with the first status-aware read and update operations, keeping the format simple enough for git and future Studio ingestion.

## Scope Analysis

### Affected Areas

| Area | Changes Required |
|------|-----------------|
| Workstream templates | Add a default `STATUS.md` template or generation path |
| CLI parsing | Add commands or flags for status read and update operations |
| Local git hooks | Refresh `updated` safely and warn before push when status review may be needed |
| Metadata parsing | Parse frontmatter from `STATUS.md` deterministically |
| README, skill, and methodology docs | Explain the stable-path status model and frontmatter contract |
| Future Studio integration | Define a metadata shape Studio can consume later |

### Affected Layers

- [x] Documentation
- [x] Workflow artifacts
- [ ] Scripts
- [x] CLI implementation

## Technical Design

### Proposed Additions

```text
resources/skills/mnemix-workflow/assets/workstream/STATUS.md
resources/hooks/
  pre-commit-status-updated
  pre-push-status-reminder
src/
  status.rs
  commands/
    status.rs
workflow/workstreams/004-status-metadata-and-cli-support/
```

### Metadata Contract

- `STATUS.md` should use frontmatter as the canonical machine-readable contract
- The initial required fields should be:
  - `status`
  - `summary`
  - `updated`
- The first optional linkage field should be:
  - `prs`
- The body should remain prose-only and should not restate those fields as a second source of truth
- Optional completion-oriented fields such as `completed_at` and `completed_by` can be added if the human wants them in v1

### CLI Surface

- Start with a small surface area, such as:
  - `mxw status <workstream>`
  - `mxw status set <workstream> <status>`
- `mxw new` should default the initial status to `open` while allowing an explicit override
- `mxw new` should create `STATUS.md` by default as part of the generated workstream shape
- `mxw list --status <value>` can land in this workstream if the human wants filtering in v1

### Local Hook Surface

- An optional local hook can refresh `updated` when workstream files change
- A local pre-push reminder can warn when workstream files changed and `STATUS.md` may need review
- Reminder messaging should clearly say that semantic changes like setting `status: completed` remain the user's responsibility
- PR-completion messaging should be phrased as a push-time reminder because local git hooks cannot run on actual GitHub PR creation

### Design Constraints

- Keep the status model simple enough to understand at a glance
- Avoid overcommitting to fields that Studio does not need yet
- Preserve stable workstream paths; status should not be encoded in folder layout
- Reuse the existing CLI conventions from `003` rather than inventing a parallel status tool

## Implementation Slices

### Slice 1: Add Status Artifact Support

- Add a default `STATUS.md` generation path for new workstreams
- Define the first metadata fields and allowed status values in code and templates
- Define how optional `prs` linkage is represented and preserved
- Decide how missing status files are recovered safely in CLI flows

### Slice 2: Add CLI Status Support

- Implement frontmatter parsing and validation
- Implement the first read/update status commands
- Add optional status-aware list filtering if it remains in scope
- Add tests for valid status reads, updates, and malformed metadata
- Extend tests for PR linkage, list filtering, or completion metadata only if those optional features are selected

### Slice 3: Add Local Hook Support

- Add an optional hook that updates `updated` for touched workstreams
- Add an optional pre-push reminder hook for status review and completion reminders
- Keep hook behavior low-risk and avoid auto-changing semantic fields like `status`

### Slice 4: Document The Stable-Path Model

- Update templates and README guidance
- Update the `mnemix-workflow` skill guidance to reflect the status workflow rules
- Document the local hook behavior and the status update rules clearly
- Explain why Mnemix Workflow uses status metadata instead of status folders
- Document how CLI and future Studio views consume the same metadata contract

## Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| The metadata contract grows too quickly | Medium | Medium | Keep the first version to a few required fields |
| Manual edits and CLI updates drift semantically | Medium | Low | Make frontmatter the only machine source of truth |
| CLI support becomes too broad in one workstream | Medium | Medium | Start with read/update primitives and leave advanced views for later |
| Hooks become too intrusive or surprising | Medium | Medium | Limit automatic hook changes to `updated` and keep semantic status changes as reminders only |

## Open Questions

- Should this workstream include optional completion metadata such as `completed_at` and `completed_by` in v1?

## References

- `workflow/workstreams/003-cli-bootstrap/spec.md`
- `workflow/workstreams/003-cli-bootstrap/plan.md`
- `workflow/decisions/004-use-status-files-for-workstream-state.md`
- `workflow/decisions/005-use-frontmatter-as-canonical-status-metadata.md`
- `workflow/decisions/006-start-with-proposed-open-completed-status-values.md`
- `workflow/decisions/007-resolve-planning-questions-during-workstream-creation.md`
- `workflow/decisions/008-create-status-file-when-a-workstream-is-created.md`
- `workflow/decisions/009-track-related-pull-requests-in-status-metadata.md`
- `docs/prd.md`
