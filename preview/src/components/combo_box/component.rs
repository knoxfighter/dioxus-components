use dioxus::prelude::*;
use dioxus_primitives::combo_box;
use dioxus_primitives::combo_box::{ComboBoxGroupLabelProps, ComboBoxGroupProps, ComboBoxListProps, ComboBoxOptionIndicatorProps, ComboBoxOptionProps, ComboBoxProps, ComboBoxTriggerInputProps, ComboBoxTriggerProps};

#[component]
pub fn ComboBox<T: Clone + PartialEq + 'static>(props: ComboBoxProps<T>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        combo_box::ComboBox {
            class: "combo-box",
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            name: props.name,
            placeholder: props.placeholder,
            allow_empty_value: props.allow_empty_value,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ComboBoxTrigger(props: ComboBoxTriggerProps) -> Element {
    rsx! {
        combo_box::ComboBoxTrigger { class: "combo-box-trigger", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn ComboBoxTriggerInput(props: ComboBoxTriggerInputProps) -> Element {
    rsx! {
        combo_box::ComboBoxTriggerInput { class: "combo-box-input", attributes: props.attributes }
    }
}

#[component]
pub fn ComboBoxTriggerIndicator() -> Element {
    rsx! {
        combo_box::ComboBoxTriggerIndicator {}
    }
}

#[component]
pub fn ComboBoxList(props: ComboBoxListProps) -> Element {
    rsx! {
        combo_box::ComboBoxList { class: "combo-box-list", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn ComboBoxOption<T: Clone + PartialEq + 'static>(props: ComboBoxOptionProps<T>) -> Element {
    rsx! {
        combo_box::ComboBoxOption::<T> {
            class: "combo-box-option",
            value: props.value,
            text_value: props.text_value,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: props.attributes,

            {props.children}
        }
    }
}

#[component]
pub fn ComboBoxOptionIndicator() -> Element {
    rsx! {
        combo_box::ComboBoxOptionIndicator {
            svg {
                class: "combo-box-check-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M5 13l4 4L19 7" }
            }
        }
    }
}

#[component]
pub fn ComboBoxGroup(props: ComboBoxGroupProps) -> Element {
    rsx! {
        combo_box::ComboBoxGroup {
            class: "combo-box-group",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ComboBoxGroupLabel(props: ComboBoxGroupLabelProps) -> Element {
    rsx! {
        combo_box::ComboBoxGroupLabel {
            class: "combo-box-group-label",
            id: props.id,
            attributes: props.attributes,
            {props.children} }
    }
}
