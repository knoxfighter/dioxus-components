use crate::combobox::context::{ComboboxContext, ComboboxGroupContext};
use crate::listbox::ListboxContext;
use crate::selectable::SelectableGroupContext;
use crate::{use_id_or, use_unique_id};
use dioxus::prelude::*;

/// The props for the [`ComboboxGroup`] component
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxGroupProps {
    /// Whether the group is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Optional ID for the group
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the group
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the group
    pub children: Element,
}

/// # ComboboxGroup
///
/// The `ComboboxGroup` component is used to group related options within a [`ComboboxList`](super::list::ComboboxList).
/// It provides a way to organize options into logical sections.
/// The group is also hidden if all child options are disabled.
///
/// This must be used inside a [`ComboboxList`](super::list::ComboboxList) component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::combobox::{
///     Combobox, ComboboxGroup, ComboboxGroupLabel, ComboboxItemIndicator, ComboboxList, ComboboxOption,
///     ComboboxEmpty,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Combobox::<String> {
///             width: "12rem",
///             aria_label: "Select a fruit",
///             placeholder: "Select a fruit...",
///             list_aria_label: "Food",
///             ComboboxEmpty { "No Food found." }
///
///             ComboboxGroup {
///                 ComboboxGroupLabel { "Fruits" }
///                 ComboboxOption::<String> {
///                     index: 0usize,
///                     value: "apple",
///                     text_value: "Apple",
///                     "Apple"
///                 }
///                 ComboboxOption::<String> {
///                     index: 1usize,
///                     value: "banana",
///                     text_value: "Banana",
///                     "Banana"
///                 }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn ComboboxGroup(props: ComboboxGroupProps) -> Element {
    let render = use_context::<ListboxContext>().render;
    let ctx = use_context::<ComboboxContext>();
    let disabled = use_memo(move || ctx.selectable.disabled.cloned() || props.disabled.cloned());

    let labeled_by = use_signal(|| None);
    let options = use_signal(Vec::new);

    use_context_provider(|| ComboboxGroupContext {
        labeled_by,
        options,
        selectable: SelectableGroupContext {
            disabled: disabled.into(),
        },
    });

    let visible = use_memo(move || {
        options
            .read()
            .iter()
            .any(|(_, index)| ctx.is_visible(*index))
    });

    rsx! {
        if render() {
            div {
                role: "group",

                display: if visible() { "" } else { "none" },

                // ARIA attributes
                aria_disabled: disabled,
                aria_labelledby: labeled_by,

                "data-disabled": disabled,

                ..props.attributes,
                {props.children}
            }
        } else {
            {props.children}
        }
    }
}

/// The props for the [`ComboboxGroupLabel`] component
#[derive(Props, Clone, PartialEq)]
pub struct ComboboxGroupLabelProps {
    /// Optional ID for the label
    pub id: ReadSignal<Option<String>>,

    /// Additional attributes for the label
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the label
    pub children: Element,
}

/// # ComboboxGroupLabel
///
/// The `ComboboxGroupLabel` component is used to render a label for a group of options within a [`ComboboxList`](super::list::ComboboxList).
///
/// This must be used inside a [`ComboboxGroup`](ComboboxGroup) component.
#[component]
pub fn ComboboxGroupLabel(props: ComboboxGroupLabelProps) -> Element {
    let render = use_context::<ListboxContext>().render;
    let mut ctx: ComboboxGroupContext = use_context();

    let id = use_unique_id();
    let id = use_id_or(id, props.id);

    use_effect(move || {
        ctx.labeled_by.set(Some(id()));
    });

    rsx! {
        if render () {
            div {
                // Set the ID for the label
                id,
                ..props.attributes,
                {props.children}
            }
        }
    }
}
