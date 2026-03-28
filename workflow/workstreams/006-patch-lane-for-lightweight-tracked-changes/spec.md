# Feature Spec: Patch Lane For Lightweight Tracked Changes

## Summary

Add a lightweight `workflow/patches/` lane so small fixes and minor
enhancements can still be planned and tracked in Mnemix Workflow without
requiring the full multi-file workstream artifact set.

## Problem

The current framework treats the full workstream as the only tracked planning
unit. That is a good fit for larger initiatives, but it creates too much
ceremony for narrow bug fixes, chores, and small enhancements. The result is a
bad tradeoff: either tiny changes feel burdensome, or teams are tempted to skip
framework tracking entirely. The framework needs a second tracked lane that is
lighter while still keeping every PR planned and attributable.

## Users

- Primary persona: maintainer or engineer shipping a small fix or narrow enhancement
- Secondary persona: AI agent that needs a clear, lightweight artifact to anchor implementation work

## Goals

- Keep the rule that every PR is tracked in Mnemix Workflow
- Introduce a lighter planning unit for small, well-bounded changes
- Make the lane obvious and teachable without requiring teams to adopt extra jargon
- Reuse the existing status model so patches can later appear in CLI and Studio views

## Non-Goals

- Replace full workstreams for larger or more ambiguous work
- Remove the need for planning before implementation
- Design the entire patch CLI and TUI surface in this planning workstream

## User Value

Users get a practical middle ground: small changes remain planned and linked to
PRs, but the artifact burden matches the size of the work. That makes the
framework easier to use consistently across day-to-day engineering work.

## Functional Requirements

- The framework should introduce `workflow/patches/` as the lightweight tracked lane
- Each patch should be represented by a single Markdown file
- Patch files should use zero-padded 4-digit ids such as `0001-fix-status-copy.md`
- Patch files should carry machine-readable frontmatter using the existing status model
- Patch files should support at least `status`, `summary`, `updated`, and optional `prs`
- The methodology should define when a change belongs in a patch versus a full workstream
- The framework should document that every PR must map to either a workstream or a patch
- Future CLI support should be able to scaffold and list patches numerically

## Constraints

- Keep the lane simpler than a full workstream
- Use clear, familiar naming; prefer `patches` over more branded alternatives
- Preserve compatibility with future CLI, TUI, and Studio views
- Avoid introducing a second status system distinct from workstreams

## Success Criteria

- Teams can track narrow fixes and chores without creating a full workstream
- The workstream-versus-patch rule is easy for humans and agents to apply
- The patch format is simple enough to author quickly but structured enough for future tooling
- The patch lane fits naturally into the existing `workflow/` artifact domain

## Risks

- A patch format that grows too large could become a disguised workstream
- A vague threshold between patches and workstreams could create inconsistency
- If the lane is under-specified, teams may still skip tracking for small fixes

## References

- `README.md`
- `docs/prd.md`
- `workflow/decisions/004-use-status-files-for-workstream-state.md`
- `workflow/decisions/005-use-frontmatter-as-canonical-status-metadata.md`
- `workflow/decisions/007-resolve-planning-questions-during-workstream-creation.md`
- `workflow/decisions/010-use-patches-for-lightweight-tracked-work.md`
