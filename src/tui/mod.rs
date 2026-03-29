mod data;
mod markdown;
mod render;
mod state;

use std::{
    io::{self, Stdout},
    path::Path,
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::status::{StatusFile, TrackedKind, resolve_tracked_path};

use self::{
    data::load_items,
    render::render,
    state::{AppState, FocusPane, next_status_value},
};

pub(crate) fn run(repo_root: &Path) -> Result<()> {
    let mut terminal = init_terminal()?;
    let result = run_app(&mut terminal, repo_root);
    restore_terminal(&mut terminal)?;
    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, repo_root: &Path) -> Result<()> {
    let mut state = AppState::new(load_items(repo_root)?);

    loop {
        terminal.draw(|frame| render(frame, &state))?;

        if !event::poll(Duration::from_millis(200))? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        if should_quit(key) {
            break;
        }

        if handle_global_key(&mut state, key) {
            continue;
        }

        match state.focus {
            FocusPane::Statuses => handle_status_pane_key(&mut state, key),
            FocusPane::Items => {
                if handle_item_pane_key(&mut state, repo_root, key)? {
                    state.refresh(load_items(repo_root)?);
                }
            }
            FocusPane::Preview => handle_preview_key(&mut state, key),
        }
    }

    Ok(())
}

fn handle_global_key(state: &mut AppState, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Tab => {
            state.set_focus_next();
            true
        }
        KeyCode::BackTab => {
            state.set_focus_previous();
            true
        }
        _ => false,
    }
}

fn handle_status_pane_key(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => state.next_status(),
        KeyCode::Up | KeyCode::Char('k') => state.previous_status(),
        KeyCode::Right | KeyCode::Enter => state.set_focus_next(),
        _ => {}
    }
}

fn handle_item_pane_key(state: &mut AppState, repo_root: &Path, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => state.next_item(),
        KeyCode::Up | KeyCode::Char('k') => state.previous_item(),
        KeyCode::Left | KeyCode::Char('h') => state.set_focus_previous(),
        KeyCode::Right | KeyCode::Enter | KeyCode::Char('l') => state.set_focus_next(),
        KeyCode::Char('s') => {
            cycle_selected_status(state, repo_root)?;
            return Ok(true);
        }
        _ => {}
    }
    Ok(false)
}

fn handle_preview_key(state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Down | KeyCode::Char('j') => state.scroll_down(),
        KeyCode::Up | KeyCode::Char('k') => state.scroll_up(),
        KeyCode::Left | KeyCode::Char('h') => state.previous_artifact(),
        KeyCode::Right | KeyCode::Char('l') => state.next_artifact(),
        KeyCode::Esc => state.set_focus_previous(),
        _ => {}
    }
}

fn cycle_selected_status(state: &mut AppState, repo_root: &Path) -> Result<()> {
    let Some(item) = state.current_item() else {
        return Ok(());
    };
    let next_status = next_status_value(&item.status);
    let item_path = resolve_tracked_path(repo_root, &item.name, item.kind)?;
    let status_path = item.kind.status_path(&item_path);
    let mut status = StatusFile::read(&status_path)?;
    status.set_status(next_status)?;
    status.touch_updated();
    status.write(&status_path)?;
    state.set_message(format!(
        "Updated {} {} to {}",
        item_type_label(item.kind),
        item.name,
        next_status
    ));
    Ok(())
}

fn item_type_label(kind: TrackedKind) -> &'static str {
    match kind {
        TrackedKind::Workstream => "workstream",
        TrackedKind::Patch => "patch",
    }
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode().context("Failed to enable raw terminal mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("Failed to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("Failed to initialize terminal")
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("Failed to disable raw terminal mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to restore cursor")?;
    Ok(())
}

fn should_quit(key: KeyEvent) -> bool {
    matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
        && !key.modifiers.contains(KeyModifiers::CONTROL)
}
