# Decisions

This folder holds durable, repo-level decisions for this repository.

Use this location when a decision:

- constrains future workstreams
- defines framework behavior or repository conventions
- should outlive the original workstream where it was discovered

Workstream-local decisions should stay beside the workstream in `workflow/workstreams/<id>/decisions/` until they need to be promoted here.

## Current ADRs

- `001` [`workflow artifact root`](./001-workflow-artifact-root.md): keep repo-level planning artifacts under `workflow/`
- `002` [`bootstrap with one skill`](./002-bootstrap-with-one-skill.md): start with a single `mnemix-workflow` skill before splitting capabilities
- `003` [`workstream numbering`](./003-workstream-numbering.md): use `001` to `999`, then continue naturally with `1000+`
- `004` [`use STATUS.md files for workstream state`](./004-use-status-files-for-workstream-state.md): keep workstream paths stable and store lifecycle state in `STATUS.md`
- `005` [`use frontmatter as canonical status metadata`](./005-use-frontmatter-as-canonical-status-metadata.md): make frontmatter the machine-readable source of truth for `STATUS.md`
- `006` [`start with proposed, open, and completed`](./006-start-with-proposed-open-completed-status-values.md): keep the initial status model intentionally small
- `007` [`resolve planning questions during workstream creation`](./007-resolve-planning-questions-during-workstream-creation.md): keep workstreams implementation-ready by default, while allowing focused open questions only when the human explicitly defers a decision
- `008` [`create STATUS.md when a workstream is created`](./008-create-status-file-when-a-workstream-is-created.md): every new workstream should start with a status file
- `009` [`track related pull requests in status metadata`](./009-track-related-pull-requests-in-status-metadata.md): support an optional `prs` field for linked PR numbers
- `010` [`use patches for lightweight tracked work`](./010-use-patches-for-lightweight-tracked-work.md): track narrow fixes and minor enhancements in `workflow/patches/` instead of forcing every PR into a full workstream
