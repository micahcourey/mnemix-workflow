# Workstream Conventions

## Generated Shape

Run the scaffold command from inside the target repository or linked git worktree. The script uses the current working directory to decide where the new workstream should be created.

New workstreams should be created under:

```text
workflow/workstreams/<id>-<slug>/
  spec.md
  ux.md
  plan.md
  tasks.md
  decisions/
    README.md
```

## Numbering

- Use zero-padded 3-digit IDs from `001` through `999`
- After `999`, continue with natural numbers starting at `1000`
- Always determine the next id numerically, not lexicographically

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
