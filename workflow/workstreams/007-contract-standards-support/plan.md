# Plan: Contract Standards Support

## Summary

Implement a narrow first slice of standards support focused on contract
artifacts: add scaffold and validation commands for `OpenAPI`, `AsyncAPI`, and
`JSON Schema`, place the resulting files under predictable per-workstream
`contracts/` paths, and update the docs so the supported standards story
matches the product reality.

## Scope Analysis

### Affected Areas

| Area | Changes Required |
|------|-----------------|
| CLI surface | Add `openapi`, `asyncapi`, and `schema` command groups |
| Scaffolding assets | Add templates for the three supported standards |
| Validation layer | Add validation helpers and dependencies |
| README and PRD | Narrow the standards story and add command examples |
| Future TUI compatibility | Keep the file layout previewable without special-case hacks |

### Affected Layers

- [x] Documentation
- [x] Workflow artifacts
- [ ] Scripts
- [x] CLI implementation

## Technical Design

### Proposed Additions

```text
resources/skills/mnemix-workflow/assets/
  openapi.yaml
  asyncapi.yaml
  schema.json
src/commands/
  openapi.rs
  asyncapi.rs
  schema.rs
workflow/workstreams/<id>/contracts/
  openapi.yaml
  asyncapi.yaml
  schemas/
    <name>.schema.json
```

### Design Constraints

- Keep commands explicit and standard-named instead of hiding them behind a vague umbrella abstraction
- Start with scaffold and validate; defer richer lint, diff, and export workflows
- Keep standards support optional and per-workstream
- Favor mature validator libraries or subprocess-based tooling that will package cleanly with Rust

## Implementation Slices

### Slice 1: Narrow And Document The Standards Scope

- Update the README and PRD so they only promise `OpenAPI`, `AsyncAPI`, and `JSON Schema`
- Record the durable repo decision that first-class standards support is contract-focused

### Slice 2: Add Contract Templates And CLI Commands

- Add asset templates for `OpenAPI`, `AsyncAPI`, and `JSON Schema`
- Add `mxw openapi init <workstream>` and `mxw openapi validate <workstream-or-path>`
- Add `mxw asyncapi init <workstream>` and `mxw asyncapi validate <workstream-or-path>`
- Add `mxw schema new <workstream> <name>` and `mxw schema validate <workstream-or-path>`
- Ensure commands create files under predictable `contracts/` paths inside the workstream

### Slice 3: Teach And Verify The Contract Workflow

- Add README examples for all three standards
- Update the skill and methodology references so agents know when to create contract artifacts
- Add tests covering scaffolding and validation for each supported standard

## Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Standards dependencies are difficult to package cleanly | Medium | Medium | Prefer lightweight validators first and keep the scope to scaffold plus validate |
| Users are unsure which standard to choose | Medium | Medium | Add explicit README guidance and examples tied to common use cases |
| Command sprawl makes the CLI harder to learn | Medium | Low | Use direct, symmetric command names and keep the surface intentionally small |

## References

- `README.md`
- `docs/prd.md`
- `workflow/decisions/011-focus-standards-support-on-openapi-asyncapi-and-json-schema.md`
