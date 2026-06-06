// Divines - 占星组件
// 参考原项目: astrostudyui/src/components/astro/

use dioxus::prelude::*;

/// 占星本命盘组件
#[component]
pub fn NatalChart() -> Element {
    rsx! {
        div { class: "astro-natal-chart",
            "占星本命盘组件"
        }
    }
}

/// 星盘绘制组件
#[component]
pub fn ChartWheel() -> Element {
    rsx! {
        div { class: "chart-wheel",
            "星盘绘制组件"
        }
    }
}

/// 行星信息组件
#[component]
pub fn PlanetInfo() -> Element {
    rsx! {
        div { class: "planet-info",
            "行星信息组件"
        }
    }
}

/// 相位表组件
#[component]
pub fn AspectTable() -> Element {
    rsx! {
        div { class: "aspect-table",
            "相位表组件"
        }
    }
}