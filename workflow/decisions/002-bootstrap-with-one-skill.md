# ADR 002: Start With One Bootstrap Skill

## Status

Accepted

## Context

Before the dedicated CLI exists, `mnemix-workflow` needs a bootstrap mechanism that agents can use consistently. A design question emerged around whether to start with one broad skill or split immediately into multiple narrower skills.

## Decision

Start with one skill:

- `resources/skills/mnemix-workflow/`

The skill owns:

- workstream templates
- the temporary scaffold script
- the core workstream conventions

## Consequences

- Discovery friction stays low for agents and contributors
- The bootstrap story remains easy to teach
- Future split points still exist if the skill grows too large, especially around validation and export
