use super::super::component::*;
use dioxus::prelude::*;
use strum::{EnumCount, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumCount, strum::EnumIter, strum::Display)]
enum Fruit {
    Apple,
    Banana,
    Orange,
    Orangeade,
    Strawberry,
    Watermelon,
}

impl Fruit {
    const fn emoji(&self) -> &'static str {
        match self {
            Fruit::Apple => "🍎",
            Fruit::Banana => "🍌",
            Fruit::Orange => "🍊",
            Fruit::Orangeade => "🧃",
            Fruit::Strawberry => "🍓",
            Fruit::Watermelon => "🍉",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumCount, strum::EnumIter, strum::Display)]
enum Vegetable {
    Broccoli,
    Carrot,
    Corn,
    Cucumber,
    Potato,
}

impl Vegetable {
    const fn emoji(&self) -> &'static str {
        match self {
            Vegetable::Broccoli => "🥦",
            Vegetable::Carrot => "🥕",
            Vegetable::Corn => "🌽",
            Vegetable::Cucumber => "🥒",
            Vegetable::Potato => "🥔",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Food {
    Fruit(Fruit),
    Vegetable(Vegetable),
}

#[component]
pub fn Demo() -> Element {
    let mut query = use_signal(String::new);

    let fruits = Fruit::iter().enumerate().map(|(i, f)| {
        rsx! {
            ComboboxOption::<Option<Food>> { index: i, value: Food::Fruit(f), text_value: "{f}",
                disabled: matches!(f, Fruit::Orange),
                "{f.emoji()} {f}"
            }
        }
    });

    let vegetables = Vegetable::iter().enumerate().map(|(i, v)| {
        rsx! {
            ComboboxOption::<Option<Food>> { index: Fruit::COUNT + i, value: Food::Vegetable(v), text_value: "{v}",
                "{v.emoji()} {v}"
            }
        }
    });

    rsx! {
        Combobox::<Option<Food>> {
            width: "12rem",
            query: Some(query()),
            on_query_change: move |next| query.set(next),

            placeholder: "Select food...",
            aria_label: "Select food",
            list_aria_label: "Food",
            ComboboxEmpty { "No Food found." }

            ComboboxGroup {
                ComboboxGroupLabel { "Fruits" }
                {fruits}
            }
            ComboboxGroup {
                disabled: true,
                ComboboxGroupLabel { "Vegetables" }
                {vegetables}
            }
            ComboboxGroup {
                ComboboxGroupLabel { "Other" }
                ComboboxOption::<Option<Food>> {
                    index: Fruit::COUNT + Vegetable::COUNT,
                    value: None,
                    text_value: "Other",
                    "Other"
                }
            }
        }
    }
}
