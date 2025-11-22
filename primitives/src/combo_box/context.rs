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

    /// The search input element
    pub search_input: Signal<Option<Rc<MountedData>>>,

    /// The value of the search input
    pub search_input_value: Signal<String>,
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
