# Plan: Slash Command Integration

## Summary

Add an agent-integration layer that installs and refreshes tool-specific
slash-command prompt files from shared repo templates while keeping `mxw` as
the authoritative engine for tracked-work behavior. The first slice should ship
the six approved commands, support a focused initial tool set, and document the
feature clearly in both the README and dedicated slash-command docs.

## Scope Analysis

### Affected Areas

| Area | Changes Required |
|------|-----------------|
| CLI commands | Add an install/update surface for assistant integrations and expose a clear supported-tools model |
| Embedded resources | Add shared slash-command templates and any tool-adapter metadata needed to render them |
| Packaging and file generation | Write repo-local or global command files into the paths required by supported assistants |
| Documentation | Add slash-command reference docs and update the README, skill docs, and conventions where needed |
| Tests | Cover command generation, file-path resolution, and user-facing CLI output for install/update flows |

### Affected Layers

- [x] Documentation
- [x] Workflow artifacts
- [ ] Scripts
- [x] CLI implementation

## Technical Design

### Proposed Additions

```text
src/cli.rs
src/commands/agent.rs
src/commands/mod.rs
src/agent/
  commands.rs
  render.rs
  tools.rs
resources/commands/
  explore.md
  track.md
  implement.md
  close.md
  sync.md
  status.md
README.md
docs/slash-commands.md
resources/skills/mnemix-workflow/SKILL.md
resources/skills/mnemix-workflow/references/workstream-conventions.md
tests/cli.rs
```

### Design Constraints

- Slash commands should remain a generated integration layer, not a second workflow engine
- The product language is fixed for this slice: `explore`, `track`, `implement`, `close`, `sync`, and `status`
- The implementation should start with a focused set of supported assistants rather than a long tail of partial integrations
- Prompt templates should be shared and rendered into tool-specific file layouts, not hand-maintained per tool
- `/mxw:sync` should preserve future tracker-provider flexibility while integrating with the tracker surface that exists today
- README and docs should explain both install/update setup and daily usage examples

## Implementation Slices

### Slice 1

- Decide the CLI surface for assistant integration management
- Define the first supported-tool matrix and file-path rules
- Define the shared prompt-template shape and any placeholder variables
- Define the exact behavioral contract for each slash command
- Decide how `/mxw:sync` maps to current GitHub behavior and future provider growth
- Add a dedicated documentation outline so README and slash-command docs stay aligned

### Slice 2

- Implement the install/update command flow in the Rust CLI
- Implement prompt rendering and tool-specific file generation
- Add the six shared slash-command templates
- Add support for the initial supported tools with deterministic output paths
- Add tests for rendering, path selection, and CLI output

### Slice 3

- Write `docs/slash-commands.md` as the full command reference
- Update `README.md` with setup, examples, and the slash-command value proposition
- Update `resources/skills/mnemix-workflow/SKILL.md` to reference the new command surface where appropriate
- Update conventions or methodology docs if slash commands change the recommended onboarding path
- Validate the documentation against the shipped command names and install flow

## Risks

| Risk | Impact | Likelihood | Mitigation |
| Tool-specific path differences create edge cases | Medium | Medium | Start with a small supported-tool set and centralize path logic in one module |
| Prompt templates drift from actual CLI behavior | High | Medium | Keep prompts thin, reuse shared language, and document that `mxw` remains authoritative |
| `/mxw:sync` overpromises future tracker support | Medium | Medium | Document GitHub-first behavior clearly and frame future providers as additive |
| README grows too large or duplicative | Low | Medium | Keep README concise and move deeper command details into `docs/slash-commands.md` |

## References

- `README.md`
- `docs/prd.md`
- `resources/skills/mnemix-workflow/SKILL.md`
- `workflow/workstreams/009-github-issue-support/plan.md`
- `workflow/workstreams/010-linear-issue-support/plan.md`
- `https://github.com/Fission-AI/OpenSpec/blob/main/docs/commands.md`
- `https://github.com/Fission-AI/OpenSpec/blob/main/docs/supported-tools.md`
