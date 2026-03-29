# Tasks: Contract Standards Support

## Workstream Goal

Make `OpenAPI`, `AsyncAPI`, and `JSON Schema` the first fully supported
contract standards in `mnemix-workflow`, with CLI scaffolding, validation, and
clear examples that fit naturally into the existing workstream model.

## Execution Slices

### Slice 1: Narrow The Standards Scope

- [x] Update the README and PRD to remove `MADR` and `Structurizr DSL` from the first-class standards story
- [x] Record the durable repo decision that standards support is focused on `OpenAPI`, `AsyncAPI`, and `JSON Schema`
- [x] Create this implementation workstream with the narrowed scope

### Slice 2: Add Contract Artifact Scaffolding

- [x] Add an `OpenAPI` template asset
- [x] Add an `AsyncAPI` template asset
- [x] Add a `JSON Schema` template asset
- [x] Add CLI support for scaffolding each artifact into the right `contracts/` location for a workstream

### Slice 3: Add Validation Commands

- [x] Add validation support for `OpenAPI`
- [x] Add validation support for `AsyncAPI`
- [x] Add validation support for `JSON Schema`
- [x] Add tests for valid and invalid examples of each supported standard

### Slice 4: Document And Teach The Contract Workflow

- [x] Add README examples for scaffold and validate commands for all three standards
- [x] Update the skill and methodology references so agents know when each standard applies
- [x] Add at least one example workstream reference that shows how contract artifacts fit beside `spec.md`, `ux.md`, `plan.md`, and `tasks.md`

## Validation Checklist

- [x] A user can explain when to use `OpenAPI`, `AsyncAPI`, and `JSON Schema`
- [x] A user can scaffold each supported contract type with a documented CLI command
- [x] A user can validate each supported contract type with a documented CLI command
- [x] The docs no longer imply support for standards outside the chosen three
- [x] The contract artifact layout is clear enough for future TUI and Studio preview support

## Notes

This workstream intentionally focuses on the three strongest contract standards.
Future workstreams can add richer linting, diffing, export flows, or additional
integrations after the core scaffold-and-validate path exists.
