use std::rc::Rc;
use dioxus::prelude::*;
use crate::combo_box::context::{ComboBoxContext, ComboBoxTriggerContext};
use crate::{use_id_or, use_unique_id};

/// The props for the [`ComboBoxTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct ComboBoxTriggerProps {
    /// Additional attributes for the trigger button
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the trigger
    pub children: Element,
}

#[component]
pub fn ComboBoxTrigger(props: ComboBoxTriggerProps) -> Element {
    let mut open = use_context::<ComboBoxContext>().open;
    let mut is_closing = use_signal(|| false);
    use_effect(move || {
        if is_closing() {
            open.set(false);
            is_closing.set(false);
        }
    });

    use_context_provider(|| ComboBoxTriggerContext {});

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

/// The props for the [`ComboBoxTriggerInput`] component
#[derive(Props, Clone, PartialEq)]
pub struct ComboBoxTriggerInputProps {
    /// Additional attributes for the trigger button
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn ComboBoxTriggerInput(props: ComboBoxTriggerInputProps) -> Element {
    let mut context = use_context::<ComboBoxContext>();
    let mut open = context.open;
    let mut search_input = context.search_input;

    // Memo is called by the Signal, which runs this code everytime the value changes.
    // This memo should update the shown value in the input field.
    let selected_text_value = use_memo(move || {
        let value = context.value.read();
        value.as_ref().and_then(|v| {
            context.options
                .read()
                .iter()
                .find(|opt| opt.value == *v)
                .map(|opt| opt.text_value.clone())
        })
    });

    let mut value = context.search_input_value;

    rsx! {
        input {
            r#type: "text",
            placeholder: "Search fruits",
            value: value(),
            oninput: move |event| {
                open.set(true);
                let val = event.value();
                if val.is_empty() {
                    context.set_value.call(None);
                }
                value.set(event.value());
            },
            onmounted: move |element| {
                search_input.set(Some(element.data()));
            },
            onkeydown: move |event| {
                let key = event.key();
                let code = event.code();

                match key {
                    Key::ArrowDown => {
                        if !open() {
                            open.set(true);
                            context.initial_focus.set((context.focus_state.item_count() > 0).then_some(0));
                        } else {
                            context.focus_state.focus_next();
                        }
                    }
                    Key::ArrowUp => {
                        if !open() {
                            open.set(true);
                            context.initial_focus.set(context.focus_state.item_count().checked_sub(1));
                        } else {
                            context.focus_state.focus_prev();
                        }
                    }
                    Key::Enter => {
                        context.select_current_item();
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    Key::Escape => {
                        if open() {
                            open.set(false);
                        } else {
                            value.set(selected_text_value().unwrap_or_default());
                        }
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    _ => {}
                }
            },
            onblur: move |_| {
                value.set(selected_text_value().unwrap_or_default());
                open.set(false);
                context.focus_state.blur();
            },
            ..props.attributes,
        }
    }
}

#[component]
pub fn ComboBoxTriggerIndicator() -> Element {
    let context = use_context::<ComboBoxContext>();
    let mut open = context.open;
    let search_input = context.search_input;

    rsx! {
        div {
            onmousedown: move |event| async move {
                // prevent blur on the input element
                if let Some(input) = search_input() {
                    input.set_focus(true).await.ok();
                }
                event.prevent_default();
                event.stop_propagation();
            },
            onclick: move |_| async move {
                open.toggle();
                if let Some(input) = search_input() {
                    input.set_focus(true).await.ok();
                }
            },
            style: "display: contents",
            svg {
                class: "combo-box-expand-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}
