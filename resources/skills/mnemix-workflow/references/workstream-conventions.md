# Workflow Conventions

## Generated Shape

Run the scaffold command from inside the target repository or linked git worktree. The script uses the current working directory to decide where the new workstream should be created.

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

## Planning Expectations

- Resolve important planning questions while creating the workstream instead of deferring them into a generic `Open Questions` section
- Keep `plan.md` focused on implementation approach and execution slices, not a separate decision phase
- If the human explicitly leaves a meaningful question unresolved, add a focused `Open Questions` section or a decision-oriented plan slice for that item
- Do not leave placeholder questions throughout the workstream when the answer is already known or can be resolved during creation
