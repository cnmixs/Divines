// 星阙 Horosa - 通用组件
// 参考原项目: astrostudyui/src/components/commtools/

use dioxus::prelude::*;

/// 日期时间选择器
#[component]
pub fn DateTimePicker(
    value: String,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "datetime-picker",
            input {
                r#type: "datetime-local",
                value: "{value}",
                oninput: move |evt| on_change.call(evt.value()),
            }
        }
    }
}

/// 地点选择器
#[component]
pub fn LocationPicker(
    value: String,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "location-picker",
            input {
                r#type: "text",
                placeholder: "输入地点名称...",
                value: "{value}",
                oninput: move |evt| on_change.call(evt.value()),
            }
        }
    }
}

/// 性别选择器
#[component]
pub fn GenderSelector(
    value: String,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "gender-selector",
            select {
                value: "{value}",
                onchange: move |evt| on_change.call(evt.value()),
                option { value: "male", "男" }
                option { value: "female", "女" }
            }
        }
    }
}

/// 宫位制选择器
#[component]
pub fn HouseSystemSelector(
    value: String,
    on_change: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "house-system-selector",
            select {
                value: "{value}",
                onchange: move |evt| on_change.call(evt.value()),
                option { value: "placidus", "Placidus" }
                option { value: "koch", "Koch" }
                option { value: "equal", "Equal" }
                option { value: "whole_sign", "Whole Sign" }
                option { value: "regiomontanus", "Regiomontanus" }
                option { value: "campanus", "Campanus" }
            }
        }
    }
}

/// 加载指示器
#[component]
pub fn Loading() -> Element {
    rsx! {
        div { class: "loading",
            "加载中..."
        }
    }
}

/// 错误提示
#[component]
pub fn ErrorMessage(message: String) -> Element {
    rsx! {
        div { class: "error-message",
            "{message}"
        }
    }
}

/// 空状态
#[component]
pub fn EmptyState(message: String) -> Element {
    rsx! {
        div { class: "empty-state",
            "{message}"
        }
    }
}