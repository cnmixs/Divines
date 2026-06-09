// Divines - 椤甸潰妯″潡
// 鍙傝€冨師椤圭洰: astrostudyui/src/pages/

pub mod home;
pub use home::*;

use dioxus::prelude::*;
use dioxus::signals::*;
use crate::Route;
use crate::services;
use chrono::Datelike;

// ============ 杈呭姪鍑芥暟 ============

fn fmt_deg(v: f64) -> String { format!("{:.2}掳", v) }
fn fmt_age(start: u64, end: u64) -> String { format!("{}-{}宀?, start, end) }
fn fmt_year(start: i64, end: i64) -> String { format!("{}-{}", start, end) }
fn fmt_mag(v: f64) -> String { format!("{:.3}", v) }
fn fmt_hour(v: f64) -> String { format!("{:.2}鏃?, v) }

/// 閫氱敤 JSON 娓叉煋鍑芥暟: 灏?JSON Value 娓叉煋涓虹粨鏋勫寲 HTML 缁勪欢
fn json_render(data: &serde_json::Value) -> Element {
    match data {
        serde_json::Value::Object(obj) => {
            let keys: Vec<&String> = obj.keys().collect();
            rsx! {
                table { class: "data-table",
                    tbody {
                        for key in keys {
                            if let Some(val) = obj.get(key) {
                                tr {
                                    td {
                                        strong { style: "color: var(--divines-muted); font-size: 12px;",
                                            "{key}"
                                        }
                                    }
                                    td { {json_render(val)} }
                                }
                            }
                        }
                    }
                }
            }
        }
        serde_json::Value::Array(arr) => {
            if arr.is_empty() {
                rsx! { span { class: "empty-state", "(鏃犳暟鎹?" } }
            } else if arr.first().and_then(|v| v.as_object()).is_some() {
                let first_keys: Vec<String> = arr.first().unwrap().as_object().unwrap().keys().cloned().collect();
                rsx! {
                    table { class: "data-table",
                        thead {
                            tr {
                                for key in &first_keys {
                                    th { "{key}" }
                                }
                            }
                        }
                        tbody {
                            for item in arr {
                                tr {
                                    for key in &first_keys {
                                        td { {json_render(&item.get(key).cloned().unwrap_or(serde_json::Value::Null))} }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                rsx! {
                    ul {
                        for item in arr {
                            li { {json_render(item)} }
                        }
                    }
                }
            }
        }
        serde_json::Value::String(s) => {
            rsx! { span { "{s}" } }
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                rsx! { span { "{i}" } }
            } else {
                rsx! { span { "{n}" } }
            }
        }
        serde_json::Value::Bool(b) => {
            rsx! { span { if *b { "鏄? } else { "鍚? } } }
        }
        _ => rsx! { span { "-" } },
    }
}

// ============ 鍗犳槦鏈懡鐩?===========

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
            h2 { "鍗犳槦鏈懡鐩?" }
            p { class: "page-desc ", "杈撳叆鍑虹敓淇℃伅,璁＄畻瑗挎磱鍗犳槦鏈懡鐩?" }

            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "鍑虹敓鏃ユ湡鏃堕棿 " }
                        input {
                            r#type: "datetime-local ",
                            value: "{datetime}",
                            oninput: move |evt| datetime.set(evt.value()),
                        }
                    }
                    div { class: "form-group ",
                        label { "濮撳悕 " }
                        input {
                            r#type: "text ",
                            placeholder: "鍙€?,
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value()),
                        }
                    }
                }
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "绾害 " }
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
                        label { "缁忓害 " }
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
                        label { "鏃跺尯 " }
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
                        label { "鍦扮偣 " }
                        input {
                            r#type: "text ",
                            placeholder: "濡?鍖椾含 ",
                            value: "{place_name}",
                            oninput: move |evt| place_name.set(evt.value()),
                        }
                    }
                    div { class: "form-group ",
                        label { "鎬у埆 " }
                        select {
                            value: "{gender}",
                            onchange: move |evt| gender.set(evt.value()),
                            option { value: "male", "鐢?" }
                            option { value: "female", "濂?" }
                        }
                    }
                }
                button {
                    class: "submit-btn ",
                    onclick: on_submit,
                    disabled: loading(),
                    "璁＄畻鏈懡鐩?"
                }
            }

            if loading() {
                div { class: "loading ", "璁＄畻涓?.. " }
            }

            if let Some(ref err) = *error.read() {
                div { class: "error-message ", "{err}" }
            }

            if let Some(ref data) = *result.read() {
                div { class: "result-card ",
                    h3 { "鏄熺洏缁撴灉 " }
                    if let Some(planets) = data.get("planets").and_then(|v| v.as_array()) {
                        div { class: "planet-list ",
                            h4 { "琛屾槦浣嶇疆 " }
                            table { class: "data-table ",
                                thead {
                                    tr {
                                        th { "琛屾槦 " }
                                        th { "鏄熷骇 " }
                                        th { "搴︽暟 " }
                                        th { "瀹綅 " }
                                        th { "閫嗚 " }
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
                            h4 { "鐩镐綅 ({aspects.len()})" }
                            table { class: "data-table ",
                                thead {
                                    tr {
                                        th { "琛屾槦1 " }
                                        th { "琛屾槦2 " }
                                        th { "鐩镐綅 " }
                                        th { "瑙掑害 " }
                                        th { "瀹硅搴?" }
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

// ============ 鏄熻繍鎺ㄨ繍 ============

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
        ("solar_arc", "澶槼寮?"),
        ("progressions", "娆￠檺娉?"),
        ("primary_dir", "涓婚檺娉?"),
        ("profections", "灏忛檺 "),
        ("firdaria", "娉曡揪鏄熼檺 "),
        ("age_point", "骞撮緞鎺ㄨ繘鐐?"),
        ("symbolic_dir", "娉㈡湡鍚戣繍 "),
        ("term_dir", "鐣岄檺娉?"),
        ("thirteenth", "绗崄涓夋湀鐩?"),
        ("harmonic", "璋冩尝鐩?"),
        ("draconic", "榫欑洏 "),
        ("year_129", "129骞寸郴缁?"),
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
            h2 { "鏄熻繍 - 鎺ㄨ繍 " }
            p { class: "page-desc ", "澶槼寮? 娆￠檺娉? 涓婚檺娉? 娉曡揪鏄熼檺, 灏忛檺绛夋帹杩愮郴缁?" }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "鍑虹敓鏃ユ湡鏃堕棿 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) }
                    }
                }
                div { class: "form-row ",
                    div { class: "form-group ", label { "绾害 " }
                        input { r#type: "number ", step: "0.0001 ", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group ", label { "缁忓害 " }
                        input { r#type: "number ", step: "0.0001 ", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                    div { class: "form-group ", label { "鏃跺尯 " }
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
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "璁＄畻 " }
            }
            if loading() { div { class: "loading ", "璁＄畻涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "鎺ㄨ繍缁撴灉 " } {json_render(data)} }
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
        ("synastry", "姣旇緝鐩?"),
        ("composite", "缁勫悎鐩?"),
        ("time_space", "鏃剁┖涓偣鐩?"),
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
            h2 { "鍚堢洏 - 鍏崇郴鐩?" }
            p { class: "page-desc ", "姣旇緝鐩? 缁勫悎鐩? 鏃剁┖涓偣鐩樺垎鏋?" }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "鍐呯洏鍑虹敓鏃堕棿 " }
                        input { r#type: "datetime-local ", value: "{inner_datetime}", oninput: move |evt| inner_datetime.set(evt.value()) } }
                }
                div { class: "form-row ",
                    div { class: "form-group ", label { "澶栫洏鍑虹敓鏃堕棿 " }
                        input { r#type: "datetime-local ", value: "{outer_datetime}", oninput: move |evt| outer_datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons ",
                    for (key, label) in tabs.clone() {
                        button { class: if active_tab() == *key { "tab-btn active " } else { "tab-btn " },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "璁＄畻鍚堢洏 " }
            }
            if loading() { div { class: "loading ", "璁＄畻涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "鍚堢洏缁撴灉 " } {json_render(data)} }
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
        ("arabic", "闃挎媺浼偣 "),
        ("aspects", "鐩镐綅璇︽儏 "),
        ("decennials", "鍗佸勾杩?"),
        ("dispositor", "鏈€缁堝畾浣嶆槦 "),
        ("lots", "鐗规畩鐐?"),
        ("zr", "榛勯亾鏄熼噴 "),
        ("return", "鍥炲綊鐩?"),
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
            h2 { "杈呯洏 - 涓撻」鍒嗘瀽 " }
            p { class: "page-desc ", "闃挎媺浼偣, 鐩镐綅, 鏄熼噴, 鍥炲綊鐩樼瓑涓撻」鍒嗘瀽 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "鍑虹敓鏃ユ湡鏃堕棿 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons ",
                    for (key, label) in tabs.clone() {
                        button { class: if active_tab() == *key { "tab-btn active " } else { "tab-btn " },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "璁＄畻 " }
            }
            if loading() { div { class: "loading ", "璁＄畻涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "鍒嗘瀽缁撴灉 " } {json_render(data)} }
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
        ("chart", "鍗板害鐩?"),
        ("dasha", "澶ц繍 "),
        ("yogas", "鏍煎眬 "),
        ("nakshatra", "27瀹?"),
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
            h2 { "鍗板害鍗犳槦 - Vedic " }
            p { class: "page-desc ", "鍖呭惈涓滃嵃搴︾洏, 鎭掓槦榛勯亾, 澶ц繍绯荤粺, 27瀹?" }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "鍑虹敓鏃ユ湡鏃堕棿 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons ",
                    for (key, label) in tabs.clone() {
                        button { class: if active_tab() == *key { "tab-btn active " } else { "tab-btn " },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "璁＄畻 " }
            }
            if loading() { div { class: "loading ", "璁＄畻涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "鍗板害鍗犳槦缁撴灉 " } {json_render(data)} }
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
            h2 { "涓冩斂鍥涗綑 - 鏋滆€佹槦瀹?" }
            p { class: "page-desc ", "杈撳叆鍑虹敓淇℃伅,鎺掍竷鏀垮洓浣欐槦鐩?鍚?8瀹? 鍛藉害韬害, 娲炲井澶ч檺, 鏋滆€佹牸灞€ " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "鍑虹敓鏃ユ湡鏃堕棿 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) }
                    }
                }
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "绾害 " }
                        input { r#type: "number ", step: "0.0001 ", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } }
                    }
                    div { class: "form-group ",
                        label { "缁忓害 " }
                        input { r#type: "number ", step: "0.0001 ", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } }
                    }
                    div { class: "form-group ",
                        label { "鏃跺尯 " }
                        input { r#type: "number ", step: "0.5 ", value: "{timezone}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { timezone.set(v); } } }
                    }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "鎺掔洏 " }
            }
            if loading() { div { class: "loading ", "鎺掔洏涓?.. " } }
            if let Some(ref err) = *error.read() { div { class: "error-message ", "{err}" } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ",
                    h3 { "涓冩斂鍥涗綑鏄熺洏 " }
                    if let Some(planets) = data.get("planets").and_then(|v| v.as_array()) {
                        div { class: "section ",
                            h4 { "琛屾槦浣嶇疆 " }
                            table { class: "data-table ",
                                thead { tr { th { "琛屾槦 " } th { "榛勭粡 " } th { "鏄熷 " } th { "瀹綅 " } th { "28瀹?" } th { "閫嗚 " } } }
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
                            h4 { "鍗佷簩瀹?" }
                            table { class: "data-table ",
                                thead { tr { th { "瀹綅 " } th { "瀹悕 " } th { "鏄熷骇 " } th { "搴︽暟 " } } }
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
                                h4 { "鏍煎眬 " }
                                ul { for p in patterns { li { {p.as_str().unwrap_or("? ")} } } }
                            }
                        }
                    }
                    if let Some(dongwei) = data.get("dong_wei").and_then(|v| v.as_array()) {
                        if !dongwei.is_empty() {
                            div { class: "section ",
                                h4 { "娲炲井澶ч檺 " }
                                table { class: "data-table ",
                                    thead { tr { th { "骞撮檺 " } th { "瀹綅 " } th { "璇存槑 " } } }
                                    tbody {
                                        for dw in dongwei {
                                            tr {
                                                td { {format!("{}-{}宀?", dw.get("start_age").and_then(|v| v.as_u64()).unwrap_or(0).to_string(), dw.get("end_age").and_then(|v| v.as_u64()).unwrap_or(0).to_string())} }
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

// ============ 鍏瓧鎺掔洏 ============

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
            h2 { "鍏瓧鎺掔洏 " }
            p { class: "page-desc ", "杈撳叆鍑虹敓鏃ユ湡鏃堕棿,鎺掑洓鏌卞叓瀛? 鍗佺, 澶ц繍,鏀寔鐪熷お闃虫椂, 鏃╂櫄瀛愭椂, 骞虫皵/瀹氭皵 " }

            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "鍑虹敓鏃ユ湡鏃堕棿 " }
                        input {
                            r#type: "datetime-local ",
                            value: "{datetime}",
                            oninput: move |evt| datetime.set(evt.value()),
                        }
                    }
                    div { class: "form-group ",
                        label { "濮撳悕(鍙€? " }
                        input {
                            r#type: "text ",
                            placeholder: "鍙€?,
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value()),
                        }
                    }
                    div { class: "form-group ",
                        label { "鎬у埆 " }
                        select {
                            value: "{gender}",
                            onchange: move |evt| gender.set(evt.value()),
                            option { value: "male", "鐢?" }
                            option { value: "female", "濂?" }
                        }
                    }
                }

                // 鎺掔洏閫夐」
                div { class: "options-section ",
                    h4 { "鎺掔洏閫夐」 " }
                    div { class: "options-grid ",
                        div { class: "option-item ",
                            label { class: "option-label ",
                                input {
                                    r#type: "checkbox ",
                                    checked: use_true_solar(),
                                    onchange: move |evt| use_true_solar.set(evt.value() == "true "),
                                }
                                span { "鐪熷お闃虫椂 " }
                            }
                        }
                        div { class: "option-item ",
                            label { class: "option-label ",
                                input {
                                    r#type: "checkbox ",
                                    checked: use_early_late_zi(),
                                    onchange: move |evt| use_early_late_zi.set(evt.value() == "true "),
                                }
                                span { "鍖哄垎鏃╂櫄瀛愭椂 " }
                            }
                        }
                        div { class: "option-item ",
                            label { class: "option-label ",
                                input {
                                    r#type: "checkbox ",
                                    checked: use_ding_qi(),
                                    onchange: move |evt| use_ding_qi.set(evt.value() == "true "),
                                }
                                span { "瀹氭皵娉?" }
                            }
                            span { class: "option-hint ", "(鍙栨秷閫夋嫨涓哄钩姘旀硶) " }
                        }
                        div { class: "form-group form-group-inline ",
                            label { "缁忓害: " }
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
                    "鎺掔洏 "
                }
            }

            if loading() {
                div { class: "loading ", "鎺掔洏涓?.. " }
            }

            if let Some(ref err) = *error.read() {
                div { class: "error-message ", "{err}" }
            }

            if let Some(ref data) = *result.read() {
                div { class: "result-card ",
                    h3 { "鍏瓧鎺掔洏缁撴灉 " }

                    // 鍥涙煴
                    div { class: "bazi-pillars ",
                        h4 { "鍥涙煴 " }
                        div { class: "pillar-grid ",
                            for pillar_key in ["year ", "month ", "day ", "hour"] {
                                div { class: "pillar-item ",
                                    div { class: "pillar-label ",
                                        {match pillar_key {
                                            "year" => "骞存煴 ",
                                            "month" => "鏈堟煴 ",
                                            "day" => "鏃ユ煴 ",
                                            "hour" => "鏃舵煴 ",
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

                    // 鏃ヤ富
                    if let Some(dm) = data.get("day_master").and_then(|v| v.as_str()) {
                        div { class: "day-master ",
                            span { "鏃ヤ富: " }
                            strong { "{dm}" }
                        }
                    }
                    if let Some(adj_hour) = data.get("adjusted_hour").and_then(|v| v.as_f64()) {
                        div { class: "adjusted-hour ",
                            span { "鏍℃鏃?" }
                            span { "{adj_hour:.2}鏃?" }
                        }
                    }

                    // 鍗佺 
                    if let Some(ten_gods) = data.get("ten_gods") {
                        div { class: "ten-gods ",
                            h4 { "鍗佺 " }
                            div { class: "ten-god-grid ",
                                for (key, label) in [("year", "骞?"), ("month", "鏈?"), ("day", "鏃?"), ("hour", "鏃?")] {
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

                    // 闀跨敓鍗佷簩绁?
                    if let Some(chang_sheng) = data.get("chang_sheng") {
                        div { class: "chang-sheng ",
                            h4 { "闀跨敓鍗佷簩绁?" }
                            div { class: "chang-sheng-grid ",
                                for (key, label) in [("year", "骞?"), ("month", "鏈?"), ("day", "鏃?"), ("hour", "鏃?")] {
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

                    // 绾抽煶
                    if let Some(na_yin) = data.get("na_yin") {
                        div { class: "na-yin ",
                            h4 { "绾抽煶 " }
                            div { class: "na-yin-grid ",
                                for (key, label) in [("year", "骞?"), ("month", "鏈?"), ("day", "鏃?"), ("hour", "鏃?")] {
                                    div { class: "na-yin-item ",
                                        span { "{label}: " }
                                        span { {na_yin.get(key).and_then(|v| v.as_str()).unwrap_or("? ")} }
                                    }
                                }
                            }
                        }
                    }

                    // 钘忓共
                    if let Some(hidden) = data.get("hidden_stems") {
                        div { class: "hidden-stems ",
                            h4 { "钘忓共 " }
                            div { class: "ten-god-grid ",
                                for (key, label) in [("year", "骞?"), ("month", "鏈?"), ("day", "鏃?"), ("hour", "鏃?")] {
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

                    // 骞叉敮鍒戝啿鍚堝害 
                    if let Some(relations) = data.get("relations").and_then(|v| v.as_array()) {
                        if !relations.is_empty() {
                            div { class: "relations ",
                                h4 { "骞叉敮鍒戝啿鍚堝害 " }
                                table { class: "data-table ",
                                    thead {
                                        tr {
                                            th { "绫诲瀷 " }
                                            th { "娑夊強鏌?" }
                                            th { "璇︽儏 " }
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

                    // 绁炵厼
                    if let Some(shen_sha) = data.get("shen_sha").and_then(|v| v.as_array()) {
                        if !shen_sha.is_empty() {
                            div { class: "shen-sha ",
                                h4 { "绁炵厼 " }
                                table { class: "data-table ",
                                    thead {
                                        tr {
                                            th { "绁炵厼 " }
                                            th { "浣嶇疆 " }
                                            th { "璇存槑 " }
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

                    // 澶ц繍
                    if let Some(qi_yun) = data.get("qi_yun_time").and_then(|v| v.as_str()) {
                        div { class: "qi-yun ",
                            h4 { "璧疯繍鏃堕棿 " }
                            p { "{qi_yun}" }
                        }
                    }

                    if let Some(da_yun) = data.get("da_yun").and_then(|v| v.as_array()) {
                        if !da_yun.is_empty() {
                            div { class: "da-yun ",
                                h4 { "澶ц繍 " }
                                table { class: "data-table ",
                                    thead {
                                        tr {
                                            th { "骞撮緞 " }
                                            th { "澶╁共 " }
                                            th { "鍦版敮 " }
                                            th { "鍗佺 " }
                                            th { "骞翠唤 " }
                                        }
                                    }
                                    tbody {
                                        for dy in da_yun {
                                            tr {
                                                td { {format!("{}-{}宀?", dy.get("start_age").and_then(|v| v.as_u64()).unwrap_or(0).to_string(), dy.get("end_age").and_then(|v| v.as_u64()).unwrap_or(0).to_string())} }
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

                    // 鎺掔洏閫夐」鍥炴樉
                    if let Some(options) = data.get("options") {
                        div { class: "options-display ",
                            h4 { "閫夐」 " }
                            div { class: "options-display-grid ",
                                span { "鐪熷お闃虫椂: " {options.get("use_true_solar_time").and_then(|v| v.as_bool()).unwrap_or(false).to_string()} }
                                span { "鏃╂櫄瀛愭椂: " {options.get("use_early_late_zi").and_then(|v| v.as_bool()).unwrap_or(false).to_string()} }
                                span { "瀹氭皵娉? " {options.get("use_ding_qi").and_then(|v| v.as_bool()).unwrap_or(true).to_string()} }
                                span { "缁忓害: " {options.get("longitude").and_then(|v| v.as_f64()).unwrap_or(0.0).to_string()} }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ============ 绱井鏂楁暟 ============

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
            h2 { "绱井鏂楁暟 " }
            p { class: "page-desc ", "杈撳叆鍑虹敓鏃ユ湡鏃堕棿,鎺掔传寰枟鏁板懡鐩?" }

            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "鍑虹敓鏃ユ湡鏃堕棿 " }
                        input {
                            r#type: "datetime-local ",
                            value: "{datetime}",
                            oninput: move |evt| datetime.set(evt.value()),
                        }
                    }
                    div { class: "form-group ",
                        label { "鎬у埆 " }
                        select {
                            value: "{gender}",
                            onchange: move |evt| gender.set(evt.value()),
                            option { value: "male", "鐢?" }
                            option { value: "female", "濂?" }
                        }
                    }
                }
                button {
                    class: "submit-btn ",
                    onclick: on_submit,
                    disabled: loading(),
                    "鎺掔洏 "
                }
            }

            if loading() {
                div { class: "loading ", "鎺掔洏涓?.. " }
            }

            if let Some(ref err) = *error.read() {
                div { class: "error-message ", "{err}" }
            }

            if let Some(ref data) = *result.read() {
                div { class: "result-card ",
                    h3 { "绱井鏂楁暟鍛界洏 " }

                    if let Some(ming_zhu) = data.get("ming_zhu").and_then(|v| v.as_str()) {
                        div { class: "zw-info ",
                            span { "鍛戒富: " }
                            strong { {ming_zhu} }
                        }
                    }
                    if let Some(shen_zhu) = data.get("shen_zhu").and_then(|v| v.as_str()) {
                        div { class: "zw-info ",
                            span { "韬富: " }
                            strong { {shen_zhu} }
                        }
                    }
                    if let Some(qi_yun) = data.get("qi_yun_age").and_then(|v| v.as_u64()) {
                        div { class: "zw-info ",
                            span { "璧疯繍骞撮緞: " }
                            strong { "{qi_yun}宀?" }
                        }
                    }

                    // 鍥涘寲
                    if let Some(si_hua) = data.get("si_hua") {
                        div { class: "si-hua ",
                            h4 { "鍥涘寲 " }
                            div { class: "si-hua-grid ",
                                for (key, label) in [("hua_lu", "鍖栫 "), ("hua_quan", "鍖栨潈 "), ("hua_ke", "鍖栫 "), ("hua_ji", "鍖栧繉 ")] {
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

                    // 鍗佷簩瀹?
                    if let Some(gongs) = data.get("gongs").and_then(|v| v.as_array()) {
                        div { class: "zw-gongs ",
                            h4 { "鍗佷簩瀹?" }
                            table { class: "data-table ",
                                thead {
                                    tr {
                                        th { "瀹綅 " }
                                        th { "鍦版敮 " }
                                        th { "涓绘槦 " }
                                        th { "杈呮槦 " }
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

                    // 澶ч檺
                    if let Some(da_xian) = data.get("da_xian").and_then(|v| v.as_array()) {
                        if !da_xian.is_empty() {
                            div { class: "da-xian ",
                                h4 { "澶ч檺 " }
                                table { class: "data-table ",
                                    thead {
                                        tr {
                                            th { "瀹綅 " }
                                            th { "骞撮緞 " }
                                            th { "涓绘槦 " }
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

// ============ 鏁扮畻 ============

#[component]
pub fn ShuSuan() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut active_tab = use_signal(|| "shaozi".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let tabs = [
        ("shaozi", "閭靛瓙绁炴暟 "),
        ("tieban", "閾佹澘绁炴暟 "),
        ("beiji", "鍖楁瀬绁炴暟 "),
        ("nanji", "鍗楁瀬绁炴暟 "),
        ("cetian", "绛栧ぉ "),
        ("chunzi", "鏄ュ瓙 "),
        ("fendjing", "鍒嗙粡 "),
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
            h2 { "鏁扮畻 - 绁炴暟鎺掔洏 " }
            p { class: "page-desc ", "閭靛瓙绁炴暟, 閾佹澘绁炴暟, 鍖楁瀬绁炴暟, 鍗楁瀬绁炴暟, 绛栧ぉ, 鏄ュ瓙, 鍒嗙粡 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "鍑虹敓鏃ユ湡鏃堕棿 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons ",
                    for (key, label) in tabs.clone() {
                        button { class: if active_tab() == *key { "tab-btn active " } else { "tab-btn " },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "鎺掔洏 " }
            }
            if loading() { div { class: "loading ", "鎺掔洏涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "鎺掔洏缁撴灉 " } {json_render(data)} }
            }
        }
    }
}

// ============ 涓夊紡 ============

#[component]
pub fn Sanshi() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut active_tab = use_signal(|| "qimen".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let tabs = [
        ("qimen", "濂囬棬閬佺敳 "),
        ("taiyi", "澶箼绁炴暟 "),
        ("liuren", "鍏， "),
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
            h2 { "涓夊紡鍚堜竴 " }
            p { class: "page-desc ", "濂囬棬, 澶箼, 鍏，涓夊紡鏁村悎鎺掔洏 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "鏃ユ湡鏃堕棿 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons ",
                    for (key, label) in tabs.clone() {
                        button { class: if active_tab() == *key { "tab-btn active " } else { "tab-btn " },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "鎺掔洏 " }
            }
            if loading() { div { class: "loading ", "鎺掔洏涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "涓夊紡缁撴灉 " } {json_render(data)} }
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
            h2 { "濂囬棬閬佺敳 " }
            p { class: "page-desc ", "杈撳叆鏃ユ湡鏃堕棿,鎺掑闂ㄩ亖鐢茬洏 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "鏃ユ湡鏃堕棿 " }
                        input {
                            r#type: "datetime-local ",
                            value: "{datetime}",
                            oninput: move |evt| datetime.set(evt.value()),
                        }
                    }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "鎺掔洏 " }
            }
            if loading() { div { class: "loading ", "鎺掔洏涓?.. " } }
            if let Some(ref err) = *error.read() { div { class: "error-message ", {err.clone()} } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ",
                    h3 { "濂囬棬鐩?" }
                    if let Some(ju) = data.get("ju") {
                        p { "鐢ㄥ眬: " {ju.as_str().unwrap_or("?")} }
                    }
                    if let Some(gongs) = data.get("gongs").and_then(|v| v.as_array()) {
                        table { class: "data-table ",
                            thead { tr { th { "瀹?" } th { "鍏崷 " } th { "澶╃洏 " } th { "鍦扮洏 " } th { "鍏棬 " } th { "涔濇槦 " } th { "鍏 " } } }
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
            h2 { "澶箼绁炴暟 " }
            p { class: "page-desc ", "澶箼绁炴暟鎺掔洏:澶箼鍗佸叚绁? 璁＄, 鏂囨槍, 濮嬪嚮, 涓诲澶у皬 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "鏃ユ湡鏃堕棿 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "鎺掔洏 " }
            }
            if loading() { div { class: "loading ", "鎺掔洏涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "澶箼鐩?" } {json_render(data)} }
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
            h2 { "鍏， " }
            p { class: "page-desc ", "澶у叚澹帓鐩?澶╁湴鐩? 鍥涜, 涓変紶, 閬佸共, 璐典汉 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "鏃ユ湡鏃堕棿 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "鎺掔洏 " }
            }
            if loading() { div { class: "loading ", "鎺掔洏涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "鍏，鐩?" } {json_render(data)} }
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
            h2 { "鍏埢 " }
            p { class: "page-desc ", "鍏埢璧峰崷:閾滈挶鎽囧崷,杈撳叆鍏埢鏁板€?6/7/8/9),閫楀彿鍒嗛殧 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "鍏埢閾滈挶鏁?" }
                        input { r#type: "text ", placeholder: "濡? 6,7,8,6,9,7 ", value: "{coins}", oninput: move |evt| coins.set(evt.value()) }
                    }
                }
                div { class: "form-row ",
                    button { class: "submit-btn ", onclick: on_cast, disabled: loading(), "璧峰崷 " }
                    button { class: "submit-btn secondary ", onclick: on_random, "闅忔満 " }
                }
            }
            if loading() { div { class: "loading ", "璧峰崷涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "鍗﹁薄 " } {json_render(data)} }
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
            h2 { "鑺傛皵鏌ヨ " }
            p { class: "page-desc ", "鏌ヨ浜屽崄鍥涜妭姘旂簿纭椂鍒?" }

            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "骞翠唤 " }
                        input {
                            r#type: "number ",
                            value: "{year}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<i32>() { year.set(v); }
                            },
                        }
                    }
                }
                button { class: "submit-btn ", onclick: on_query, disabled: loading(), "鏌ヨ鑺傛皵 " }
            }

            if loading() { div { class: "loading ", "鏌ヨ涓?.. " } }

            if let Some(ref data) = *result.read() {
                if let Some(list) = data.as_array() {
                    div { class: "result-card ",
                        h3 { "{year()}骞?浜屽崄鍥涜妭姘?" }
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
                                        {if jq.get("is_jie").and_then(|v| v.as_bool()).unwrap_or(false) { "鑺?" } else { "姘?" }}
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
            h2 { "椋庢按 " }
            p { class: "page-desc ", "鍏畢鍛藉崷, 鐜勭┖椋炴槦, 涓夊厓涔濊繍 " }
            div { class: "form-card ",
                div { class: "tab-buttons ",
                    button { class: if active_tab() == "ming_gua" { "tab-btn active " } else { "tab-btn " },
                        onclick: move |_| active_tab.set("ming_gua".to_string()), "鍏畢鍛藉崷 " }
                    button { class: if active_tab() == "flying_stars" { "tab-btn active " } else { "tab-btn " },
                        onclick: move |_| active_tab.set("flying_stars".to_string()), "鐜勭┖椋炴槦 " }
                }
                if active_tab() == "ming_gua" {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "鍑虹敓骞翠唤 " }
                            input { r#type: "number ", value: "{year}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<i32>() { year.set(v); } } } }
                        div { class: "form-group ", label { "鎬у埆 " }
                            select { value: "{gender}", onchange: move |evt| gender.set(evt.value()),
                                option { value: "male", "鐢?" } option { value: "female", "濂?" } } }
                    }
                } else {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "寤烘埧骞翠唤 " }
                            input { r#type: "number ", value: "{build_year}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<i32>() { build_year.set(v); } } } }
                        div { class: "form-group ", label { "鏈濆悜(搴? " }
                            input { r#type: "number ", step: "0.1 ", value: "{facing}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { facing.set(v); } } } }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "璁＄畻 " }
            }
            if loading() { div { class: "loading ", "璁＄畻涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "椋庢按缁撴灉 " } {json_render(data)} }
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
    let mut di_fen = use_signal(|| "瀛?.to_string());
    let mut active_tab = use_signal(|| "jinkou".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let tabs = [
        ("jinkou", "閲戝彛璇€ "), ("jingjue", "鑽嗚瘈 "), ("shenyishu", "绁炴槗鏁?"),
        ("wuzhao", "浜斿厗 "), ("taixuan", "澶巹 "), ("xianqin", "鍏堢Е鍗犲崪 "),
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
            ("/xianqin/divination", std::sync::Arc::new(serde_json::json!({ "seed": seed(), "method": "钃嶈崏" })))
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
            h2 { "鍏朵粬鍗滄硶 " }
            p { class: "page-desc ", "閲戝彛璇€, 鑽嗚瘈, 绁炴槗鏁? 浜斿厗, 澶巹, 鍏堢Е鍗犲崪 " }
            div { class: "form-card ",
                div { class: "tab-buttons ",
                    for (key, label) in tabs.clone() {
                        button { class: if active_tab() == *key { "tab-btn active " } else { "tab-btn " },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                if active_tab() == "jinkou" {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "鏃ユ湡鏃堕棿 " }
                            input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                        div { class: "form-group ", label { "鍦板垎 " }
                            input { r#type: "text ", value: "{di_fen}", oninput: move |evt| di_fen.set(evt.value()) } }
                    }
                } else if active_tab() == "shenyishu" {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "鏁颁竴 " }
                            input { r#type: "number ", value: "{num1}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num1.set(v); } } } }
                        div { class: "form-group ", label { "鏁颁簩 " }
                            input { r#type: "number ", value: "{num2}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num2.set(v); } } } }
                        div { class: "form-group ", label { "鏁颁笁 " }
                            input { r#type: "number ", value: "{num3}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num3.set(v); } } } }
                    }
                } else if active_tab() == "wuzhao" {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "闂簨 " }
                            input { r#type: "text ", value: "{question}", oninput: move |evt| question.set(evt.value()) } }
                    }
                } else if active_tab() == "taixuan" || active_tab() == "xianqin" {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "绉嶅瓙鏁?" }
                            input { r#type: "number ", value: "{seed}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { seed.set(v); } } } }
                    }
                } else {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "鏃ユ湡鏃堕棿 " }
                            input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "鎺ㄧ畻 " }
            }
            if loading() { div { class: "loading ", "鎺ㄧ畻涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "鎺ㄧ畻缁撴灉 " } {json_render(data)} }
            }
        }
    }
}

// ============ 宸ュ叿 ============

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
            h2 { "AI 鍒嗘瀽 " }
            p { class: "page-desc ", "澶氭ā鍨嬫帴鍏? 娴嬭瘯瀵硅瘽, 鍛界悊鐞嗚В璇?" }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "鎻愰棶 " }
                        textarea { value: "{message}", oninput: move |evt| message.set(evt.value()),
                            placeholder: "杈撳叆鍛界悊鍒嗘瀽闂... ", rows: "4 " } }
                }
                button { class: "submit-btn ", onclick: on_send, disabled: loading(), "鍙戦€?" }
            }
            if loading() { div { class: "loading ", "AI鎬濊€冧腑... " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "AI 鍥炵瓟 " } {json_render(data)} }
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
            h2 { "澶╂枃棣?" }
            p { class: "page-desc ", "瀹炴椂澶╄薄:澶槼鏄熷害, 鏈堢浉, 鍙琛屾槦浣嶇疆 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "绾害 " }
                        input { r#type: "number ", step: "0.0001 ", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group ", label { "缁忓害 " }
                        input { r#type: "number ", step: "0.0001 ", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                }
                button { class: "submit-btn ", onclick: on_query, disabled: loading(), "鏌ヨ澶╄薄 " }
            }
            if loading() { div { class: "loading ", "鏌ヨ涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "褰撳墠澶╄薄 " } {json_render(data)} }
            }
        }
    }
}

// ============ 涓囧勾鍘?榛勫巻)============

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
            h2 { "榛勫巻 - 涓囧勾鍘?" }
            p { class: "page-desc ", "瀵挎槦澶╂枃鍘?---鍏巻/鍐滃巻/鍥炲巻涓夊巻杞崲, 鏃ユ湀椋? 骞叉敮鑺傛皵 " }

            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "骞?" }
                        input {
                            r#type: "number ",
                            value: "{year}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<i32>() { year.set(v); }
                            },
                        }
                    }
                    div { class: "form-group ",
                        label { "鏈?" }
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
                        label { "鏃?" }
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
                        "鍏巻杞啘鍘?"
                    }
                    button {
                        class: if active_tab() == "ganzhi" { "tab-btn active " } else { "tab-btn " },
                        onclick: move |_| { active_tab.set("ganzhi".to_string()); },
                        "骞叉敮鏌ヨ "
                    }
                    button {
                        class: if active_tab() == "eclipse" { "tab-btn active " } else { "tab-btn " },
                        onclick: move |_| { active_tab.set("eclipse".to_string()); },
                        "鏃ユ湀椋?"
                    }
                }

                div { class: "tab-content ",
                    if active_tab() == "lunar" {
                        div {
                            button {
                                class: "submit-btn ",
                                onclick: on_solar_to_lunar,
                                disabled: loading(),
                                "鏌ヨ鍐滃巻 "
                            }
                            if loading() { div { class: "loading ", "鏌ヨ涓?.. " } }
                            if let Some(ref data) = *lunar_result.read() {
                                div { class: "result-card lunar-card ",
                                    h3 { "鍐滃巻杞崲缁撴灉 " }
                                    div { class: "lunar-info ",
                                        div { class: "lunar-row ",
                                            span { class: "lunar-label ", "鍐滃巻鏃ユ湡: " }
                                            span { class: "lunar-value ",
                                                {data.get("year").and_then(|v| v.as_i64()).unwrap_or(0).to_string()} "骞?
                                                {data.get("month_name_zh").and_then(|v| v.as_str()).unwrap_or("? ")}
                                                {data.get("day_name_zh").and_then(|v| v.as_str()).unwrap_or("? ")}
                                            }
                                        }
                                        div { class: "lunar-row ",
                                            span { class: "lunar-label ", "骞村共鏀?" }
                                            span { class: "lunar-value ", {data.get("year_ganzhi").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                        }
                                        div { class: "lunar-row ",
                                            span { class: "lunar-label ", "鐢熻倴: " }
                                            span { class: "lunar-value ", {data.get("zodiac_animal").and_then(|v| v.as_str()).unwrap_or("? ")} }
                                        }
                                        if let Some(leap) = data.get("is_leap_month").and_then(|v| v.as_bool()) {
                                            if leap {
                                                div { class: "lunar-row ",
                                                    span { class: "lunar-label lunar-leap ", "(闂版湀) " }
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
                                "鏌ヨ骞叉敮 "
                            }
                            if loading() { div { class: "loading ", "鏌ヨ涓?.. " } }
                            if let Some(ref data) = *ganzhi_result.read() {
                                div { class: "result-card ",
                                    h3 { "骞叉敮淇℃伅 " }
                                    table { class: "data-table ",
                                        tbody {
                                            tr { td { "骞村共鏀?" } td { {data.get("year_ganzhi").and_then(|v| v.as_str()).unwrap_or("? ")} } }
                                            tr { td { "鐢熻倴 " } td { {data.get("zodiac").and_then(|v| v.as_str()).unwrap_or("? ")} } }
                                            tr { td { "骞村彿 " } td { {data.get("nianhao").and_then(|v| v.as_str()).unwrap_or("? ")} } }
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
                                "鏌ヨ鏃ユ湀椋?"
                            }
                            if loading() { div { class: "loading ", "鏌ヨ涓?.. " } }
                            if let Some(ref data) = *eclipse_result.read() {
                                if let Some(list) = data.as_array() {
                                    div { class: "result-card ",
                                        h3 { "{year()}骞?鏃ユ湀椋?" }
                                        if list.is_empty() {
                                            p { class: "empty-state ", "璇ュ勾鏃犳棩鏈堥 " }
                                        } else {
                                            table { class: "data-table ",
                                                thead {
                                                    tr {
                                                        th { "鏃ユ湡 " }
                                                        th { "绫诲瀷 " }
                                                        th { "椋熷垎 " }
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

// ============ 鍏朵粬椤甸潰 ============

#[component]
pub fn References() -> Element {
    rsx! {
        div { class: "page ",
            h2 { "杈呭姪鍙傝€?" }
            p { class: "page-desc ", "鍏崷鍒嗙被, 澶╁共鍦版敮, 鑺傛皵鏌ヨ " }
            div { class: "result-card ",
                h3 { "鍏崄鍥涘崷 " }
                div { class: "ref-grid ",
                    for gua in &["涔?", "鍧?", "灞?", "钂?", "闇€ ", "璁?", "甯?", "姣?", "灏忕暅 ", "灞?", "娉?", "鍚?", "鍚屼汉 ", "澶ф湁 ", "璋?", "璞?", "闅?", "铔?", "涓?", "瑙?", "鍣棏 ", "璐?", "鍓?", "澶?", "鏃犲 ", "澶х暅 ", "棰?", "澶ц繃 ", "鍧?", "绂?", "鍜?", "鎭?", "閬?", "澶у． ", "鏅?", "鏄庡し ", "瀹朵汉 ", "鐫?", "韫?", "瑙?", "鎹?", "鐩?", "澶?", "濮?", "钀?", "鍗?", "鍥?", "浜?", "闈?", "榧?", "闇?", "鑹?", "娓?", "褰掑 ", "涓?", "鏃?", "宸?", "鍏?", "娑?", "鑺?", "涓瓪 ", "灏忚繃 ", "鏃㈡祹 ", "鏈祹 "] {
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
            h2 { "璁剧疆 " }
            p { class: "page-desc ", "搴旂敤璁剧疆 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "涓婚 " }
                        select { value: "{theme}", onchange: move |evt| theme.set(evt.value()),
                            option { value: "light", "娴呰壊 " } option { value: "dark", "娣辫壊 " } } }
                    div { class: "form-group ", label { "璇█ " }
                        select { value: "{language}", onchange: move |evt| language.set(evt.value()),
                            option { value: "zh", "涓枃 " } option { value: "en", "English " } } }
                }
                button { class: "submit-btn ", onclick: on_save, "淇濆瓨璁剧疆 " }
                if saved() { div { class: "success-msg ", "璁剧疆宸蹭繚瀛?" } }
            }
        }
    }
}

#[component]
pub fn GuoLao() -> Element {
    rsx! {
        div { class: "page ",
            h2 { "鏋滆€佹槦瀹?" }
            p { class: "page-desc ", "鏋滆€佹槦瀹楁帹婕? 浜屽崄鍏鍛藉害韬害 " }
            p { "璇蜂娇鐢ㄤ竷鏀垮洓浣欓〉闈㈣繘琛屾帓鐩?鏋滆€佹槦瀹椾笌涓冩斂鍥涗綑浣跨敤鍚屼竴璁＄畻寮曟搸. " }
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
            h2 { "鍗﹀崰 " }
            p { class: "page-desc ", "姊呰姳鏄撴暟, 鍏埢鍗﹀崰 " }
            div { class: "form-card ",
                div { class: "tab-buttons ",
                    button { class: if active_tab() == "meihua" { "tab-btn active " } else { "tab-btn " },
                        onclick: move |_| active_tab.set("meihua".to_string()), "姊呰姳鏄撴暟 " }
                    button { class: if active_tab() == "meiyi" { "tab-btn active " } else { "tab-btn " },
                        onclick: move |_| active_tab.set("meiyi".to_string()), "鍏埢鍗?" }
                }
                if active_tab() == "meihua" {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "鏁颁竴 " }
                            input { r#type: "number ", value: "{num1}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num1.set(v); } } } }
                        div { class: "form-group ", label { "鏁颁簩 " }
                            input { r#type: "number ", value: "{num2}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num2.set(v); } } } }
                        div { class: "form-group ", label { "鏁颁笁 " }
                            input { r#type: "number ", value: "{num3}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num3.set(v); } } } }
                    }
                } else {
                    div { class: "form-row ",
                        div { class: "form-group ", label { "鏃ユ湡鏃堕棿 " }
                            input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                    }
                }
                button { class: "submit-btn ", onclick: on_calc, disabled: loading(), "璧峰崷 " }
            }
            if loading() { div { class: "loading ", "璧峰崷涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "鍗﹁薄 " } {json_render(data)} }
            }
        }
    }
}

#[component]
pub fn DunJia() -> Element {
    rsx! {
        div { class: "page ",
            h2 { "閬佺敳 " }
            p { class: "page-desc ", "閬佺敳鍖呮嫭:闈掗緳閬? 鐧借檸閬佺瓑 " }
            p { "璇蜂娇鐢ㄥ闂ㄩ亖鐢查〉闈㈣繘琛屾帓鐩?閬佺敳涓庡闂ㄤ娇鐢ㄥ悓涓€璁＄畻寮曟搸. " }
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
            h2 { "鍗﹁薄 " }
            p { class: "page-desc ", "鍏崄鍥涘崷, 鍗﹁薄鍏崇郴, 鍗﹁緸鐖昏緸 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "鍗﹀簭 (0-63) " }
                        input { r#type: "number ", min: "0 ", max: "63 ", value: "{gua_seq}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { gua_seq.set(v); } } } }
                }
                button { class: "submit-btn ", onclick: on_query, disabled: loading(), "鏌ヨ " }
            }
            if loading() { div { class: "loading ", "鏌ヨ涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "鍗﹁薄璇︽儏 " } {json_render(data)} }
            }
        }
    }
}

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "page about-page ",
            h2 { "鍏充簬Divines " }
            p { "鐗堟湰: 0.1.0 (鍩轰簬 Rust 閲嶅啓) " }
            p { "Divines 鏄竴涓叏闈㈢殑鐜勫宸ヤ綔绔? " }
            p { "瑗挎柟鍗犳槦鐨勬湰鍛? 鎺ㄨ繍, 鍏崇郴鐩?杩炲悓鍏瓧, 绱井, 濂囬棬, 鍏，, 澶箼杩欎簺涓浗浼犵粺鏈暟,琚斁杩涘悓涓€涓簲鐢ㄩ噷. " }
            p { "鏈€鏂扮増鏈熀浜?Rust 鍏ㄦ爤閲嶅啓,鍓嶇浣跨敤 Dioxus 0.7.9. " }
            p { "鍘熼」鐩湴鍧€: https://github.com/Horace-Maxwell/divines-Web-App-comprehensively-improved-MacOS " }
            p { "涓囧勾鍘嗗弬鑰? 瀵挎槦澶╂枃鍘?sxwnl) " }
            p { "璁稿彲: AGPL-3.0 " }
        }
    }
}

// ============ 浼犵粺鏈暟 - 鏁扮畻涓庣鏁?============

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
            h2 { "鐨囨瀬缁忎笘 " }
            p { class: "page-desc ", "鐨囨瀬缁忎笘鍏冧細杩愪笘鎺ㄧ畻,鍊煎勾鍗? 鍊间簨鍗?" }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ",
                        label { "骞翠唤 " }
                        input {
                            r#type: "number ",
                            value: "{year}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<i32>() { year.set(v); }
                            },
                        }
                    }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "鎺ㄧ畻 " }
            }
            if loading() { div { class: "loading ", "鎺ㄧ畻涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ",
                    h3 { "鐨囨瀬缁忎笘缁撴灉 " }
                    {json_render(data)}
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
            h2 { "鑽嗚瘈 " }
            p { class: "page-desc ", "鑽嗚瘈娴佸勾鎺ㄦ紨:浠ュ嚭鐢熸椂闂存帹绠楀悇骞磋繍鍔?" }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "鍑虹敓鏃ユ湡鏃堕棿 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                    div { class: "form-group ", label { "鏌ヨ骞翠唤 " }
                        input { r#type: "number ", value: "{query_year}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<i32>() { query_year.set(v); } } } }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "鎺ㄧ畻 " }
            }
            if loading() { div { class: "loading ", "鎺ㄧ畻涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "鑽嗚瘈鎺ㄦ紨缁撴灉 " } {json_render(data)} }
            }
        }
    }
}

#[component]
pub fn JinKou() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut di_fen = use_signal(|| "瀛?.to_string());
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
            h2 { "閲戝彛璇€ " }
            p { class: "page-desc ", "閲戝彛璇€鎺掔洏:鏈堝皢, 鍦板垎, 灏嗙, 璐电, 浜哄厓 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "鏃ユ湡鏃堕棿 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                    div { class: "form-group ", label { "鍦板垎 " }
                        input { r#type: "text ", placeholder: "瀛愪笐瀵呭嵂杈板烦鍗堟湭鐢抽厜鎴屼亥 ", value: "{di_fen}", oninput: move |evt| di_fen.set(evt.value()) } }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "鎺掔洏 " }
            }
            if loading() { div { class: "loading ", "鎺掔洏涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "閲戝彛璇€鎺掔洏缁撴灉 " } {json_render(data)} }
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
            h2 { "绁炴槗鏁?" }
            p { class: "page-desc ", "绁炴槗鏁颁笁鏁拌捣鍗?浠ヤ笁涓暟瀛楄捣鍗︽帹鏂悏鍑?" }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "鏁颁竴 " }
                        input { r#type: "number ", value: "{num1}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num1.set(v); } } } }
                    div { class: "form-group ", label { "鏁颁簩 " }
                        input { r#type: "number ", value: "{num2}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num2.set(v); } } } }
                    div { class: "form-group ", label { "鏁颁笁 " }
                        input { r#type: "number ", value: "{num3}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num3.set(v); } } } }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "璧峰崷 " }
            }
            if loading() { div { class: "loading ", "璧峰崷涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "绁炴槗鏁扮粨鏋?" } {json_render(data)} }
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
            h2 { "浜斿厗 " }
            p { class: "page-desc ", "浜斿厗鍗滃崰鍗?浠ラ棶浜嬩负寮?鎺ㄦ紨浜旇鍏嗚薄 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "闂簨 " }
                        input { r#type: "text ", placeholder: "杈撳叆鎮ㄦ兂闂殑浜?.. ", value: "{question}", oninput: move |evt| question.set(evt.value()) } }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "鍗犲崪 " }
            }
            if loading() { div { class: "loading ", "鍗犲崪涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "浜斿厗鍗犲崪缁撴灉 " } {json_render(data)} }
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
            h2 { "澶巹 " }
            p { class: "page-desc ", "澶巹缁忔硶:闅忔満鎺ㄧ畻,81棣?29璧?" }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "绉嶅瓙鏁?" }
                        input { r#type: "number ", value: "{seed}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { seed.set(v); } } } }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "鎺ㄧ畻 " }
            }
            if loading() { div { class: "loading ", "鎺ㄧ畻涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "澶巹缁撴灉 " } {json_render(data)} }
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
            h2 { "鍖楁瀬绁炴暟 " }
            p { class: "page-desc ", "鍖楁瀬绁炴暟鎺掔洏:鍏瓧鎺ㄧ畻, 鍏埢瀹氫綅, 绁炴暟鏉℃枃 " }
            div { class: "form-card ",
                div { class: "form-row ",
                    div { class: "form-group ", label { "鍑虹敓鏃ユ湡鏃堕棿 " }
                        input { r#type: "datetime-local ", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn ", onclick: on_submit, disabled: loading(), "鎺掔洏 " }
            }
            if loading() { div { class: "loading ", "鎺掔洏涓?.. " } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card ", h3 { "鍖楁瀬绁炴暟鎺掔洏缁撴灉 " } {json_render(data)} }
            }
        }
    }
}

// ============ 鏁扮畻绁炴暟绯诲垪 ============

#[component]
pub fn CeTian() -> Element {
    rsx! { div { class: "page", h2 { "绛栧ぉ" } p { class: "page-desc", "绛栧ぉ鎺ㄦ紨:浠ュぉ鏃朵汉浜嬬瓥绠楀悏鍑?缁撳悎骞叉敮涓庡崷璞℃帹婕? }
        div { class: "result-card", p { "绛栧ぉ鎺掔洏鍔熻兘寮€鍙戜腑,灏嗘敮鎸佸共鏀捣璇句笌绛栫畻鎺ㄦ紨銆? } } } }
}

#[component]
pub fn ChunZi() -> Element {
    rsx! { div { class: "page", h2 { "鏄ュ瓙" } p { class: "page-desc", "鏄ュ瓙鎺ㄧ畻娉?浠ュ嚭鐢熷勾骞茶捣绠?鍒嗘槬瀛愬崄浜屽嵎" }
        div { class: "result-card", p { "鏄ュ瓙鎺掔洏鍔熻兘寮€鍙戜腑,灏嗘敮鎸佸崄浜屽嵎鏉℃枃妫€绱€? } } } }
}

#[component]
pub fn FenJing() -> Element {
    rsx! { div { class: "page", h2 { "鍒嗙粡" } p { class: "page-desc", "鍒嗙粡鎺ㄦ紨:浠ュ垎缁忔硶鎺ㄦ紨鍏崄鍥涘崷,缁撳悎涓栧簲鐖讳綅" }
        div { class: "result-card", p { "鍒嗙粡鎺掔洏鍔熻兘寮€鍙戜腑,灏嗘敮鎸佸崷鐖诲垎缁忔绱€? } } } }
}

#[component]
pub fn NanJi() -> Element {
    rsx! { div { class: "page", h2 { "鍗楁瀬绁炴暟" } p { class: "page-desc", "鍗楁瀬绁炴暟:浠ョ敓杈板叓瀛楁帹绠楃鏁版潯鏂?鍖呭惈鍥涚櫨浜屽崄鍏潯" }
        div { class: "result-card", p { "鍗楁瀬绁炴暟鎺掔洏鍔熻兘寮€鍙戜腑,灏嗘敮鎸佸洓鏌辫捣鏁颁笌鏉℃枃鏌ュ銆? } } } }
}

#[component]
pub fn ShaoZi() -> Element {
    rsx! { div { class: "page", h2 { "閭靛瓙绁炴暟" } p { class: "page-desc", "閭靛瓙绁炴暟:閭甸泹鎵€浼犵鏁版帹婕旂郴缁?鍖呭惈涓€鍗冮浂浜屽崄鍥涘崷" }
        div { class: "result-card", p { "閭靛瓙绁炴暟鎺掔洏鍔熻兘寮€鍙戜腑,灏嗘敮鎸佸厓浼氳繍涓栬捣鍗︺€? } } } }
}

#[component]
pub fn TieBan() -> Element {
    rsx! { div { class: "page", h2 { "閾佹澘绁炴暟" } p { class: "page-desc", "閾佹澘绁炴暟:浠ュ洓鏌卞叓瀛椾负鍩虹,缁撳悎鍗﹁薄鎺ㄦ紨涓€鐢熻繍鍔? }
        div { class: "result-card", p { "閾佹澘绁炴暟鎺掔洏鍔熻兘寮€鍙戜腑,灏嗘敮鎸佸崄浜屼竾鏉℃潯鏂囨绱€? } } } }
}

#[component]
pub fn XianQin() -> Element {
    rsx! { div { class: "page", h2 { "鍏堢Е鍗犲崪" } p { class: "page-desc", "鍏堢Е鍗犲崪:鍖呮嫭鐢查鐏奸緹銆佽搷鑽夊崰鍗溿€佽繛灞卞綊钘忕瓑涓婂彜鍗滄硶" }
        div { class: "result-card", p { "鍏堢Е鍗犲崪鍔熻兘寮€鍙戜腑,灏嗘敮鎸佽搷鑽夋紨鍗︿笌鐢查鐏煎厗銆? } } } }
}

// ============ 瑗挎柟鍗犳槦 - 涓撻」 ============

#[component]
pub fn AstroHellenistic() -> Element {
    rsx! { div { class: "page", h2 { "甯岃厞鍗犳槦" } p { class: "page-desc", "鍙ゅ吀甯岃厞鍗犳槦鏈?鏁村鍒躲€佺晫涓绘槦銆佷竷鏄熶富鏄熴€侀粍閬撻噴鏀炬硶" }
        div { class: "result-card", h3 { "鍔熻兘瑙勫垝" }
            ul { li { "鏁村鍒舵帓鐩? } li { "鐣屼富鏄熶笌涓夊垎涓绘槦" } li { "榛勯亾閲婃斁娉?(Zodiacal Releasing)" } li { "涓冩槦涓绘槦鍒嗘瀽" } } } } }
}

#[component]
pub fn AstroHorary() -> Element {
    rsx! { div { class: "page", h2 { "鍗滃崷鍗犳槦" } p { class: "page-desc", "鍗犳槦鍗滃崷鐩?浠ユ彁闂椂鍒昏捣鐩?鍒嗘瀽浜嬩欢璧板悜" }
        div { class: "result-card", h3 { "鍔熻兘瑙勫垝" }
            ul { li { "璧烽棶鏃跺埢鎺掔洏" } li { "搴欐椇寮遍櫡鍒嗘瀽" } li { "鐗规畩鐐?(Lot) 璁＄畻" } li { "鏈堜寒绌轰骸妫€鏌? } } } } }
}

#[component]
pub fn AstroElectional() -> Element {
    rsx! { div { class: "page", h2 { "鎷╂棩鍗犳槦" } p { class: "page-desc", "鎷╂棩鍗犳槦:閫夋嫨鍚夋棩鑹景,浼樺寲浜嬩欢寮€灞曟椂鏈? }
        div { class: "result-card", h3 { "鍔熻兘瑙勫垝" }
            ul { li { "鎷╂棩鏄熺洏鎺掔洏" } li { "琛屾槦鏃惰景 (Planetary Hours)" } li { "鏈堜寒绌轰骸妫€娴? } li { "鍚夊嚩璇勫垎绯荤粺" } } } } }
}

#[component]
pub fn AstroMundane() -> Element {
    rsx! { div { class: "page", h2 { "涓栬繍鍗犳槦" } p { class: "page-desc", "涓栬繍鍗犳槦 (Mundane Astrology):鍒嗘瀽鍥藉銆佹椂浠ｈ繍鍔? }
        div { class: "result-card", h3 { "鍔熻兘瑙勫垝" }
            ul { li { "鏄ョ鍒嗙洏 (Aries Ingress)" } li { "鏃ラ鏈堥鐩? } li { "澶у悎鐩稿垎鏋? } li { "鍥藉鏄熺洏绠＄悊" } } } } }
}

#[component]
pub fn AstroGermany() -> Element {
    rsx! { div { class: "page", h2 { "寰峰浗瀛︽淳" } p { class: "page-desc", "寰峰浗姹夊牎瀛︽淳鍗犳槦:涓偣娉曘€?0搴﹂噺琛ㄣ€佽秴琛屾槦" }
        div { class: "result-card", h3 { "鍔熻兘瑙勫垝" }
            ul { li { "90搴﹂噺琛?(90掳 Dial)" } li { "涓偣鏍?(Midpoint Tree)" } li { "瓒呰鏄熺鍙蜂綋绯? } li { "瀵圭О鎬у垎鏋? } } } } }
}

#[component]
pub fn AstroSynastry() -> Element {
    rsx! { div { class: "page", h2 { "鍚堢洏杩涢樁" } p { class: "page-desc", "杩涢樁鍚堢洏鍒嗘瀽:姣旇緝鐩樸€佺粍鍚堢洏銆佹椂绌轰腑鐐圭洏銆佹埓缁存．鐩? }
        div { class: "result-card",
            p { "鍚堢洏鍔熻兘璇蜂娇鐢ㄥ乏渚у鑸腑鐨勩€屽悎鐩樸€嶉〉闈€? }
            p { "鏀寔鐨勭洏鍨? 姣旇緝鐩?琛屾槦钀藉+鐩镐綅)銆佺粍鍚堢洏(Compsite)銆佹椂绌轰腑鐐圭洏(Time/Space Midpoint)銆? } } } }
}

#[component]
pub fn AstroAcg() -> Element {
    rsx! { div { class: "page", h2 { "ACG 鍦扮悊鍗犳槦" } p { class: "page-desc", "Astro*Carto*Graphy 鍦扮悊鍗犳槦:琛屾槦绾夸笌涓栫晫鍦板浘" }
        div { class: "result-card", h3 { "鍔熻兘瑙勫垝" }
            ul { li { "琛屾槦鍗囪捣绾?(ASC Lines)" } li { "澶╅《绾?(MC Lines)" } li { "澶╁簳绾?(IC Lines)" } li { "涓嬮檷绾?(DSC Lines)" } } } } }
}

#[component]
pub fn AstroRectification() -> Element {
    rsx! { div { class: "page", h2 { "鐢熸椂鏍℃" } p { class: "page-desc", "鍑虹敓鏃堕棿鏍℃:閫氳繃閲嶅ぇ浜嬩欢鍙嶆帹绮剧‘鍑虹敓鏃堕棿" }
        div { class: "result-card", h3 { "鍔熻兘瑙勫垝" }
            ul { li { "浜嬩欢褰曞叆涓庢椂闂存帹绠? } li { "澶槼寮ф牎姝ｆ硶" } li { "涓婚檺娉曟牎姝? } li { "澶氫簨浠朵氦鍙夐獙璇? } } } } }
}

// ============ 宸ュ叿 - 楠板瓙 / 浜屽崄鍏 ============

#[component]
pub fn Dice() -> Element {
    rsx! { div { class: "page", h2 { "鍗犳槦楠板瓙" } p { class: "page-desc", "鍗犳槦楠板瓙鍗犲崪:涓夋灇楠板瓙鍒嗗埆瀵瑰簲琛屾槦銆佹槦搴с€佸浣? }
        div { class: "result-card", h3 { "鍔熻兘瑙勫垝" }
            p { "鍗犳槦楠板瓙灏嗘敮鎸侀殢鏈烘幏楠颁笌瑙ｈ銆傝鏄熼 脳 鏄熷骇楠?脳 瀹綅楠?= 蹇€熷崰鍗溿€? } } } }
}

#[component]
pub fn Su28() -> Element {
    rsx! { div { class: "page", h2 { "浜屽崄鍏" } p { class: "page-desc", "浜屽崄鍏瑙傛祴涓庡懡鐞?璺濇槦銆佸害鏁般€佸鍚嶅搴? }
        div { class: "result-card",
            p { "浜屽崄鍏绯荤粺宸插湪涓冩斂鍥涗綑椤甸潰涓睍绀?琛屾槦浣嶇疆甯︽湁瀹垮害淇℃伅)銆? }
            p { "鐙珛浜屽崄鍏鏌ラ槄椤甸潰寮€鍙戜腑,灏嗘敮鎸佸搴︽煡璇笌瀹垮懡瀵瑰簲鍏崇郴銆? } } } }
}

// ============ 閭靛瓙绯诲垪 ============

#[component]
pub fn SzBaGua() -> Element {
    rsx! { div { class: "page", h2 { "閭靛瓙鍏崷" } p { class: "page-desc", "閭甸泹鍏堝ぉ鍏崷浣撶郴:鍏崷鏂逛綅銆佹搴忋€佽薄鎰? }
        div { class: "result-card", h3 { "鍔熻兘瑙勫垝" }
            ul { li { "鍏堝ぉ鍏崷鏂逛綅鍥? } li { "浼忕静鍏崄鍥涘崷娆″簭" } li { "鍏崷涓囩墿绫昏薄" } li { "鍗﹀簭鎺ㄦ紨" } } } } }
}

#[component]
pub fn SzDunJia() -> Element {
    rsx! { div { class: "page", h2 { "閭靛瓙閬佺敳" } p { class: "page-desc", "閭甸泹閬佺敳浣撶郴:浠ュぉ鍦颁汉涓夌洏缁撳悎閬佺敳涔濆" }
        div { class: "result-card", p { "閭靛瓙閬佺敳鎺ㄦ紨鍔熻兘寮€鍙戜腑銆傚熀纭€鎺掔洏璇蜂娇鐢ㄣ€屽闂ㄩ亖鐢层€嶉〉闈€? } } } }
}

#[component]
pub fn SzTaiYi() -> Element {
    rsx! { div { class: "page", h2 { "閭靛瓙澶箼" } p { class: "page-desc", "閭甸泹澶箼浣撶郴:缁撳悎澶箼鍗佸叚绁炰笌閭甸泹鍏冧細杩愪笘" }
        div { class: "result-card", p { "閭靛瓙澶箼鎺ㄦ紨鍔熻兘寮€鍙戜腑銆傚熀纭€鎺掔洏璇蜂娇鐢ㄣ€屽お涔欑鏁般€嶉〉闈€? } } } }
}

#[component]
pub fn SzFangWei() -> Element {
    rsx! { div { class: "page", h2 { "閭靛瓙鏂逛綅" } p { class: "page-desc", "閭靛瓙鏂逛綅鎺ㄧ畻:浠ュ厓浼氳繍涓栨帹婕斿悏鍑舵柟浣? }
        div { class: "result-card", p { "閭靛瓙鏂逛綅鎺ㄦ紨鍔熻兘寮€鍙戜腑,灏嗘敮鎸佹椂绌哄悏鍑舵柟浣嶈绠椼€? } } } }
}

#[component]
pub fn SzFengYe() -> Element {
    rsx! { div { class: "page", h2 { "閭靛瓙鍒嗛噹" } p { class: "page-desc", "閭靛瓙鍒嗛噹:浠ヤ簩鍗佸叓瀹垮垎閲庢硶瀵瑰簲鍦扮悊鍖哄煙" }
        div { class: "result-card", p { "閭靛瓙鍒嗛噹鍔熻兘寮€鍙戜腑,灏嗘敮鎸佸搴︿笌鍦扮悊鍖哄煙瀵瑰簲銆? } } } }
}

#[component]
pub fn SzNiXiang() -> Element {
    rsx! { div { class: "page", h2 { "閭靛瓙閫嗚薄" } p { class: "page-desc", "閭靛瓙閫嗚薄鎺ㄦ紨:浠ュ崷璞′箣閫嗛『鎺ㄦ紨鍚夊嚩鍙樺寲" }
        div { class: "result-card", p { "閭靛瓙閫嗚薄鎺ㄦ紨鍔熻兘寮€鍙戜腑,灏嗘敮鎸佸崷璞￠€嗛『鍙樺寲鍒嗘瀽銆? } } } }
}

#[component]
pub fn SzSign() -> Element {
    rsx! { div { class: "page", h2 { "閭靛瓙寰佸簲" } p { class: "page-desc", "閭靛瓙寰佸簲:浠ュぉ鍦板洓璞℃帹婕斾汉浜嬪緛搴? }
        div { class: "result-card", p { "閭靛瓙寰佸簲鍔熻兘寮€鍙戜腑,灏嗘敮鎸佸ぉ鍦板洓璞′笌浜轰簨寰佸簲鍒嗘瀽銆? } } } }
}

// ============ 鍛界悊鍏朵粬 ============

#[component]
pub fn MingOther() -> Element {
    rsx! { div { class: "page", h2 { "鍏朵粬鍛界悊" } p { class: "page-desc", "鍏朵粬鍛界悊鎶€娉?绾崇敳銆佹槦骞充細娴枫€佸叞鍙板閫夈€佹渤娲涚悊鏁扮瓑" }
        div { class: "result-card", h3 { "寰呴泦鎴愭妧娉? }
            div { class: "feature-grid",
                div { class: "feature-card", h3 { "绾崇敳" } p { "浠ュぉ骞茬撼鍦版敮,缁撳悎鍗﹁薄鎺ㄦ紨" } }
                div { class: "feature-card", h3 { "鏄熷钩浼氭捣" } p { "鏁村悎鍗犳槦涓庡叓瀛楃殑楂樼骇鎶€娉? } }
                div { class: "feature-card", h3 { "鍏板彴濡欓€? } p { "浠ョ撼闊充簲琛屾帹婕旀牸灞€" } }
                div { class: "feature-card", h3 { "娌虫礇鐞嗘暟" } p { "浠ユ渤鍥炬礇涔︽帹婕斿懡鐞? } }
            } } } }
}

#[component]
pub fn SuZhan() -> Element {
    rsx! { div { class: "page", h2 { "瀹垮崰" } p { class: "page-desc", "浜屽崄鍏鍗犲崪:浠ユ湀瀹夸簩鍗佸叓瀹夸负鍗?缁撳悎鏈堢浉涓庡搴? }
        div { class: "result-card", h3 { "鍔熻兘瑙勫垝" }
            ul { li { "褰撴湀鏈堝鎺ㄧ畻" } li { "浜屽崄鍏鍚夊嚩" } li { "瀹垮崰缁煎悎瑙ｈ" } } } } }
}

#[component]
pub fn TongSheFa() -> Element {
    rsx! { div { class: "page", h2 { "閫氭秹娉? } p { class: "page-desc", "閫氭秹娉?鐜勫閫氭秹杞崲鎶€娉?璺ㄨ秺涓嶅悓鏈暟浣撶郴" }
        div { class: "result-card", p { "閫氭秹娉曞姛鑳藉紑鍙戜腑,灏嗘敮鎸佷笉鍚屾湳鏁颁綋绯荤殑浜掗€氳浆鎹€? } } } }
}

#[component]
pub fn OtherBu() -> Element {
    rsx! { div { class: "page", h2 { "鍏朵粬鍗滄硶" } p { class: "page-desc", "鍏朵粬鍗犲崪鏂规硶:鎶界銆佽В姊︺€佹祴瀛椼€佺伒妫嬬粡绛? }
        div { class: "result-card", h3 { "寰呴泦鎴愬崰娉? }
            div { class: "feature-grid",
                div { class: "feature-card", h3 { "鎶界" } p { "鍏冲笣鐏电銆佽闊崇绛夊绛剧郴缁? } }
                div { class: "feature-card", h3 { "瑙ｆⅵ" } p { "鍛ㄥ叕瑙ｆⅵ涓庡共鏀薄鎰忕患鍚堣В璇? } }
                div { class: "feature-card", h3 { "娴嬪瓧" } p { "浠ュ瓧褰㈢瑪鐢诲崰鏂悏鍑? } }
                div { class: "feature-card", h3 { "鐏垫缁? } p { "鍗佷簩鏋氭瀛愭紨鍗︽帹婕? } }
            } } } }
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