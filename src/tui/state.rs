use super::data::{ArtifactKind, TrackedItem};

pub(crate) const STATUSES: [&str; 3] = ["proposed", "open", "completed"];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FocusPane {
    Statuses,
    Items,
    Preview,
}

pub(crate) struct AppState {
    pub(crate) items: Vec<TrackedItem>,
    pub(crate) selected_status: usize,
    pub(crate) selected_item: usize,
    pub(crate) selected_artifact: usize,
    pub(crate) scroll: u16,
    pub(crate) focus: FocusPane,
    pub(crate) message: Option<String>,
}

impl AppState {
    pub(crate) fn new(items: Vec<TrackedItem>) -> Self {
        Self {
            items,
            selected_status: 1,
            selected_item: 0,
            selected_artifact: 0,
            scroll: 0,
            focus: FocusPane::Items,
            message: None,
        }
    }

    pub(crate) fn selected_status_value(&self) -> &'static str {
        STATUSES[self.selected_status]
    }

    pub(crate) fn count_for_status(&self, status: &str) -> usize {
        self.items
            .iter()
            .filter(|item| item.status == status)
            .count()
    }

    pub(crate) fn filtered_indices(&self) -> Vec<usize> {
        self.items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
                (item.status == self.selected_status_value()).then_some(index)
            })
            .collect()
    }

    pub(crate) fn current_item(&self) -> Option<&TrackedItem> {
        let filtered = self.filtered_indices();
        filtered
            .get(self.selected_item)
            .and_then(|index| self.items.get(*index))
    }

    pub(crate) fn current_artifacts(&self) -> Vec<ArtifactKind> {
        self.current_item()
            .map(TrackedItem::artifact_kinds)
            .unwrap_or_default()
    }

    pub(crate) fn current_artifact(&self) -> Option<ArtifactKind> {
        self.current_artifacts()
            .get(self.selected_artifact)
            .copied()
    }

    pub(crate) fn set_focus_next(&mut self) {
        self.focus = match self.focus {
            FocusPane::Statuses => FocusPane::Items,
            FocusPane::Items => FocusPane::Preview,
            FocusPane::Preview => FocusPane::Statuses,
        };
    }

    pub(crate) fn set_focus_previous(&mut self) {
        self.focus = match self.focus {
            FocusPane::Statuses => FocusPane::Preview,
            FocusPane::Items => FocusPane::Statuses,
            FocusPane::Preview => FocusPane::Items,
        };
    }

    pub(crate) fn next_status(&mut self) {
        self.selected_status = (self.selected_status + 1) % STATUSES.len();
        self.reset_item_selection();
    }

    pub(crate) fn previous_status(&mut self) {
        self.selected_status = (self.selected_status + STATUSES.len() - 1) % STATUSES.len();
        self.reset_item_selection();
    }

    pub(crate) fn next_item(&mut self) {
        let len = self.filtered_indices().len();
        if len == 0 {
            self.selected_item = 0;
            return;
        }
        self.selected_item = (self.selected_item + 1) % len;
        self.reset_preview_position();
    }

    pub(crate) fn previous_item(&mut self) {
        let len = self.filtered_indices().len();
        if len == 0 {
            self.selected_item = 0;
            return;
        }
        self.selected_item = (self.selected_item + len - 1) % len;
        self.reset_preview_position();
    }

    pub(crate) fn next_artifact(&mut self) {
        let len = self.current_artifacts().len();
        if len == 0 {
            self.selected_artifact = 0;
            return;
        }
        self.selected_artifact = (self.selected_artifact + 1) % len;
        self.scroll = 0;
    }

    pub(crate) fn previous_artifact(&mut self) {
        let len = self.current_artifacts().len();
        if len == 0 {
            self.selected_artifact = 0;
            return;
        }
        self.selected_artifact = (self.selected_artifact + len - 1) % len;
        self.scroll = 0;
    }

    pub(crate) fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }

    pub(crate) fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub(crate) fn refresh(&mut self, items: Vec<TrackedItem>) {
        let current_name = self.current_item().map(|item| item.name.clone());
        self.items = items;
        let filtered = self.filtered_indices();
        if filtered.is_empty() {
            self.selected_item = 0;
            self.selected_artifact = 0;
            self.scroll = 0;
            return;
        }

        if let Some(name) = current_name {
            if let Some(position) = filtered
                .iter()
                .position(|index| self.items.get(*index).is_some_and(|item| item.name == name))
            {
                self.selected_item = position;
            } else if self.selected_item >= filtered.len() {
                self.selected_item = filtered.len() - 1;
            }
        } else if self.selected_item >= filtered.len() {
            self.selected_item = filtered.len() - 1;
        }

        let artifact_len = self.current_artifacts().len();
        if artifact_len == 0 {
            self.selected_artifact = 0;
        } else if self.selected_artifact >= artifact_len {
            self.selected_artifact = 0;
        }
        self.scroll = 0;
    }

    pub(crate) fn set_message(&mut self, message: impl Into<String>) {
        self.message = Some(message.into());
    }

    fn reset_item_selection(&mut self) {
        self.selected_item = 0;
        self.selected_artifact = 0;
        self.scroll = 0;
    }

    fn reset_preview_position(&mut self) {
        self.selected_artifact = 0;
        self.scroll = 0;
    }
}

pub(crate) fn next_status_value(current: &str) -> &'static str {
    let index = STATUSES
        .iter()
        .position(|candidate| *candidate == current)
        .unwrap_or(0);
    STATUSES[(index + 1) % STATUSES.len()]
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::status::TrackedKind;

    use super::{AppState, FocusPane, next_status_value};
    use crate::tui::data::TrackedItem;

    fn item(name: &str, status: &str, kind: TrackedKind) -> TrackedItem {
        TrackedItem {
            kind,
            path: PathBuf::from(name),
            name: name.to_owned(),
            status: status.to_owned(),
            summary: String::new(),
            updated: "2026-03-28".to_owned(),
            prs: Vec::new(),
        }
    }

    #[test]
    fn filters_items_by_selected_status() {
        let state = AppState::new(vec![
            item("001-foo", "open", TrackedKind::Workstream),
            item("0001-bar", "completed", TrackedKind::Patch),
        ]);

        assert_eq!(state.filtered_indices(), vec![0]);
    }

    #[test]
    fn patch_has_single_artifact() {
        let state = AppState::new(vec![item("0001-bar", "open", TrackedKind::Patch)]);
        let artifacts = state.current_artifacts();
        assert_eq!(artifacts.len(), 1);
    }

    #[test]
    fn focus_cycles_through_panes() {
        let mut state = AppState::new(vec![item("001-foo", "open", TrackedKind::Workstream)]);
        assert_eq!(state.focus, FocusPane::Items);
        state.set_focus_next();
        assert_eq!(state.focus, FocusPane::Preview);
        state.set_focus_next();
        assert_eq!(state.focus, FocusPane::Statuses);
    }

    #[test]
    fn next_status_value_cycles() {
        assert_eq!(next_status_value("proposed"), "open");
        assert_eq!(next_status_value("open"), "completed");
        assert_eq!(next_status_value("completed"), "proposed");
    }
}
