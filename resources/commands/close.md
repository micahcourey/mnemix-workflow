# /mxw:close

Use this command to mark tracked work finished in the Mnemix Workflow model.

## Expected Behavior

1. Confirm the tracked work is ready to finish.
2. Run or summarize the expected validation for the work.
3. Update the relevant workstream or patch status to `completed`.
4. Refresh summaries, linked PR metadata, or related workflow notes when
   appropriate.
5. Suggest `/mxw:sync` if the repository mirrors tracked work into an external
   issue tracker.

## Workflow Guardrails

- Use Mnemix Workflow status language such as `completed`.
- Do not assume archive-folder semantics.
- If meaningful work remains open, explain what is still incomplete instead of
  closing the tracked item prematurely.
