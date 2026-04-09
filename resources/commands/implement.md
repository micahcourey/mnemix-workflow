# /mxw:implement

Use this command to implement from existing workflow artifacts.

## Expected Behavior

1. Read the relevant workstream artifacts or patch content before changing code.
2. Implement the requested work using the tracked artifacts as the shared source
   of intent.
3. Update task state or related workflow metadata when the implementation
   materially advances the tracked work.
4. Run the smallest meaningful validation set for the changed behavior.
5. Summarize what changed, what was validated, and any follow-up risk.

## Workflow Guardrails

- Treat `spec.md`, `ux.md`, `plan.md`, `tasks.md`, and `STATUS.md` as the
  workflow source of truth.
- If the tracked artifacts are missing or materially incomplete, stop and use
  `/mxw:track` or ask for clarification before coding.
- Prefer finishing the requested implementation end to end, not just partial
  analysis.
