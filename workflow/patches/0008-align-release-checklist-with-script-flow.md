---
status: open
summary: Align the maintainer release checklist with the script-based prep and publish flow.
updated: 2026-03-30
---

# Patch: Align Release Checklist With Script Flow

## Summary

Update the maintainer runbook so the documented release flow matches the
repository's actual scripted release-prep and publish process.

## Reason

The release checklist still reads like manual version bumping is the default
release path, even though this repo now ships `scripts/release.sh` and
`scripts/publish-release.sh` as the intended workflow. That mismatch makes the
runbook confusing during real releases.

## Scope

- Make the script-based release-prep and publish flow the primary documented path
- Reframe manual version editing as an explicit fallback path
- Keep the checklist aligned with the current release scripts
- Do not redesign the release scripts themselves in this patch

## Implementation Notes

- Update the architecture, health checks, incidents, and deployment procedures
  so they describe `./scripts/release.sh X.Y.Z` and
  `./scripts/publish-release.sh X.Y.Z`
- Keep the runbook explicit about where `check-python-package.sh` and
  `check-linux-release-build.sh` fit into the scripted flow

## Validation

- Confirm the checklist now presents `./scripts/release.sh X.Y.Z` as the
  standard version-bump path
- Confirm the checklist points to `./scripts/publish-release.sh X.Y.Z` as the
  publish step after the prep PR merges
- Confirm manual version editing is described only as a fallback

## References

- `docs/release-checklist.md`
- `scripts/release.sh`
- `scripts/publish-release.sh`
