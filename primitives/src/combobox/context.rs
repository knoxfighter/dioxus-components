//! Shared state for the combobox component.

use crate::selectable::{OptionState, SelectableContext, SelectableGroupContext};
use dioxus::prelude::*;

/// The default case-insensitive substring filter.
pub fn default_combobox_filter(query: &str, text: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty() || text.to_lowercase().contains(&query)
}

#[derive(Clone, Copy)]
pub(super) struct ComboboxContext {
    pub selectable: SelectableContext,
    pub query: Memo<String>,
    pub set_query: Callback<String>,
    pub filter: Callback<(String, String), bool>,
}

impl ComboboxContext {
    pub fn set_open(&mut self, open: bool) {
        if open {
            self.selectable.collection.clear_focus();
        }
        self.selectable.set_open(open);
    }

    fn predicate_for(&self, query: String) -> impl Fn(&OptionState) -> bool {
        let filter = self.filter;
        move |option| filter.call((query.clone(), option.text_value.clone()))
    }

    fn predicate(&self) -> impl Fn(&OptionState) -> bool {
        self.predicate_for(self.query.cloned())
    }

    pub fn is_visible(&self, tab_index: usize) -> bool {
        let predicate = self.predicate();
        self.selectable
            .options
            .read()
            .iter()
            .find(|option| option.index == tab_index)
            .is_some_and(predicate)
    }

    pub fn has_visible_options(&self) -> bool {
        self.selectable.options.read().iter().any(self.predicate())
    }

    pub fn open_with_empty_query_and_focus_first(&mut self) {
        let query = String::new();
        self.set_query.call(query.clone());
        let initial_focus = self
            .selectable
            .first_matching_enabled_index(self.predicate_for(query));
        self.selectable.initial_focus.set(initial_focus);
        self.set_open(true);
    }

    pub fn open_with_empty_query_and_focus_last(&mut self) {
        let query = String::new();
        self.set_query.call(query.clone());
        let initial_focus = self
            .selectable
            .last_matching_enabled_index(self.predicate_for(query));
        self.selectable.initial_focus.set(initial_focus);
        self.set_open(true);
    }

    pub fn focused_option_id(&self) -> Option<String> {
        self.selectable.focused_option_id()
    }

    pub fn focus_next_visible(&mut self) {
        self.selectable.focus_next_where(self.predicate());
    }

    pub fn focus_prev_visible(&mut self) {
        self.selectable.focus_prev_where(self.predicate());
    }

    pub fn focus_first_visible(&mut self) {
        self.selectable.focus_first_where(self.predicate());
    }

    pub fn focus_last_visible(&mut self) {
        self.selectable.focus_last_where(self.predicate());
    }

    pub fn select_focused(&mut self) {
        self.selectable.select_focused();
    }
}

/// Context for combobox group components
#[derive(Clone, Copy)]
pub(super) struct ComboboxGroupContext {
    /// the selectable context
    pub selectable: SelectableGroupContext,

    /// ID of the element that labels this group
    pub labeled_by: Signal<Option<String>>,

    /// List of IDs and tab_indices of the options that are children of this group.
    pub options: Signal<Vec<(String, usize)>>,
}

impl ComboboxGroupContext {
    pub(crate) fn update_option(&mut self, id: &String, index: usize) {
        let existing = self
            .options
            .peek()
            .iter()
            .find(|(option_id, _)| option_id == id)
            .map(|(_, option_index)| *option_index);

        // let t = self.options.peek().iter().find(|(option_id, _)| option_id == id);
        // let t = self.options.peek().iter().position(|(option_id, _)| option_id == id);

        if existing == Some(index) {
            return;
        }

        let mut options = self.options.write();

        if let Some(entry) = options.iter_mut().find(|(option_id, _)| option_id == id) {
            entry.1 = index;
        } else {
            options.push((id.clone(), index));
        }
    }

    pub(crate) fn remove_option(&mut self, id: &str) {
        if !self
            .options
            .peek()
            .iter()
            .any(|(option_id, _)| option_id == id)
        {
            return;
        }

        self.options
            .write()
            .retain(|(option_id, _)| option_id != id);
        self.options
            .write()
            .retain(|(option_id, _)| option_id != id);
    }
}
