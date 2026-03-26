# Plan: Bootstrap Mnemix Workflow

## Summary

This workstream establishes the initial repository structure and the first set of planning artifacts so the framework can be developed by dogfooding its own method.

## Scope Analysis

### Affected Areas

| Area | Changes Required |
|------|-----------------|
| Repository structure | Add root docs and a workflow artifact root containing workstreams and decisions |
| Methodology docs | Define vocabulary and initial framework narrative |
| Planning artifacts | Seed the first workstream with spec, UX, plan, and tasks |

### Affected Layers

- [x] Documentation
- [x] Workflow artifacts
- [ ] CLI implementation
- [ ] Standards adapters
- [ ] Validation engine

## Technical Design

### Repository Structure

```text
docs/
  methodology/
  plans/
workflow/
  decisions/
  workstreams/
    001-bootstrap-mnemix-workflow/
```

### Standards Usage

- No standards adapter is required to create the initial repository shape.
- The bootstrap work should define where repo-level decisions and future standards-backed artifacts will live.

### Integration Strategy

- Keep the repo standalone and lightweight.
- Preserve compatibility with the larger Mnemix ecosystem vocabulary.
- Use the first workstream as the source of truth for the next implementation phase.

## Implementation Phases

### Phase 1: Seed The Repository

- Add `README.md`
- Add methodology docs
- Add repo-level `workflow/decisions/`
- Add the first workstream structure under `workflow/workstreams/`

### Phase 2: Refine The Method

- Review whether file naming and folder naming still feel right in practice
- Identify which foundational decisions deserve repo-level ADRs
- Prepare the next workstream for CLI bootstrap

### Phase 3: Transition To Implementation

- Create the next workstream focused on CLI and template scaffolding
- Begin actual Rust implementation planning

## Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| The framework feels too heavy on first contact | High | Medium | Keep the initial repo shape minimal and centered on one active workstream |
| Naming feels too abstract | Medium | Medium | Keep file names plain and explain vocabulary in the naming-system doc |
| Repo docs drift from active workstreams | Medium | Low | Treat workstreams as the operational center and keep docs concise |

## Open Questions

- Should the next workstream be `002-cli-bootstrap` or should CLI planning stay inside `001` a bit longer?
- Which foundational repo decisions should be formalized first?

## References

- `docs/prd.md`
- `docs/methodology/naming-system.md`
- `workflow/workstreams/001-bootstrap-mnemix-workflow/spec.md`
- `workflow/workstreams/001-bootstrap-mnemix-workflow/ux.md`
