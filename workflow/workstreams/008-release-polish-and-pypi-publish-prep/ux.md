# UX Spec: Release Polish And Pypi Publish Prep

## Summary

The release and install experience should feel boring in the best way: a
maintainer should be able to follow a short documented release path, and a new
user should be able to install the product with `pipx` and immediately discover
the right entrypoints.

## Users And Context

- Primary persona: Maintainer preparing the first public release
- Context of use: terminal-driven release work in the `mnemix-workflow` repo and
  clean-environment install verification
- Preconditions: the repo is on a release-ready commit and the maintainer has
  GitHub and PyPI publishing access

## User Goals

- Cut a release with confidence using a repeatable checklist and script path
- Verify that the shipped package installs and launches the expected commands
- Understand quickly whether to use `mnemix-workflow`, `mxw`, or `mnx`

## Experience Principles

- Familiar to maintainers already using the `mnemix` release flow
- Explicit over magical for release operations
- Clear command discoverability for first-time installed users

## Primary Journey

1. The maintainer checks the release checklist and confirms the target version.
2. They run local preflight and release-prep commands from a clean `main`
   checkout.
3. They merge the release-prep changes and run the publish command or workflow.
4. They verify the package on PyPI and in a clean `pipx` install.
5. A new user installs `mnemix-workflow` and can discover `mxw` and `mnx`
   without confusion.

## Alternate Flows

### Flow: Preflight Failure

- Trigger: a local release preflight script fails
- Path: the maintainer gets a clear failing step and does not continue to the
  publish stage
- Expected outcome: the release is blocked until the underlying issue is fixed

### Flow: Install Verification Failure

- Trigger: `pipx install mnemix-workflow` succeeds but one or more commands are
  missing or broken
- Path: the smoke test fails and the package is not considered release-ready
- Expected outcome: packaging is fixed before a public release is announced

## Surfaces

### Surface: Release Checklist / Runbook

- Purpose: guide a maintainer through the repeatable release path
- Key information: version alignment, preflight commands, publish commands,
  post-release verification
- Available actions: prepare release, publish release, verify package
- Navigation expectations: easy to scan top-to-bottom during a release

### Surface: CLI Help

- Purpose: help a new installed user understand the available entrypoints
- Key information: `mnemix-workflow`, `mxw`, `mnx`, release-related commands,
  hook install, validation
- Available actions: inspect help, run commands, move into TUI
- Navigation expectations: short, obvious, and product-facing

## States

### Ready To Release

- The release checklist is green and publish can proceed

### Release Blocked

- A preflight or smoke test failure stops the release path with a clear next step

### Published

- The package is live on PyPI and install verification passes

### Error

- Script and workflow failures should be explicit and actionable, not vague

## Interaction Details

- Command inputs should be explicit and copyable from docs
- Help output should clearly distinguish the roles of `mnemix-workflow`, `mxw`,
  and `mnx`
- The release flow should support dry-run style verification where practical
- The TUI shortcut should remain discoverable from installed usage docs and help

## Content And Tone

- Important labels/messages:
  - package install command
  - release-prep command
  - publish command
  - install smoke-test command
- Voice and tone notes:
  - release docs should be direct, operational, and low-drama
  - installed help should be concise and welcoming rather than internal-facing

## Accessibility Requirements

- All release instructions should be copyable as plain text
- Terminal workflows should remain keyboard-first
- Help output should stay readable in standard terminal environments

## Acceptance Scenarios

```gherkin
Scenario: Maintainer prepares a release using the documented flow
  Given the repository is on a release-ready main commit
  And the maintainer has the required publishing access
  When they follow the release checklist and run the release-prep commands
  Then the release path should stay aligned with the documented Mnemix release flow
  And the next publish step should be unambiguous

Scenario: New user installs the package and finds the right commands
  Given the package has been published to PyPI
  When a user runs pipx install mnemix-workflow
  Then the installed environment should expose mnemix-workflow, mxw, and mnx
  And the user should be able to discover the intended entrypoints from help output

Scenario: Release smoke test catches a broken packaged command
  Given the package artifacts have been built
  When the install smoke test runs in a clean environment
  Then it should fail if any expected command is missing or broken
  And the release should not be treated as ready
```

## References

- `mnemix/docs/release-checklist.md`
- `README.md`
- `docs/prd.md`
