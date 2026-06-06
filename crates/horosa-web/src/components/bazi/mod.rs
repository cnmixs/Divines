// 星阙 Horosa - 八字组件
// 参考原项目: astrostudyui/src/components/bazi/

use dioxus::prelude::*;

/// 八字排盘组件
#[component]
pub fn BaziChart() -> Element {
    rsx! {
        div { class: "bazi-chart",
            h3 { "八字排盘" }
            p { "八字排盘组件开发中..." }
        }
    }
}

/// 四柱展示组件
#[component]
pub fn Pillars() -> Element {
    rsx! {
        div { class: "pillars",
            "四柱展示组件"
        }
    }
}