# Feature Spec: Workflow Skill Bootstrap

## Summary

Create the first reusable bootstrap layer for `mnemix-workflow` as a real Agent Skills Open Standard skill, with workstream templates in the skill's `assets/` folder and a simple scaffold script in the skill's `scripts/` folder that agents can use before the dedicated CLI exists.

## Problem

The repository now demonstrates the methodology through one bootstrap workstream, but there is no reusable mechanism for agents or maintainers to start the next workstream consistently. Without a standards-compliant skill, starter templates, and a simple scaffold script, every new workstream risks drifting in structure and quality, and the bootstrap path will not mirror how agents are actually expected to consume the framework.

## Users

- Primary persona: AI implementation agent asked to start a new workstream
- Secondary persona: maintainer creating or reviewing new workstreams

## Goals

- Define a real Agent Skills Open Standard skill for `mnemix-workflow`
- Define reusable workstream templates in the skill's `assets/` folder
- Define a simple script-based scaffolding path in the skill's `scripts/` folder for pre-CLI usage
- Keep the bootstrap mechanism minimal and easy for agents to follow
- Use this workstream to clarify the handoff between templates now and CLI later
- Decide whether `mnemix-workflow` should start with one skill or multiple skills

## Non-Goals

- Build the full Rust CLI
- Finalize all validation or standards integration behavior
- Support every possible workstream shape from the first script version

## User Value

Agents and maintainers will be able to create new workstreams quickly and consistently, reducing drift while the full CLI is still under development.

## Functional Requirements

- The repo should define a standards-compliant skill directory with `SKILL.md`
- The repo should place the skill under `resources/skills/`
- The repo should place workstream artifact templates under the skill's `assets/` folder
- The repo should place the bootstrap scaffold script under the skill's `scripts/` folder
- The repo should use the skill's `references/` folder for supporting guidance and examples as needed
- The repo should define the expected minimum output of a scaffolded workstream
- The repo should plan a simple script that creates a new workstream folder with the standard artifact set
- The workstream should explain how this temporary script fits into the path toward `mnemix workflow new`
- The workstream should recommend whether to begin with one skill or multiple skills

## Constraints

- The interim script must stay simple enough for agents to run and understand
- Templates should be plain Markdown and easy to inspect
- The bootstrap path should not introduce a second hidden metadata system
- The skill should follow the Agent Skills directory model cleanly rather than inventing a parallel layout

## Success Criteria

- A contributor can understand what reusable templates will exist, where they live, and how agents will consume them
- An agent can understand the expected behavior of the pre-CLI scaffold script
- The next implementation step is clear enough to build without further methodology debate
- The repository has a clear recommendation on starting with one skill or multiple skills
- The repository contains a real `resources/skills/mnemix-workflow/` implementation that agents can use today

## Risks

- The temporary script may become overly complex and delay the CLI
- Template structure may harden too early before real usage feedback

## Open Questions

- Should the skill eventually grow validation and export helpers, or should those split into separate skills first?

## References

- `docs/prd.md`
- `docs/methodology/naming-system.md`
- `workflow/workstreams/001-bootstrap-mnemix-workflow/spec.md`
- `README.md`
- `https://agentskills.io/specification`
