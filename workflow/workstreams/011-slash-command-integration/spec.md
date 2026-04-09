# Feature Spec: Slash Command Integration

## Summary

Add opt-in slash-command support so `mnemix-workflow` can install tool-specific
prompt files for supported AI coding assistants and expose a Mnemix-native
command set in chat: `/mxw:explore`, `/mxw:track`, `/mxw:implement`,
`/mxw:close`, `/mxw:sync`, and `/mxw:status`.

## Problem

`mnemix-workflow` already has a strong repo-native CLI and skill, but it still
depends on users and agents remembering the explicit `mxw` command surface or
loading the workflow skill manually. Competing workflow tools have made chat
entrypoints feel more immediate by installing slash commands directly into the
assistant environment. Without an equivalent layer, Mnemix Workflow is easier
to adopt for maintainers who already know the CLI than for teams that want a
chat-first operational path.

## Users

- Primary persona: maintainer or engineering lead who wants a chat-native way to use Mnemix Workflow in supported AI tools
- Secondary persona: AI implementation agent that needs deterministic, repo-aligned command prompts instead of ad hoc chat instructions

## Goals

- Create a Mnemix-native slash-command vocabulary that reflects the product's tracked-work model rather than copying another tool's naming
- Install and refresh tool-specific command files from shared templates
- Keep the existing Rust CLI and workflow artifacts as the source of truth for behavior
- Document the slash-command experience clearly in the README and dedicated docs so teams can adopt it without guessing

## Non-Goals

- Implement a generic runtime slash-command parser inside the Rust CLI
- Copy OpenSpec's change and archive semantics into Mnemix Workflow
- Support every AI coding tool in the first slice
- Deliver bidirectional tracker editing or broad new tracker-provider work as part of this slice

## User Value

Teams get the convenience of chat-native workflow commands without giving up
repo-native planning artifacts, explicit status metadata, or the existing
`mxw` command model. The feature lowers adoption friction while preserving the
product's actual methodology.

## Functional Requirements

- The product should ship a first-class slash-command set with these names:
  `/mxw:explore`, `/mxw:track`, `/mxw:implement`, `/mxw:close`, `/mxw:sync`,
  and `/mxw:status`
- There should be an install/update workflow that writes tool-specific command
  files for supported assistants from shared repo-owned templates
- Slash-command templates should encode Mnemix Workflow behavior and naming,
  not OpenSpec naming
- `/mxw:track` should guide the agent toward the correct tracked unit
  (workstream or patch) and the right artifact-creation flow
- `/mxw:implement` should guide the agent to execute from tracked artifacts,
  update task state, and validate work where appropriate
- `/mxw:close` should reflect Mnemix Workflow's status model by marking tracked
  work finished rather than assuming archive-folder semantics
- `/mxw:status` should help the agent inspect or report the state of a
  workstream or patch
- `/mxw:sync` should be phrased as a tracker-sync command that works with
  GitHub first and leaves room for future configured tracker providers
- The implementation should support at least a small initial set of tools with
  documented install paths and update behavior
- The README and dedicated documentation should explain the command set,
  supported tools, installation flow, and the relationship between slash
  commands, the CLI, and the workflow skill

## Constraints

- The repo remains canonical; slash commands are an integration layer, not a new source of state
- Existing `mxw` and `mnx` behavior should remain valid and should not become second-class
- Command prompts need to work with tool-specific file layouts rather than assuming one universal slash-command format
- The design should preserve room for future tracker-provider expansion behind `/mxw:sync`

## Success Criteria

- A maintainer can install slash-command support into supported AI tools from the repository workflow
- Supported tools expose the six Mnemix-native commands with behavior aligned to the repo methodology
- The generated command files are clearly derived from shared templates and can be refreshed on update
- The README and dedicated slash-command docs make setup and day-to-day usage easy to understand
- The feature strengthens adoption without forcing archive semantics or other product-language drift

## Risks

- Tool-specific prompt-file conventions may vary enough to make the integration layer broader than expected
- Generic `/mxw:sync` language may get ahead of the currently shipped tracker-provider surface if the mapping is not documented carefully
- If the slash-command prompts become too smart, they could drift from the real CLI behavior and create a maintenance burden
- Naming is now intentionally Mnemix-specific, so weak documentation would make the command set feel arbitrary to new users

## References

- `README.md`
- `docs/prd.md`
- `resources/skills/mnemix-workflow/SKILL.md`
- `resources/skills/mnemix-workflow/references/workstream-conventions.md`
- `workflow/workstreams/009-github-issue-support/spec.md`
- `workflow/workstreams/010-linear-issue-support/spec.md`
- `https://github.com/Fission-AI/OpenSpec/blob/main/docs/commands.md`
- `https://github.com/Fission-AI/OpenSpec/blob/main/docs/supported-tools.md`
