# UX Spec: Slash Command Integration

## Summary

The slash-command experience should make `mnemix-workflow` feel immediately
usable inside supported AI assistants. A maintainer should be able to install
the commands once, tell the agent `/mxw:track` or `/mxw:implement`, and get a
workflow-consistent result without remembering the underlying CLI details.

## Users And Context

- Primary persona: maintainer working inside a repository that uses or is adopting Mnemix Workflow
- Context of use: chat with an AI coding assistant such as Codex, Claude Code, or Cursor while operating inside a local repo
- Preconditions: the repository is initialized for Mnemix Workflow and the tool-specific slash-command files are installed or refreshed

## User Goals

- Start or continue tracked work from chat using names that match the Mnemix product language
- Avoid memorizing the exact `mxw` subcommand graph for common tasks
- Trust that slash commands still lead back to repo-native artifacts, status metadata, and validation behavior

## Experience Principles

- Mnemix-native language first: prefer names like `track` and `close` that fit the actual workflow model
- Thin integration, real workflow: slash commands should guide or trigger the existing methodology rather than invent a parallel one
- Low-friction setup: install and update flow should feel explicit but lightweight
- Clear mental model: users should understand when the agent is exploring, creating tracked work, implementing, syncing, or reporting status

## Primary Journey

1. A maintainer installs or refreshes slash-command support for one or more AI tools from the repository.
2. Inside a supported assistant, the maintainer invokes `/mxw:track` with a request for new work.
3. The agent chooses the right tracked lane, creates or updates the relevant artifacts, and explains the next step in the workflow.
4. Later, the maintainer uses `/mxw:implement`, `/mxw:status`, `/mxw:sync`, or `/mxw:close` to continue and finish the work.

## Alternate Flows

### Flow: Tool Not Yet Configured

- Trigger: the user invokes a slash command in a tool that has not had Mnemix Workflow commands installed
- Path: the command is unavailable, so the user follows the documented install flow and retries
- Expected outcome: the recovery path is obvious from the docs and setup output

### Flow: Repo Uses GitHub Today And Another Tracker Later

- Trigger: the user invokes `/mxw:sync`
- Path: the agent uses the configured tracker integration for the repository, starting with GitHub in the first shipped slice
- Expected outcome: the command name stays stable even as the tracker-provider surface expands later

## Surfaces

### Surface: Installer Or Update Command Output

- Purpose: confirm which tools were configured and where command files were written
- Key information: selected tools, generated file paths, skipped tools, and update guidance
- Available actions: install, refresh, inspect generated paths, and retry with a narrower tool set
- Navigation expectations: output should be legible in a terminal and easy to paste into docs or issue comments

### Surface: Generated Slash-Command Files

- Purpose: give each supported assistant a native command entrypoint
- Key information: command intent, when to use it, and the Mnemix Workflow behavior it should follow
- Available actions: invoke `/mxw:explore`, `/mxw:track`, `/mxw:implement`, `/mxw:close`, `/mxw:sync`, and `/mxw:status`
- Navigation expectations: file names and directory layout should match each tool's conventions cleanly

### Surface: Documentation

- Purpose: teach setup, naming, and the relation between slash commands, the CLI, and the workflow skill
- Key information: supported tools, install/update flow, command reference, and examples
- Available actions: follow setup, choose the right command, and understand product-language intent
- Navigation expectations: README stays concise while dedicated docs carry the deeper command reference

## States

### Loading

- Installer or update operations report progress per tool and do not leave the user guessing whether files were written

### Empty

- If no tools are selected or detected, the user gets a clear message and a path to install for a specific tool

### Success

- The user can invoke the slash commands immediately in the configured assistant and the docs reflect the same command set

### Error

- Unsupported tools, missing install locations, or write failures produce actionable errors with file-path context

## Interaction Details

- Command prompts should accept normal natural-language follow-up in chat rather than forcing rigid argument syntax
- Install and update feedback should enumerate created or refreshed files
- Generated commands should remain easy to invoke with standard slash-command completion in tools that support it
- Global-path tools and repo-local-path tools should both be documented clearly so setup feels predictable

## Content And Tone

- Use explicit, instructional labels such as "Installed commands for Cursor" or "No supported tools selected"
- Keep command descriptions calm and direct; the product should feel workflow-native rather than gimmicky
- The docs should explain that slash commands are a convenience layer on top of repo-native workflow artifacts

## Accessibility Requirements

- Terminal setup output should remain plain-text and screen-reader friendly
- Generated docs and command files should avoid unnecessary visual complexity
- Naming should stay short and distinct enough for keyboard command pickers and autocomplete lists

## Acceptance Scenarios

```gherkin
Scenario: Install slash commands for a supported tool
  Given a repository that uses Mnemix Workflow
  And a maintainer wants chat-native workflow commands in a supported assistant
  When they run the slash-command install flow
  Then tool-specific command files are written from shared templates
  And the setup output explains which commands are now available

Scenario: Track work from chat using Mnemix-native language
  Given slash commands are installed
  When the maintainer invokes /mxw:track in a supported assistant
  Then the agent chooses the correct tracked lane for the requested work
  And the result aligns with repo-native workstream or patch conventions

Scenario: Close tracked work without archive-language drift
  Given a tracked item is ready to finish
  When the maintainer invokes /mxw:close
  Then the agent updates the tracked work using Mnemix Workflow's status model
  And the experience does not assume archive-folder semantics
```

## References

- `README.md`
- `resources/skills/mnemix-workflow/SKILL.md`
- `workflow/workstreams/005-interactive-tui-mode/ux.md`
- `https://github.com/Fission-AI/OpenSpec/blob/main/docs/commands.md`
