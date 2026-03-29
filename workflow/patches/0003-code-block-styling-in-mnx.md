---
status: completed
summary: Styled fenced Markdown code blocks as distinct terminal blocks in mnx.
updated: 2026-03-29
prs:
- 11

---

# Patch: Code Block Styling In Mnx

## Summary

Improve the `mnx` Markdown preview so fenced code blocks render as distinct
terminal code blocks instead of blending into normal paragraph text. The goal
is to keep code samples visually separate and easier to scan without turning
the TUI into a full rich-text editor.

## Reason

The current Markdown preview handles headings, lists, and inline emphasis, but
fenced code blocks still read like plain text. That makes examples, Gherkin
snippets, and command sequences harder to distinguish when reviewing specs and
patches inside the TUI.

## Scope

- Add clearer visual treatment for fenced code blocks in the `mnx` preview pane
- Preserve existing Markdown rendering for headings, lists, quotes, and inline code
- Keep the renderer lightweight and terminal-native
- Do not add syntax highlighting or a full Markdown layout engine
- Do not add editing support inside the TUI

## Implementation Notes

- Update the Markdown renderer to emit a visible block wrapper for fenced code sections
- Style code block lines differently from normal text so they read as grouped content
- Keep the preview scroll and artifact navigation behavior unchanged
- Extend renderer tests to cover the new code block presentation

## Validation

- Preview a `ux.md` or `spec.md` file with fenced code and confirm the block is visually distinct
- Verify code block contents remain readable in narrow terminal widths
- Run the Rust test suite and confirm Markdown renderer tests pass

## Example Blocks

```ts
export function createWorkstreamSlug(name: string): string {
  return name
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
}
```

```rust
pub fn next_status(current: &str) -> &'static str {
    match current {
        "proposed" => "open",
        "open" => "completed",
        _ => "proposed",
    }
}
```

```python
def summarize_patch(title: str, pr_number: int) -> str:
    return f"{title} shipped in PR #{pr_number}"
```

```json
{
  "status": "completed",
  "summary": "Styled fenced Markdown code blocks as distinct terminal blocks in mnx.",
  "updated": "2026-03-29",
  "prs": [11]
}
```

## References

- `workflow/patches/0001-formatted-markdown-preview-in-mnx.md`
- `src/tui/markdown.rs`
- `src/tui/render.rs`
