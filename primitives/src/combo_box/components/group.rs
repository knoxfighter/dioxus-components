use dioxus::prelude::*;
use crate::combo_box::context::{ComboBoxContext, ComboBoxGroupContext, ComboBoxListContext};
use crate::{use_id_or, use_unique_id};

/// The props for the [`ComboBoxGroup`] component
#[derive(Props, Clone, PartialEq)]
pub struct ComboBoxGroupProps {
    /// Optional ID for the group
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the group
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the group
    pub children: Element,
}

#[component]
pub fn ComboBoxGroup(props: ComboBoxGroupProps) -> Element {
    let context = use_context::<ComboBoxContext>();

    let labeled_by = use_signal(|| None);

    use_context_provider(|| ComboBoxGroupContext { labeled_by });
    let render = use_context::<ComboBoxListContext>().render;

    rsx! {
        if render() {
            div {
                role: "group",
                ..props.attributes,
                {props.children}
            }
        } else {
            // If we are not rendering, still render the children components
            {props.children}
        }
    }
}

/// The props for the [`ComboBoxGroupLabel`] component
#[derive(Props, Clone, PartialEq)]
pub struct ComboBoxGroupLabelProps {
    /// Optional ID for the label
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the label
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the label
    pub children: Element,
}

#[component]
pub fn ComboBoxGroupLabel(props: ComboBoxGroupLabelProps) -> Element {
    let mut context: ComboBoxGroupContext = use_context();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    use_effect(move || {
        context.labeled_by.set(Some(id()));
    });

    let render = use_context::<ComboBoxListContext>().render;

    rsx! {
        if render() {
            div {
                role: "group",
                id,
                tabindex: "-1",

                ..props.attributes,
                {props.children}
            }
        }
    }
}
