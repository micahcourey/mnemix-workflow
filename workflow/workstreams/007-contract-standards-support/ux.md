# UX Spec: Contract Standards Support

## Summary

The standards experience should feel like a natural extension of the existing
CLI, not a separate toolchain bolted onto the side. A user should be able to
decide which contract standard applies, scaffold the file in the right place,
validate it, and understand how it fits back into the workstream they are
already using.

## Users And Context

- Primary persona: maintainer or engineer planning and implementing an API or shared contract change
- Secondary persona: AI implementation agent operating inside a repo that uses Mnemix Workflow
- Context of use: terminal-driven planning and implementation inside a repository initialized with `mxw init`
- Preconditions: the repository has `workflow/`, and the relevant workstream already exists

## User Goals

- Decide quickly whether the work needs `OpenAPI`, `AsyncAPI`, `JSON Schema`, or a combination
- Create the right artifact in the right place without manual path guessing
- Validate contract files before or during implementation

## Experience Principles

- Keep standards support additive, not mandatory
- Favor clear command names over clever abstraction
- Make the file locations predictable from the workstream id
- Keep examples concrete enough that users can copy them directly

## Primary Journey

1. A user is working in an existing workstream that adds a REST endpoint, event, or shared payload shape.
2. The user chooses the appropriate contract standard: `OpenAPI`, `AsyncAPI`, or `JSON Schema`.
3. The user runs the corresponding `mxw` scaffold command against the workstream.
4. The CLI creates the contract artifact in a predictable `contracts/` location inside that workstream.
5. The user edits the generated contract file.
6. The user runs the matching validation command and gets a clear success or error result.
7. The contract file becomes part of the workstream's durable implementation record.

## Alternate Flows

### Flow: Workstream Needs More Than One Standard

- Trigger: the feature adds more than one type of contract artifact
- Path: the user scaffolds multiple supported standards inside the same workstream
- Expected outcome: the standards commands compose cleanly without implying only one contract artifact may exist

### Flow: Contract Not Needed

- Trigger: the workstream does not touch a public API, async interface, or reusable data shape
- Path: the user skips standards commands entirely
- Expected outcome: standards support feels optional and situational rather than mandatory ceremony

## Surfaces

### Surface: Contract Commands

- Purpose: scaffold and validate supported standards within the workstream workflow
- Key information: workstream id, standard type, target path under `contracts/`, and validation outcome
- Available actions: scaffold `OpenAPI`, scaffold `AsyncAPI`, scaffold `JSON Schema`, and validate each artifact type
- Navigation expectations: command names should be easy to remember and mirror the standard names directly

### Surface: Repository Docs

- Purpose: teach when to use each standard and show copyable examples
- Key information: when to choose each standard, where files live, and exact example commands
- Available actions: scan examples and cross-reference the workstream model
- Navigation expectations: a user should be able to find the right example in the root README without digging through multiple files

## States

### Loading

- Validation should make progress and result states obvious.

### Empty

- A workstream may have no contract artifacts yet. That should be normal until the user explicitly adds one.

### Success

- The user scaffolded the correct contract, validated it, and understands where it now lives in the workstream.

### Error

- If the user targets a missing workstream or provides an invalid contract file, the CLI should explain exactly what is wrong and what command to run next.

## Interaction Details

- Inputs: workstream id, contract standard type, and optionally a schema or artifact name
- Feedback: scaffold commands should print the created path; validation commands should print pass/fail with actionable errors
- Transitions: workstreams may move from having no contracts to multiple validated contracts as implementation evolves
- Keyboard behavior: all commands should work comfortably in a normal terminal workflow
- Responsive behavior: future TUI and Studio views should be able to preview contract files without changing the underlying layout

## Content And Tone

- Prefer direct terminology from the standards themselves
- Avoid implying that users must learn a separate vocabulary to adopt standards support
- Be explicit about the difference between sync APIs, async interfaces, and shared schemas

## Accessibility Requirements

- Keep generated artifacts plain-text and accessible in editors, terminals, and screen readers
- Ensure validation messages are readable and specific rather than relying on color or visual formatting alone

## Acceptance Scenarios

```gherkin
Scenario: Scaffold an OpenAPI contract for a workstream
  Given a repository initialized with Mnemix Workflow
  And a workstream that adds a REST endpoint
  When the user runs the OpenAPI scaffold command
  Then the CLI should create an OpenAPI file under that workstream's contracts folder
  And the user should be able to validate it with a matching command

Scenario: Validate a JSON Schema artifact
  Given a workstream that contains a JSON Schema file
  When the user runs the schema validation command
  Then the CLI should report whether the schema is valid
  And any validation errors should identify the failing file clearly
```

## References

- `README.md`
- `docs/prd.md`
- `workflow/decisions/011-focus-standards-support-on-openapi-asyncapi-and-json-schema.md`
