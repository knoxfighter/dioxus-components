use dioxus::prelude::*;
use crate::combo_box::context::{ComboBoxContext, ComboBoxListContext};
use crate::{use_animated_open, use_id_or, use_unique_id};

#[derive(Props, Clone, PartialEq)]
pub struct ComboBoxListProps {
    /// The ID of the list for ARIA attributes
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the list
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the list
    pub children: Element,
}

#[component]
pub fn ComboBoxList(props: ComboBoxListProps) -> Element {
    let mut context = use_context::<ComboBoxContext>();

    let open = context.open;

    let id = use_unique_id();
    let id = use_id_or(id, props.id);
    use_effect(move || {
        context.list_id.set(Some(id()));
    });

    let render = use_animated_open(id, open);
    let render = use_memo(render);

    use_context_provider(|| ComboBoxListContext {
        render: render.into(),
    });

    rsx! {
        if render() {
            div {
                id,
                role: "listbox",
                tabindex: "-1",
                "data-state": if open() { "open" } else { "closed" },

                ..props.attributes,

                {props.children}
            }
        } else {
            {props.children}
        }
    }
}
