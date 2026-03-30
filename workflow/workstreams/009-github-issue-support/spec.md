# Feature Spec: Github Issue Support

## Summary

Add optional GitHub Issues support so repositories using `mnemix-workflow` can
mirror workstreams and patches into GitHub for execution visibility without
moving the source of truth out of the repo. In the v1 model, a workstream maps
to one parent issue plus sub-issues for `spec.md`, `ux.md`, `plan.md`, and
`tasks.md`, while a patch maps to a single issue.

## Problem

Many teams already use GitHub Issues and Projects as their day-to-day execution
surface. Today `mnemix-workflow` keeps planning and status in the repo, but it
has no first-class way to mirror that work into GitHub. That leaves teams
choosing between repo-native planning and GitHub-native tracking instead of
getting both in a coherent, repo-canonical workflow.

## Users

- Primary persona: maintainer or engineering lead who wants repo-native planning and GitHub-native execution visibility
- Secondary persona: AI implementation agent that needs a predictable way to create and sync issue mirrors without inventing issue structure ad hoc

## Goals

- Make GitHub issue support optional and opt-in rather than a required part of the methodology
- Keep the repo as the only source of truth for titles, bodies, status, and work structure
- Create one parent issue per workstream plus sub-issues for the core workstream files
- Support a lighter single-issue mirror for patches
- Add automation so issue mirrors stay current without manual copy/paste

## Non-Goals

- Support two-way collaborative editing of issue bodies
- Treat GitHub Issues as an equal or competing source of planning truth
- Mirror every file in a workstream, including `STATUS.md` or local decisions folders, as separate issues in v1
- Build deep GitHub Project automation beyond the basics needed to fit issue mirrors into existing project workflows

## User Value

Teams get the clarity of repo-native, spec-driven development without giving up
the operational visibility of GitHub Issues. Humans and agents can continue to
plan in Markdown artifacts, then project that work into GitHub so execution can
show up in issue lists, Projects, filters, and notifications.

## Functional Requirements

- The CLI should support enabling or initializing GitHub issue mirroring for a repository
- The CLI should create and update a parent issue for a workstream
- The CLI should create and update sub-issues for `spec.md`, `ux.md`, `plan.md`, and `tasks.md`
- The CLI should create and update a single issue for a patch
- Workstreams and patches should store GitHub issue linkage in repo metadata
- Sync should replace issue titles and bodies from repo artifacts instead of attempting body merges
- Status changes in repo metadata should drive issue open/closed state on sync
- The CLI should support `mxw github sync --changed` so automation can refresh only affected already-linked items
- The CLI should support `mxw github sync --all` for backfilling existing tracked items after a repository enables GitHub issue support
- The CLI should support filtered sync such as `mxw github sync --status open` and `mxw github sync --status completed`
- The first creation of issue mirrors should remain an explicit manual sync action in v1
- Backfilled completed items should be created in the closed issue state so mirrored history matches repo status
- The framework docs should explain that issue mirrors are system-managed and should not be edited directly
- The docs should recommend tightening GitHub issue creation/edit permissions where practical to reduce drift

## Constraints

- The repo remains canonical; GitHub is a mirrored execution layer only
- GitHub issue body edits are unsupported and may be overwritten on sync
- The first slice should prefer the simplest reliable GitHub API path instead of optimizing for every GitHub feature at once
- Auto-sync should update already-linked mirrors, not implicitly create brand-new mirrors on every push
- The v1 design should remain compatible with future `mnx` and Studio issue views without requiring a model rewrite
- The feature should stay optional so repositories can adopt the methodology without GitHub issue mirroring

## Success Criteria

- A repository can opt into GitHub issue mirroring without changing the underlying workstream and patch methodology
- A workstream sync creates one parent issue plus sub-issues for `spec.md`, `ux.md`, `plan.md`, and `tasks.md`
- A patch sync creates one issue with repo metadata linkage
- Re-running sync updates mirrored issues deterministically from repo content
- `mxw github sync --all` can backfill previously-created workstreams and patches, including completed items
- Filtered sync gives teams a narrower rollout path when they do not want to backfill everything at once
- The docs make it clear that GitHub issues are mirrors and that repo artifacts remain the source of truth

## Risks

- GitHub sub-issue APIs may introduce edge cases or permissions friction across repositories
- Overly broad auto-sync could surprise users if issue mirrors update at times they do not expect
- Poorly chosen metadata shape could make future Studio and TUI integrations harder than necessary
- Teams may still edit issue bodies manually unless the docs and product behavior make the repo-canonical model obvious

## References

- `README.md`
- `docs/prd.md`
- `workflow/workstreams/005-interactive-tui-mode/plan.md`
- `workflow/workstreams/006-patch-lane-for-lightweight-tracked-changes/spec.md`
