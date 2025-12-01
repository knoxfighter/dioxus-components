use std::any::{type_name, type_name_of_val};
use std::rc::Rc;
use dioxus::core::use_after_render;
use dioxus::dioxus_core::queue_effect;
use dioxus::prelude::*;
use crate::combo_box::context::ComboBoxContext;
use crate::focus::use_focus_provider;
use crate::select::context::{OptionState, RcPartialEqValue};
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

    /// Whether the combobox is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Name of the combobox for form submission
    #[props(default)]
    pub name: ReadSignal<String>,

    /// Optional placeholder text
    #[props(default = ReadSignal::new(Signal::new(String::from("Select an option"))))]
    pub placeholder: ReadSignal<String>,

    /// Whether an empty value is allowed.
    /// Make sure to set a default value, else the ComboBox still allows an empty value.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub allow_empty_value: ReadSignal<bool>,

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
        use_controlled(props.value, props.default_value.clone(), props.on_value_change);

    let open = use_signal(|| false);
    let list_id = use_signal(|| None);
    let focus_state = use_focus_provider(props.roving_loop);

    let search_input: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let value = use_memo(move || value().map(RcPartialEqValue::new));
    let set_value = use_callback(move |cursor_opt: Option<RcPartialEqValue>| {
        if let Some(value) = cursor_opt {
            // value
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
    let search_input_value = use_signal(|| None);
    let initial_focus = use_signal(|| None);

    use_context_provider(|| ComboBoxContext {
        open,
        value,
        set_value,
        options,
        list_id,
        focus_state,
        disabled: props.disabled,
        name: props.name,
        placeholder: props.placeholder,
        search_input,
        search_input_value,
        initial_focus,
        allow_empty_value: props.allow_empty_value,
    });

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },

            // Pass through other attributes
            ..props.attributes,

            // Render children (options)
            {props.children}
        }
    }
}
