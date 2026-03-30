# UX Spec: Github Issue Support

## Summary

The GitHub issues experience should feel like an optional projection layer on
top of the repo-native workflow, not a second planning system. Users should be
able to enable issue support, sync a workstream or patch, and trust that GitHub
now reflects the repo state without needing to manually rewrite or maintain
issue bodies.

## Users And Context

- Primary persona: maintainer working in a repository that already uses GitHub Issues for day-to-day execution
- Context of use: terminal-driven workflow management in a repository that adopts Mnemix Workflow conventions
- Preconditions: the repository uses `workflow/` artifacts, GitHub issue support has been configured for the target repo, and the user has permission to create and edit issues

## User Goals

- Mirror repo-managed work into GitHub without duplicating planning effort
- Keep issue structure predictable enough that humans can browse it and automation can trust it
- Avoid manual issue maintenance for titles, descriptions, and status-driven open/closed behavior

## Experience Principles

- Repo-first: all authoritative planning and status changes happen in repo artifacts
- Optional adoption: GitHub issue support should layer onto the existing methodology without becoming mandatory
- Deterministic sync: repeated sync runs should be safe, predictable, and idempotent
- Clear ownership: the CLI should make it obvious that mirrored issues are system-managed

## Primary Journey

1. A maintainer enables GitHub issue support for the repository with `mxw github init` or equivalent setup.
2. The CLI creates a small repo config describing the GitHub repository target and issue-sync behavior.
3. The maintainer runs `mxw github sync 009` for a workstream.
4. The CLI creates or updates a parent issue for the workstream.
5. The CLI creates or updates sub-issues for `spec.md`, `ux.md`, `plan.md`, and `tasks.md`, then links them beneath the parent issue.
6. The workstream `STATUS.md` stores the parent and sub-issue linkage so later syncs are incremental and deterministic.
7. On later repo changes, the maintainer can re-run sync manually or rely on configured auto-sync, which uses `mxw github sync --changed` to refresh already-linked mirrors.

## Alternate Flows

### Flow: Patch Sync

- Trigger: a user wants GitHub visibility for a lightweight patch rather than a full workstream
- Path: `mxw github sync 0008` creates or updates one mirrored issue for the patch file
- Expected outcome: the patch shows up in GitHub as a single tracked issue without sub-issues

### Flow: Auto-Sync Existing Mirrors

- Trigger: a push to `main` changes repo artifacts that already have GitHub issue linkage
- Path: a GitHub Action runs `mxw github sync --changed`
- Expected outcome: only already-linked workstreams and patches are refreshed; brand-new mirrors are not created implicitly

### Flow: Backfill Existing History

- Trigger: a repository has already been using Mnemix Workflow before enabling GitHub issue support
- Path: a maintainer runs `mxw github sync --all` or a filtered variant such as `mxw github sync --status open`
- Expected outcome: existing workstreams and patches are mirrored into GitHub, and completed items are created as closed issues

## Surfaces

### Surface: CLI Setup

- Purpose: enable GitHub issue support for a repository
- Key information: target repository, whether sync is enabled, and whether auto-sync is configured
- Available actions: initialize config, validate config, enable or disable sync
- Navigation expectations: setup should be explicit and low-ceremony because issue support is optional

### Surface: CLI Sync

- Purpose: project repo artifacts into GitHub issues
- Key information: target workstream or patch, created versus updated issue counts, and issue links
- Available actions: sync one item, sync all items, dry-run preview, and validate current issue mappings
- Navigation expectations: the command output should clearly say when GitHub issues are system-managed mirrors

### Surface: Future `mnx` / Studio Views

- Purpose: show GitHub linkage and sync state without replacing repo-native artifact views
- Key information: linked parent issue, linked sub-issues, last sync status, and any drift warnings
- Available actions: open linked issues, trigger a sync, and inspect mirrored relationships
- Navigation expectations: issue mirrors should appear as an extension of the workstream or patch, not as a separate planning system

## States

### Loading

- Show that the CLI is connecting to GitHub and resolving current linkage before creating or updating issues

### Empty

- If no GitHub config exists, the CLI should explain how to initialize support instead of failing opaquely
- If a workstream or patch has no linked issues yet, the CLI should treat the next sync as a first creation flow

### Success

- Show what was created versus updated
- Show parent issue and sub-issue numbers or URLs
- Remind the user that issue bodies are repo-managed mirrors

### Error

- If the GitHub token or permissions are insufficient, show the missing capability clearly
- If issue linkage metadata is malformed, explain which repo file needs correction
- If GitHub issue support is disabled for the repo, fail with a clear setup path instead of implicit behavior

## Interaction Details

- Sync commands should support a dry-run mode so users can preview intended issue changes
- Output should clearly distinguish workstream parent issues from file sub-issues
- Status mapping should be explicit, e.g. repo `completed` maps to closed issues on sync
- Backfill commands should make it clear whether they are syncing all tracked items or a filtered subset
- Auto-sync should be opt-in, clearly described in docs and config, and based on `--changed` rather than full-repo sync by default

## Acceptance Scenarios

```gherkin
Scenario: Sync a workstream into GitHub issues
  Given a repository has enabled GitHub issue support
  And workstream 009 exists with spec, ux, plan, tasks, and status metadata
  When the user runs `mxw github sync 009`
  Then the CLI should create or update one parent issue for the workstream
  And the CLI should create or update sub-issues for spec, ux, plan, and tasks
  And the workstream status metadata should store the GitHub linkage

Scenario: Sync a patch into GitHub issues
  Given a repository has enabled GitHub issue support
  And patch 0008 exists
  When the user runs `mxw github sync 0008`
  Then the CLI should create or update one mirrored GitHub issue
  And the patch frontmatter should store the linked issue number

Scenario: Manual GitHub body edits are overwritten on sync
  Given a mirrored workstream issue was manually edited on GitHub
  When the user runs `mxw github sync 009`
  Then the issue title and body should be replaced from the repo artifacts
  And the CLI should treat the repo as the source of truth

Scenario: Auto-sync refreshes already-linked items only
  Given a repository has enabled GitHub issue support
  And workstream 009 already has linked GitHub issues
  And a push to `main` changes files inside that workstream
  When the GitHub Action runs `mxw github sync --changed`
  Then the linked GitHub issues should be updated from the repo
  And the action should not create brand-new mirrors for unrelated unlinked items

Scenario: Backfill previously completed work
  Given a repository used Mnemix Workflow before enabling GitHub issue support
  And workstream 003 is already marked completed in repo metadata
  When the maintainer runs `mxw github sync --all`
  Then workstream 003 should be mirrored into GitHub
  And the created mirrored issues should be closed to match the repo status

Scenario: Filtered sync narrows rollout
  Given a repository has both open and completed tracked items
  When the maintainer runs `mxw github sync --status open`
  Then only open tracked items should be mirrored or refreshed
  And completed items should be left untouched by that command
```

## References

- `workflow/workstreams/005-interactive-tui-mode/plan.md`
- `workflow/workstreams/006-patch-lane-for-lightweight-tracked-changes/spec.md`
- `README.md`
