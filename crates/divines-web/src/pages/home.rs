// Divines - 首页
// 参考原项目: astrostudyui/src/pages/index.js

use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "page home-page",
            div { class: "hero",
                h1 { "Divines" }
                p { class: "subtitle",
                    "把占星与中国术数，收进一个原生工作站"
                }
                p { class: "subtitle-en",
                    "Western astrology and Chinese metaphysics, in one workstation"
                }
            }

            div { class: "feature-grid",
                // 命
                div { class: "feature-card",
                    h3 { "命 · 命盘与推运" }
                    ul {
                        li { "占星本命盘 · 三维盘" }
                        li { "主限法 · 黄道星释 · 法达 · 太阳弧" }
                        li { "合盘 · 比较盘 · 组合盘" }
                        li { "八字 · 紫微斗数 · 七政四余" }
                        li { "印度占星 · 数算" }
                    }
                }

                // 卜
                div { class: "feature-card",
                    h3 { "卜 · 易与三式" }
                    ul {
                        li { "奇门遁甲 · 太乙 · 六壬" }
                        li { "三式合一综合面" }
                        li { "六爻 · 卦占" }
                        li { "节气盘 · 风水" }
                        li { "金口诀 · 皇极 · 五兆 · 太玄" }
                    }
                }

                // 工具
                div { class: "feature-card",
                    h3 { "工具 · 工作台" }
                    ul {
                        li { "AI 分析 · 多模型接入" }
                        li { "天文馆 · 实时三维天象" }
                        li { "黄历 · 寿星万年历" }
                        li { "八卦类象 · 十二宫 · 规则速查" }
                        li { "本地保存 · JSON 导入导出" }
                    }
                }
            }

            // 快速入口
            div { class: "quick-actions",
                h2 { "快速开始" }
                div { class: "action-buttons",
                    Link { to: Route::Bazi {}, class: "action-btn", "八字排盘" }
                    Link { to: Route::Ziwei {}, class: "action-btn", "紫微斗数" }
                    Link { to: Route::AstroNatal {}, class: "action-btn", "占星本命盘" }
                    Link { to: Route::Almanac {}, class: "action-btn", "万年历" }
                    Link { to: Route::AiAnalysis {}, class: "action-btn", "AI 分析" }
                }
            }
        }
    }
}