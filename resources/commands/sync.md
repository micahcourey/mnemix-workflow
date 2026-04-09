# /mxw:sync

Use this command to sync tracked work into the repository's configured issue
tracker integration.

## Expected Behavior

1. Inspect the repository configuration and determine which tracker integration
   is available.
2. If GitHub issue mirroring is configured, use the `mxw github ...` command
   surface to sync the requested work.
3. If no tracker integration is configured, explain that clearly and tell the
   user what setup step is missing.
4. Keep the repo as the source of truth and treat external issues as mirrors.

## Workflow Guardrails

- Today GitHub is the primary shipped sync surface. Future tracker providers may
  be added behind the same command language.
- Do not hand-edit mirrored issue bodies when `mxw` can sync from repo
  artifacts.
- Be explicit about whether you are syncing one tracked item, all tracked work,
  or a filtered slice.
