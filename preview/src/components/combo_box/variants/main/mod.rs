use dioxus::prelude::*;
use strum::{IntoEnumIterator};
use crate::components::combo_box::{ComboBoxGroup, ComboBoxGroupLabel, ComboBoxList, ComboBoxOption, ComboBoxOptionIndicator, ComboBoxTrigger, ComboBoxTriggerIndicator, ComboBoxTriggerInput};
use crate::components::combo_box::component::ComboBox;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumCount, strum::EnumIter, strum::Display)]
enum Fruit {
    Apple,
    Banana,
    Orange,
    Strawberry,
    Watermelon,
}

impl Fruit {
    const fn emoji(&self) -> &'static str {
        match self {
            Fruit::Apple => "🍎",
            Fruit::Banana => "🍌",
            Fruit::Orange => "🍊",
            Fruit::Strawberry => "🍓",
            Fruit::Watermelon => "🍉",
        }
    }
}

#[component]
pub fn Demo() -> Element {
    let fruits = Fruit::iter().enumerate().map(|(i, f)| {
        rsx! {
            ComboBoxOption::<Option<Fruit>> { index: i, value: f, text_value: "{f}",
                {format!("{} {f}", f.emoji())}
                ComboBoxOptionIndicator {}
            }
        }
    });

    rsx! {
        ComboBox<Option<Fruit>> {
            ComboBoxTrigger {
                ComboBoxTriggerInput {}
                ComboBoxTriggerIndicator {}
            }
            ComboBoxList {
                ComboBoxGroup {
                    ComboBoxGroupLabel { "Fruits" }
                    ComboBoxOption::<Option<Fruit>> {
                        index: 0usize,
                        value: Some(Fruit::Apple),
                        text_value: Some("Apple".to_string()),
                        "Apple"
                        ComboBoxOptionIndicator {}
                    }
                    ComboBoxOption::<Option<Fruit>> {
                        index: 1usize,
                        value: Some(Fruit::Banana),
                        text_value: Some("Banana".to_string()),
                        "Banana"
                        ComboBoxOptionIndicator {}
                    }
                }
            }
        }
    }

}
