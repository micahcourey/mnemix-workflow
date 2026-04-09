# Slash Commands

`mnemix-workflow` can install repo-local slash-command files for supported AI
assistants so teams can use Mnemix-native workflow commands directly in chat.

The current command set is:

- `/mxw:explore`
- `/mxw:track`
- `/mxw:implement`
- `/mxw:close`
- `/mxw:sync`
- `/mxw:status`

These commands are an integration layer, not a second workflow engine. `mxw`
and the repo's workflow artifacts remain the source of truth.

## Supported Tools

The first shipped slice supports repo-local command installation for:

- Claude Code
- Cursor

Use:

```bash
mxw agent tools
```

to show the supported integrations and the directory each one uses inside the
current repository.

## Install

Install the slash commands for all supported tools:

```bash
mxw agent install
```

Install for a narrower set:

```bash
mxw agent install --tool claude
mxw agent install --tool cursor
mxw agent install --tool claude --tool cursor
```

The initial supported layout is:

```text
.claude/commands/mxw/
  explore.md
  track.md
  implement.md
  close.md
  sync.md
  status.md

.cursor/commands/mxw/
  explore.md
  track.md
  implement.md
  close.md
  sync.md
  status.md
```

## Update

Refresh generated command files after upgrading `mnemix-workflow` or after
changing the bundled templates:

```bash
mxw agent update
```

If an installed command file has been edited manually, `mxw agent install`
refuses to overwrite it and tells you to use `mxw agent update` instead.

## Command Reference

### `/mxw:explore`

Use this when you want the agent to investigate, reason through the current
state, and recommend the next workflow action before making changes.

### `/mxw:track`

Use this when new work should become a tracked workstream or patch. The agent
should choose the right lane and create repo-native workflow artifacts.

### `/mxw:implement`

Use this when tracked work already exists and implementation should proceed from
those artifacts.

### `/mxw:close`

Use this when tracked work is ready to finish. The command should update Mnemix
Workflow status to `completed` rather than assuming archive semantics.

### `/mxw:sync`

Use this when tracked work should be synced into the configured issue tracker.
Today that primarily maps to the GitHub mirror flow behind `mxw github ...`.

### `/mxw:status`

Use this when you want to inspect the status of one workstream, one patch, or a
filtered set of tracked items.

## How This Relates To The CLI

The slash commands are a chat-native front door for the same workflow model:

- `/mxw:track` maps onto workstream or patch creation
- `/mxw:status` maps onto `mxw status ...` or `mxw patch status ...`
- `/mxw:sync` maps onto the tracker integration configured for the repo
- `/mxw:close` uses the same status model as `mxw status set ... completed`

Use normal `mxw` commands whenever explicit terminal control is more useful than
chat invocation.

## Notes

- The installed files are repo-local, which keeps the integration explicit and
  portable with the repository.
- The generated templates are bundled under `resources/commands/` in the
  `mnemix-workflow` repository.
- The command names are intentionally Mnemix-native rather than copied from
  another workflow framework.
