# Plan: Linear Issue Support

## Summary

Implement Linear support as a second tracker provider that follows the same
repo-canonical philosophy as GitHub support, but use Linear's issue hierarchy
instead of projects in v1. The first slice should add Linear config, linkage
metadata, CLI sync commands, and a provider-specific API client built on the
official GraphQL API.

## Scope Analysis

### Affected Areas

| Area | Changes Required |
|------|-----------------|
| CLI commands | Add `mxw linear init` and `mxw linear sync` flows |
| Provider layer | Add a Linear API client and sync logic alongside the GitHub provider |
| Workflow artifacts | Store Linear linkage metadata in `STATUS.md` and patch frontmatter |
| Documentation | Explain the issue-first model, auth and team setup, and repo-canonical rules |
| Automation | Add optional changed-only auto-sync support later in the same shape as GitHub support |

### Affected Layers

- [ ] Documentation
- [ ] Workflow artifacts
- [ ] Scripts
- [ ] CLI implementation

## Technical Design

### Proposed Additions

```text
workflow/linear.yml
src/linear.rs
src/commands/linear.rs
src/cli.rs
src/status.rs
README.md
docs/prd.md
resources/skills/mnemix-workflow/SKILL.md
resources/skills/mnemix-workflow/references/workstream-conventions.md
workflow/workstreams/010-linear-issue-support/
```

### Design Constraints

- Linear issues, not Linear Projects, are the v1 mirror model
- Team mapping is mandatory because every Linear issue belongs to a team
- The provider should use Linear's official GraphQL API directly
- Personal API key support is the easiest first auth path; OAuth can remain a later expansion path
- The repo remains canonical; Linear descriptions are overwritten on sync
- The design should leave room for future webhook-based or changed-only automation without making it mandatory in the first slice

## Implementation Slices

### Slice 1

- Define the Linear config shape in `workflow/linear.yml`
- Define linkage metadata fields for parent issue and child issues
- Decide the required team mapping model
- Decide the status-to-Linear-state mapping
- Decide the first auth mode and document why
- Document why Projects are intentionally out of scope for v1

### Slice 2

- Implement `mxw linear init`
- Implement `mxw linear sync <target>` for workstreams and patches
- Implement `mxw linear sync --all` for backfill
- Implement filtered sync such as `mxw linear sync --status open`
- Create or update one parent issue plus child issues for `spec.md`, `ux.md`, `plan.md`, and `tasks.md`
- Create or update one issue for patches
- Persist Linear linkage back into repo metadata

### Slice 3

- Implement `mxw linear sync --changed`
- Add dry-run output for sync operations
- Add optional automation support in the same shape as GitHub's changed-only sync flow
- Update README, PRD, skill docs, and conventions docs
- Add implementation notes about future Projects mode without making it part of v1

## Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Team mapping is underspecified | Sync cannot create valid Linear issues | High | Make team selection explicit in config and docs from the start |
| Linear issue hierarchy differs from GitHub enough to resist provider reuse | Abstraction becomes messy | Medium | Keep a shared sync shape but allow provider-specific linkage and API logic |
| OAuth complexity delays delivery | Shared or team installs take longer than expected | Medium | Start with personal API key auth and design room for OAuth later |
| Projects prove necessary for some teams sooner than expected | V1 feels too narrow for large programs | Medium | Document Projects as an intentional later mode rather than an accidental omission |

## References

- `workflow/workstreams/009-github-issue-support/plan.md`
- `https://linear.app/developers/graphql`
- `https://linear.app/developers/oauth-2-0-authentication`
- `https://linear.app/developers/webhooks`
- `https://linear.app/docs/parent-and-sub-issues`
- `https://linear.app/docs/project-overview`
