---
status: completed
summary: Added a root release-notes file for the current mnemix-workflow release flow.
updated: 2026-03-31
prs:
- 16
github:
  issue:
    id: 4174790600
    number: 72
    url: https://github.com/micahcourey/mnemix-workflow/issues/72
  parent_issue: null
  sub_issues: {}

---

# Patch: Add Release Notes Template

## Summary

Add a root `RELEASE_NOTES.md` file modeled on the `mnemix` release-notes
shape so the repo has a ready-to-edit release body for GitHub releases.

## Reason

The release flow now has a checklist, publish script, and PyPI workflow, but it
does not yet have a repository-level release notes document like `mnemix`.
Adding one gives the project a clear place to draft and refine the release body
before tagging and publishing.

## Scope

- Add a root `RELEASE_NOTES.md`
- Model the structure on `mnemix/RELEASE_NOTES.md`
- Tailor the content to `mnemix-workflow` and the current `v0.1.1` release path
- Reference the release-notes file from the release checklist
- Do not redesign the entire release process beyond adding this release-notes artifact

## Implementation Notes

- Keep the document concise and release-oriented
- Use the current shipped scope for `mnemix-workflow` rather than copying
  `mnemix` wording verbatim
- Make the file usable as a GitHub release notes source with
  `gh release edit <tag> --notes-file RELEASE_NOTES.md`

## Validation

- Confirm the file exists at the repo root
- Confirm the content is specific to `mnemix-workflow`
- Confirm the release checklist points maintainers at the file

## References

- `../mnemix/RELEASE_NOTES.md`
- `docs/release-checklist.md`
- `scripts/publish-release.sh`
