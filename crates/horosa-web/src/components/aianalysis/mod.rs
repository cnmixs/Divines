// 星阙 Horosa - AI 分析组件
// 参考原项目: astrostudyui/src/components/aianalysis/

use dioxus::prelude::*;

/// AI 对话组件
#[component]
pub fn AiChat() -> Element {
    rsx! {
        div { class: "ai-chat",
            h3 { "AI 分析" }
            p { "AI 对话功能开发中..." }
        }
    }
}

/// AI 模型选择组件
#[component]
pub fn ModelSelector() -> Element {
    rsx! {
        div { class: "model-selector",
            "模型选择组件"
        }
    }
}