use std::any::type_name;
use dioxus::core::{use_drop, AttributeValue};
use dioxus::html::input_data::MouseButton;
use dioxus::prelude::*;
pub(crate) use crate::combo_box::context::ComboBoxContext;
use crate::combo_box::context::{ComboBoxListContext, ComboBoxOptionContext};
use crate::select::context::{OptionState, RcPartialEqValue};
use crate::{use_effect_cleanup, use_id_or, use_unique_id};
use crate::focus::use_focus_controlled_item;

/// The props for the [`SelectOption`] component
#[derive(Props, Clone, PartialEq)]
pub struct ComboBoxOptionProps<T: Clone + PartialEq + 'static> {
    /// The value of the option
    pub value: ReadSignal<T>,

    /// The text value of the option used for typeahead search
    #[props(default)]
    pub text_value: ReadSignal<Option<String>>,

    /// Optional ID for the option
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// The index of the option in the list. This is used to define the focus order for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Optional label for the option (for accessibility)
    #[props(default)]
    pub aria_label: Option<String>,

    /// Optional description role for the option (for accessibility)
    #[props(default)]
    pub aria_roledescription: Option<String>,

    /// Additional attributes for the option element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the option
    pub children: Element,
}

#[component]
pub fn ComboBoxOption<T: Clone + PartialEq + 'static>(props: ComboBoxOptionProps<T>) -> Element {
    // Generate a unique ID for this option for accessibility
    let option_id = use_unique_id();

    // Use use_id_or to handle the ID
    let id = use_id_or(option_id, props.id);

    let index = props.index;
    let value = props.value;

    let render = use_context::<ComboBoxListContext>().render;
    let mut context = use_context::<ComboBoxContext>();

    let text_value = use_memo(move || match (props.text_value)() {
        Some(text) => text,
        None => {
            let value = value.read();
            let as_any: &dyn std::any::Any = &*value;
            as_any
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| as_any.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| {
                    tracing::warn!("ComboBoxOption with non-string types requires text_value to be set");
                    String::new()
                })
        }
    });

    use_effect(move || {
        let option_state = OptionState {
            tab_index: index(),
            value: RcPartialEqValue::new(value.cloned()),
            text_value: text_value.cloned(),
            id: id(),
        };
        context.options.write().push(option_state);
    });
    use_effect_cleanup(move || {
        context.options.write().retain(|opt| opt.id != *id.read());
    });

    let onmounted = use_focus_controlled_item(props.index);
    let focused = move || context.focus_state.is_focused(index());
    let selected = use_memo(move || {
        context.value.read().as_ref().and_then(|v| v.as_ref::<T>()) == Some(&props.value.read())
    });

    use_context_provider(|| ComboBoxOptionContext {selected: selected.into()});

    let mut attributes = props.attributes.clone();
    let mut classes = (&mut attributes)
        .iter()
        .find(|attr| attr.name == "class")
        .map(|attr| &attr.value)
        .map(|value| {
            if let AttributeValue::Text(class) = value {
                return Some(class.clone());
            }
            None
        })
        .flatten()
        .unwrap_or_default();

    if focused() {
        classes.push_str(" focus");
    }

    rsx! {
        if render() {
            div {
                role: "option",
                id,
                class: classes,

                // ARIA attributes
                aria_selected: selected(),
                aria_label: props.aria_label.clone(),
                aria_roledescription: props.aria_roledescription.clone(),
                onmounted,

                onmousedown: move |event| {
                    // prevent mousedownevents so that blur events are not triggered
                    event.prevent_default();
                    event.stop_propagation();
                },
                onclick: move |event| {
                    if event.trigger_button() == Some(MouseButton::Primary) {
                        context.set_value.call(Some(RcPartialEqValue::new(props.value.cloned())));
                        context.search_input_value.set(text_value.read().clone());
                        context.focus_state.blur();
                        context.open.set(false);
                    }
                },

                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`ComboBoxOptionIndicator`] component
#[derive(Props, Clone, PartialEq)]
pub struct ComboBoxOptionIndicatorProps {
    /// The children to render inside the indicator
    pub children: Element,
}

#[component]
pub fn ComboBoxOptionIndicator(props: ComboBoxOptionIndicatorProps) -> Element {
    let context: ComboBoxOptionContext = use_context();
    if !(context.selected)() {
        return rsx! {};
    }
    rsx! {
        {props.children}
    }
}
