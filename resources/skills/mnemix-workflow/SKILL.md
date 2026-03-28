---
name: mnemix-workflow
description: Bootstrap and maintain mnemix-workflow workstreams. Use when creating a new workstream, updating spec/ux/plan/tasks artifacts, or applying the repository's workflow conventions before the dedicated CLI exists.
---

# Mnemix Workflow

Use this skill when the task is to create or maintain a `mnemix-workflow` workstream in this repository or another repository using the same conventions.

## What This Skill Owns

- the standard workstream artifact set
- the pre-CLI scaffold script
- the numbering and naming rules for workstreams

## Default Workflow

1. If a new workstream is needed, run `scripts/new-workstream.py "<name>"`.
2. Open the generated `STATUS.md`, `spec.md`, `ux.md`, `plan.md`, and `tasks.md`.
3. Resolve material planning questions with the user while creating the workstream so the artifacts are ready for execution.
4. Fill in the artifacts for the actual work without leaving placeholder "open questions" behind by default.
5. If the human explicitly does not want to decide something yet, add a focused `Open Questions` section or a decision-oriented plan slice for that unresolved item.
6. Record workstream-local decisions in `decisions/`.
7. Promote durable framework decisions to `workflow/decisions/` when needed.

## Bundled Resources

- Templates: `assets/workstream/`
- Scaffold script: `scripts/new-workstream.py`
- Conventions reference: `references/workstream-conventions.md`

## When To Read The Reference

Read `references/workstream-conventions.md` when you need:

- the workstream numbering rules
- the generated folder shape
- guidance on when to use repo-level vs workstream-level decisions

## Notes

- Use the scaffold script instead of recreating the folder structure by hand unless there is a good reason not to.
- Run the scaffold script from inside the target repository or git worktree so new workstreams are created in the right checkout.
- The numbering rule is `001` through `999`, then `1000+`; treat the numeric prefix as an integer, not a fixed-width string.
- New workstreams should include `STATUS.md` from the start.
- `STATUS.md` frontmatter should use `status`, `summary`, and `updated` as required fields, with optional `prs` for linked pull request numbers.
- This skill is a temporary bridge to the future `mnemix workflow new` CLI command.
