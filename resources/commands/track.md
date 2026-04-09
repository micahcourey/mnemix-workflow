# /mxw:track

Use this command to create the right tracked unit for new work.

## Expected Behavior

1. Decide whether the request should become a full workstream or a lightweight
   patch.
2. Review existing tracked work first when that context could avoid duplicate or
   overlapping planning.
3. Use `mxw new "<name>"` for a workstream or `mxw patch new "<name>"` for a
   patch.
4. Fill in the created artifact or artifacts so they are ready for execution,
   resolving important planning questions with the user when needed.
5. Keep the resulting planning repo-native and aligned with Mnemix Workflow
   conventions.

## Workflow Guardrails

- Prefer a workstream for larger multi-artifact work.
- Prefer a patch for narrow, well-bounded fixes or enhancements.
- Do not leave placeholder planning sections behind when the answer is already
  known or can be resolved now.
