use std::rc::Rc;
use dioxus::prelude::*;
use crate::focus::FocusState;
use crate::select::context::{OptionState, RcPartialEqValue};

#[derive(Copy, Clone)]
pub(crate) struct ComboBoxContext {
    /// If the select is open
    pub open: Signal<bool>,

    /// Current value
    pub value: Memo<Option<RcPartialEqValue>>,

    /// Set the value callback
    pub set_value: Callback<Option<RcPartialEqValue>>,

    /// A list of options with their states
    pub options: Signal<Vec<OptionState>>,
    
    /// The ID of the list for ARIA attributes
    pub list_id: Signal<Option<String>>,

    /// The focus state for the select
    pub focus_state: FocusState,

    /// Whether the combobox is disabled
    pub disabled: ReadSignal<bool>,

    /// The placeholder text
    pub placeholder: ReadSignal<String>,

    /// The search input element
    pub search_input: Signal<Option<Rc<MountedData>>>,

    /// The value of the search input
    /// If this is None, the current value will be shown
    pub search_input_value: Signal<Option<String>>,

    /// The initial element to focus once the list is rendered
    pub initial_focus: Signal<Option<usize>>,
}

impl ComboBoxContext {
    /// Select the currently focused item
    pub fn select_current_item(&mut self) {
        // If the select is open, select the focused item
        if self.open.cloned() {
            if let Some(focused_index) = self.focus_state.current_focus() {
                let options = self.options.read();
                if let Some(option) = options.iter().find(|opt| opt.tab_index == focused_index) {
                    self.set_value.call(Some(option.value.clone()));
                    self.search_input_value.set(None);
                    self.focus_state.blur();
                    self.open.set(false);
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct ComboBoxTriggerContext {

}

#[derive(Clone, Copy)]
pub(super) struct ComboBoxListContext {
    /// Whether to render in the dom (or just run logic)
    pub render: ReadSignal<bool>,
}

/// Context for ComboBox option components to know if they're selected
#[derive(Clone, Copy)]
pub(super) struct ComboBoxOptionContext {
    /// Whether this option is currently selected
    pub selected: ReadSignal<bool>,
}

/// Context for select group components
#[derive(Clone, Copy)]
pub(super) struct ComboBoxGroupContext {
    /// ID of the element that labels this group
    pub labeled_by: Signal<Option<String>>,
}
