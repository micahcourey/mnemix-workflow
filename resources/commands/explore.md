# /mxw:explore

Use this command when you need to think, investigate, or gather context before
changing tracked work.

## Expected Behavior

1. Inspect the repository, relevant code, tests, and existing workflow artifacts.
2. If tracked work already exists, read the relevant `spec.md`, `ux.md`,
   `plan.md`, `tasks.md`, `STATUS.md`, or patch file before suggesting changes.
3. Summarize what you found, identify risks or unknowns, and recommend the next
   workflow step.
4. Prefer analysis and recommendations over code changes unless the user also
   asked you to implement.

## Workflow Guardrails

- Keep `mxw` and repo artifacts as the source of truth.
- If no tracked work exists yet and new work should be created, suggest or use
  `/mxw:track`.
- If implementation should begin immediately from tracked artifacts, suggest or
  use `/mxw:implement`.
