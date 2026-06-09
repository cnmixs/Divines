// Divines - 页面模块
// 参考原项目: astrostudyui/src/pages/

pub mod home;
pub use home::*;

use dioxus::prelude::*;
use dioxus::signals::*;
use crate::Route;
use crate::services;
use chrono::Datelike;

// ============ 辅助函数 ============

fn fmt_deg(v: f64) -> String { format!("{:.2}度", v) }
fn fmt_age(start: u64, end: u64) -> String { format!("{}-{}岁", start, end) }
fn fmt_year(start: i64, end: i64) -> String { format!("{}-{}", start, end) }
fn fmt_mag(v: f64) -> String { format!("{:.3}", v) }
fn fmt_hour(v: f64) -> String { format!("{:.2}时", v) }

// ============ 占星本命盘============

#[component]
pub fn AstroNatal() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut timezone = use_signal(|| 8.0_f64);
    let mut place_name = use_signal(|| String::new());
    let mut name = use_signal(|| String::new());
    let mut gender = use_signal(|| "male".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    let on_submit = move |_| {
        loading.set(true);
        error.set(None);
        let req = std::sync::Arc::new(serde_json::json!({
            "datetime": datetime(),
            "latitude": latitude(),
            "longitude": longitude(),
            "timezone": timezone(),
            "place_name": place_name(),
            "name": name(),
            "gender": gender(),
        }));
                spawn(async move {
            let fut = services::astro::get_natal_chart(&*req);
            match fut.await {
                Ok(data) => {
                    result.set(Some(data));
                    loading.set(false);
                }
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "占星本命盘 " }
            p { class: "page-desc ", "输入出生信息,计算西洋占星本命盘 " }

            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "出生日期时间 " }
                        input {
                            r#type: "datetime-local ",
                            value: "{datetime}",
                            oninput: move |evt| datetime.set(evt.value()),
                        }
                    }
                    div { class: "form-group ",
                        label { "姓名 " }
                        input {
                            r#type: "text ",
                            placeholder: "可选",
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value()),
                        }
                    }
                }
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "纬度 " }
                        input {
                            r#type: "number ",
                            step: "0.0001 ",
                            value: "{latitude}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<f64>() {
                                    latitude.set(v);
                                }
                            },
                        }
                    }
                    div { class: "form-group ",
                        label { "经度 " }
                        input {
                            r#type: "number ",
                            step: "0.0001 ",
                            value: "{longitude}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<f64>() {
                                    longitude.set(v);
                                }
                            },
                        }
                    }
                    div { class: "form-group ",
                        label { "时区 " }
                        input {
                            r#type: "number ",
                            step: "0.5 ",
                            value: "{timezone}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<f64>() {
                                    timezone.set(v);
                                }
                            },
                        }
                    }
                }
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "地点 " }
                        input {
                            r#type: "text ",
                            placeholder: "如:北京 ",
                            value: "{place_name}",
                            oninput: move |evt| place_name.set(evt.value()),
                        }
                    }
                    div { class: "form-group ",
                        label { "性别 " }
                        select {
                            value: "{gender}",
                            onchange: move |evt| gender.set(evt.value()),
                            option { value: "male", "男 " }
                            option { value: "female", "女 " }
                        }
                    }
                }
                button {
                    class: "submit-btn ",
                    onclick: on_submit,
                    disabled: loading(),
                    "计算本命盘 "
                }
            }

            if loading() {
                div { class: "loading ", "计算中... " }
            }

            if let Some(ref err) = *error.read() {
                div { class: "error-message ", "{err}" }
            }

            if let Some(ref data) = *result.read() {
                div { class: "result-card ",
                    h3 { "星盘结果 " }
                    if let Some(planets) = data.get("planets").and_then(|v| v.as_array()) {
                        div { class: "planet-list ",
                            h4 { "行星位置 " }
                            table { class: "data-table ",
                                thead {
                                    tr {
                                        th { "行星 " }
                                        th { "星座 " }
                                        th { "度数 " }
                                        th { "宫位 " }
                                        th { "逆行 " }
                                    }
                                }
                                tbody {
                                    for planet in planets {
                                        tr {
                                            td { {planet.get("planet").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            td { {planet.get("sign").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            td { {fmt_deg(planet.get("degree_in_sign").and_then(|v| v.as_f64()).unwrap_or(0.0))} }
                                            td { {planet.get("house").and_then(|v| v.as_u64()).unwrap_or(0).to_string()} }
                                            td { {planet.get("is_retrograde").and_then(|v| v.as_bool()).unwrap_or(false).to_string()} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if let Some(aspects) = data.get("aspects").and_then(|v| v.as_array()) {
                        div { class: "aspect-list ",
                            h4 { "相位 ({aspects.len()})" }
                            table { class: "data-table ",
                                thead {
                                    tr {
                                        th { "行星1 " }
                                        th { "行星2 " }
                                        th { "相位 " }
                                        th { "角度 " }
                                        th { "容许度 " }
                                    }
                                }
                                tbody {
                                    for aspect in aspects {
                                        tr {
                                            td { {aspect.get("planet1").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            td { {aspect.get("planet2").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            td { {aspect.get("aspect_type").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            td { {fmt_deg(aspect.get("angle").and_then(|v| v.as_f64()).unwrap_or(0.0))} }
                                            td { {fmt_deg(aspect.get("orb").and_then(|v| v.as_f64()).unwrap_or(0.0))} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ============ 星运推运 ============

#[component]
pub fn AstroTiming() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut timezone = use_signal(|| 8.0_f64);
    let mut active_tab = use_signal(|| "solar_arc".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let tabs = [
        ("solar_arc", "太阳弧 "),
        ("progressions", "次限法 "),
        ("primary_dir", "主限法 "),
        ("profections", "小限 "),
        ("firdaria", "法达星限 "),
        ("age_point", "年龄推进点 "),
        ("symbolic_dir", "波期向运 "),
        ("term_dir", "界限法 "),
        ("thirteenth", "第十三月盘 "),
        ("harmonic", "调波盘 "),
        ("draconic", "龙盘 "),
        ("year_129", "129年系统 "),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({
            "datetime": datetime(), "latitude": latitude(),
            "longitude": longitude(), "timezone": timezone(),
        }));
        let endpoint = match active_tab().as_str() {
            "solar_arc" => "/predict/solar-arc ",
            "progressions" => "/predict/progressions ",
            "primary_dir" => "/predict/primary-directions ",
            "profections" => "/predict/profections ",
            "firdaria" => "/astro/firdaria ",
            "age_point" => "/predict/age-point ",
            "symbolic_dir" => "/predict/symbolic-dir ",
            "term_dir" => "/predict/term-direction ",
            "thirteenth" => "/predict/thirteenth-chart ",
            "harmonic" => "/predict/harmonic-chart ",
            "draconic" => "/predict/draconic-chart ",
            "year_129" => "/predict/year-system-129 ",
            _ => "/predict/solar-arc ",
        };
                spawn(async move {
            let fut = services::astro::api_request("POST", endpoint, Some(&*req));
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "星运 - 推运 " }
            p { class: "page-desc ", "太阳弧, 次限法, 主限法, 法达星限, 小限等推运系统 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "出生日期时间 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) }
                    }
                }
                div { class: "form-row ",
                    div { class: "form-group ", label { "纬度 " }
                        input { r#type: "number ", step: "0.0001 ", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group ", label { "经度 " }
                        input { r#type: "number ", step: "0.0001 ", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                    div { class: "form-group ", label { "时区 " }
                        input { r#type: "number ", step: "0.5 ", value: "{timezone}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { timezone.set(v); } } } }
                }
                div { class: "tab-buttons ",
                    for (key, label) in tabs.clone() {
                        button {
                            class: if active_tab() == *key { "tab-btn active " } else { "tab-btn " },
                            onclick: move |_| active_tab.set(key.to_string()),
                            "{label}" 
                        }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "计算 " }
            }
            if loading() { div { class: "loading ", "计算中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "推运结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn AstroRelationship() -> Element {
    let mut inner_datetime = use_signal(|| String::new());
    let mut outer_datetime = use_signal(|| String::new());
    let mut active_tab = use_signal(|| "synastry".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let tabs = [
        ("synastry", "比较盘 "),
        ("composite", "组合盘 "),
        ("time_space", "时空中点盘 "),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({
            "inner": { "datetime": inner_datetime() },
            "outer": { "datetime": outer_datetime() },
        }));
        let endpoint = if active_tab() == "composite" { "/astro/composite " } else { "/astro/synastry " };
                spawn(async move {
            let fut = services::astro::api_request("POST", endpoint, Some(&*req));
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "合盘 - 关系盘 " }
            p { class: "page-desc ", "比较盘, 组合盘, 时空中点盘分析 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "内盘出生时间 " }
                        input { r#type: "datetime-local ", value: "{inner_datetime}", oninput: move |evt| inner_datetime.set(evt.value()) } }
                }
                div { class: "form-row ",
                    div { class: "form-group ", label { "外盘出生时间 " }
                        input { r#type: "datetime-local ", value: "{outer_datetime}", oninput: move |evt| outer_datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons ",
                    for (key, label) in tabs.clone() {
                        button { class: if active_tab() == *key { "tab-btn active " } else { "tab-btn " },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "计算合盘 " }
            }
            if loading() { div { class: "loading ", "计算中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "合盘结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn AstroSpecialty() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut timezone = use_signal(|| 8.0_f64);
    let mut active_tab = use_signal(|| "arabic".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let tabs = [
        ("arabic", "阿拉伯点 "),
        ("aspects", "相位详情 "),
        ("decennials", "十年运 "),
        ("dispositor", "最终定位星 "),
        ("lots", "特殊点 "),
        ("zr", "黄道星释 "),
        ("return", "回归盘 "),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({
            "datetime": datetime(), "latitude": latitude(),
            "longitude": longitude(), "timezone": timezone(),
        }));
        let endpoint = match active_tab().as_str() {
            "arabic" => "/astro/arabic-points ",
            "aspects" => "/astro/aspects ",
            _ => "/astro/natal ",
        };
                spawn(async move {
            let fut = services::astro::api_request("POST", endpoint, Some(&*req));
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "辅盘 - 专项分析 " }
            p { class: "page-desc ", "阿拉伯点, 相位, 星释, 回归盘等专项分析 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "出生日期时间 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons ",
                    for (key, label) in tabs.clone() {
                        button { class: if active_tab() == *key { "tab-btn active " } else { "tab-btn " },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "计算 " }
            }
            if loading() { div { class: "loading ", "计算中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "分析结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn AstroVedic() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut timezone = use_signal(|| 8.0_f64);
    let mut active_tab = use_signal(|| "chart".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let tabs = [
        ("chart", "印度盘 "),
        ("dasha", "大运 "),
        ("yogas", "格局 "),
        ("nakshatra", "27宿 "),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({
            "datetime": datetime(), "latitude": latitude(),
            "longitude": longitude(), "timezone": timezone(),
        }));
        let endpoint = match active_tab().as_str() {
            "chart" => "/vedic/chart ",
            "dasha" => "/vedic/dasha ",
            "yogas" => "/vedic/yogas ",
            "nakshatra" => "/vedic/nakshatra ",
            _ => "/vedic/chart ",
        };
                spawn(async move {
            let fut = services::astro::api_request("POST", endpoint, Some(&*req));
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "印度占星 - Vedic " }
            p { class: "page-desc ", "包含东印度盘, 恒星黄道, 大运系统, 27宿 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "出生日期时间 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons ",
                    for (key, label) in tabs.clone() {
                        button { class: if active_tab() == *key { "tab-btn active " } else { "tab-btn " },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "计算 " }
            }
            if loading() { div { class: "loading ", "计算中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "印度占星结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn AstroQizheng() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut timezone = use_signal(|| 8.0_f64);
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    let on_submit = move |_| {
        loading.set(true);
        error.set(None);
        let req = std::sync::Arc::new(serde_json::json!({
            "datetime": datetime(),
            "latitude": latitude(),
            "longitude": longitude(),
            "timezone": timezone(),
        }));
                spawn(async move {
            let fut = services::qizheng::get_chart(&*req);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(e) => { error.set(Some(e)); loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "七政四余 - 果老星宗 " }
            p { class: "page-desc ", "输入出生信息,排七政四余星盘,含28宿, 命度身度, 洞微大限, 果老格局 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "出生日期时间 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) }
                    }
                }
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "纬度 " }
                        input { r#type: "number ", step: "0.0001 ", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } }
                    }
                    div { class: "form-group ",
                        label { "经度 " }
                        input { r#type: "number ", step: "0.0001 ", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } }
                    }
                    div { class: "form-group ",
                        label { "时区 " }
                        input { r#type: "number ", step: "0.5 ", value: "{timezone}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { timezone.set(v); } } }
                    }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "排盘 " }
            }
            if loading() { div { class: "loading ", "排盘中... " } }
            if let Some(ref err) = *error.read() { div { class: "error-message ", "{err}" } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ",
                    h3 { "七政四余星盘 " }
                    if let Some(planets) = data.get("planets").and_then(|v| v.as_array()) {
                        div { class: "section ",
                            h4 { "行星位置 " }
                            table { class: "data-table ",
                                thead { tr { th { "行星 " } th { "黄经 " } th { "星宿 " } th { "宫位 " } th { "28宿 " } th { "逆行 " } } }
                                tbody {
                                    for p in planets {
                                        tr {
                                            td { {p.get("name_zh").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            td { {fmt_deg(p.get("longitude").and_then(|v| v.as_f64()).unwrap_or(0.0))} }
                                            td { {p.get("sign_zh").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            td { {p.get("house").and_then(|v| v.as_u64()).unwrap_or(0).to_string()} }
                                            td { {p.get("su_name").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            td { {p.get("is_retrograde").and_then(|v| v.as_bool()).unwrap_or(false).to_string()} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if let Some(houses) = data.get("houses").and_then(|v| v.as_array()) {
                        div { class: "section ",
                            h4 { "十二宫 " }
                            table { class: "data-table ",
                                thead { tr { th { "宫位 " } th { "宫名 " } th { "星座 " } th { "度数 " } } }
                                tbody {
                                    for h in houses {
                                        tr {
                                            td { {h.get("house_num").and_then(|v| v.as_u64()).unwrap_or(0).to_string()} }
                                            td { {h.get("name_zh").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            td { {h.get("sign_zh").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            td { {fmt_deg(h.get("cusp").and_then(|v| v.as_f64()).unwrap_or(0.0))} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if let Some(patterns) = data.get("patterns").and_then(|v| v.as_array()) {
                        if !patterns.is_empty() {
                            div { class: "section ",
                                h4 { "格局 " }
                                ul { for p in patterns { li { {p.as_str().unwrap_or("? ")} } } }
                            }
                        }
                    }
                    if let Some(dongwei) = data.get("dong_wei").and_then(|v| v.as_array()) {
                        if !dongwei.is_empty() {
                            div { class: "section ",
                                h4 { "洞微大限 " }
                                table { class: "data-table ",
                                    thead { tr { th { "年限 " } th { "宫位 " } th { "说明 " } } }
                                    tbody {
                                        for dw in dongwei {
                                            tr {
                                                td { {format!("{}-{}岁 ", dw.get("start_age").and_then(|v| v.as_u64()).unwrap_or(0).to_string(), dw.get("end_age").and_then(|v| v.as_u64()).unwrap_or(0).to_string())} }
                                                td { {dw.get("house_name").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                                td { {dw.get("description").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ============ 八字排盘 ============

#[component]
pub fn Bazi() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut name = use_signal(|| String::new());
    let mut gender = use_signal(|| "male".to_string());
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut use_true_solar = use_signal(|| false);
    let mut use_early_late_zi = use_signal(|| false);
    let mut use_ding_qi = use_signal(|| true);
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    let on_submit = move |_| {
        loading.set(true);
        error.set(None);
        let req = std::sync::Arc::new(serde_json::json!({
            "datetime": datetime(),
            "name": name(),
            "gender": gender(),
            "longitude": longitude(),
            "use_true_solar_time": use_true_solar(),
            "use_early_late_zi": use_early_late_zi(),
            "use_ding_qi": use_ding_qi(),
        }));
                spawn(async move {
            let fut = services::bazi::calculate(&*req);
            match fut.await {
                Ok(data) => {
                    result.set(Some(data));
                    loading.set(false);
                }
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "八字排盘 " }
            p { class: "page-desc ", "输入出生日期时间,排四柱八字, 十神, 大运,支持真太阳时, 早晚子时, 平气/定气 " }

            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "出生日期时间 " }
                        input {
                            r#type: "datetime-local ",
                            value: "{datetime}",
                            oninput: move |evt| datetime.set(evt.value()),
                        }
                    }
                    div { class: "form-group ",
                        label { "姓名(可选) " }
                        input {
                            r#type: "text ",
                            placeholder: "可选",
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value()),
                        }
                    }
                    div { class: "form-group ",
                        label { "性别 " }
                        select {
                            value: "{gender}",
                            onchange: move |evt| gender.set(evt.value()),
                            option { value: "male", "男 " }
                            option { value: "female", "女 " }
                        }
                    }
                }

                // 排盘选项
                div { class: "options-section ",
                    h4 { "排盘选项 " }
                    div { class: "options-grid ",
                        div { class: "option-item ",
                            label { class: "option-label ",
                                input {
                                    r#type: "checkbox ",
                                    checked: use_true_solar(),
                                    onchange: move |evt| use_true_solar.set(evt.value() == "true "),
                                }
                                span { "真太阳时 " }
                            }
                        }
                        div { class: "option-item ",
                            label { class: "option-label ",
                                input {
                                    r#type: "checkbox ",
                                    checked: use_early_late_zi(),
                                    onchange: move |evt| use_early_late_zi.set(evt.value() == "true "),
                                }
                                span { "区分早晚子时 " }
                            }
                        }
                        div { class: "option-item ",
                            label { class: "option-label ",
                                input {
                                    r#type: "checkbox ",
                                    checked: use_ding_qi(),
                                    onchange: move |evt| use_ding_qi.set(evt.value() == "true "),
                                }
                                span { "定气法 " }
                            }
                            span { class: "option-hint ", "(取消选择为平气法) " }
                        }
                        div { class: "form-group form-group-inline ",
                            label { "经度: " }
                            input {
                                r#type: "number ",
                                step: "0.0001 ",
                                value: "{longitude}",
                                style: "width: 100px ",
                                oninput: move |evt| {
                                    if let Ok(v) = evt.value().parse::<f64>() {
                                        longitude.set(v);
                                    }
                                },
                            }
                        }
                    }
                }

                button {
                    class: "submit-btn ",
                    onclick: on_submit,
                    disabled: loading(),
                    "排盘 "
                }
            }

            if loading() {
                div { class: "loading ", "排盘中... " }
            }

            if let Some(ref err) = *error.read() {
                div { class: "error-message ", "{err}" }
            }

            if let Some(ref data) = *result.read() {
                div { class: "result-card ",
                    h3 { "八字排盘结果 " }

                    // 四柱
                    div { class: "bazi-pillars ",
                        h4 { "四柱 " }
                        div { class: "pillar-grid ",
                            for pillar_key in ["year ", "month ", "day ", "hour"] {
                                div { class: "pillar-item ",
                                    div { class: "pillar-label ",
                                        {match pillar_key {
                                            "year" => "年柱 ",
                                            "month" => "月柱 ",
                                            "day" => "日柱 ",
                                            "hour" => "时柱 ",
                                            _ => "",
                                        }}
                                    }
                                    if let Some(pillar) = data.get(pillar_key) {
                                        div { class: "pillar-tg ",
                                            {pillar.get("tian_gan").and_then(|v| v.as_str()).unwrap_or("? ")}
                                        }
                                        div { class: "pillar-dz ",
                                            {pillar.get("di_zhi").and_then(|v| v.as_str()).unwrap_or("? ")}
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 日主
                    if let Some(dm) = data.get("day_master").and_then(|v| v.as_str()) {
                        div { class: "day-master ",
                            span { "日主: " }
                            strong { "{dm}" }
                        }
                    }
                    if let Some(adj_hour) = data.get("adjusted_hour").and_then(|v| v.as_f64()) {
                        div { class: "adjusted-hour ",
                            span { "校正时 " }
                            span { "{adj_hour:.2}时 " }
                        }
                    }

                    // 十神 
                    if let Some(ten_gods) = data.get("ten_gods") {
                        div { class: "ten-gods ",
                            h4 { "十神 " }
                            div { class: "ten-god-grid ",
                                for (key, label) in [("year", "年 "), ("month", "月 "), ("day", "日 "), ("hour", "时 ")] {
                                    div { class: "ten-god-item ",
                                        span { "{label}: " }
                                        span { class: "ten-god-value ",
                                            {ten_gods.get(key).and_then(|v| v.as_str()).unwrap_or("? ")}
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 长生十二神
                    if let Some(chang_sheng) = data.get("chang_sheng") {
                        div { class: "chang-sheng ",
                            h4 { "长生十二神 " }
                            div { class: "chang-sheng-grid ",
                                for (key, label) in [("year", "年 "), ("month", "月 "), ("day", "日 "), ("hour", "时 ")] {
                                    div { class: "chang-sheng-item ",
                                        span { "{label}: " }
                                        span { class: "chang-sheng-value ",
                                            {chang_sheng.get(key).and_then(|v| v.as_str()).unwrap_or("? ")}
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 纳音
                    if let Some(na_yin) = data.get("na_yin") {
                        div { class: "na-yin ",
                            h4 { "纳音 " }
                            div { class: "na-yin-grid ",
                                for (key, label) in [("year", "年 "), ("month", "月 "), ("day", "日 "), ("hour", "时 ")] {
                                    div { class: "na-yin-item ",
                                        span { "{label}: " }
                                        span { {na_yin.get(key).and_then(|v| v.as_str()).unwrap_or("? ")} }
                                    }
                                }
                            }
                        }
                    }

                    // 藏干
                    if let Some(hidden) = data.get("hidden_stems") {
                        div { class: "hidden-stems ",
                            h4 { "藏干 " }
                            div { class: "ten-god-grid ",
                                for (key, label) in [("year", "年 "), ("month", "月 "), ("day", "日 "), ("hour", "时 ")] {
                                    div { class: "ten-god-item ",
                                        span { "{label}: " }
                                        if let Some(arr) = hidden.get(key).and_then(|v| v.as_array()) {
                                            span { class: "hidden-value ",
                                                {arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", ") }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 干支刑冲合度 
                    if let Some(relations) = data.get("relations").and_then(|v| v.as_array()) {
                        if !relations.is_empty() {
                            div { class: "relations ",
                                h4 { "干支刑冲合度 " }
                                table { class: "data-table ",
                                    thead {
                                        tr {
                                            th { "类型 " }
                                            th { "涉及柱 " }
                                            th { "详情 " }
                                        }
                                    }
                                    tbody {
                                        for rel in relations {
                                            tr {
                                                td { {rel.get("relation_type").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                                td {
                                                    if let Some(pillars) = rel.get("pillars").and_then(|v| v.as_array()) {
                                                        {pillars.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", ") }
                                                    }
                                                }
                                                td { {rel.get("detail").and_then(|v| v.as_str()).unwrap_or("")} }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 神煞
                    if let Some(shen_sha) = data.get("shen_sha").and_then(|v| v.as_array()) {
                        if !shen_sha.is_empty() {
                            div { class: "shen-sha ",
                                h4 { "神煞 " }
                                table { class: "data-table ",
                                    thead {
                                        tr {
                                            th { "神煞 " }
                                            th { "位置 " }
                                            th { "说明 " }
                                        }
                                    }
                                    tbody {
                                        for ss in shen_sha {
                                            tr {
                                                td { {ss.get("name").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                                td { {ss.get("pillar").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                                td { {ss.get("description").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 大运
                    if let Some(qi_yun) = data.get("qi_yun_time").and_then(|v| v.as_str()) {
                        div { class: "qi-yun ",
                            h4 { "起运时间 " }
                            p { "{qi_yun}" }
                        }
                    }

                    if let Some(da_yun) = data.get("da_yun").and_then(|v| v.as_array()) {
                        if !da_yun.is_empty() {
                            div { class: "da-yun ",
                                h4 { "大运 " }
                                table { class: "data-table ",
                                    thead {
                                        tr {
                                            th { "年龄 " }
                                            th { "天干 " }
                                            th { "地支 " }
                                            th { "十神 " }
                                            th { "年份 " }
                                        }
                                    }
                                    tbody {
                                        for dy in da_yun {
                                            tr {
                                                td { {format!("{}-{}岁 ", dy.get("start_age").and_then(|v| v.as_u64()).unwrap_or(0).to_string(), dy.get("end_age").and_then(|v| v.as_u64()).unwrap_or(0).to_string())} }
                                                td {
                                                    {dy.get("pillar").and_then(|v| v.get("tian_gan")).and_then(|v| v.as_str()).unwrap_or("? ")}
                                                }
                                                td {
                                                    {dy.get("pillar").and_then(|v| v.get("di_zhi")).and_then(|v| v.as_str()).unwrap_or("? ")}
                                                }
                                                td { {dy.get("ten_god").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                                td {
                                                    {format!("{}-{}", dy.get("start_year").and_then(|v| v.as_i64()).unwrap_or(0).to_string(), dy.get("end_year").and_then(|v| v.as_i64()).unwrap_or(0).to_string())}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 排盘选项回显
                    if let Some(options) = data.get("options") {
                        div { class: "options-display ",
                            h4 { "选项 " }
                            div { class: "options-display-grid ",
                                span { "真太阳时: " {options.get("use_true_solar_time").and_then(|v| v.as_bool()).unwrap_or(false).to_string()} }
                                span { "早晚子时: " {options.get("use_early_late_zi").and_then(|v| v.as_bool()).unwrap_or(false).to_string()} }
                                span { "定气法: " {options.get("use_ding_qi").and_then(|v| v.as_bool()).unwrap_or(true).to_string()} }
                                span { "经度: " {options.get("longitude").and_then(|v| v.as_f64()).unwrap_or(0.0).to_string()} }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ============ 紫微斗数 ============

#[component]
pub fn Ziwei() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut gender = use_signal(|| "male".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    let on_submit = move |_| {
        loading.set(true);
        error.set(None);
        let req = std::sync::Arc::new(serde_json::json!({
            "datetime": datetime(),
            "gender": gender(),
        }));
                spawn(async move {
            let fut = services::ziwei::calculate(&*req);
            match fut.await {
                Ok(data) => {
                    result.set(Some(data));
                    loading.set(false);
                }
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "紫微斗数 " }
            p { class: "page-desc ", "输入出生日期时间,排紫微斗数命盘 " }

            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "出生日期时间 " }
                        input {
                            r#type: "datetime-local ",
                            value: "{datetime}",
                            oninput: move |evt| datetime.set(evt.value()),
                        }
                    }
                    div { class: "form-group ",
                        label { "性别 " }
                        select {
                            value: "{gender}",
                            onchange: move |evt| gender.set(evt.value()),
                            option { value: "male", "男 " }
                            option { value: "female", "女 " }
                        }
                    }
                }
                button {
                    class: "submit-btn ",
                    onclick: on_submit,
                    disabled: loading(),
                    "排盘 "
                }
            }

            if loading() {
                div { class: "loading ", "排盘中... " }
            }

            if let Some(ref err) = *error.read() {
                div { class: "error-message ", "{err}" }
            }

            if let Some(ref data) = *result.read() {
                div { class: "result-card ",
                    h3 { "紫微斗数命盘 " }

                    if let Some(ming_zhu) = data.get("ming_zhu").and_then(|v| v.as_str()) {
                        div { class: "zw-info ",
                            span { "命主: " }
                            strong { {ming_zhu} }
                        }
                    }
                    if let Some(shen_zhu) = data.get("shen_zhu").and_then(|v| v.as_str()) {
                        div { class: "zw-info ",
                            span { "身主: " }
                            strong { {shen_zhu} }
                        }
                    }
                    if let Some(qi_yun) = data.get("qi_yun_age").and_then(|v| v.as_u64()) {
                        div { class: "zw-info ",
                            span { "起运年龄: " }
                            strong { "{qi_yun}岁 " }
                        }
                    }

                    // 四化
                    if let Some(si_hua) = data.get("si_hua") {
                        div { class: "si-hua ",
                            h4 { "四化 " }
                            div { class: "si-hua-grid ",
                                for (key, label) in [("hua_lu", "化禄 "), ("hua_quan", "化权 "), ("hua_ke", "化科 "), ("hua_ji", "化忌 ")] {
                                    if let Some(item) = si_hua.get(key).and_then(|v| v.as_array()) {
                                        if item.len() >= 2 {
                                            div { class: "si-hua-item ",
                                                span { class: "si-hua-label ", "{label}: " }
                                                span { {item[0].as_str().unwrap_or("? ")} }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 十二宫
                    if let Some(gongs) = data.get("gongs").and_then(|v| v.as_array()) {
                        div { class: "zw-gongs ",
                            h4 { "十二宫 " }
                            table { class: "data-table ",
                                thead {
                                    tr {
                                        th { "宫位 " }
                                        th { "地支 " }
                                        th { "主星 " }
                                        th { "辅星 " }
                                    }
                                }
                                tbody {
                                    for gong in gongs {
                                        tr {
                                            td { {gong.get("name").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            td { {gong.get("di_zhi").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                            td {
                                                if let Some(arr) = gong.get("zhu_xing").and_then(|v| v.as_array()) {
                                                    {arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", ") }
                                                }
                                            }
                                            td {
                                                if let Some(arr) = gong.get("fu_xing").and_then(|v| v.as_array()) {
                                                    {arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", ") }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 大限
                    if let Some(da_xian) = data.get("da_xian").and_then(|v| v.as_array()) {
                        if !da_xian.is_empty() {
                            div { class: "da-xian ",
                                h4 { "大限 " }
                                table { class: "data-table ",
                                    thead {
                                        tr {
                                            th { "宫位 " }
                                            th { "年龄 " }
                                            th { "主星 " }
                                        }
                                    }
                                    tbody {
                                        for dx in da_xian {
                                            tr {
                                                td { {dx.get("gong_name").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                                td { {fmt_age(dx.get("start_age").and_then(|v| v.as_u64()).unwrap_or(0), dx.get("end_age").and_then(|v| v.as_u64()).unwrap_or(0))} }
                                                td {
                                                    if let Some(arr) = dx.get("zhu_xing").and_then(|v| v.as_array()) {
                                                        {arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", ") }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ============ 数算 ============

#[component]
pub fn ShuSuan() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut active_tab = use_signal(|| "shaozi".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let tabs = [
        ("shaozi", "邵子神数 "),
        ("tieban", "铁板神数 "),
        ("beiji", "北极神数 "),
        ("nanji", "南极神数 "),
        ("cetian", "策天 "),
        ("chunzi", "春子 "),
        ("fendjing", "分经 "),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "datetime": datetime() }));
        let endpoint = match active_tab().as_str() {
            "shaozi" => "/shaozi/api",
            "tieban" => "/tieban/api",
            "beiji" => "/beiji/api",
            "nanji" => "/nanji/api",
            "cetian" => "/cetian/api",
            "chunzi" => "/chunzi/api",
            "fendjing" => "/fendjing/api",
            _ => "/shaozi/api",
        };
                spawn(async move {
            let fut = services::astro::api_request("POST", endpoint, Some(&*req));
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "数算 - 神数排盘 " }
            p { class: "page-desc ", "邵子神数, 铁板神数, 北极神数, 南极神数, 策天, 春子, 分经 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "出生日期时间 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons ",
                    for (key, label) in tabs.clone() {
                        button { class: if active_tab() == *key { "tab-btn active " } else { "tab-btn " },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "排盘 " }
            }
            if loading() { div { class: "loading ", "排盘中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "排盘结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

// ============ 三式 ============

#[component]
pub fn Sanshi() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut active_tab = use_signal(|| "qimen".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let tabs = [
        ("qimen", "奇门遁甲 "),
        ("taiyi", "太乙神数 "),
        ("liuren", "六壬 "),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "datetime": datetime() }));
        let endpoint = match active_tab().as_str() {
            "qimen" => "/qimen/api",
            "taiyi" => "/taiyi/api",
            "liuren" => "/liuren/api",
            _ => "/qimen/api",
        };
                spawn(async move {
            let fut = services::astro::api_request("POST", endpoint, Some(&*req));
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "三式合一 " }
            p { class: "page-desc ", "奇门, 太乙, 六壬三式整合排盘 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "日期时间 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons ",
                    for (key, label) in tabs.clone() {
                        button { class: if active_tab() == *key { "tab-btn active " } else { "tab-btn " },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "排盘 " }
            }
            if loading() { div { class: "loading ", "排盘中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "三式结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn Qimen() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);

    let on_submit = move |_| {
        loading.set(true);
        error.set(None);
        let req = std::sync::Arc::new(serde_json::json!({ "datetime": datetime() }));
                spawn(async move {
            let fut = services::sanshi::qimen(&*req);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(e) => { error.set(Some(e)); loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "奇门遁甲 " }
            p { class: "page-desc ", "输入日期时间,排奇门遁甲盘 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "日期时间 " }
                        input {
                            r#type: "datetime-local ",
                            value: "{datetime}",
                            oninput: move |evt| datetime.set(evt.value()),
                        }
                    }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "排盘 " }
            }
            if loading() { div { class: "loading ", "排盘中... " } }
            if let Some(ref err) = *error.read() { div { class: "error-message ", {err.clone()} } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ",
                    h3 { "奇门盘 " }
                    if let Some(ju) = data.get("ju") {
                        p { "用局: " {ju.as_str().unwrap_or("?")} }
                    }
                    if let Some(gongs) = data.get("gongs").and_then(|v| v.as_array()) {
                        table { class: "data-table ",
                            thead { tr { th { "宫 " } th { "八卦 " } th { "天盘 " } th { "地盘 " } th { "八门 " } th { "九星 " } th { "八神 " } } }
                            tbody {
                                for gong in gongs {
                                    tr {
                                        td { {gong.get("number").and_then(|v| v.as_u64()).unwrap_or(0).to_string()} }
                                        td { {gong.get("ba_gua").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                        td { {gong.get("tian_pan_gan").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                        td { {gong.get("di_pan_gan").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                        td { {gong.get("ba_men").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                        td { {gong.get("jiu_xing").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                        td { {gong.get("ba_shen").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Taiyi() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_calc = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "datetime": datetime() }));
                spawn(async move {
            let fut = services::sanshi::taiyi(&*req);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "太乙神数 " }
            p { class: "page-desc ", "太乙神数排盘:太乙十六神, 计神, 文昌, 始击, 主客大小 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "日期时间 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "排盘 " }
            }
            if loading() { div { class: "loading ", "排盘中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "太乙盘 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn Liuren() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_calc = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "datetime": datetime() }));
                spawn(async move {
            let fut = services::sanshi::liuren(&*req);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "六壬 " }
            p { class: "page-desc ", "大六壬排盘:天地盘, 四课, 三传, 遁干, 贵人 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "日期时间 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "排盘 " }
            }
            if loading() { div { class: "loading ", "排盘中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "六壬盘 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn Liuyao() -> Element {
    let mut coins = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_cast = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "coins": coins() }));
                spawn(async move {
            let fut = services::astro::api_request("POST", "/liuyao/cast ", Some(&*req));
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    let on_random = move |_| {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let c: String = (0..6).map(|_| (rng.gen_range(0..4) + 6).to_string()).collect::<Vec<_>>().join(",");
        coins.set(c);
    };

    rsx! {
        div { class: "page ",
            h2 { "六爻 " }
            p { class: "page-desc ", "六爻起卦:铜钱摇卦,输入六爻数值(6/7/8/9),逗号分隔 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "六爻铜钱数 " }
                        input { r#type: "text ", placeholder: "如: 6,7,8,6,9,7 ", value: "{coins}", oninput: move |evt| coins.set(evt.value()) }
                    }
                }
                div { class: "form-row ",
                    button { class: "submit-btn ", onclick: on_cast, disabled: loading(), "起卦 " }
                    button { class: "submit-btn secondary ", onclick: on_random, "随机 " }
                }
            }
            if loading() { div { class: "loading ", "起卦中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "卦象 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn Jieqi() -> Element {
    let mut year = use_signal(|| chrono::Local::now().year());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_query = move |_| {
        loading.set(true);
        let y = year();
                spawn(async move {
            let fut = services::calendar::get_jieqi(y);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "节气查询 " }
            p { class: "page-desc ", "查询二十四节气精确时刻 " }

            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "年份 " }
                        input {
                            r#type: "number ",
                            value: "{year}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<i32>() { year.set(v); }
                            },
                        }
                    }
                }
                button { class: "submit-btn ", onclick: on_query, disabled: loading(), "查询节气 " }
            }

            if loading() { div { class: "loading ", "查询中... " } }

            if let Some(ref data) = *result.read() {
                if let Some(list) = data.as_array() {
                    div { class: "result-card ",
                        h3 { "{year()}年 二十四节气 " }
                        div { class: "jieqi-grid ",
                            for (i, jq) in list.iter().enumerate() {
                                div { class: "jieqi-item ",
                                    div { class: "jieqi-name ",
                                        {jq.get("name_zh").and_then(|v| v.as_str()).unwrap_or("? ")}
                                    }
                                    div { class: "jieqi-date ",
                                        {jq.get("datetime").and_then(|v| v.as_str()).unwrap_or("? ")}
                                    }
                                    div { class: "jieqi-type ",
                                        {if jq.get("is_jie").and_then(|v| v.as_bool()).unwrap_or(false) { "节 " } else { "气 " }}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn FengShui() -> Element {
    let mut year = use_signal(|| chrono::Local::now().year());
    let mut gender = use_signal(|| "male".to_string());
    let mut build_year = use_signal(|| chrono::Local::now().year());
    let mut facing = use_signal(|| 0.0_f64);
    let mut active_tab = use_signal(|| "ming_gua".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_calc = move |_| {
        loading.set(true);
        let endpoint = if active_tab() == "ming_gua" { "/fengshui/ming-gua " } else { "/fengshui/flying-stars " };
        let req = std::sync::Arc::new(if active_tab() == "ming_gua" {
            serde_json::json!({ "year": year(), "gender": gender() })
        } else {
            serde_json::json!({ "build_year": build_year(), "facing": facing() })
        });
                spawn(async move {
            let fut = services::astro::api_request("POST", endpoint, Some(&*req));
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "风水 " }
            p { class: "page-desc ", "八宅命卦, 玄空飞星, 三元九运 " }
            div { class: "form-card ",
                div { class: "tab-buttons ",
                    button { class: if active_tab() == "ming_gua" { "tab-btn active " } else { "tab-btn " },
                        onclick: move |_| active_tab.set("ming_gua".to_string()), "八宅命卦 " }
                    button { class: if active_tab() == "flying_stars" { "tab-btn active " } else { "tab-btn " },
                        onclick: move |_| active_tab.set("flying_stars".to_string()), "玄空飞星 " }
                }
                if active_tab() == "ming_gua" {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "出生年份 " }
                            input { r#type: "number ", value: "{year}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<i32>() { year.set(v); } } } }
                        div { class: "form-group ", label { "性别 " }
                            select { value: "{gender}", onchange: move |evt| gender.set(evt.value()),
                                option { value: "male", "男 " } option { value: "female", "女 " } } }
                    }
                } else {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "建房年份 " }
                            input { r#type: "number ", value: "{build_year}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<i32>() { build_year.set(v); } } } }
                        div { class: "form-group ", label { "朝向(度) " }
                            input { r#type: "number ", step: "0.1 ", value: "{facing}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { facing.set(v); } } } }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "计算 " }
            }
            if loading() { div { class: "loading ", "计算中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "风水结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn DivinationOther() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut question = use_signal(|| String::new());
    let mut num1 = use_signal(|| 0u32);
    let mut num2 = use_signal(|| 0u32);
    let mut num3 = use_signal(|| 0u32);
    let mut seed = use_signal(|| 0u32);
    let mut di_fen = use_signal(|| "子".to_string());
    let mut active_tab = use_signal(|| "jinkou".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let tabs = [
        ("jinkou", "金口诀 "), ("jingjue", "荆诀 "), ("shenyishu", "神易数 "),
        ("wuzhao", "五兆 "), ("taixuan", "太玄 "), ("xianqin", "先秦占卜 "),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let tab = active_tab();
        let (endpoint, req) = if tab == "jinkou" {
            ("/jinkou/api", std::sync::Arc::new(serde_json::json!({ "datetime": datetime(), "di_fen": di_fen() })))
        } else if tab == "jingjue" {
            ("/jingjue/api", std::sync::Arc::new(serde_json::json!({ "birth": { "datetime": datetime() }, "query_year": chrono::Local::now().year() })))
        } else if tab == "shenyishu" {
            ("/shenyishu/api", std::sync::Arc::new(serde_json::json!({ "num1": num1(), "num2": num2(), "num3": num3() })))
        } else if tab == "wuzhao" {
            ("/wuzhao/api", std::sync::Arc::new(serde_json::json!({ "question": question() })))
        } else if tab == "taixuan" {
            ("/taixuan/api", std::sync::Arc::new(serde_json::json!({ "seed": seed() })))
        } else if tab == "xianqin" {
            ("/xianqin/divination", std::sync::Arc::new(serde_json::json!({ "seed": seed(), "method": "蓍草" })))
        } else {
            ("/jinkou/api", std::sync::Arc::new(serde_json::json!({})))
        };
                spawn(async move {
            let fut = services::astro::api_request("POST", endpoint, Some(&*req));
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "其他卜法 " }
            p { class: "page-desc ", "金口诀, 荆诀, 神易数, 五兆, 太玄, 先秦占卜 " }
            div { class: "form-card ",
                div { class: "tab-buttons ",
                    for (key, label) in tabs.clone() {
                        button { class: if active_tab() == *key { "tab-btn active " } else { "tab-btn " },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                if active_tab() == "jinkou" {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "日期时间 " }
                            input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                        div { class: "form-group ", label { "地分 " }
                            input { r#type: "text ", value: "{di_fen}", oninput: move |evt| di_fen.set(evt.value()) } }
                    }
                } else if active_tab() == "shenyishu" {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "数一 " }
                            input { r#type: "number ", value: "{num1}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num1.set(v); } } } }
                        div { class: "form-group ", label { "数二 " }
                            input { r#type: "number ", value: "{num2}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num2.set(v); } } } }
                        div { class: "form-group ", label { "数三 " }
                            input { r#type: "number ", value: "{num3}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num3.set(v); } } } }
                    }
                } else if active_tab() == "wuzhao" {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "问事 " }
                            input { r#type: "text ", value: "{question}", oninput: move |evt| question.set(evt.value()) } }
                    }
                } else if active_tab() == "taixuan" || active_tab() == "xianqin" {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "种子数 " }
                            input { r#type: "number ", value: "{seed}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { seed.set(v); } } } }
                    }
                } else {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "日期时间 " }
                            input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "推算 " }
            }
            if loading() { div { class: "loading ", "推算中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "推算结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

// ============ 工具 ============

#[component]
pub fn AiAnalysis() -> Element {
    let mut message = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_send = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "message": message() }));
                spawn(async move {
            let fut = services::ai::chat(&*req);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "AI 分析 " }
            p { class: "page-desc ", "多模型接入, 测试对话, 命理理解读 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "提问 " }
                        textarea { value: "{message}", oninput: move |evt| message.set(evt.value()),
                            placeholder: "输入命理分析问题... ", rows: "4 " } }
                }
                button { class: "submit-btn ", onclick: on_send, disabled: loading(), "发送 " }
            }
            if loading() { div { class: "loading ", "AI思考中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "AI 回答 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn Planetarium() -> Element {
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_query = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "latitude": latitude(), "longitude": longitude() }));
                spawn(async move {
            let fut = services::astro::planetarium_current(&*req);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "天文馆 " }
            p { class: "page-desc ", "实时天象:太阳星度, 月相, 可见行星位置 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "纬度 " }
                        input { r#type: "number ", step: "0.0001 ", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group ", label { "经度 " }
                        input { r#type: "number ", step: "0.0001 ", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                }
                button { class: "submit-btn ", onclick: on_query, disabled: loading(), "查询天象 " }
            }
            if loading() { div { class: "loading ", "查询中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "当前天象 " } pre { {data.to_string()} } }
            }
        }
    }
}

// ============ 万年历(黄历)============

#[component]
pub fn Almanac() -> Element {
    let mut year = use_signal(|| chrono::Local::now().year());
    let mut month = use_signal(|| chrono::Local::now().month());
    let mut day = use_signal(|| chrono::Local::now().day());
    let mut lunar_result = use_signal(|| None::<serde_json::Value>);
    let mut eclipse_result = use_signal(|| None::<serde_json::Value>);
    let mut ganzhi_result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);
    let mut active_tab = use_signal(|| "lunar".to_string());

    let on_solar_to_lunar = move |_| {
        loading.set(true);
        let y = year(); let m = month(); let d = day();
                spawn(async move {
            let fut = services::calendar::solar_to_lunar(y, m, d);
            match fut.await {
                Ok(data) => { lunar_result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    let on_eclipses = move |_| {
        loading.set(true);
        let y = year();
                spawn(async move {
            let fut = services::calendar::get_eclipses(y);
            match fut.await {
                Ok(data) => { eclipse_result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    let on_ganzhi = move |_| {
        loading.set(true);
        let y = year(); let m = month(); let d = day();
                spawn(async move {
            let fut = services::calendar::get_ganzhi(y, m, d);
            match fut.await {
                Ok(data) => { ganzhi_result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "黄历 - 万年历 " }
            p { class: "page-desc ", "寿星天文历----公历/农历/回历三历转换, 日月食, 干支节气 " }

            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "年 " }
                        input {
                            r#type: "number ",
                            value: "{year}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<i32>() { year.set(v); }
                            },
                        }
                    }
                    div { class: "form-group ",
                        label { "月 " }
                        input {
                            r#type: "number ",
                            min: "1 ",
                            max: "12 ",
                            value: "{month}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<u32>() { month.set(v); }
                            },
                        }
                    }
                    div { class: "form-group ",
                        label { "日 " }
                        input {
                            r#type: "number ",
                            min: "1 ",
                            max: "31 ",
                            value: "{day}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<u32>() { day.set(v); }
                            },
                        }
                    }
                }

                div { class: "tab-buttons ",
                    button {
                        class: if active_tab() == "lunar" { "tab-btn active " } else { "tab-btn " },
                        onclick: move |_| { active_tab.set("lunar".to_string()); },
                        "公历转农历 "
                    }
                    button {
                        class: if active_tab() == "ganzhi" { "tab-btn active " } else { "tab-btn " },
                        onclick: move |_| { active_tab.set("ganzhi".to_string()); },
                        "干支查询 "
                    }
                    button {
                        class: if active_tab() == "eclipse" { "tab-btn active " } else { "tab-btn " },
                        onclick: move |_| { active_tab.set("eclipse".to_string()); },
                        "日月食 "
                    }
                }

                div { class: "tab-content ",
                    if active_tab() == "lunar" {
                        div {
                            button {
                                class: "submit-btn ",
                                onclick: on_solar_to_lunar,
                                disabled: loading(),
                                "查询农历 "
                            }
                            if loading() { div { class: "loading ", "查询中... " } }
                            if let Some(ref data) = *lunar_result.read() {
                                div { class: "result-card lunar-card ",
                                    h3 { "农历转换结果 " }
                                    div { class: "lunar-info ",
                                        div { class: "lunar-row ",
                                            span { class: "lunar-label ", "农历日期: " }
                                            span { class: "lunar-value ",
                                                {data.get("year").and_then(|v| v.as_i64()).unwrap_or(0).to_string()} "年"
                                                {data.get("month_name_zh").and_then(|v| v.as_str()).unwrap_or("? ")}
                                                {data.get("day_name_zh").and_then(|v| v.as_str()).unwrap_or("? ")}
                                            }
                                        }
                                        div { class: "lunar-row ",
                                            span { class: "lunar-label ", "年干支 " }
                                            span { class: "lunar-value ", {data.get("year_ganzhi").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                        }
                                        div { class: "lunar-row ",
                                            span { class: "lunar-label ", "生肖: " }
                                            span { class: "lunar-value ", {data.get("zodiac_animal").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                        }
                                        if let Some(leap) = data.get("is_leap_month").and_then(|v| v.as_bool()) {
                                            if leap {
                                                div { class: "lunar-row ",
                                                    span { class: "lunar-label lunar-leap ", "(闰月) " }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if active_tab() == "ganzhi" {
                        div {
                            button {
                                class: "submit-btn ",
                                onclick: on_ganzhi,
                                disabled: loading(),
                                "查询干支 "
                            }
                            if loading() { div { class: "loading ", "查询中... " } }
                            if let Some(ref data) = *ganzhi_result.read() {
                                div { class: "result-card ",
                                    h3 { "干支信息 " }
                                    table { class: "data-table ",
                                        tbody {
                                            tr { td { "年干支 " } td { {data.get("year_ganzhi").and_then(|v| v.as_str()).unwrap_or("? ")} } }
                                            tr { td { "生肖 " } td { {data.get("zodiac").and_then(|v| v.as_str()).unwrap_or("? ")} } }
                                            tr { td { "年号 " } td { {data.get("nianhao").and_then(|v| v.as_str()).unwrap_or("? ")} } }
                                        }
                                    }
                                }
                            }
                        }
                    } else if active_tab() == "eclipse" {
                        div {
                            button {
                                class: "submit-btn ",
                                onclick: on_eclipses,
                                disabled: loading(),
                                "查询日月食 "
                            }
                            if loading() { div { class: "loading ", "查询中... " } }
                            if let Some(ref data) = *eclipse_result.read() {
                                if let Some(list) = data.as_array() {
                                    div { class: "result-card ",
                                        h3 { "{year()}年 日月食 " }
                                        if list.is_empty() {
                                            p { class: "empty-state ", "该年无日月食 " }
                                        } else {
                                            table { class: "data-table ",
                                                thead {
                                                    tr {
                                                        th { "日期 " }
                                                        th { "类型 " }
                                                        th { "食分 " }
                                                    }
                                                }
                                                tbody {
                                                    for eclipse in list {
                                                        tr {
                                                            td { {eclipse.get("date").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                                            td { {eclipse.get("eclipse_type").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                                            td { {format!("{:.3}", eclipse.get("magnitude").and_then(|v| v.as_f64()).unwrap_or(0.0))} }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ============ 其他页面 ============

#[component]
pub fn References() -> Element {
    rsx! {
        div { class: "page ",
            h2 { "辅助参考 " }
            p { class: "page-desc ", "八卦分类, 天干地支, 节气查询 " }
            div { class: "result-card ",
                h3 { "六十四卦 " }
                div { class: "ref-grid ",
                    for gua in &["乾 ", "坤 ", "屯 ", "蒙 ", "需 ", "讼 ", "师 ", "比 ", "小畜 ", "履 ", "泰 ", "否 ", "同人 ", "大有 ", "谦 ", "豫 ", "随 ", "蛊 ", "临 ", "观 ", "噬嗑 ", "贲 ", "剥 ", "复 ", "无妄 ", "大畜 ", "颐 ", "大过 ", "坎 ", "离 ", "咸 ", "恒 ", "遁 ", "大壮 ", "晋 ", "明夷 ", "家人 ", "睽 ", "蹇 ", "解 ", "损 ", "益 ", "夬 ", "姤 ", "萃 ", "升 ", "困 ", "井 ", "革 ", "鼎 ", "震 ", "艮 ", "渐 ", "归妹 ", "丰 ", "旅 ", "巽 ", "兑 ", "涣 ", "节 ", "中孚 ", "小过 ", "既济 ", "未济 "] {
                        div { class: "ref-item ", "{gua}" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Settings() -> Element {
    let mut theme = use_signal(|| "light".to_string());
    let mut language = use_signal(|| "zh".to_string());
    let mut saved = use_signal(|| false);

    let on_save = move |_| {
        saved.set(true);
    };

    rsx! {
        div { class: "page ",
            h2 { "设置 " }
            p { class: "page-desc ", "应用设置 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "主题 " }
                        select { value: "{theme}", onchange: move |evt| theme.set(evt.value()),
                            option { value: "light", "浅色 " } option { value: "dark", "深色 " } } }
                    div { class: "form-group ", label { "语言 " }
                        select { value: "{language}", onchange: move |evt| language.set(evt.value()),
                            option { value: "zh", "中文 " } option { value: "en", "English " } } }
                }
                button { class: "submit-btn ", onclick: on_save, "保存设置 " }
                if saved() { div { class: "success-msg ", "设置已保存 " } }
            }
        }
    }
}

#[component]
pub fn GuoLao() -> Element {
    rsx! {
        div { class: "page ",
            h2 { "果老星宗 " }
            p { class: "page-desc ", "果老星宗推演, 二十八宿命度身度 " }
            p { "请使用七政四余页面进行排盘,果老星宗与七政四余使用同一计算引擎. " }
        }
    }
}

#[component]
pub fn GuaZhan() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut num1 = use_signal(|| 0u32);
    let mut num2 = use_signal(|| 0u32);
    let mut num3 = use_signal(|| 0u32);
    let mut active_tab = use_signal(|| "meihua".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_calc = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(if active_tab() == "meihua" {
            serde_json::json!({ "num1": num1(), "num2": num2(), "num3": num3() })
        } else {
            serde_json::json!({ "datetime": datetime() })
        });
        let endpoint = if active_tab() == "meihua" { "/gua/meihua " } else { "/gua/meiyi " };
                spawn(async move {
            let fut = services::astro::api_request("POST", endpoint, Some(&*req));
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "卦占 " }
            p { class: "page-desc ", "梅花易数, 六爻卦占 " }
            div { class: "form-card ",
                div { class: "tab-buttons ",
                    button { class: if active_tab() == "meihua" { "tab-btn active " } else { "tab-btn " },
                        onclick: move |_| active_tab.set("meihua".to_string()), "梅花易数 " }
                    button { class: if active_tab() == "meiyi" { "tab-btn active " } else { "tab-btn " },
                        onclick: move |_| active_tab.set("meiyi".to_string()), "六爻占 " }
                }
                if active_tab() == "meihua" {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "数一 " }
                            input { r#type: "number ", value: "{num1}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num1.set(v); } } } }
                        div { class: "form-group ", label { "数二 " }
                            input { r#type: "number ", value: "{num2}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num2.set(v); } } } }
                        div { class: "form-group ", label { "数三 " }
                            input { r#type: "number ", value: "{num3}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num3.set(v); } } } }
                    }
                } else {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "日期时间 " }
                            input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "起卦 " }
            }
            if loading() { div { class: "loading ", "起卦中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "卦象 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn DunJia() -> Element {
    rsx! {
        div { class: "page ",
            h2 { "遁甲 " }
            p { class: "page-desc ", "遁甲包括:青龙遁, 白虎遁等 " }
            p { "请使用奇门遁甲页面进行排盘,遁甲与奇门使用同一计算引擎. " }
        }
    }
}

#[component]
pub fn Gua() -> Element {
    let mut gua_seq = use_signal(|| 0u32);
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_query = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "seq": gua_seq() }));
                spawn(async move {
            let fut = services::astro::api_request("POST", "/gua/desc ", Some(&*req));
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "卦象 " }
            p { class: "page-desc ", "六十四卦, 卦象关系, 卦辞爻辞 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "卦序 (0-63) " }
                        input { r#type: "number ", min: "0 ", max: "63 ", value: "{gua_seq}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { gua_seq.set(v); } } } }
                }
                button { class: "submit-btn ", onclick: on_query, disabled: loading(), "查询 " }
            }
            if loading() { div { class: "loading ", "查询中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "卦象详情 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "page about-page ",
            h2 { "关于Divines " }
            p { "版本: 0.1.0 (基于 Rust 重写) " }
            p { "Divines 是一个全面的玄学工作站. " }
            p { "西方占星的本命, 推运, 关系盘,连同八字, 紫微, 奇门, 六壬, 太乙这些中国传统术数,被放进同一个应用里. " }
            p { "最新版本基于 Rust 全栈重写,前端使用 Dioxus 0.7.9. " }
            p { "原项目地址: https://github.com/Horace-Maxwell/divines-Web-App-comprehensively-improved-MacOS " }
            p { "万年历参考: 寿星天文历(sxwnl) " }
            p { "许可: AGPL-3.0 " }
        }
    }
}

// ============ 传统术数 - 数算与神数 ============

#[component]
pub fn HuangJi() -> Element {
    let mut year = use_signal(|| chrono::Local::now().year());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "year": year() }));
                spawn(async move {
            let fut = services::astro::huangji_yuan_hui(&*req);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "皇极经世 " }
            p { class: "page-desc ", "皇极经世元会运世推算,值年卦, 值事卦 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "年份 " }
                        input {
                            r#type: "number ",
                            value: "{year}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<i32>() { year.set(v); }
                            },
                        }
                    }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "推算 " }
            }
            if loading() { div { class: "loading ", "推算中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ",
                    h3 { "皇极经世结果 " }
                    pre { {data.to_string()} }
                }
            }
        }
    }
}

#[component]
pub fn JingJue() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut query_year = use_signal(|| chrono::Local::now().year());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "birth": { "datetime": datetime() }, "query_year": query_year() }));
                spawn(async move {
            let fut = services::astro::jingjue_calculate(&*req);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "荆诀 " }
            p { class: "page-desc ", "荆诀流年推演:以出生时间推算各年运势 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "出生日期时间 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                    div { class: "form-group ", label { "查询年份 " }
                        input { r#type: "number ", value: "{query_year}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<i32>() { query_year.set(v); } } } }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "推算 " }
            }
            if loading() { div { class: "loading ", "推算中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "荆诀推演结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn JinKou() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut di_fen = use_signal(|| "子".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "datetime": datetime(), "di_fen": di_fen() }));
                spawn(async move {
            let fut = services::astro::jinkou_calculate(&*req);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "金口诀 " }
            p { class: "page-desc ", "金口诀排盘:月将, 地分, 将神, 贵神, 人元 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "日期时间 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                    div { class: "form-group ", label { "地分 " }
                        input { r#type: "text ", placeholder: "子丑寅卯辰巳午未申酉戌亥 ", value: "{di_fen}", oninput: move |evt| di_fen.set(evt.value()) } }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "排盘 " }
            }
            if loading() { div { class: "loading ", "排盘中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "金口诀排盘结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn ShenYiShu() -> Element {
    let mut num1 = use_signal(|| 0u32);
    let mut num2 = use_signal(|| 0u32);
    let mut num3 = use_signal(|| 0u32);
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "num1": num1(), "num2": num2(), "num3": num3() }));
                spawn(async move {
            let fut = services::astro::shenyishu_calculate(&*req);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "神易数 " }
            p { class: "page-desc ", "神易数三数起卦:以三个数字起卦推断吉凶 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "数一 " }
                        input { r#type: "number ", value: "{num1}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num1.set(v); } } } }
                    div { class: "form-group ", label { "数二 " }
                        input { r#type: "number ", value: "{num2}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num2.set(v); } } } }
                    div { class: "form-group ", label { "数三 " }
                        input { r#type: "number ", value: "{num3}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num3.set(v); } } } }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "起卦 " }
            }
            if loading() { div { class: "loading ", "起卦中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "神易数结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn WuZhao() -> Element {
    let mut question = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "question": question() }));
                spawn(async move {
            let fut = services::astro::wuzhao_calculate(&*req);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "五兆 " }
            p { class: "page-desc ", "五兆卜占卜:以问事为引,推演五行兆象 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "问事 " }
                        input { r#type: "text ", placeholder: "输入您想问的事... ", value: "{question}", oninput: move |evt| question.set(evt.value()) } }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "占卜 " }
            }
            if loading() { div { class: "loading ", "占卜中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "五兆占卜结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn TaiXuan() -> Element {
    let mut seed = use_signal(|| 0u32);
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "seed": seed() }));
                spawn(async move {
            let fut = services::astro::taixuan_calculate(&*req);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "太玄 " }
            p { class: "page-desc ", "太玄经法:随机推算,81首729赞 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "种子数 " }
                        input { r#type: "number ", value: "{seed}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { seed.set(v); } } } }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "推算 " }
            }
            if loading() { div { class: "loading ", "推算中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "太玄结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn BeiJi() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = std::sync::Arc::new(serde_json::json!({ "datetime": datetime() }));
                spawn(async move {
            let fut = services::astro::beiji_calculate(&*req);
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page ",
            h2 { "北极神数 " }
            p { class: "page-desc ", "北极神数排盘:八字推算, 六爻定位, 神数条文 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "出生日期时间 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "排盘 " }
            }
            if loading() { div { class: "loading ", "排盘中... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "北极神数排盘结果 " } pre { {data.to_string()} } }
            }
        }
    }
}

#[component]
pub fn CeTian() -> Element {
    VNode::empty()
}

#[component]
pub fn ChunZi() -> Element {
    VNode::empty()
}

#[component]
pub fn FenJing() -> Element {
    VNode::empty()
}

#[component]
pub fn NanJi() -> Element {
    VNode::empty()
}

#[component]
pub fn ShaoZi() -> Element {
    VNode::empty()
}

#[component]
pub fn TieBan() -> Element {
    VNode::empty()
}

#[component]
pub fn XianQin() -> Element {
    VNode::empty()
}

// ============ 西方占星 - 专项 ============

#[component]
pub fn AstroHellenistic() -> Element {
    VNode::empty()
}

#[component]
pub fn AstroHorary() -> Element {
    VNode::empty()
}

#[component]
pub fn AstroElectional() -> Element {
    VNode::empty()
}

#[component]
pub fn AstroMundane() -> Element {
    VNode::empty()
}

#[component]
pub fn AstroGermany() -> Element {
    VNode::empty()
}

#[component]
pub fn AstroSynastry() -> Element {
    VNode::empty()
}

#[component]
pub fn AstroAcg() -> Element {
    VNode::empty()
}

#[component]
pub fn AstroRectification() -> Element {
    VNode::empty()
}

// ============ 工具 - 骰子 / 二十八宿 ============

#[component]
pub fn Dice() -> Element {
    VNode::empty()
}

#[component]
pub fn Su28() -> Element {
    VNode::empty()
}

// ============ 邵子系列 ============

#[component]
pub fn SzBaGua() -> Element {
    VNode::empty()
}

#[component]
pub fn SzDunJia() -> Element {
    VNode::empty()
}

#[component]
pub fn SzTaiYi() -> Element {
    VNode::empty()
}

// ============ 邵子扩展 ============

#[component]
pub fn SzFangWei() -> Element {
    VNode::empty()
}

#[component]
pub fn SzFengYe() -> Element {
    VNode::empty()
}

#[component]
pub fn SzNiXiang() -> Element {
    VNode::empty()
}

#[component]
pub fn SzSign() -> Element {
    VNode::empty()
}

// ============ 命理其他 ============

#[component]
pub fn MingOther() -> Element {
    VNode::empty()
}

// ============ 宿占 ============

#[component]
pub fn SuZhan() -> Element {
    VNode::empty()
}

// ============ 通涉法 ============

#[component]
pub fn TongSheFa() -> Element {
    VNode::empty()
}

// ============ 其他 ============

#[component]
pub fn OtherBu() -> Element {
    VNode::empty()
}

// ============ 404 ============

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "page not-found",
            h2 { "404 - Page Not Found" }
            p { "The page you requested was not found." }
        }
    }
}