# UX Spec: Workflow Skill Bootstrap

## Summary

The experience of starting a new workstream should feel immediate, predictable, and lightweight for both humans and agents. The user should not need to remember file structure details or manually recreate the artifact set, and agents should be able to activate one well-described skill that contains the templates and helper script in the standard locations.

## Users And Context

- Primary persona: AI agent asked to bootstrap a new workstream
- Secondary persona: maintainer starting a new feature manually
- Context of use: local repository work before a dedicated CLI exists
- Preconditions: the repository already contains the naming system and artifact model

## User Goals

- Start a new workstream with minimal ceremony
- Get the correct folder and file structure automatically
- Begin filling in meaningful planning artifacts instead of creating boilerplate by hand

## Experience Principles

- Bootstrapping should feel fast
- The temporary experience should be easy to replace later with the CLI
- File structure should be obvious after generation
- The script should be agent-friendly and human-readable

## Primary Journey

1. The user decides a new workstream is needed.
2. They run a simple scaffold script with a workstream name.
3. The script creates the new numbered workstream folder and starter files.
4. The user opens `spec.md`, `ux.md`, `plan.md`, and `tasks.md` and begins planning.

## Alternate Flows

### Flow: Agent uses templates manually

- Trigger: the script is unavailable or the user wants more control
- Path: the agent reads the template references and creates the files manually
- Expected outcome: the workstream still conforms to the expected structure

### Flow: Maintainer reviews generated structure

- Trigger: a new workstream has been scaffolded
- Path: the maintainer inspects the generated folder and starter content
- Expected outcome: the structure is predictable and low-friction to review

## Surfaces

### Surface: Skill root

- Purpose: provide the standard entrypoint through `SKILL.md`
- Key information: what the skill does, when to use it, and which bundled resources exist
- Available actions: read instructions, activate helper resources
- Navigation expectations: the skill should be understandable without reading the whole repository
- Location: `resources/skills/mnemix-workflow/`

### Surface: Skill `assets/`

- Purpose: hold reusable starter templates
- Key information: expected artifact set and starter structure
- Available actions: inspect, copy, adapt through the scaffold script
- Navigation expectations: easy for agents to access after loading the skill

### Surface: Skill `references/`

- Purpose: hold focused supporting guidance and examples
- Key information: naming rules, generated structure, edge-case notes
- Available actions: read on demand
- Navigation expectations: progressive disclosure rather than mandatory reading

### Surface: Scaffold script

- Purpose: create a new workstream from the template set
- Key information: input name, output location, numbering behavior
- Available actions: generate a new workstream
- Navigation expectations: obvious how to use without deep documentation

## States

### Success

- A correctly named workstream folder is created with the expected starter files

### Error

- If numbering collides or arguments are invalid, the message should be understandable and fixable

### Empty

- If no workstreams exist yet, the script should still define a sane starting number

## Interaction Details

- Inputs should be simple and positional
- Output should confirm the created path
- The script should avoid surprising side effects
- Generated files should be plain Markdown and immediately editable

## Content And Tone

- Messages should be concise and practical
- Template content should be clear, not verbose

## Accessibility Requirements

- Script output should be plain text
- Generated files should use consistent Markdown heading structure

## Acceptance Scenarios

```gherkin
Scenario: Agent scaffolds a new workstream
  Given the repository contains a mnemix-workflow skill with reusable workstream templates
  When the agent runs the scaffold script with a workstream name
  Then a new numbered folder should be created under workflow/workstreams/
  And it should contain spec.md, ux.md, plan.md, and tasks.md

Scenario: Maintainer can inspect templates before generation
  Given the repository contains the workflow skill
  When the maintainer opens the skill's assets folder
  Then they should find the starter workstream templates
  And they should understand the intended generated structure
```

## Open Questions

- Should one skill own both template scaffolding and future workflow guidance, or should those split later?
- Should the script also create `decisions/` by default?
- Should the templates include guidance comments or stay very sparse?

## References

- `https://agentskills.io/specification`
- `workflow/workstreams/002-workflow-skill-bootstrap/spec.md`
