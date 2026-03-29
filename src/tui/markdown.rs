use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Parser, Tag, TagEnd};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Clone, Debug)]
enum ListKind {
    Unordered,
    Ordered(u64),
}

pub(crate) fn render_markdown(content: &str, width: usize) -> Text<'static> {
    let parser = Parser::new(content);
    let mut renderer = MarkdownRenderer {
        code_block_width: width.max(8),
        ..Default::default()
    };
    renderer.render(parser);
    Text::from(renderer.finish())
}

#[derive(Default)]
struct MarkdownRenderer {
    lines: Vec<Line<'static>>,
    current: Vec<Span<'static>>,
    list_stack: Vec<ListKind>,
    blockquote_depth: usize,
    item_prefix: Option<String>,
    heading_level: Option<HeadingLevel>,
    code_block: bool,
    code_block_width: usize,
    code_block_language: Option<String>,
    strong_depth: usize,
    emphasis_depth: usize,
    strike_depth: usize,
}

impl MarkdownRenderer {
    fn render<'a>(&mut self, parser: Parser<'a>) {
        for event in parser {
            match event {
                Event::Start(tag) => self.start_tag(tag),
                Event::End(tag) => self.end_tag(tag),
                Event::Text(text) => self.push_text(&text),
                Event::Code(text) => self.push_inline_code(&text),
                Event::SoftBreak | Event::HardBreak => self.push_current_line(),
                Event::Rule => {
                    self.push_current_line();
                    self.lines.push(Line::from(vec![Span::styled(
                        "────────────────────────",
                        Style::default().fg(Color::DarkGray),
                    )]));
                }
                Event::Html(text) | Event::InlineHtml(text) => {
                    self.push_spans(vec![Span::styled(
                        text.to_string(),
                        Style::default().fg(Color::DarkGray),
                    )]);
                }
                _ => {}
            }
        }

        self.push_current_line();
    }

    fn start_tag(&mut self, tag: Tag<'_>) {
        match tag {
            Tag::Paragraph => {}
            Tag::Heading { level, .. } => {
                self.push_current_line();
                self.heading_level = Some(level);
            }
            Tag::BlockQuote(_) => {
                self.push_current_line();
                self.blockquote_depth += 1;
            }
            Tag::List(Some(start)) => self.list_stack.push(ListKind::Ordered(start)),
            Tag::List(None) => self.list_stack.push(ListKind::Unordered),
            Tag::Item => {
                self.push_current_line();
                self.item_prefix = Some(self.next_item_prefix());
            }
            Tag::CodeBlock(kind) => {
                self.push_current_line();
                self.code_block = true;
                self.code_block_language = match kind {
                    CodeBlockKind::Fenced(lang) if !lang.is_empty() => Some(lang.to_string()),
                    _ => None,
                };
                self.lines.push(code_block_header(
                    self.code_block_language.as_deref(),
                    self.code_block_width,
                ));
            }
            Tag::Strong => self.strong_depth += 1,
            Tag::Emphasis => self.emphasis_depth += 1,
            Tag::Strikethrough => self.strike_depth += 1,
            _ => {}
        }
    }

    fn end_tag(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => {
                self.push_current_line();
                if self.list_stack.is_empty() {
                    self.push_blank_line();
                }
            }
            TagEnd::Heading(_) => {
                self.push_current_line();
                self.push_blank_line();
                self.heading_level = None;
            }
            TagEnd::BlockQuote(_) => {
                self.push_current_line();
                self.push_blank_line();
                self.blockquote_depth = self.blockquote_depth.saturating_sub(1);
            }
            TagEnd::List(_) => {
                self.push_current_line();
                self.push_blank_line();
                self.list_stack.pop();
            }
            TagEnd::Item => self.push_current_line(),
            TagEnd::CodeBlock => {
                self.push_current_line();
                self.lines.push(code_block_footer(self.code_block_width));
                self.push_blank_line();
                self.code_block = false;
                self.code_block_language = None;
            }
            TagEnd::Strong => self.strong_depth = self.strong_depth.saturating_sub(1),
            TagEnd::Emphasis => self.emphasis_depth = self.emphasis_depth.saturating_sub(1),
            TagEnd::Strikethrough => self.strike_depth = self.strike_depth.saturating_sub(1),
            _ => {}
        }
    }

    fn push_text(&mut self, text: &str) {
        if self.code_block {
            for line in text.split_terminator('\n') {
                for wrapped in wrap_code_chunks(line, code_block_inner_width(self.code_block_width))
                {
                    self.lines
                        .push(code_block_line(&wrapped, self.code_block_width));
                }
            }
            return;
        }

        if text.is_empty() {
            return;
        }

        self.push_spans(vec![Span::styled(text.to_owned(), self.inline_style())]);
    }

    fn push_inline_code(&mut self, text: &str) {
        self.push_spans(vec![Span::styled(
            text.to_owned(),
            Style::default().fg(Color::Yellow).bg(Color::DarkGray),
        )]);
    }

    fn push_spans(&mut self, spans: Vec<Span<'static>>) {
        self.ensure_prefix();
        self.current.extend(spans);
    }

    fn ensure_prefix(&mut self) {
        if !self.current.is_empty() {
            return;
        }

        let mut prefix = String::new();
        if self.blockquote_depth > 0 {
            prefix.push_str(&"> ".repeat(self.blockquote_depth));
        }
        if let Some(item_prefix) = self.item_prefix.take() {
            prefix.push_str(&item_prefix);
        }

        if !prefix.is_empty() {
            self.current.push(Span::styled(
                prefix,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ));
        }
    }

    fn push_current_line(&mut self) {
        if self.current.is_empty() {
            return;
        }

        let line = if let Some(level) = self.heading_level {
            Line::from(self.current.drain(..).collect::<Vec<_>>()).style(heading_style(level))
        } else {
            Line::from(self.current.drain(..).collect::<Vec<_>>())
        };
        self.lines.push(line);
    }

    fn push_blank_line(&mut self) {
        if self
            .lines
            .last()
            .is_some_and(|line| line.spans.is_empty() || line.width() == 0)
        {
            return;
        }
        self.lines.push(Line::default());
    }

    fn next_item_prefix(&mut self) -> String {
        match self.list_stack.last_mut() {
            Some(ListKind::Unordered) => "• ".to_owned(),
            Some(ListKind::Ordered(next)) => {
                let prefix = format!("{next}. ");
                *next += 1;
                prefix
            }
            None => String::new(),
        }
    }

    fn inline_style(&self) -> Style {
        let mut style = Style::default();
        if self.strong_depth > 0 {
            style = style.add_modifier(Modifier::BOLD);
        }
        if self.emphasis_depth > 0 {
            style = style.add_modifier(Modifier::ITALIC);
        }
        if self.strike_depth > 0 {
            style = style.add_modifier(Modifier::CROSSED_OUT);
        }
        style
    }

    fn finish(mut self) -> Vec<Line<'static>> {
        while self.lines.last().is_some_and(|line| line.spans.is_empty()) {
            self.lines.pop();
        }
        if self.lines.is_empty() {
            self.lines.push(Line::default());
        }
        self.lines
    }
}

fn heading_style(level: HeadingLevel) -> Style {
    match level {
        HeadingLevel::H1 => Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        HeadingLevel::H2 => Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
        HeadingLevel::H3 => Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
        _ => Style::default().add_modifier(Modifier::BOLD),
    }
}

fn code_block_header(language: Option<&str>, width: usize) -> Line<'static> {
    let label = match language {
        Some(language) => format!("┌ code: {language} "),
        None => "┌ code ".to_owned(),
    };
    let line = fill_to_width(label, '─', width.saturating_sub(1));
    let content = format!("{line}┐");

    Line::from(vec![Span::styled(
        content,
        Style::default()
            .fg(Color::Yellow)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    )])
}

fn code_block_line(line: &str, width: usize) -> Line<'static> {
    let style = Style::default().fg(Color::Green).bg(Color::DarkGray);
    let content_width = code_block_inner_width(width);
    let padded = pad_to_width(line, content_width);
    let content = format!("│ {padded} │");
    Line::from(vec![Span::styled(content, style)])
}

fn code_block_footer(width: usize) -> Line<'static> {
    let content = format!("└{}┘", "─".repeat(width.saturating_sub(2)));
    Line::from(vec![Span::styled(
        content,
        Style::default()
            .fg(Color::Yellow)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    )])
}

fn code_block_inner_width(width: usize) -> usize {
    width.saturating_sub(4)
}

fn wrap_code_chunks(line: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 {
        return vec![String::new()];
    }

    if line.is_empty() {
        return vec![String::new()];
    }

    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut current_width = 0;

    for ch in line.chars() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(1).max(1);
        if current_width + ch_width > max_width && !current.is_empty() {
            chunks.push(current);
            current = String::new();
            current_width = 0;
        }
        current.push(ch);
        current_width += ch_width;
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}

fn fill_to_width(mut content: String, fill: char, target_width: usize) -> String {
    let current_width = UnicodeWidthStr::width(content.as_str());
    if current_width < target_width {
        content.push_str(&fill.to_string().repeat(target_width - current_width));
    }
    content
}

fn pad_to_width(content: &str, target_width: usize) -> String {
    let mut padded = content.to_owned();
    let current_width = UnicodeWidthStr::width(content);
    if current_width < target_width {
        padded.push_str(&" ".repeat(target_width - current_width));
    }
    padded
}

#[cfg(test)]
mod tests {
    use ratatui::style::{Color, Modifier};

    use super::render_markdown;

    #[test]
    fn renders_headings_and_lists() {
        let text = render_markdown("# Title\n\n- item one\n- item two", 40);
        assert_eq!(text.lines[0].spans[0].content.as_ref(), "Title");
        assert!(text.lines[0].style.add_modifier.contains(Modifier::BOLD));
        assert!(text.lines[2].to_string().contains("• item one"));
        assert!(text.lines[3].to_string().contains("• item two"));
    }

    #[test]
    fn renders_blockquotes_and_code_fences() {
        let text = render_markdown("> quoted\n\n```rs\nlet x = 1;\n```", 24);
        assert!(text.lines[0].to_string().contains("> quoted"));
        assert_eq!(text.lines[2].width(), 24);
        assert_eq!(text.lines[3].width(), 24);
        assert_eq!(text.lines[4].width(), 24);
        assert!(text.lines[2].to_string().starts_with("┌ code: rs "));
        assert!(text.lines[3].to_string().starts_with("│ let x = 1;"));
        assert!(text.lines[3].to_string().ends_with("│"));
        assert_eq!(text.lines[3].spans[0].style.bg, Some(Color::DarkGray));
        assert_eq!(text.lines[3].spans[0].style.fg, Some(Color::Green));
        assert!(text.lines[4].to_string().starts_with("└"));
    }

    #[test]
    fn renders_inline_code_and_emphasis() {
        let text = render_markdown("Use `mxw` and *readable* output.", 40);
        let line = &text.lines[0];
        assert!(line.to_string().contains("Use mxw and readable output."));
        assert_eq!(line.spans[1].style.fg, Some(Color::Yellow));
        assert!(line.spans[3].style.add_modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn wraps_long_code_lines_inside_the_block_width() {
        let text = render_markdown(
            "```ts\nconst reallyLongIdentifier = veryLongCallChain();\n```",
            24,
        );
        assert_eq!(text.lines[1].width(), 24);
        assert_eq!(text.lines[2].width(), 24);
        assert!(text.lines[1].to_string().starts_with("│ "));
        assert!(text.lines[2].to_string().starts_with("│ "));
    }
}
