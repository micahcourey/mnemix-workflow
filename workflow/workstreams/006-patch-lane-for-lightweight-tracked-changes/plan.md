# Plan: Patch Lane For Lightweight Tracked Changes

## Summary

Define and implement a second tracked planning lane under `workflow/patches/`
for small changes. The lane should reuse the existing status model and future
tooling architecture while staying intentionally smaller than a full
workstream.

## Scope Analysis

### Affected Areas

| Area | Changes Required |
|------|-----------------|
| CLI | Add patch scaffolding and future patch status/list support |
| Templates | Add a single-file patch template with frontmatter and concise sections |
| Methodology Docs | Explain the patch lane and the workstream-versus-patch rule |
| Skills | Teach agents when to create a patch instead of a workstream |
| TUI / Studio Future | Keep the format compatible with future list and detail views |

### Affected Layers

- [x] Documentation
- [x] Workflow artifacts
- [x] Scripts
- [x] CLI implementation

## Technical Design

### Proposed Additions

```text
workflow/
  patches/
    0001-some-change.md

resources/skills/mnemix-workflow/assets/
  patch.md
```

### Design Constraints

- Keep the patch format to a single file
- Reuse `status`, `summary`, `updated`, and optional `prs`
- Use 4-digit numbering because patches may accumulate more quickly than workstreams
- Keep the manual authoring path simple even before future CLI and TUI improvements

### Patch File Shape

Each patch file should contain:

- frontmatter status metadata
- a short summary
- reason
- scope
- implementation notes
- validation
- references

### Decision Rule

Use a patch when:

- the change is narrow and well-bounded
- there are no major unresolved planning decisions
- the work does not need first-class UX exploration across multiple artifacts
- a single artifact can describe the intent and validation clearly

Use a full workstream when:

- the change spans multiple surfaces or systems
- UX needs explicit treatment
- multiple decisions are still open
- the work is large enough that spec, UX, plan, and tasks should be separated

## Implementation Slices

### Slice 1: Define The Patch Model

- [ ] Finalize the patch file shape and naming convention
- [ ] Record the durable decision that `workflow/patches/` is the lightweight tracked lane
- [ ] Document the rule for choosing a patch versus a workstream

### Slice 2: Scaffold And Status Support

- [ ] Add a patch template
- [ ] Add CLI support to scaffold patches numerically
- [ ] Reuse status metadata parsing and updates for patch files

### Slice 3: Documentation And Agent Guidance

- [ ] Update the README and PRD to include patches
- [ ] Update the skill and reference docs so agents choose the right lane
- [ ] Explain how future TUI and Studio views should treat patches

## Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Patch format expands too far | Medium | Medium | Keep the required sections small and compare examples against workstreams |
| Patch vs. workstream choice stays fuzzy | High | Medium | Document explicit threshold rules and examples |
| CLI duplicates workstream logic | Medium | Low | Reuse shared status and scaffolding abstractions where possible |

## References

- `README.md`
- `docs/prd.md`
- `workflow/decisions/003-workstream-numbering.md`
- `workflow/decisions/004-use-status-files-for-workstream-state.md`
- `workflow/decisions/005-use-frontmatter-as-canonical-status-metadata.md`
- `workflow/decisions/010-use-patches-for-lightweight-tracked-work.md`
