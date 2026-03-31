---
status: completed
summary: Updated the release-prep script so release version bumps include Cargo.lock.
updated: 2026-03-31
prs:
- 16
github:
  issue:
    id: 4174790538
    number: 71
    url: https://github.com/micahcourey/mnemix-workflow/issues/71
  parent_issue: null
  sub_issues: {}

---

# Patch: Include Cargo Lock In Release Prep

## Summary

Update the release-prep script so version-bump PRs include `Cargo.lock`
alongside `Cargo.toml` and the Python package version file.

## Reason

The first release-prep flow bumped the package versions but left `Cargo.lock`
behind, which created an inconsistent `main` branch and required a follow-up PR
to correct the lockfile. The release script should stage the lockfile whenever
the workspace version changes.

## Scope

- Update `scripts/release.sh` to include `Cargo.lock` in the staged release-prep changes
- Adjust release-script messaging so it reflects the lockfile as part of the
  release-prep diff
- Verify the script remains syntactically valid
- Do not redesign the full release flow beyond this lockfile fix

## Implementation Notes

- Add `Cargo.lock` to the `git add` step in `scripts/release.sh`
- Update the script help text and dry-run messaging to mention the lockfile
- Keep the rest of the release-prep PR flow unchanged

## Validation

- `bash -n scripts/release.sh`
- Review the staged diff produced by the release-prep flow and confirm the
  lockfile is included when versions change

## References

- `scripts/release.sh`
- PR #14 release-prep follow-up where `Cargo.lock` was missed
