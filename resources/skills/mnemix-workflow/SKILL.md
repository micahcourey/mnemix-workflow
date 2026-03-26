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
2. Open the generated `spec.md`, `ux.md`, `plan.md`, and `tasks.md`.
3. Fill in the artifacts for the actual work.
4. Record workstream-local decisions in `decisions/`.
5. Promote durable framework decisions to `workflow/decisions/` when needed.

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
- The numbering rule is `001` through `999`, then `1000+`; treat the numeric prefix as an integer, not a fixed-width string.
- This skill is a temporary bridge to the future `mnemix workflow new` CLI command.
