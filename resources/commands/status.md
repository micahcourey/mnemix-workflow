# /mxw:status

Use this command to inspect or report the status of tracked work.

## Expected Behavior

1. Determine whether the user wants the status of one workstream, one patch, or
   a broader list.
2. Use `mxw status ...` for workstreams and `mxw patch status ...` for patches.
3. Report the current status value, summary, update date, and linked PRs when
   present.
4. If the user is unsure which tracked item to inspect, list the relevant open
   or completed items first.

## Workflow Guardrails

- Keep status reporting grounded in repo metadata, not guesswork.
- Use the numeric id or full tracked item name when referring to a workstream or
  patch.
- If the requested tracked item does not exist, say so clearly and recommend the
  next best workflow action.
