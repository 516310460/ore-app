use dioxus::prelude::*;

use crate::components::UserIcon;

#[component]
pub fn UserBubble<'a>(cx: Scope, class: Option<&'a str>) -> Element {
    let class = class.unwrap_or("");
    render! {
        div {
            class: "flex rounded-full text-gray-300 bg-gray-200 dark:bg-gray-700 dark:text-gray-900 {class}",
            UserIcon {
                class: "h-1/2 m-auto"
            }
        }
    }
}
