// 星阙 Horosa - 天文馆组件
// 参考原项目: astrostudyui/src/components/planetarium/

use dioxus::prelude::*;

/// 天文馆 3D 组件
#[component]
pub fn Planetarium3D() -> Element {
    rsx! {
        div { class: "planetarium",
            h3 { "天文馆" }
            p { "三维天象功能开发中..." }
        }
    }
}