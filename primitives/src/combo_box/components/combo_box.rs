use std::any::{type_name, type_name_of_val};
use std::rc::Rc;
use dioxus::prelude::*;
use crate::combo_box::context::ComboBoxContext;
use crate::focus::use_focus_provider;
use crate::select::context::RcPartialEqValue;
use crate::use_controlled;

/// Props for the main ComboBox component
#[derive(Props, Clone, PartialEq)]
pub struct ComboBoxProps<T: Clone + PartialEq + 'static = String> {
    /// The controlled value of the ComboBox
    #[props(default)]
    pub value: ReadSignal<Option<Option<T>>>,

    /// The default value of the ComboBox
    #[props(default)]
    pub default_value: Option<T>,

    /// Callback when the value changes
    #[props(default)]
    pub on_value_change: Callback<Option<T>>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes for the ComboBox element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the ComboBox component
    pub children: Element,
}

#[component]
pub fn ComboBox<T: Clone + PartialEq + 'static>(props: ComboBoxProps<T>) -> Element {
    let (value, set_value_internal) =
        use_controlled(props.value, props.default_value, props.on_value_change);

    let open = use_signal(|| false);
    let list_id = use_signal(|| None);
    let focus_state = use_focus_provider(props.roving_loop);

    let mut search_input: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let value = use_memo(move || value().map(RcPartialEqValue::new));
    let set_value = use_callback(move |cursor_opt: Option<RcPartialEqValue>| {
        if let Some(value) = cursor_opt {
            // value
            if let Some(search_input) = search_input() {

            }
            set_value_internal.call(Some(
                value
                    .as_ref::<T>()
                    .unwrap_or_else(|| {
                        core::panic!("The values of combobox and all options must match types")
                    })
                    .clone(),
            ));
        } else {
            set_value_internal.call(None);
        }
    });
    let options = use_signal(Vec::default);
    let search_input_value = use_signal(String::new);

    use_context_provider(|| ComboBoxContext {open, list_id, focus_state, value, set_value, search_input, options, search_input_value });

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },

            onmounted: move |e| async move {
                tracing::info!("ComboBox mounted");
            },

            // Pass through other attributes
            ..props.attributes,

            // Render children (options)
            {props.children}
        }
    }
}
