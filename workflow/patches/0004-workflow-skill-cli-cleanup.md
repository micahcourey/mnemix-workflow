---
status: completed
summary: Updated the workflow skill and docs to be fully CLI-first and removed the old scaffold script.
updated: 2026-03-29
prs:
- 12

---

# Patch: Workflow Skill Cli Cleanup

## Summary

Clean up the `mnemix-workflow` skill and supporting docs so the shipped CLI is
the primary workflow path, and remove stale script-oriented wording from the
conventions reference.

## Reason

The framework now has real `mxw` and `mnx` entrypoints, but the bundled skill
still contains pre-CLI guidance and references to the old scaffold script. That
creates confusion for agents and humans by mixing current workflow behavior
with no-longer-needed bootstrap instructions.

## Scope

- Update the workflow skill so `mxw` is the canonical creation path
- Remove script mentions from `references/workstream-conventions.md`
- Remove the bundled legacy scaffold script from the skill resources
- Align the PRD language with the current CLI-first workflow and skill role
- Align the skill resource list and usage notes with the shipped CLI surface
- Keep the broader methodology and tracked-lane model unchanged
- Do not redesign the whole skill system or rewrite unrelated workstream history

## Implementation Notes

- Update `resources/skills/mnemix-workflow/SKILL.md` to emphasize `mxw new`,
  `mxw patch new`, `mxw status`, `mxw openapi`, `mxw asyncapi`, `mxw schema`,
  and `mnx`
- Remove stale script-oriented wording from the conventions reference so it
  only describes the current CLI workflow
- Remove the old scaffold script once the skill and product docs no longer
  depend on it
- Update the PRD so the workflow skill is described as a current companion to
  the CLI rather than a pre-CLI bootstrap layer
- Keep the skill concise and execution-oriented for agents

## Validation

- A reader of the skill can understand the current CLI-first workflow without
  seeing outdated script guidance
- `references/workstream-conventions.md` contains no remaining references to
  the scaffold script
- The updated skill instructions still match the current README and shipped CLI
- The repo no longer contains `resources/skills/mnemix-workflow/scripts/new-workstream.py`

## References

- `resources/skills/mnemix-workflow/SKILL.md`
- `resources/skills/mnemix-workflow/references/workstream-conventions.md`
- `docs/prd.md`
- `README.md`
