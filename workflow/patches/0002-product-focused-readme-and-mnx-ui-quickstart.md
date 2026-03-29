---
status: completed
summary: Reworked the README into a more product-focused entrypoint with clearer mnx and quickstart guidance.
updated: 2026-03-29
prs:
- 10

---

# Patch: Product Focused Readme And Mnx Ui Quickstart

## Summary

Update the root README so it reads more like a product entrypoint than an
internal methodology note, with clearer positioning for `mnemix-workflow`, a
dedicated section for `mnx` as the interactive UI, and a more direct quickstart
for install and day-one usage.

## Reason

The current README explains the methodology well, but it undersells the product
experience and makes the interactive CLI harder to discover. Now that `mnx`
exists as the shortcut into the TUI, the docs should present it as a first-class
way to use the tool instead of burying it among lower-level commands.

## Scope

- Rewrite the top of the README to be more product-focused
- Add a dedicated `mnx` / interactive UI section
- Make install guidance and quickstart usage clearer and more copyable
- Clarify the relationship between `mnx` and `mxw`
- Keep the underlying methodology and command set intact
- Do not redesign the whole documentation set beyond the root README

## Implementation Notes

- Tighten the opening product description and value proposition
- Restructure quickstart so install, `mnx`, `mxw`, and common flows are easy to scan
- Add examples for launching the TUI, initializing a repo, creating workstreams,
  creating patches, and checking status
- Preserve links to the PRD and methodology docs for deeper reading

## Validation

- A new user can tell what `mnemix-workflow` is within the first few paragraphs
- A new user can quickly find how to launch `mnx`
- A new user can quickly find install guidance and the most common `mxw` commands
- The README still accurately reflects the current shipped feature set

## References

- `README.md`
- `workflow/workstreams/005-interactive-tui-mode/spec.md`
