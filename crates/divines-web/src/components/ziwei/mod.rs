// Divines - 紫微斗数组件
// 参考原项目: astrostudyui/src/components/ziwei/

use dioxus::prelude::*;

/// 紫微斗数盘组件
#[component]
pub fn ZiweiChart() -> Element {
    rsx! {
        div { class: "ziwei-chart",
            h3 { "紫微斗数" }
            p { "紫微斗数排盘组件开发中..." }
        }
    }
}