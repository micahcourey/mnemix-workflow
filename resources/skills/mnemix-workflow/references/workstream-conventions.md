# Workflow Conventions

## Generated Shape

Run `mxw` from inside the target repository or linked git worktree so new
workstreams and patches are created in the right checkout.

New workstreams should be created under:

```text
workflow/workstreams/<id>-<slug>/
  STATUS.md
  spec.md
  ux.md
  plan.md
  tasks.md
  decisions/
    README.md
```

Lightweight patches should be created under:

```text
workflow/patches/0001-some-change.md
```

## Numbering

- Use zero-padded 3-digit IDs from `001` through `999`
- After `999`, continue with natural numbers starting at `1000`
- Always determine the next id numerically, not lexicographically

Patch ids use zero-padded 4-digit prefixes from `0001` onward.

Examples:

- `001-bootstrap-mnemix-workflow`
- `014-foo`
- `999-bar`
- `1000-baz`

## Naming

- Slugs should be lowercase kebab-case
- Keep the slug descriptive but compact
- Use the provided name as the title source for template placeholders

## Decisions

- Keep local decisions in `workflow/workstreams/<id>-<slug>/decisions/`
- Promote durable framework decisions to `workflow/decisions/`

## Choosing The Right Lane

Use a workstream when:

- the change spans multiple surfaces or systems
- UX needs explicit treatment
- multiple planning decisions are still open
- the work benefits from separate `spec.md`, `ux.md`, `plan.md`, and `tasks.md`

Use a patch when:

- the change is narrow and well-bounded
- a single file can capture the intent and validation clearly
- the work is a fix, chore, or minor enhancement
- every PR still needs a tracked planning artifact

## Status Metadata

- New workstreams should include `STATUS.md` when they are created
- Required frontmatter fields are:
  - `status`
  - `summary`
  - `updated`
- Optional frontmatter fields include:
  - `prs`

Patch files use the same frontmatter fields directly in the patch file itself.

Helpful commands:

- `mxw status list --status open`
- `mxw patch status list --status completed`
- `mxw agent install`
- `mxw agent update`
- `mxw validate`
- `mxw hooks install`

## Optional Slash Commands

When a repository wants chat-native workflow entrypoints in supported AI
assistants, install the repo-local slash commands with:

```bash
mxw agent install
```

Refresh them with:

```bash
mxw agent update
```

The bundled command set is:

- `/mxw:explore`
- `/mxw:track`
- `/mxw:implement`
- `/mxw:close`
- `/mxw:sync`
- `/mxw:status`

These commands are a convenience layer over the normal `mxw` workflow rather
than a separate planning system.

## Optional GitHub Issue Mirroring

When a repository wants GitHub issue visibility without moving planning source
of truth out of the repo, initialize optional mirroring with:

```bash
mxw github init --enable-auto-sync
```

This writes:

```text
workflow/github.yml
.github/workflows/mxw-github-sync.yml
```

Use:

- `mxw github sync 009` to mirror one workstream
- `mxw github sync 0005` to mirror one patch
- `mxw github sync --all` to backfill all tracked items
- `mxw github sync --status open --all` to backfill or refresh a filtered slice
- `mxw github sync --changed` for automation that refreshes already-linked changed items

Mirroring rules:

- The repo remains canonical; GitHub issues are mirrors
- Workstreams map to one parent issue plus sub-issues for `spec.md`, `ux.md`, `plan.md`, and `tasks.md`
- Patches map to one issue each
- Completed tracked items are mirrored as closed GitHub issues
- Issue titles and bodies are system-managed and may be overwritten on sync
- Teams should keep GitHub issue creation/edit permissions narrow where practical

## Optional Contract Standards

When a workstream needs machine-readable contracts, keep them inside the
workstream:

```text
workflow/workstreams/<id>-<slug>/
  contracts/
    openapi.yaml
    asyncapi.yaml
    schemas/
      some-shape.schema.json
```

Use:

- `OpenAPI` for HTTP APIs
- `AsyncAPI` for async channels or operations
- `JSON Schema` for reusable data shapes

These artifacts are optional and should only be added when the workstream
actually needs them.

## Planning Expectations

- Resolve important planning questions while creating the workstream instead of deferring them into a generic `Open Questions` section
- Keep `plan.md` focused on implementation approach and execution slices, not a separate decision phase
- If the human explicitly leaves a meaningful question unresolved, add a focused `Open Questions` section or a decision-oriented plan slice for that item
- Do not leave placeholder questions throughout the workstream when the answer is already known or can be resolved during creation
