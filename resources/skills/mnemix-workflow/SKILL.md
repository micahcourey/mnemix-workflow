---
name: mnemix-workflow
description: Create and maintain mnemix-workflow workstreams and patches. Use when choosing the right tracked lane, reviewing tracked work, or applying the repository's workflow conventions through the CLI.
---

# Mnemix Workflow

Use this skill when the task is to create, review, or maintain tracked
`mnemix-workflow` artifacts in this repository or another repository using the
same conventions.

## What This Skill Owns

- the standard workstream artifact set
- the lightweight patch lane
- the numbering and naming rules for workstreams and patches
- optional repo-canonical GitHub issue mirroring
- optional contract artifacts for `OpenAPI`, `AsyncAPI`, and `JSON Schema`

## Default Workflow

1. Decide whether the change needs a full workstream or a lightweight patch.
2. Review existing tracked work with `mnx` or `mxw ui` when you need quick context before creating something new.
3. For a workstream, run `mxw new "<name>"`.
4. For a patch, run `mxw patch new "<name>"`.
5. Resolve material planning questions with the user while creating the tracked artifact so it is ready for execution.
6. Fill in the artifact(s) for the actual work without leaving placeholder "open questions" behind by default.
7. If the human explicitly does not want to decide something yet, add a focused `Open Questions` section or a decision-oriented plan slice for that unresolved item.
8. Keep metadata current with `mxw status`, `mxw patch status`, or the in-TUI status action.
9. Run `mxw validate` before wrapping up meaningful work so tracked artifacts and optional contracts stay healthy.
10. If the repository uses GitHub issue mirroring, initialize it with `mxw github init` and sync with `mxw github sync ...` instead of hand-authoring issue bodies.
11. Offer `mxw hooks install` when the repository would benefit from automatic `updated` refreshes and push-time reminders.
12. Record workstream-local decisions in `decisions/`.
13. Promote durable framework decisions to `workflow/decisions/` when needed.

## Bundled Resources

- Templates: `assets/workstream/`
- Patch template: `assets/patch.md`
- Contract templates: `assets/openapi.yaml`, `assets/asyncapi.yaml`, `assets/schema.json`
- Conventions reference: `references/workstream-conventions.md`

## When To Read The Reference

Read `references/workstream-conventions.md` when you need:

- the workstream numbering rules
- the generated artifact shapes
- the patch-versus-workstream rule
- guidance on when to use repo-level vs workstream-level decisions
- the current CLI-first workflow shape

## Notes

- Prefer the CLI over manual scaffolding when it is available.
- Use `mnx` or `mxw ui` to browse current workstreams and patches before creating new tracked work when that context would help.
- Run `mxw` from inside the target repository or linked git worktree so new workstreams and patches are created in the right checkout.
- The numbering rule is `001` through `999`, then `1000+`; treat the numeric prefix as an integer, not a fixed-width string.
- New workstreams should include `STATUS.md` from the start.
- `STATUS.md` frontmatter should use `status`, `summary`, and `updated` as required fields, with optional `prs` for linked pull request numbers.
- Patches are single files under `workflow/patches/` and carry the same frontmatter metadata directly in the patch file.
- Every PR should map to either a workstream or a patch.
- Use `mxw status list` and `mxw patch status list` when you need a non-TUI view of open or completed tracked work.
- Use `mxw validate` to run an umbrella check across tracked metadata and any present contract artifacts.
- Use `mxw hooks install` to install the bundled `pre-commit` and `pre-push` hook helpers when the repo wants the status nudges.
- When a repo wants GitHub execution visibility, use `mxw github init --enable-auto-sync`, then `mxw github sync <target>`, `mxw github sync --all`, or `mxw github sync --status open --all`.
- Keep the repo as the source of truth. Mirrored GitHub issue titles and bodies are system-managed and may be overwritten on sync.
- When work touches HTTP APIs, async interfaces, or reusable data shapes, use `mxw openapi`, `mxw asyncapi`, or `mxw schema` to scaffold and validate contract artifacts under the workstream's `contracts/` folder.
