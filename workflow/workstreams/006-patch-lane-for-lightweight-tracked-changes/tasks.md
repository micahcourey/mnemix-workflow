# Tasks: Patch Lane For Lightweight Tracked Changes

## Workstream Goal

Add a lightweight `workflow/patches/` lane so every PR can stay tracked in
Mnemix Workflow without forcing narrow fixes and chores into the full
workstream artifact set.

## Execution Slices

### Slice 1: Define The Patch Model

- [x] Define the patch file format and required sections
- [x] Define the status metadata fields for patches
- [x] Define the threshold rule for patch versus workstream
- [x] Record the durable naming and methodology decision in `workflow/decisions/`

### Slice 2: Add Scaffolding And CLI Support

- [x] Add a patch template under the framework assets
- [x] Add CLI support for creating a new patch
- [x] Add CLI support for reading and updating patch status metadata
- [x] Ensure patch numbering uses 4-digit numeric prefixes

### Slice 3: Document And Teach The Lane

- [x] Update the root README with the patch lane and usage examples
- [x] Update the PRD to include patches in the product model
- [x] Update the skill and methodology reference docs so agents know when to create a patch
- [x] Add at least one concrete example comparing a patch-sized change to a workstream-sized change

## Validation Checklist

- [x] A user can explain the difference between a patch and a workstream
- [x] A user can scaffold a patch with one command
- [x] A patch carries status and PR linkage metadata consistent with workstreams
- [x] The patch lane stays clearly smaller and faster than a full workstream
- [x] Future CLI and Studio views can include patches without a second state model

## Notes

This workstream is about defining the lightweight tracked lane itself. Future
workstreams can extend TUI and Studio views to surface patches alongside
workstreams once the core patch model and CLI support exist.
