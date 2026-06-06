// 星阙 Horosa - 三式组件
// 参考原项目: astrostudyui/src/components/sanshi/

use dioxus::prelude::*;

/// 三式合一组件
#[component]
pub fn SanshiUnited() -> Element {
    rsx! { div { class: "sanshi", "三式合一组件" } }
}

/// 奇门盘组件
#[component]
pub fn QimenChart() -> Element {
    rsx! { div { class: "qimen", "奇门盘组件" } }
}

/// 六壬盘组件
#[component]
pub fn LiurenChart() -> Element {
    rsx! { div { class: "liuren", "六壬盘组件" } }
}

/// 太乙盘组件
#[component]
pub fn TaiyiChart() -> Element {
    rsx! { div { class: "taiyi", "太乙盘组件" } }
}