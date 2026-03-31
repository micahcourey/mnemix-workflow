# Feature Spec: Linear Issue Support

## Summary

Add optional Linear support so repositories using `mnemix-workflow` can mirror
workstreams and patches into Linear while keeping repo artifacts as the source
of truth. The first slice should be issue-first: a workstream mirrors to one
parent Linear issue plus child issues for `spec.md`, `ux.md`, `plan.md`, and
`tasks.md`, while a patch mirrors to a single Linear issue.

## Problem

Some teams run execution out of Linear instead of GitHub, but the newly shipped
external-tracker support in `mnemix-workflow` only covers GitHub Issues today.
That leaves Linear-heavy teams without the same repo-canonical sync path and
forces them to either duplicate planning manually or abandon repo-native
tracking.

## Users

- Primary persona: maintainer or engineering lead who wants repo-native planning with Linear-native execution visibility
- Secondary persona: AI implementation agent that needs a deterministic way to create and sync Linear mirrors from repo artifacts

## Goals

- Make Linear support optional and opt-in, just like GitHub support
- Keep the repo canonical for titles, descriptions, statuses, and work structure
- Use Linear's issue hierarchy as the default mapping for workstreams and patches
- Design the provider layer so GitHub and Linear can share sync concepts where practical

## Non-Goals

- Use Linear Projects as the default v1 mirror model
- Support two-way collaborative editing of Linear issue descriptions
- Mirror every workstream file or local `decisions/` entry as its own Linear artifact
- Finalize Projects, Initiatives, or deeper Linear roadmap integrations in this first slice

## User Value

Teams that plan in repo-native Markdown but execute inside Linear get the same
benefit GitHub teams now have: one source of truth in the repo with a mirrored
operational surface in the tracker their team already uses.

## Functional Requirements

- The CLI should support enabling or initializing Linear sync for a repository
- The CLI should support syncing one workstream or one patch into Linear
- The CLI should support backfill with `--all`
- The CLI should support filtered sync such as `--status open`
- The CLI should support `--changed` for automation-friendly refreshes of already-linked items
- A workstream should map to one parent Linear issue plus child issues for `spec.md`, `ux.md`, `plan.md`, and `tasks.md`
- A patch should map to one Linear issue
- Repo metadata should store Linear linkage so sync is deterministic and repeatable
- Repo `status` values should drive Linear issue state transitions on sync
- The docs should make it explicit that Linear descriptions are system-managed mirrors and may be overwritten from repo artifacts
- The configuration model should account for required Linear team mapping
- The first slice should use Linear's API directly rather than depending on a shell-based CLI integration

## Constraints

- The repo remains canonical; Linear is a mirrored execution layer only
- The v1 model is issue-first; Projects are intentionally out of scope
- Every mirrored issue must belong to a Linear team, so team mapping is a required configuration concern
- The implementation should prefer the official Linear GraphQL API rather than inventing a brittle browser or unofficial CLI flow
- Auto-sync should refresh already-linked items and avoid surprising broad creation behavior

## Success Criteria

- A repository can opt into Linear support without changing the underlying workstream and patch methodology
- A workstream sync creates one parent Linear issue plus child issues for `spec.md`, `ux.md`, `plan.md`, and `tasks.md`
- A patch sync creates one Linear issue with repo metadata linkage
- Re-running sync updates existing Linear items deterministically from repo content
- The docs clearly explain why Linear issues are the default model and why Projects are deferred

## Risks

- Team mapping may be awkward for repos whose workstreams span multiple Linear teams
- Linear issue hierarchy semantics may differ enough from GitHub sub-issues that the provider abstraction needs refinement
- OAuth and webhook choices may add complexity if the project later wants richer shared/team installations
- Users may still edit Linear issue descriptions manually unless the repo-canonical model is obvious in docs and product behavior

## References

- `workflow/workstreams/009-github-issue-support/spec.md`
- `workflow/workstreams/009-github-issue-support/plan.md`
- `https://linear.app/docs/conceptual-model`
- `https://linear.app/docs/parent-and-sub-issues`
- `https://linear.app/docs/project-overview`
- `https://linear.app/developers/graphql`
- `https://linear.app/developers/oauth-2-0-authentication`
- `https://linear.app/developers/webhooks`
