# Plan: Github Issue Support

## Summary

Implement GitHub issue support as an optional repo extension, not a required
methodology layer. The v1 path should add a small GitHub config file, repo-side
issue linkage metadata, CLI commands for mirroring workstreams and patches, and
an opt-in auto-sync workflow that uses `mxw github sync --changed` to project
repo changes into already-linked GitHub issues.

## Scope Analysis

### Affected Areas

| Area | Changes Required |
|------|-----------------|
| CLI commands | Add GitHub issue setup, sync, and validation commands |
| Workflow artifacts | Store GitHub linkage in `STATUS.md` and patch frontmatter |
| Documentation | Explain repo-canonical GitHub mirroring and permission guidance |
| GitHub Actions | Add optional repo-to-GitHub auto-sync workflow |
| TUI planning | Preserve future compatibility for showing GitHub linkage in `mnx` |

### Affected Layers

- [x] Documentation
- [x] Workflow artifacts
- [x] Scripts
- [x] CLI implementation

## Technical Design

### Proposed Additions

```text
workflow/github.yml
src/github.rs
src/commands/github.rs
.github/workflows/mxw-github-sync.yml
workflow/workstreams/<id>/STATUS.md        # add github linkage metadata
workflow/patches/<id>-<slug>.md            # add github linkage metadata
README.md
docs/prd.md
resources/skills/mnemix-workflow/SKILL.md
resources/skills/mnemix-workflow/references/workstream-conventions.md
```

### Design Constraints

- The repo stays canonical for all planning content and status fields
- GitHub issue titles and bodies are full replacements from repo artifacts, not merge targets
- `STATUS.md` is not mirrored as its own sub-issue
- Workstream-local `decisions/` remain repo artifacts and are not mirrored as separate issues in v1
- Auto-sync must be opt-in and safe to disable
- The implementation should prefer the simplest GitHub API path that supports parent issues plus sub-issues reliably

## Implementation Slices

### Slice 1

- Add the GitHub config model and linkage metadata shape
- Define the issue templates produced for workstream parent issues, workstream file sub-issues, and patches
- Add read and write support for GitHub linkage metadata in workstream `STATUS.md` and patch frontmatter
- Define `--changed` semantics around changed tracked items that already have GitHub linkage
- Define `--all` backfill behavior for previously-created tracked items
- Define filtered sync behavior such as `--status open` and `--status completed`

### Slice 2

- Implement `mxw github init`
- Implement `mxw github sync <target>` for one workstream or patch
- Implement `mxw github sync --all` for backfill
- Implement filtered sync by status
- Implement parent issue plus sub-issue creation and update for workstreams
- Implement single-issue creation and update for patches
- Map repo `status` to GitHub issue open/closed state on sync
- Keep first issue creation explicit and manual in v1 instead of hiding it behind auto-sync

### Slice 3

- Implement `mxw github sync --changed`
- Add validation and dry-run output for sync operations
- Add opt-in GitHub Actions auto-sync for repo-to-GitHub mirroring on push to `main`
  using `mxw github sync --changed`
- Document issue-edit restrictions and recommended GitHub permission model

## Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| GitHub API permissions differ by repo or token | Sync may fail or create partial mirrors | Medium | Validate auth up front and emit precise permission errors |
| Sub-issue support behaves differently than expected | Workstream hierarchy could be incomplete | Medium | Start with a thin abstraction around GitHub issue operations and verify against GitHub's native sub-issue support |
| Users manually edit mirrored issues | Repo and GitHub can drift temporarily | High | Document mirrors as system-managed and overwrite titles and bodies on sync |
| Auto-sync surprises users | Teams may hesitate to enable the feature | Medium | Keep auto-sync opt-in, limit it to `--changed` updates for already-linked items, and document exact triggers |
| Large backfill runs create too many mirrors at once | Teams may avoid adoption or hit rate limits | Medium | Support filtered sync and make `--all` an explicit backfill action |

## References

- `README.md`
- `docs/prd.md`
- `workflow/workstreams/005-interactive-tui-mode/plan.md`
- `workflow/workstreams/006-patch-lane-for-lightweight-tracked-changes/spec.md`
