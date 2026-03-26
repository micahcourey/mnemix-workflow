# Tasks: Workflow Skill Bootstrap

## Workstream Goal

Define and then implement the minimal bootstrap mechanism for creating new workstreams before the dedicated CLI exists, packaged as a real Agent Skills Open Standard skill.

## Execution Slices

### Slice 1: Define Template Set

- [x] Confirm the minimum starter artifact set
- [x] Decide to start with one `mnemix-workflow` skill or multiple skills
- [x] Create `resources/skills/mnemix-workflow/SKILL.md`
- [x] Create `resources/skills/mnemix-workflow/assets/workstream/spec.md`
- [x] Create `resources/skills/mnemix-workflow/assets/workstream/ux.md`
- [x] Create `resources/skills/mnemix-workflow/assets/workstream/plan.md`
- [x] Create `resources/skills/mnemix-workflow/assets/workstream/tasks.md`
- [x] Create focused supporting files under `resources/skills/mnemix-workflow/references/`

### Slice 2: Define Scaffold Script

- [x] Choose the implementation language for the temporary script
- [x] Define numbering and slug rules
- [x] Define generated folder structure
- [x] Define output and error messages
- [x] Place the script under `resources/skills/mnemix-workflow/scripts/`

### Slice 3: Implement And Document

- [x] Add `resources/skills/mnemix-workflow/scripts/new-workstream.py`
- [x] Add usage notes for agents in `SKILL.md`
- [x] Verify the script creates a valid starter workstream

## Validation Checklist

- [x] Skill directory follows the Agent Skills spec shape
- [x] Template location is documented
- [x] Script output path matches `workflow/workstreams/`
- [x] Generated workstream includes the required starter files
- [x] The path to the future CLI remains clear

## Notes

- This workstream is intentionally transitional.
- The goal is to reduce planning friction now without overdesigning the pre-CLI experience.
- The current recommendation is to start with one skill and split only if real complexity emerges.
