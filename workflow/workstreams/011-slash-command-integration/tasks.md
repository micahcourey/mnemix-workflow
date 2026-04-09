# Tasks: Slash Command Integration

## Workstream Goal

Ship installable slash-command support that exposes a Mnemix-native chat
command surface for supported AI tools and teaches it clearly through product
documentation.

## Execution Slices

### Slice 1

- [x] Finalize the assistant-integration CLI surface for installing and refreshing slash commands
- [x] Finalize the initial supported-tool set and their command-file path rules
- [x] Define the behavioral contract for `/mxw:explore`, `/mxw:track`, `/mxw:implement`, `/mxw:close`, `/mxw:sync`, and `/mxw:status`
- [x] Define how `/mxw:sync` maps to current GitHub support and future tracker-provider growth

### Slice 2

- [x] Implement prompt-template rendering and tool-specific command-file generation
- [x] Add shared template files for the six slash commands
- [x] Implement the CLI install/update flow for the initial supported tools
- [x] Add automated tests for rendering, output paths, and install/update CLI behavior

### Slice 3

- [x] Write `docs/slash-commands.md` with setup, supported tools, command reference, and examples
- [x] Update `README.md` to introduce slash commands and link to the full command docs
- [x] Update `resources/skills/mnemix-workflow/SKILL.md` to reflect the new chat-native entrypoint where appropriate
- [x] Update any conventions or methodology docs that should mention slash-command setup
- [x] Run validation and a documentation pass to confirm the command names and examples are consistent

## Validation Checklist

- [x] `cargo test`
- [x] `cargo run --bin mxw -- --help`
- [x] Exercise the install/update flow in a temporary tool-directory fixture
- [x] Confirm README and docs use the same six command names and describe the same setup path

## Notes

- Keep product language aligned with the chosen Mnemix-native names rather than OpenSpec-compatible names
- Treat slash commands as an integration convenience layer over the existing repo-native workflow model
