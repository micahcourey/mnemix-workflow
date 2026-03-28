# UX Spec: Patch Lane For Lightweight Tracked Changes

## Summary

The patch lane should feel fast, obvious, and lower-friction than a full
workstream. A user creating a small bug fix plan should be able to capture the
intent, scope, and validation in one file without wondering whether they are
breaking the framework.

## Users And Context

- Primary persona: engineer making a small bug fix, chore, or narrow enhancement
- Secondary persona: AI implementation agent that needs a concise artifact before coding
- Context of use: terminal- or editor-driven planning inside a repository that uses Mnemix Workflow
- Preconditions: the repository has already been initialized with `workflow/`

## User Goals

- Decide quickly whether a change needs a patch or a full workstream
- Create a lightweight tracked artifact with minimal ceremony
- Understand how the patch maps to a PR and status lifecycle

## Experience Principles

- Choose the smallest responsible planning unit
- Keep the patch artifact readable in one screen or short editor pass
- Reuse familiar workflow concepts instead of inventing a parallel vocabulary
- Make future automation possible without making the manual path awkward

## Primary Journey

1. A user identifies a small change such as a bug fix or minor enhancement.
2. The user applies the framework rule and decides the work is too small for a full workstream.
3. The user creates a new patch file under `workflow/patches/`.
4. The user fills in the frontmatter and concise sections for intent, scope, implementation notes, and validation.
5. The user links the eventual PR in the patch metadata and updates the status as work progresses.
6. The patch lands as the planning record for the PR without requiring a full folder of artifacts.

## Alternate Flows

### Work Grows Mid-Implementation

If the change turns out to involve broader UX impact, multiple open decisions,
or larger coordination risk, the patch should be promoted into a full
workstream rather than overloading the lightweight format.

### Tiny Non-Behavioral Edit

If the change is truly trivial and has no behavioral impact, teams may still
choose to use a patch because the project rule is that every PR is tracked.
The patch format should remain light enough that this does not feel punishing.

## Surfaces

### Surface: Patch File

- Purpose: hold the full planning record for a small tracked change in one place
- Key information:
  - frontmatter status metadata
  - summary and reason
  - narrow scope
  - implementation notes
  - validation
  - linked PRs
- Available actions:
  - create
  - update status
  - record implementation and verification details
- Navigation expectations: the file should be easy to open directly and later easy for CLI and Studio to surface

## States

### Loading

[Not applicable for the file format itself, but future CLI views should be able to load patches alongside workstreams.]

### Empty

[A repository may have no patches yet. This should not imply misconfiguration.]

### Success

[The patch clearly communicates what changed and why, with current status metadata.]

### Error

[If required frontmatter is missing or malformed, future tooling should warn clearly.]

## Interaction Details

- Inputs: a short descriptive title, concise scope notes, and status metadata
- Feedback: future CLI commands should confirm whether a patch was created or updated successfully
- Transitions: status should follow the same `proposed`, `open`, `completed` model as workstreams
- Keyboard behavior: authoring should work comfortably in plain text editors and terminal tools
- Responsive behavior: future TUI and Studio views should be able to render patch metadata without special-case complexity

## Content And Tone

- Use direct, practical language
- Make it obvious that patches are for narrow tracked work, not informal notes
- Explain promotion to a workstream as a positive step, not a failure

## Accessibility Requirements

- Keep the artifact plain-text and screen-reader friendly
- Prefer simple section headings and short paragraphs over dense prose
- Ensure future CLI and Studio views expose patch metadata with the same accessibility expectations as workstreams

## Acceptance Scenarios

```gherkin
Scenario: Create a lightweight tracked patch
  Given a repository uses Mnemix Workflow
  And a user has a narrow bug fix to make
  When the user creates a patch
  Then the patch should live in a single file under workflow/patches
  And the file should contain the required status metadata
  And the patch should be lighter than a full workstream

Scenario: Promote a patch into a workstream
  Given a patch grows beyond a narrow change
  When the user realizes the work now has larger coordination or UX scope
  Then the framework should direct them toward a full workstream
  And the patch should not be stretched to carry full-workstream complexity
```

## References

- `README.md`
- `docs/prd.md`
- `workflow/workstreams/004-status-metadata-and-cli-support/spec.md`
