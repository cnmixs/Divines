// Divines - 黄历/万年历组件
// 参考原项目: astrostudyui/src/components/calendar/

use dioxus::prelude::*;

/// 万年历组件
#[component]
pub fn AlmanacView() -> Element {
    rsx! {
        div { class: "almanac",
            h3 { "寿星万年历" }
            p { "万年历组件开发中..." }
        }
    }
}