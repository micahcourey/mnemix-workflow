use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use super::{
    data::{preview_title, read_artifact},
    markdown::render_markdown,
    state::{AppState, FocusPane, STATUSES},
};

pub(crate) fn render(frame: &mut Frame, state: &AppState) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(2)])
        .split(frame.area());

    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(18),
            Constraint::Length(40),
            Constraint::Min(40),
        ])
        .split(root[0]);

    render_statuses(frame, state, panes[0]);
    render_items(frame, state, panes[1]);
    render_preview(frame, state, panes[2]);
    render_footer(frame, state, root[1]);
}

fn render_statuses(frame: &mut Frame, state: &AppState, area: ratatui::layout::Rect) {
    let items = STATUSES
        .iter()
        .map(|status| ListItem::new(format!("{status} ({})", state.count_for_status(status))))
        .collect::<Vec<_>>();
    let mut list_state = ListState::default().with_selected(Some(state.selected_status));
    let block = Block::default()
        .title("Statuses")
        .borders(Borders::ALL)
        .border_style(border_style(state.focus == FocusPane::Statuses));
    let list = List::new(items)
        .block(block)
        .highlight_style(selected_style())
        .highlight_symbol("> ");
    frame.render_stateful_widget(list, area, &mut list_state);
}

fn render_items(frame: &mut Frame, state: &AppState, area: ratatui::layout::Rect) {
    let filtered = state.filtered_indices();
    let items = filtered
        .iter()
        .filter_map(|index| state.items.get(*index))
        .map(|item| {
            let prs = if item.prs.is_empty() {
                String::new()
            } else {
                format!(
                    " | PRs {}",
                    item.prs
                        .iter()
                        .map(u64::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            ListItem::new(vec![
                Line::from(format!("[{}] {}", item.kind_label(), item.name)),
                Line::from(format!("{} | {}{}", item.updated, item.summary, prs)).dim(),
            ])
        })
        .collect::<Vec<_>>();

    let block = Block::default()
        .title("Tracked Items")
        .borders(Borders::ALL)
        .border_style(border_style(state.focus == FocusPane::Items));

    if items.is_empty() {
        frame.render_widget(
            Paragraph::new(format!(
                "No items with status `{}`.\n\nTry a different status or create a new workstream or patch.",
                state.selected_status_value()
            ))
            .block(block)
            .wrap(Wrap { trim: false }),
            area,
        );
        return;
    }

    let mut list_state = ListState::default().with_selected(Some(state.selected_item));
    let list = List::new(items)
        .block(block)
        .highlight_style(selected_style())
        .highlight_symbol("> ");
    frame.render_stateful_widget(list, area, &mut list_state);
}

fn render_preview(frame: &mut Frame, state: &AppState, area: ratatui::layout::Rect) {
    let block = Block::default()
        .title("Preview")
        .borders(Borders::ALL)
        .border_style(border_style(state.focus == FocusPane::Preview));

    let Some(item) = state.current_item() else {
        frame.render_widget(
            Paragraph::new("No tracked item selected.")
                .block(block)
                .wrap(Wrap { trim: false }),
            area,
        );
        return;
    };

    let Some(artifact) = state.current_artifact() else {
        frame.render_widget(
            Paragraph::new("No artifact available for the selected item.")
                .block(block)
                .wrap(Wrap { trim: false }),
            area,
        );
        return;
    };

    let title = preview_title(item, artifact);
    let preview_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style(state.focus == FocusPane::Preview));
    let content = read_artifact(item, artifact);

    frame.render_widget(
        Paragraph::new(render_markdown(&content))
            .block(preview_block)
            .wrap(Wrap { trim: false })
            .scroll((state.scroll, 0)),
        area,
    );
}

fn render_footer(frame: &mut Frame, state: &AppState, area: ratatui::layout::Rect) {
    let message = state.message.as_deref().unwrap_or(
        "Tab focus | j/k move | h/l switch status or artifact | s cycle status | q quit",
    );
    frame.render_widget(
        Paragraph::new(message).block(Block::default().borders(Borders::TOP).title("Help")),
        area,
    );
}

fn border_style(active: bool) -> Style {
    if active {
        Style::default().fg(ratatui::style::Color::Cyan)
    } else {
        Style::default()
    }
}

fn selected_style() -> Style {
    Style::default()
        .fg(ratatui::style::Color::Black)
        .bg(ratatui::style::Color::Cyan)
        .add_modifier(Modifier::BOLD)
}
