---
status: completed
summary: Added basic Markdown-aware formatting to the mnx preview pane.
updated: 2026-03-29
prs:
- 9

---

# Patch: Formatted Markdown Preview In Mnx

## Summary

Improve the `mnx` preview pane so Markdown artifacts are rendered with basic
terminal formatting instead of being shown as raw plain text. The goal is a
more readable browse experience for workstreams and patches without turning the
TUI into a full rich-text renderer.

## Reason

The current preview pane displays file contents as an unstyled string, which
makes longer specs and UX docs harder to scan in the terminal. Headings, lists,
code fences, and emphasis all collapse into flat text. A lightweight Markdown
renderer would make `mnx` feel much more like a usable workflow cockpit without
changing the underlying repo-native file model.

## Scope

- Add basic Markdown-aware rendering for previewed artifacts in `mnx`
- Support readable formatting for headings, paragraphs, bullet/numbered lists,
  blockquotes, code fences, and simple emphasis where practical
- Keep plain-text fallback behavior for unsupported Markdown constructs
- Do not attempt full GitHub-perfect Markdown fidelity
- Do not add editing flows or a full document authoring mode in this patch

## Implementation Notes

- Parse Markdown preview content before rendering instead of passing raw strings
  directly to the preview `Paragraph`
- Map the most common Markdown structures into styled `ratatui` lines/spans
- Keep the renderer narrow and terminal-first so it does not balloon the TUI
  codebase
- Preserve scrolling and existing artifact navigation behavior
- Consider a small renderer module to keep Markdown-specific logic out of the
  main TUI rendering file

## Validation

- Preview a workstream `spec.md` and confirm headings and lists are easier to scan
- Preview a `ux.md` file with Gherkin/code blocks and confirm fenced sections
  remain readable
- Preview a patch file and confirm formatting still works for single-file
  artifacts
- Ensure unsupported constructs degrade gracefully instead of breaking the UI

## References

- `workflow/workstreams/005-interactive-tui-mode/spec.md`
- `src/tui/render.rs`
- `src/tui/data.rs`
