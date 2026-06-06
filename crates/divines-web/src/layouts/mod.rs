// Divines - 布局组件
// 完全对齐源仓库: astrostudyui/src/layouts/index.js, app.js, pages/index.js
// 以及 PageHeader.js 的完整三段式工具栏布局

use dioxus::prelude::*;
use crate::Route;

/// 导航标签定义 - 对齐源仓库 navigationPages
#[derive(Clone, PartialEq)]
struct NavTab {
    label: &'static str,
    route: Route,
    icon: &'static str,
    group: &'static str,
    key: &'static str,
}

/// 主导航布局 - 完全对齐源仓库
/// 顶部: PageHeader (品牌区 | 新命盘按钮组 | 当前模块导航 | 命令中心 | 实用工具栏 | 用户账号)
/// 左侧: 垂直标签导航
/// 右侧: 内容区
#[component]
pub fn Layout() -> Element {
    let mut active_tab = use_signal(|| "astrochart".to_string());
    let mut appearance = use_signal(|| "dark".to_string());
    let mut nav_open = use_signal(|| false);
    let mut settings_open = use_signal(|| false);
    let mut help_open = use_signal(|| false);
    let mut about_open = use_signal(|| false);

    let cycle_appearance = move |_| {
        let next = if appearance() == "dark" { "light" } else { "dark" };
        appearance.set(next.to_string());
    };

    // 导航分组定义 - 对齐源仓库 navigationPages
    let all_tabs: &[NavTab] = &[
        // 命 · 命盘与推运
        NavTab { label: "占星", key: "astrochart", icon: "★", group: "命", route: Route::AstroNatal {} },
        NavTab { label: "星运", key: "direction", icon: "↗", group: "命", route: Route::AstroTiming {} },
        NavTab { label: "八字", key: "bazi", icon: "甲", group: "命", route: Route::Bazi {} },
        NavTab { label: "紫微", key: "ziwei", icon: "斗", group: "命", route: Route::Ziwei {} },
        NavTab { label: "七政", key: "guolao", icon: "政", group: "命", route: Route::GuoLao {} },
        NavTab { label: "印占", key: "indiachart", icon: "卍", group: "命", route: Route::AstroVedic {} },
        NavTab { label: "辅盘", key: "auxchart", icon: "◈", group: "命", route: Route::AstroSpecialty {} },
        NavTab { label: "合盘", key: "relativechart", icon: "☯", group: "命", route: Route::AstroRelationship {} },
        NavTab { label: "数算", key: "shusuan", icon: "算", group: "命", route: Route::ShuSuan {} },
        NavTab { label: "其他", key: "mingother", icon: "…", group: "命", route: Route::MingOther {} },
        // 卜 · 易与三式
        NavTab { label: "三式", key: "sanshiunited", icon: "式", group: "卜", route: Route::Sanshi {} },
        NavTab { label: "六壬", key: "liureng", icon: "壬", group: "卜", route: Route::Liuren {} },
        NavTab { label: "遁甲", key: "dunjia", icon: "遁", group: "卜", route: Route::DunJia {} },
        NavTab { label: "六爻", key: "guazhan", icon: "爻", group: "卜", route: Route::Liuyao {} },
        NavTab { label: "太乙", key: "taiyi", icon: "乙", group: "卜", route: Route::Taiyi {} },
        NavTab { label: "分至", key: "jieqichart", icon: "节", group: "卜", route: Route::Jieqi {} },
        NavTab { label: "风水", key: "fengshui", icon: "風", group: "卜", route: Route::FengShui {} },
        NavTab { label: "其他", key: "cnyibu", icon: "…", group: "卜", route: Route::DivinationOther {} },
        // 工具 · 工具工作台
        NavTab { label: "AI分析", key: "aianalysis", icon: "AI", group: "工具", route: Route::AiAnalysis {} },
        NavTab { label: "天文馆", key: "planetarium", icon: "🔭", group: "工具", route: Route::Planetarium {} },
        NavTab { label: "黄历", key: "calendar", icon: "曆", group: "工具", route: Route::Almanac {} },
        NavTab { label: "辅助", key: "cntradition", icon: "⚙", group: "工具", route: Route::References {} },
    ];

    let ming_tabs: Vec<&NavTab> = all_tabs.iter().filter(|t| t.group == "命").collect();
    let bu_tabs: Vec<&NavTab> = all_tabs.iter().filter(|t| t.group == "卜").collect();
    let tool_tabs: Vec<&NavTab> = all_tabs.iter().filter(|t| t.group == "工具").collect();

    let current_page_label = all_tabs.iter()
        .find(|t| t.key == active_tab())
        .map(|t| t.label)
        .unwrap_or("导航");

    let is_dark = appearance() == "dark";

    rsx! {
        div {
            class: "divines-app divines-workspace-shell",
            "data-divines-appearance": if is_dark { "dark" } else { "light" },

            // ============================================================
            // 顶部工具栏 - 完全对齐源仓库 PageHeader
            // 三段式: 品牌区+新命盘 | 当前模块导航(居中) | 命令中心+实用工具+账号
            // ============================================================
            header { class: "divines-astro-header",
                div { class: "divines-userbox divines-astro-userbox",

                    // 左侧: 品牌区 + 新命盘按钮组
                    div { class: "divines-astro-brand",
                        button {
                            class: "divines-brand-button divines-astro-brand-button",
                            onclick: move |_| nav_open.set(!nav_open()),
                            span { class: "divines-brand-mark divines-astro-brand-mark", "★" }
                            span { class: "divines-brand-text divines-astro-brand-text", "Divines" }
                        }
                        div { class: "divines-astro-new-chart-group",
                            button {
                                class: "divines-astro-primary-command",
                                onclick: move |_| {},
                                "新命盘"
                            }
                            button {
                                class: "divines-astro-split-button",
                                onclick: move |_| {},
                                "▾"
                            }
                        }
                    }

                    // 中间: 当前模块导航按钮 - 绝对居中
                    div { class: "divines-astro-current-module-wrap",
                        button {
                            class: "divines-astro-current-module",
                            onclick: move |_| nav_open.set(!nav_open()),
                            span { class: "divines-astro-current-module-icon", "☰" }
                            span { "{current_page_label}" }
                            span { class: "divines-astro-current-module-chevron", "▾" }
                        }
                    }

                    // 右侧: 命令中心
                    div { class: "divines-astro-command-center",
                        button { class: "divines-astro-header-command",
                            onclick: move |_| {},
                            "AI导出"
                        }
                        button { class: "divines-astro-header-command",
                            onclick: move |_| settings_open.set(!settings_open()),
                            "设置"
                        }
                        button { class: "divines-astro-header-command",
                            onclick: move |_| help_open.set(!help_open()),
                            "帮助"
                        }
                    }

                    // 最右侧: 实用工具栏 + 用户账号
                    div { class: "divines-astro-utility-bar",
                        button {
                            class: "divines-astro-round-button",
                            onclick: cycle_appearance,
                            title: if is_dark { "切换亮色主题" } else { "切换暗色主题" },
                            if is_dark { "☀" } else { "🌙" }
                        }
                        div { class: "divines-astro-header-divider" }
                        div { class: "divines-account divines-astro-account",
                            div { class: "divines-avatar", "👤" }
                            span { class: "divines-name", "用户" }
                        }
                    }
                }
            }

            // 导航弹窗 (HomePageSetup)
            if nav_open() {
                div {
                    class: "divines-nav-overlay",
                    onclick: move |_| nav_open.set(false),
                    div {
                        class: "divines-nav-popup",
                        onclick: move |evt| evt.stop_propagation(),
                        div { class: "divines-nav-popup-header",
                            input {
                                class: "divines-nav-search-input",
                                r#type: "text",
                                placeholder: "搜索模块...",
                                autofocus: true,
                            }
                            button {
                                class: "divines-nav-popup-close",
                                onclick: move |_| nav_open.set(false),
                                "✕"
                            }
                        }
                        div { class: "divines-nav-popup-body",
                            // 命 · 命盘与推运
                            div { class: "divines-nav-popup-group",
                                div { class: "divines-nav-popup-group-title", "命 · 命盘与推运" }
                                div { class: "divines-nav-popup-grid",
                                    for tab in &ming_tabs {
                                        button {
                                            class: if tab.key == active_tab() { "divines-nav-popup-item active" } else { "divines-nav-popup-item" },
                                            onclick: {
                                                let key = tab.key.to_string();
                                                let route = tab.route.clone();
                                                move |_| {
                                                    active_tab.set(key.clone());
                                                    nav_open.set(false);
                                                }
                                            },
                                            span { class: "divines-nav-popup-item-icon", "{tab.icon}" }
                                            span { class: "divines-nav-popup-item-label", "{tab.label}" }
                                        }
                                    }
                                }
                            }
                            // 卜 · 易与三式
                            div { class: "divines-nav-popup-group",
                                div { class: "divines-nav-popup-group-title", "卜 · 易与三式" }
                                div { class: "divines-nav-popup-grid",
                                    for tab in &bu_tabs {
                                        button {
                                            class: if tab.key == active_tab() { "divines-nav-popup-item active" } else { "divines-nav-popup-item" },
                                            onclick: {
                                                let key = tab.key.to_string();
                                                move |_| {
                                                    active_tab.set(key.clone());
                                                    nav_open.set(false);
                                                }
                                            },
                                            span { class: "divines-nav-popup-item-icon", "{tab.icon}" }
                                            span { class: "divines-nav-popup-item-label", "{tab.label}" }
                                        }
                                    }
                                }
                            }
                            // 工具 · 工具工作台
                            div { class: "divines-nav-popup-group",
                                div { class: "divines-nav-popup-group-title", "工具 · 工作台" }
                                div { class: "divines-nav-popup-grid",
                                    for tab in &tool_tabs {
                                        button {
                                            class: if tab.key == active_tab() { "divines-nav-popup-item active" } else { "divines-nav-popup-item" },
                                            onclick: {
                                                let key = tab.key.to_string();
                                                move |_| {
                                                    active_tab.set(key.clone());
                                                    nav_open.set(false);
                                                }
                                            },
                                            span { class: "divines-nav-popup-item-icon", "{tab.icon}" }
                                            span { class: "divines-nav-popup-item-label", "{tab.label}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 设置弹窗
            if settings_open() {
                div {
                    class: "divines-modal-overlay",
                    onclick: move |_| settings_open.set(false),
                    div {
                        class: "divines-modal",
                        onclick: move |evt| evt.stop_propagation(),
                        div { class: "divines-modal-header",
                            span { "全局设置" }
                            button {
                                class: "divines-modal-close",
                                onclick: move |_| settings_open.set(false),
                                "✕"
                            }
                        }
                        div { class: "divines-modal-body",
                            div { class: "divines-settings-group",
                                div { class: "divines-settings-group-title", "日界点（日柱换日规则）" }
                                div { class: "divines-segmented",
                                    button { class: "divines-segmented-item active", "23点算第二天" }
                                    button { class: "divines-segmented-item", "24点算第二天" }
                                }
                                div { class: "divines-settings-hint", "作为所有技法的默认换日规则。'23点算第二天'=23点起日柱进位次日。" }
                            }
                            div { class: "divines-settings-group",
                                div { class: "divines-settings-group-title", "晚子时·时柱起干" }
                                div { class: "divines-segmented",
                                    button { class: "divines-segmented-item active", "晚子时按次日日柱计算" }
                                    button { class: "divines-segmented-item", "晚子时按当日柱计算" }
                                }
                                div { class: "divines-settings-hint", "晚子时=23:00-24:00。只在23:00-23:59时段影响时干。" }
                            }
                        }
                        div { class: "divines-modal-footer",
                            button {
                                class: "divines-btn divines-btn-primary",
                                onclick: move |_| settings_open.set(false),
                                "完成"
                            }
                        }
                    }
                }
            }

            // 帮助弹窗
            if help_open() {
                div {
                    class: "divines-modal-overlay",
                    onclick: move |_| help_open.set(false),
                    div {
                        class: "divines-modal",
                        onclick: move |evt| evt.stop_propagation(),
                        div { class: "divines-modal-header",
                            span { "{current_page_label}页帮助" }
                            button {
                                class: "divines-modal-close",
                                onclick: move |_| help_open.set(false),
                                "✕"
                            }
                        }
                        div { class: "divines-modal-body",
                            p { "左侧用于排盘输入与显示设置，中间保留原星盘绘制，右侧集中查看信息、相位、行星、古典与可能性。" }
                            p { "底部快捷功能会跳转到对应技法或打开已有抽屉，不改变排盘接口与本地服务调用。" }
                        }
                        div { class: "divines-modal-footer",
                            button {
                                class: "divines-btn divines-btn-primary",
                                onclick: move |_| help_open.set(false),
                                "知道了"
                            }
                        }
                    }
                }
            }

            // 关于弹窗
            if about_open() {
                div {
                    class: "divines-modal-overlay",
                    onclick: move |_| about_open.set(false),
                    div {
                        class: "divines-modal",
                        onclick: move |evt| evt.stop_propagation(),
                        div { class: "divines-modal-header",
                            span { "关于Divines" }
                            button {
                                class: "divines-modal-close",
                                onclick: move |_| about_open.set(false),
                                "✕"
                            }
                        }
                        div { class: "divines-modal-body",
                            div { class: "divines-about-body",
                                div { class: "divines-about-head-row",
                                    div { class: "divines-about-logo", "★" }
                                    div {
                                        div { class: "divines-about-name", "Divines" }
                                        div { class: "divines-about-version", "版本 v0.1.0" }
                                    }
                                }
                                div { class: "divines-about-desc", "本地优先的玄学与星座桌面应用，涵盖占星、八字、紫微、七政四余、三式与数算等技法，并内置 AI 分析与挂载上下文。" }
                            }
                        }
                        div { class: "divines-modal-footer",
                            button {
                                class: "divines-btn divines-btn-primary",
                                onclick: move |_| about_open.set(false),
                                "完成"
                            }
                        }
                    }
                }
            }

            // ============================================================
            // 主体 - 左侧垂直标签 + 右侧内容区
            // ============================================================
            div { class: "divines-app-body",

                // 左侧导航标签 - 对齐源仓库 mainRootTabs
                nav { class: "divines-nav-tabs",

                    // 命 · 命盘与推运 分组
                    div { class: "divines-nav-tab-group",
                        div { class: "divines-nav-tab-group-header", "命 · 命盘与推运" }
                        for tab in &ming_tabs {
                            NavTabItem {
                                tab: (*tab).clone(),
                                active: active_tab(),
                                on_click: {
                                    let key = tab.key.to_string();
                                    move |_| active_tab.set(key.clone())
                                },
                            }
                        }
                    }

                    // 卜 · 易与三式 分组
                    div { class: "divines-nav-tab-group",
                        div { class: "divines-nav-tab-group-header", "卜 · 易与三式" }
                        for tab in &bu_tabs {
                            NavTabItem {
                                tab: (*tab).clone(),
                                active: active_tab(),
                                on_click: {
                                    let key = tab.key.to_string();
                                    move |_| active_tab.set(key.clone())
                                },
                            }
                        }
                    }

                    // 工具 · 工具工作台 分组
                    div { class: "divines-nav-tab-group",
                        div { class: "divines-nav-tab-group-header", "工具 · 工作台" }
                        for tab in &tool_tabs {
                            NavTabItem {
                                tab: (*tab).clone(),
                                active: active_tab(),
                                on_click: {
                                    let key = tab.key.to_string();
                                    move |_| active_tab.set(key.clone())
                                },
                            }
                        }
                    }
                }

                // 右侧内容区
                main { class: "divines-main-content",
                    div { class: "divines-content-pane",
                        Outlet::<Route> {}
                    }
                }
            }

            // 底部状态栏
            footer { class: "divines-app-footer",
                "Divines v0.1.0 · 纯 Rust 全栈实现 · 基于寿星万年历引擎"
            }
        }
    }
}

/// 单个导航标签项
#[component]
fn NavTabItem(tab: NavTab, active: String, on_click: EventHandler<MouseEvent>) -> Element {
    let is_active = tab.key == active;

    rsx! {
        Link {
            to: tab.route.clone(),
            class: if is_active { "divines-nav-tab active" } else { "divines-nav-tab" },
            onclick: move |evt| {
                on_click.call(evt);
            },
            span { class: "divines-nav-tab-icon", "{tab.icon}" }
            span { class: "divines-nav-tab-label", "{tab.label}" }
        }
    }
}