# Plan: Workflow Skill Bootstrap

## Summary

This workstream defines the temporary bootstrap path for creating new workstreams before the dedicated CLI exists by packaging the bootstrap mechanism as a real Agent Skills Open Standard skill.

## Scope Analysis

### Affected Areas

| Area | Changes Required |
|------|-----------------|
| Skills | Add a standards-compliant `mnemix-workflow` skill |
| Skill assets | Add reusable workstream templates under `assets/` |
| Skill scripts | Add a simple scaffold script for new workstreams under `scripts/` |
| Skill references | Add focused supporting docs under `references/` as needed |
| Workflow docs | Explain how the interim bootstrap path fits with the future CLI |

### Affected Layers

- [x] Documentation
- [x] References/templates
- [x] Simple scripting
- [ ] CLI implementation

## Technical Design

### Proposed Repository Additions

```text
resources/
  skills/
    mnemix-workflow/
      SKILL.md
      assets/
        workstream/
          spec.md
          ux.md
          plan.md
          tasks.md
      scripts/
        new-workstream.py
      references/
        workstream-conventions.md
workflow/
  workstreams/
    002-workflow-skill-bootstrap/
```

### Script Behavior

- Accept a workstream name
- Determine the next numeric workstream id
- Create `workflow/workstreams/<id>-<slug>/`
- Copy starter templates from the skill's `assets/workstream/` folder into the new folder
- Print the created path and next suggested action

### Skill Structure Notes

The Agent Skills specification defines a skill as a directory containing `SKILL.md` plus optional `scripts/`, `references/`, and `assets/` directories. The bootstrap artifact should follow that model directly so the repository can dogfood a real open-standard skill rather than a custom pseudo-skill layout.

### One Skill Or Many

The recommended v0 approach is to start with one skill:

- `resources/skills/mnemix-workflow/`

Reasons:

- the framework is still young and the core workflow narrative should stay unified
- a single skill reduces discovery friction for agents
- templates, bootstrap scripting, and methodology guidance are tightly related at this stage

Potential future split points, only if the skill grows too large:

- `mnemix-workstream-bootstrap`
- `mnemix-workflow-validation`
- `mnemix-workflow-export`

### Design Constraints

- Keep the script simple enough that agents can read it quickly
- Prefer deterministic behavior over configurability
- Treat this as a stepping stone toward `mnemix workflow new`
- Keep the initial skill small enough that one `SKILL.md` remains understandable

## Implementation Phases

### Phase 1: Add Templates

- Created `resources/skills/mnemix-workflow/SKILL.md`
- Created starter `spec.md`, `ux.md`, `plan.md`, and `tasks.md` templates
- Stored them under `resources/skills/mnemix-workflow/assets/workstream/`
- Added focused supporting docs under `resources/skills/mnemix-workflow/references/`

### Phase 2: Add Scaffold Script

- Implemented a simple script that copies the templates into a new numbered workstream
- Placed it under `resources/skills/mnemix-workflow/scripts/`
- Ensured output paths follow the naming-system conventions

### Phase 3: Document Agent Usage

- Explained how agents should use the templates and script before the CLI exists
- Clarified the future migration path to the CLI
- Explained why the initial implementation uses one skill instead of multiple skills

## Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| The temporary script becomes sticky and delays CLI design | Medium | Medium | Keep the scope intentionally narrow |
| Templates become too rigid | Medium | Medium | Keep starter text lightweight and revise based on real usage |
| Script behavior diverges from future CLI behavior | Medium | Medium | Make the script mirror the intended CLI mental model |
| The skill grows too broad and becomes hard for agents to load effectively | Medium | Medium | Start with one skill, keep `SKILL.md` concise, and split only when real complexity appears |

## Open Questions

- Should numbering be strictly sequential based on existing folders?
- Should the script validate names or just normalize them?
- When should validation and export behaviors split into additional skills, if ever?

## References

- `docs/methodology/naming-system.md`
- `workflow/workstreams/002-workflow-skill-bootstrap/spec.md`
- `workflow/workstreams/002-workflow-skill-bootstrap/ux.md`
- `resources/skills/mnemix-workflow/SKILL.md`
- `https://agentskills.io/specification`
