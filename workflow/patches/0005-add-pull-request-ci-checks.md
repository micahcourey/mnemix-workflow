---
status: completed
summary: Added a dedicated PR CI workflow for Rust, shell, and Python package checks.
updated: 2026-03-31
prs:
- 17
github:
  issue:
    id: 4174790479
    number: 70
    url: https://github.com/micahcourey/mnemix-workflow/issues/70
  parent_issue: null
  sub_issues: {}

---

# Patch: Add Pull Request Ci Checks

## Summary

Add a normal GitHub Actions CI workflow for pull requests so release-prep and
feature branches show automated checks before merge.

## Reason

The repo now has release publishing automation, but ordinary pull requests do
not run any GitHub checks. That makes release-prep PRs and regular feature PRs
feel unfinished and removes an important feedback loop that should exist before
merge.

## Scope

- Add a CI workflow that runs on `pull_request`
- Include the most useful baseline checks such as `cargo test`
- Decide whether the Python package preflight should run in full or in a
  lighter PR-safe subset
- Make the workflow visible as normal PR checks in GitHub
- Do not change the release-publish workflow itself
- Do not redesign the broader release process beyond adding pre-merge checks

## Implementation Notes

- Add a dedicated workflow file, likely `ci.yml`, triggered on pull requests
- Reuse the existing local verification commands where practical so CI matches
  maintainer expectations
- Keep the first version of CI focused on fast, high-signal checks rather than
  duplicating the full release publish matrix

## Validation

- Open a PR and confirm GitHub shows running checks
- Confirm failures surface directly in the PR UI
- Confirm the workflow stays separate from the release-triggered PyPI publish
  workflow

## References

- `workflow/workstreams/008-release-polish-and-pypi-publish-prep/spec.md`
- PR #14 release-prep discussion about missing PR checks
