# UX Spec: Status Metadata And Cli Support

## Summary

Workstream status should feel explicit and low-friction: the repository keeps stable paths, while `mxw` and later Studio provide the operational view of which workstreams are proposed, open, or completed.

## Users And Context

- Primary persona: maintainer or AI implementation agent working inside a repository that uses Mnemix Workflow
- Context of use: checking, setting, and listing workstream status from the CLI while keeping repository artifacts stable
- Preconditions: the repository has been initialized for Mnemix Workflow and contains one or more workstreams

## User Goals

- See whether a workstream is proposed, open, or completed without guessing from checklists
- Mark a workstream complete without moving its folder
- Keep the status readable in git while still giving tooling a structured source of truth
- Get a helpful reminder before push when workstream changes may require a status review

## Experience Principles

- Keep the repository path stable and move status into metadata, not folder placement
- Use structured metadata for tools and plain prose for humans
- Make the happy path obvious from CLI commands and outputs
- Avoid forcing users to maintain duplicate status information
- Automate low-risk metadata refreshes, but keep semantic status changes explicit

## Primary Journey

1. The user creates or opens a workstream in a repository using Mnemix Workflow.
2. New workstreams start with a default `STATUS.md` using `open` unless the user explicitly chooses another supported initial status.
3. The workstream can optionally include related PR numbers in status metadata once implementation begins.
4. The user runs a CLI command to inspect, update, or list workstream status.
5. The CLI reads or updates the frontmatter while leaving the workstream path unchanged.
6. If the human wants list filtering in v1, the user can later list workstreams by status through the CLI, and Studio can consume the same metadata contract.
7. Before push, an optional local hook can refresh `updated` and warn that if this push leads to a PR that completes the workstream, `STATUS.md` should be changed to `completed`.

## Alternate Flows

### Flow: Missing Status File

- Trigger: a workstream has no `STATUS.md`
- Path: the CLI should create a default status file when appropriate or fail with a clear corrective message when recovery is unsafe
- Expected outcome: the user gets a direct next step rather than silent inference

### Flow: Invalid Frontmatter

- Trigger: `STATUS.md` exists but the frontmatter is malformed or missing required fields
- Path: the CLI rejects the update or read operation
- Expected outcome: stderr explains what field or format is invalid

## Surfaces

### Surface: STATUS.md

- Purpose: the durable workstream status artifact in the repo
- Key information: machine-readable status metadata, optional related PR numbers, and a short human-readable note
- Available actions: inspect in git, edit manually if needed, or update through the CLI
- Navigation expectations: humans should understand the file quickly without needing to learn a custom binary format

### Surface: CLI Status Output

- Purpose: the operational view of workstream status
- Key information: current status, summary, and updated date
- Available actions: show status, set status, list by status
- Navigation expectations: the most common status operations should not require opening files manually

### Surface: Local Hook Warning

- Purpose: remind the user about status responsibilities before code leaves the local checkout
- Key information: touched workstreams, whether `STATUS.md` may need review, and a reminder about setting `completed` when a PR fully completes the workstream
- Available actions: update `STATUS.md`, proceed intentionally, or bypass if the reminder does not apply
- Navigation expectations: the message should be short, direct, and easy to act on

## States

### Loading

- Status reads and updates should be fast enough that no explicit progress UI is needed

### Empty

- A new workstream should be able to start with a default status file or a clearly documented way to add one

### Success

- The CLI confirms the new or updated status and points at the affected workstream when helpful

### Error

- Errors should explain whether the problem is a missing file, invalid frontmatter, or unknown status value

### Warning

- Local hook warnings should explain what changed, what metadata may need review, and that semantic status changes remain the user's decision

## Interaction Details

- Input behavior: accept status values as a controlled set such as `proposed`, `open`, and `completed`
- Input behavior: `mxw new` should allow an explicit initial status while defaulting to `open`
- Input behavior: related PR numbers should be represented as a simple list when present
- Feedback: keep read output concise and update output explicit
- Feedback: hook messages should remind the user, for example, `If this push is for a PR that completes the workstream, update STATUS.md to completed.`
- Keyboard behavior: all flows should work cleanly from a keyboard-only terminal session
- Responsive behavior: not applicable beyond normal terminal wrapping

## Content And Tone

- Messages should use direct labels like `Status: open` or `Updated status for workstream 004`
- Errors should be specific and corrective rather than vague

## Accessibility Requirements

- The full workflow must be usable from a keyboard-only terminal session
- Output should remain readable in terminal screen readers
- Status should never rely on color alone

## Acceptance Scenarios

```gherkin
Scenario: Read the current status of a workstream
  Given a workstream contains STATUS.md with valid frontmatter
  When the user runs a CLI command to inspect its status
  Then the CLI should show the current status value
  And the CLI should show related PR numbers when they exist
  And the CLI should show the summary without inferring from prose or task checkboxes

Scenario: Mark a workstream completed
  Given a workstream currently has status open
  When the user runs a CLI command to set the status to completed
  Then the CLI should update the STATUS.md frontmatter
  And the workstream folder path should remain unchanged

Scenario: Reject malformed status metadata
  Given a workstream has STATUS.md with invalid frontmatter
  When the user runs a CLI command to inspect or update status
  Then the CLI should exit with a non-zero status
  And stderr should explain the metadata problem clearly

Scenario: Warn before push when workstream files changed
  Given a user changed files inside a workstream
  And STATUS.md was not reviewed in the same change
  When the local reminder hook runs before push
  Then the hook should warn that STATUS.md may need review
  And the warning should remind the user to set status to completed if the PR completes the workstream
```

## Open Questions

- If the human wants richer completion reporting in v1, should completion metadata such as `completed_at` and `completed_by` be surfaced immediately?

## References

- `workflow/workstreams/003-cli-bootstrap/spec.md`
- `workflow/workstreams/003-cli-bootstrap/ux.md`
- `workflow/decisions/007-resolve-planning-questions-during-workstream-creation.md`
- `workflow/decisions/008-create-status-file-when-a-workstream-is-created.md`
- `workflow/decisions/009-track-related-pull-requests-in-status-metadata.md`
- `README.md`
