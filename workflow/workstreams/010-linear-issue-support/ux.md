# UX Spec: Linear Issue Support

## Summary

The CLI should make Linear support feel like a natural sibling to GitHub
support: explicit, repo-canonical, and predictable. Maintainers should be able
to initialize Linear sync, mirror one tracked item or a filtered slice, and
trust that reruns update the same Linear issues instead of creating tracker
drift.

## Users And Context

- Primary persona: maintainer or engineering lead
- Context of use: a repository that already uses `mnemix-workflow` and wants Linear as its execution surface
- Preconditions: the repo has tracked workstreams and patches, and the maintainer has valid Linear credentials plus a chosen team mapping

## User Goals

- Mirror repo-tracked work into Linear without copy/pasting descriptions by hand
- Keep the repo as the planning source of truth even when the team operates day to day in Linear
- Backfill older tracked work or sync only open work when rolling the integration out

## Experience Principles

- Repo-first and tracker-second
- Clear sync scope with low surprise
- Same mental model as GitHub support where possible

## Primary Journey

1. A maintainer enables Linear support with `mxw linear init`.
2. The CLI writes repo config for workspace/team mapping and chosen auth mode.
3. The maintainer runs `mxw linear sync 010` for a workstream.
4. The CLI creates or updates one parent Linear issue plus child issues for `spec.md`, `ux.md`, `plan.md`, and `tasks.md`.
5. The CLI writes Linear linkage metadata back into the repo artifact.
6. On later changes, the maintainer re-runs sync manually or uses optional automation built around `mxw linear sync --changed`.

## Alternate Flows

### Flow: Backfill Existing Work

- Trigger: a repo adopts Linear support after already using `mnemix-workflow`
- Path: `mxw linear sync --all` or `mxw linear sync --status open --all`
- Expected outcome: older workstreams and patches are mirrored into Linear, with completed items mapped appropriately from repo status

## Surfaces

### Surface: `mxw linear init`

- Purpose: configure repository-level Linear support
- Key information: workspace slug or team key, auth mode, auto-sync mode, repo-canonical guidance
- Available actions: initialize config, opt into future auto-sync, verify prerequisites
- Navigation expectations: short and explicit, similar to `mxw github init`

### Surface: `mxw linear sync`

- Purpose: create or update mirrored Linear issues from repo artifacts
- Key information: sync scope, affected tracked items, created or updated issue identifiers, and any skipped items
- Available actions: target sync, backfill sync, filtered sync, changed-only sync, dry-run
- Navigation expectations: output should clearly separate parent issue updates from child issue updates

## States

### Loading

- Show which tracked item or sync mode is being processed

### Empty

- If no tracked items match the requested scope, explain that nothing matched instead of silently succeeding

### Success

- Report the Linear issue identifiers created or updated and confirm repo linkage metadata was refreshed

### Error

- Show actionable messages for missing auth, missing team mapping, API permission problems, or unsupported targets

## Interaction Details

- Input behavior should mirror GitHub support flags where possible
- Feedback should make it obvious that Linear issue descriptions are system-managed mirrors
- Dry-run mode should print intended creates or updates without mutating Linear or repo metadata
- Errors should distinguish configuration issues from API or auth failures

## Content And Tone

- Important messages should reinforce that the repo remains canonical
- Wording should avoid implying that users should edit Linear descriptions directly

## Accessibility Requirements

- CLI help and output must remain readable in standard terminal flows
- Future TUI or Studio Linear views should preserve keyboard-first navigation expectations established elsewhere in the product
- Text output should avoid relying on color alone to distinguish created, updated, or skipped items

## Acceptance Scenarios

```gherkin
Scenario: Sync one workstream into Linear
  Given a repository has enabled Linear support
  And workstream 010 exists in repo artifacts
  When the user runs `mxw linear sync 010`
  Then the CLI should create or update one parent Linear issue
  And the CLI should create or update child issues for spec, ux, plan, and tasks
  And the repo should store the resulting Linear linkage metadata

Scenario: Backfill only open tracked work
  Given a repository has both open and completed tracked items
  When the user runs `mxw linear sync --status open --all`
  Then only open tracked items should be mirrored or refreshed
  And completed tracked items should not be touched by that sync run
```

## References

- `workflow/workstreams/009-github-issue-support/ux.md`
- `https://linear.app/docs/parent-and-sub-issues`
- `https://linear.app/docs/conceptual-model`
