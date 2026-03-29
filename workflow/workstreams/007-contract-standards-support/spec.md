# Feature Spec: Contract Standards Support

## Summary

Add first-class support for `OpenAPI`, `AsyncAPI`, and `JSON Schema` so
repositories using `mnemix-workflow` can scaffold, validate, and document
machine-readable API and data contracts with the same CLI and methodology that
already manage workstreams, patches, and status.

## Problem

`mnemix-workflow` currently says it selectively uses open standards, but that
promise is only documented conceptually. There are no standards-specific
commands, templates, validation helpers, or examples in the CLI. That creates a
gap between the framework narrative and the product reality, especially for
teams that expect machine-readable contract artifacts to be part of a planning
and implementation workflow.

## Users

- Primary persona: maintainer or engineer adding a new API or shared contract to a repository
- Secondary persona: AI implementation agent that needs contract artifacts to be scaffolded and validated consistently

## Goals

- Make `OpenAPI`, `AsyncAPI`, and `JSON Schema` first-class supported standards in the CLI
- Provide a clear file layout and workflow for contract artifacts inside workstreams
- Add validation commands so contract files are not just templates
- Add concrete examples so users understand when and how to use each standard

## Non-Goals

- Add support for ADR-specific standards or architecture DSLs in this workstream
- Build every possible linting, diffing, or export workflow for these standards
- Force every workstream to include contract artifacts whether or not they need them

## User Value

Users get a coherent contract story instead of a hand-wavy one: when a feature
touches APIs or shared data models, they can scaffold the right standard-backed
artifact with `mxw`, validate it, and keep it connected to the rest of the
workstream narrative.

## Functional Requirements

- The CLI should scaffold an `OpenAPI` contract for a workstream
- The CLI should scaffold an `AsyncAPI` contract for a workstream
- The CLI should scaffold one or more `JSON Schema` files for a workstream
- The CLI should validate each of the three supported standards
- The README should include at least one example command for each supported standard
- The methodology docs should explain when to choose each standard
- The TUI plan should remain compatible with previewing these artifacts later without requiring a redesign

## Constraints

- Keep the first slice intentionally narrow: scaffold, validate, and document
- Use per-workstream contract locations rather than inventing a separate hidden store
- Preserve the existing workstream and patch mental model
- Avoid overstating support for standards that are not implemented

## Success Criteria

- A user can scaffold and validate `OpenAPI`, `AsyncAPI`, and `JSON Schema` artifacts with `mxw`
- The README and PRD no longer imply a broader standards scope than the product supports
- Contract artifacts fit naturally into workstream planning without adding too much ceremony
- The command surface is clear enough that future TUI and Studio integration can build on it

## Risks

- Trying to do too much standards tooling at once could bloat the CLI
- Poor command naming could make the standards layer feel bolted on
- Heavy validator dependencies could complicate packaging

## References

- `README.md`
- `docs/prd.md`
- `workflow/decisions/011-focus-standards-support-on-openapi-asyncapi-and-json-schema.md`
