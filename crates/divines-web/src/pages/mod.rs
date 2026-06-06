// Divines - 椤甸潰妯″潡
// 鍙傝€冨師椤圭洰: astrostudyui/src/pages/

pub mod home;

use dioxus::prelude::*;
use dioxus::signals::*;
use crate::Route;
use crate::services;

// ============ 鍗犳槦鏈懡鐩?============

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
        let req = serde_json::json!({
            "datetime": datetime(),
            "latitude": latitude(),
            "longitude": longitude(),
            "timezone": timezone(),
            "place_name": place_name(),
            "name": name(),
            "gender": gender(),
        });
        let fut = services::astro::get_natal_chart(&req);
        spawn(async move {
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
        div { class: "page",
            h2 { "鍗犳槦鏈懡鐩? }
            p { class: "page-desc", "杈撳叆鍑虹敓淇℃伅锛岃绠楄タ娲嬪崰鏄熸湰鍛界洏" }

            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group",
                        label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input {
                            r#type: "datetime-local",
                            value: "{datetime}",
                            oninput: move |evt| datetime.set(evt.value()),
                        }
                    }
                    div { class: "form-group",
                        label { "濮撳悕" }
                        input {
                            r#type: "text",
                            placeholder: "鍙€?,
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value()),
                        }
                    }
                }
                div { class: "form-row",
                    div { class: "form-group",
                        label { "绾害" }
                        input {
                            r#type: "number",
                            step: "0.0001",
                            value: "{latitude}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<f64>() {
                                    latitude.set(v);
                                }
                            },
                        }
                    }
                    div { class: "form-group",
                        label { "缁忓害" }
                        input {
                            r#type: "number",
                            step: "0.0001",
                            value: "{longitude}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<f64>() {
                                    longitude.set(v);
                                }
                            },
                        }
                    }
                    div { class: "form-group",
                        label { "鏃跺尯" }
                        input {
                            r#type: "number",
                            step: "0.5",
                            value: "{timezone}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<f64>() {
                                    timezone.set(v);
                                }
                            },
                        }
                    }
                }
                div { class: "form-row",
                    div { class: "form-group",
                        label { "鍦扮偣" }
                        input {
                            r#type: "text",
                            placeholder: "濡傦細鍖椾含",
                            value: "{place_name}",
                            oninput: move |evt| place_name.set(evt.value()),
                        }
                    }
                    div { class: "form-group",
                        label { "鎬у埆" }
                        select {
                            value: "{gender}",
                            onchange: move |evt| gender.set(evt.value()),
                            option { value: "male", "鐢? }
                            option { value: "female", "濂? }
                        }
                    }
                }
                button {
                    class: "submit-btn",
                    onclick: on_submit,
                    disabled: loading(),
                    "璁＄畻鏈懡鐩?
                }
            }

            if loading() {
                div { class: "loading", "璁＄畻涓?.." }
            }

            if let Some(ref err) = *error.read() {
                div { class: "error-message", "{err}" }
            }

            if let Some(ref data) = *result.read() {
                div { class: "result-card",
                    h3 { "鏄熺洏缁撴灉" }
                    if let Some(planets) = data.get("planets").and_then(|v| v.as_array()) {
                        div { class: "planet-list",
                            h4 { "琛屾槦浣嶇疆" }
                            table { class: "data-table",
                                thead {
                                    tr {
                                        th { "琛屾槦" }
                                        th { "鏄熷骇" }
                                        th { "搴︽暟" }
                                        th { "瀹綅" }
                                        th { "閫嗚" }
                                    }
                                }
                                tbody {
                                    for planet in planets {
                                        tr {
                                            td { {planet.get("planet").and_then(|v| v.as_str()).unwrap_or("?")} }
                                            td { {planet.get("sign").and_then(|v| v.as_str()).unwrap_or("?")} }
                                            td { {format!("{:.2}掳", planet.get("degree_in_sign").and_then(|v| v.as_f64()).unwrap_or(0.0))} }
                                            td { {planet.get("house").and_then(|v| v.as_u64()).unwrap_or(0)} }
                                            td { {planet.get("is_retrograde").and_then(|v| v.as_bool()).unwrap_or(false)} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if let Some(aspects) = data.get("aspects").and_then(|v| v.as_array()) {
                        div { class: "aspect-list",
                            h4 { "鐩镐綅 ({aspects.len()})" }
                            table { class: "data-table",
                                thead {
                                    tr {
                                        th { "琛屾槦1" }
                                        th { "琛屾槦2" }
                                        th { "鐩镐綅" }
                                        th { "瑙掑害" }
                                        th { "瀹硅搴? }
                                    }
                                }
                                tbody {
                                    for aspect in aspects {
                                        tr {
                                            td { {aspect.get("planet1").and_then(|v| v.as_str()).unwrap_or("?")} }
                                            td { {aspect.get("planet2").and_then(|v| v.as_str()).unwrap_or("?")} }
                                            td { {aspect.get("aspect_type").and_then(|v| v.as_str()).unwrap_or("?")} }
                                            td { {format!("{:.2}掳", aspect.get("angle").and_then(|v| v.as_f64()).unwrap_or(0.0))} }
                                            td { {format!("{:.2}掳", aspect.get("orb").and_then(|v| v.as_f64()).unwrap_or(0.0))} }
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
        ("solar_arc", "澶槼寮?),
        ("progressions", "娆￠檺娉?),
        ("primary_dir", "涓婚檺娉?),
        ("profections", "灏忛檺"),
        ("firdaria", "娉曡揪鏄熼檺"),
        ("age_point", "骞撮緞鎺ㄨ繘鐐?),
        ("symbolic_dir", "娉㈡柉鍚戣繍"),
        ("term_dir", "鐣岄檺娉?),
        ("thirteenth", "绗崄涓夊鐩?),
        ("harmonic", "璋冩尝鐩?),
        ("draconic", "榫欑洏"),
        ("year_129", "129骞寸郴缁?),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let req = serde_json::json!({
            "datetime": datetime(), "latitude": latitude(),
            "longitude": longitude(), "timezone": timezone(),
        });
        let endpoint = match active_tab().as_str() {
            "solar_arc" => "/predict/solar-arc",
            "progressions" => "/predict/progressions",
            "primary_dir" => "/predict/primary-directions",
            "profections" => "/predict/profections",
            "firdaria" => "/astro/firdaria",
            "age_point" => "/predict/age-point",
            "symbolic_dir" => "/predict/symbolic-dir",
            "term_dir" => "/predict/term-direction",
            "thirteenth" => "/predict/thirteenth-chart",
            "harmonic" => "/predict/harmonic-chart",
            "draconic" => "/predict/draconic-chart",
            "year_129" => "/predict/year-system-129",
            _ => "/predict/solar-arc",
        };
        let fut = services::astro::api_request("POST", endpoint, Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鏄熻繍 路 鎺ㄨ繍" }
            p { class: "page-desc", "澶槼寮с€佹闄愭硶銆佷富闄愭硶銆佹硶杈炬槦闄愩€佸皬闄愮瓑鎺ㄨ繍绯荤粺" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group",
                        label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) }
                    }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "绾害" }
                        input { r#type: "number", step: "0.0001", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group", label { "缁忓害" }
                        input { r#type: "number", step: "0.0001", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                    div { class: "form-group", label { "鏃跺尯" }
                        input { r#type: "number", step: "0.5", value: "{timezone}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { timezone.set(v); } } } }
                }
                div { class: "tab-buttons",
                    for (key, label) in &tabs {
                        button {
                            class: if active_tab() == *key { "tab-btn active" } else { "tab-btn" },
                            onclick: move |_| active_tab.set(key.to_string()),
                            "{label}"
                        }
                    }
                }
                button { class: "submit-btn", onclick: on_calc, disabled: loading(), "璁＄畻" }
            }
            if loading() { div { class: "loading", "璁＄畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鎺ㄨ繍缁撴灉" } pre { "{data}" } }
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
        ("synastry", "姣旇緝鐩?),
        ("composite", "缁勫悎鐩?),
        ("time_space", "鏃剁┖涓偣鐩?),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let req = serde_json::json!({
            "inner": { "datetime": inner_datetime() },
            "outer": { "datetime": outer_datetime() },
        });
        let endpoint = if active_tab() == "composite" { "/astro/composite" } else { "/astro/synastry" };
        let fut = services::astro::api_request("POST", endpoint, Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍚堢洏 路 鍏崇郴鐩? }
            p { class: "page-desc", "姣旇緝鐩樸€佺粍鍚堢洏銆佹椂绌轰腑鐐圭洏鍒嗘瀽" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍐呯洏鍑虹敓鏃堕棿" }
                        input { r#type: "datetime-local", value: "{inner_datetime}", oninput: move |evt| inner_datetime.set(evt.value()) } }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "澶栫洏鍑虹敓鏃堕棿" }
                        input { r#type: "datetime-local", value: "{outer_datetime}", oninput: move |evt| outer_datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons",
                    for (key, label) in &tabs {
                        button { class: if active_tab() == *key { "tab-btn active" } else { "tab-btn" },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn", onclick: on_calc, disabled: loading(), "璁＄畻鍚堢洏" }
            }
            if loading() { div { class: "loading", "璁＄畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍚堢洏缁撴灉" } pre { "{data}" } }
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
        ("arabic", "闃挎媺浼偣"),
        ("aspects", "鐩镐綅璇︽儏"),
        ("decennials", "鍗佸勾杩?),
        ("dispositor", "鏈€缁堝畾浣嶆槦"),
        ("lots", "鐗规畩鐐?),
        ("zr", "榛勯亾鏄熼噴"),
        ("return", "鍥炲綊鐩?),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let req = serde_json::json!({
            "datetime": datetime(), "latitude": latitude(),
            "longitude": longitude(), "timezone": timezone(),
        });
        let endpoint = match active_tab().as_str() {
            "arabic" => "/astro/arabic-points",
            "aspects" => "/astro/aspects",
            _ => "/astro/natal",
        };
        let fut = services::astro::api_request("POST", endpoint, Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "杈呯洏 路 涓撻」鍒嗘瀽" }
            p { class: "page-desc", "闃挎媺浼偣銆佺浉浣嶃€佹槦閲娿€佸洖褰掔洏绛変笓椤瑰垎鏋? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons",
                    for (key, label) in &tabs {
                        button { class: if active_tab() == *key { "tab-btn active" } else { "tab-btn" },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn", onclick: on_calc, disabled: loading(), "璁＄畻" }
            }
            if loading() { div { class: "loading", "璁＄畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍒嗘瀽缁撴灉" } pre { "{data}" } }
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
        ("chart", "鍗板害鐩?),
        ("dasha", "澶ц繍"),
        ("yogas", "鏍煎眬"),
        ("nakshatra", "27瀹?),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let req = serde_json::json!({
            "datetime": datetime(), "latitude": latitude(),
            "longitude": longitude(), "timezone": timezone(),
        });
        let endpoint = match active_tab().as_str() {
            "chart" => "/vedic/chart",
            "dasha" => "/vedic/dasha",
            "yogas" => "/vedic/yogas",
            "nakshatra" => "/vedic/nakshatra",
            _ => "/vedic/chart",
        };
        let fut = services::astro::api_request("POST", endpoint, Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍗板害鍗犳槦 路 Vedic" }
            p { class: "page-desc", "鍖?鍗?涓滃嵃搴︾洏銆佹亽鏄熼粍閬撱€佸ぇ杩愮郴缁熴€?7瀹? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons",
                    for (key, label) in &tabs {
                        button { class: if active_tab() == *key { "tab-btn active" } else { "tab-btn" },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn", onclick: on_calc, disabled: loading(), "璁＄畻" }
            }
            if loading() { div { class: "loading", "璁＄畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍗板害鍗犳槦缁撴灉" } pre { "{data}" } }
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
        let req = serde_json::json!({
            "datetime": datetime(),
            "latitude": latitude(),
            "longitude": longitude(),
            "timezone": timezone(),
        });
        let fut = services::qizheng::get_chart(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(e) => { error.set(Some(e)); loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "涓冩斂鍥涗綑 路 鏋滆€佹槦瀹? }
            p { class: "page-desc", "杈撳叆鍑虹敓淇℃伅锛屾帓涓冩斂鍥涗綑鏄熺洏锛屽惈28瀹裤€佸懡搴﹁韩搴︺€佹礊寰ぇ闄愩€佹灉鑰佹牸灞€" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group",
                        label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) }
                    }
                }
                div { class: "form-row",
                    div { class: "form-group",
                        label { "绾害" }
                        input { r#type: "number", step: "0.0001", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } }
                    }
                    div { class: "form-group",
                        label { "缁忓害" }
                        input { r#type: "number", step: "0.0001", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } }
                    }
                    div { class: "form-group",
                        label { "鏃跺尯" }
                        input { r#type: "number", step: "0.5", value: "{timezone}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { timezone.set(v); } } }
                    }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺掔洏" }
            }
            if loading() { div { class: "loading", "鎺掔洏涓?.." } }
            if let Some(ref err) = *error.read() { div { class: "error-message", "{err}" } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card",
                    h3 { "涓冩斂鍥涗綑鏄熺洏" }
                    if let Some(planets) = data.get("planets").and_then(|v| v.as_array()) {
                        div { class: "section",
                            h4 { "琛屾槦浣嶇疆" }
                            table { class: "data-table",
                                thead { tr { th { "琛屾槦" } th { "榛勭粡" } th { "鏄熸" } th { "瀹綅" } th { "28瀹? } th { "閫嗚" } } }
                                tbody {
                                    for p in planets {
                                        tr {
                                            td { {p.get("name_zh").and_then(|v| v.as_str()).unwrap_or("?")} }
                                            td { {format!("{:.2}掳", p.get("longitude").and_then(|v| v.as_f64()).unwrap_or(0.0))} }
                                            td { {p.get("sign_zh").and_then(|v| v.as_str()).unwrap_or("?")} }
                                            td { {p.get("house").and_then(|v| v.as_u64()).unwrap_or(0)} }
                                            td { {p.get("su_name").and_then(|v| v.as_str()).unwrap_or("?")} }
                                            td { {p.get("is_retrograde").and_then(|v| v.as_bool()).unwrap_or(false)} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if let Some(houses) = data.get("houses").and_then(|v| v.as_array()) {
                        div { class: "section",
                            h4 { "鍗佷簩瀹? }
                            table { class: "data-table",
                                thead { tr { th { "瀹綅" } th { "瀹悕" } th { "鏄熷骇" } th { "搴︽暟" } } }
                                tbody {
                                    for h in houses {
                                        tr {
                                            td { {h.get("house_num").and_then(|v| v.as_u64()).unwrap_or(0)} }
                                            td { {h.get("name_zh").and_then(|v| v.as_str()).unwrap_or("?")} }
                                            td { {h.get("sign_zh").and_then(|v| v.as_str()).unwrap_or("?")} }
                                            td { {format!("{:.2}掳", h.get("cusp").and_then(|v| v.as_f64()).unwrap_or(0.0))} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if let Some(patterns) = data.get("patterns").and_then(|v| v.as_array()) {
                        if !patterns.is_empty() {
                            div { class: "section",
                                h4 { "鏍煎眬" }
                                ul { for p in patterns { li { {p.as_str().unwrap_or("?")} } } }
                            }
                        }
                    }
                    if let Some(dongwei) = data.get("dong_wei").and_then(|v| v.as_array()) {
                        if !dongwei.is_empty() {
                            div { class: "section",
                                h4 { "娲炲井澶ч檺" }
                                table { class: "data-table",
                                    thead { tr { th { "骞撮檺" } th { "瀹綅" } th { "璇存槑" } } }
                                    tbody {
                                        for dw in dongwei {
                                            tr {
                                                td { {format!("{}-{}宀?", dw.get("start_age").and_then(|v| v.as_u64()).unwrap_or(0), dw.get("end_age").and_then(|v| v.as_u64()).unwrap_or(0))} }
                                                td { {dw.get("house_name").and_then(|v| v.as_str()).unwrap_or("?")} }
                                                td { {dw.get("description").and_then(|v| v.as_str()).unwrap_or("?")} }
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
        let req = serde_json::json!({
            "datetime": datetime(),
            "name": name(),
            "gender": gender(),
            "longitude": longitude(),
            "use_true_solar_time": use_true_solar(),
            "use_early_late_zi": use_early_late_zi(),
            "use_ding_qi": use_ding_qi(),
        });
        let fut = services::bazi::calculate(&req);
        spawn(async move {
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
        div { class: "page",
            h2 { "鍏瓧鎺掔洏" }
            p { class: "page-desc", "杈撳叆鍑虹敓鏃ユ湡鏃堕棿锛屾帓鍥涙煴鍏瓧銆佸崄绁炪€佸ぇ杩愩€傛敮鎸佺湡澶槼鏃躲€佹棭鏅氬瓙鏃躲€佸钩姘?瀹氭皵" }

            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group",
                        label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input {
                            r#type: "datetime-local",
                            value: "{datetime}",
                            oninput: move |evt| datetime.set(evt.value()),
                        }
                    }
                    div { class: "form-group",
                        label { "濮撳悕锛堝彲閫夛級" }
                        input {
                            r#type: "text",
                            placeholder: "鍙€?,
                            value: "{name}",
                            oninput: move |evt| name.set(evt.value()),
                        }
                    }
                    div { class: "form-group",
                        label { "鎬у埆" }
                        select {
                            value: "{gender}",
                            onchange: move |evt| gender.set(evt.value()),
                            option { value: "male", "鐢? }
                            option { value: "female", "濂? }
                        }
                    }
                }

                // 鎺掔洏閫夐」
                div { class: "options-section",
                    h4 { "鎺掔洏閫夐」" }
                    div { class: "options-grid",
                        div { class: "option-item",
                            label { class: "option-label",
                                input {
                                    r#type: "checkbox",
                                    checked: use_true_solar(),
                                    onchange: move |evt| use_true_solar.set(evt.value() == "true"),
                                }
                                span { "鐪熷お闃虫椂" }
                            }
                        }
                        div { class: "option-item",
                            label { class: "option-label",
                                input {
                                    r#type: "checkbox",
                                    checked: use_early_late_zi(),
                                    onchange: move |evt| use_early_late_zi.set(evt.value() == "true"),
                                }
                                span { "鍖哄垎鏃╂櫄瀛愭椂" }
                            }
                        }
                        div { class: "option-item",
                            label { class: "option-label",
                                input {
                                    r#type: "checkbox",
                                    checked: use_ding_qi(),
                                    onchange: move |evt| use_ding_qi.set(evt.value() == "true"),
                                }
                                span { "瀹氭皵娉? }
                            }
                            span { class: "option-hint", "锛堝彇娑堥€夋嫨涓哄钩姘旀硶锛? }
                        }
                        div { class: "form-group form-group-inline",
                            label { "缁忓害: " }
                            input {
                                r#type: "number",
                                step: "0.0001",
                                value: "{longitude}",
                                style: "width: 100px",
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
                    class: "submit-btn",
                    onclick: on_submit,
                    disabled: loading(),
                    "鎺掔洏"
                }
            }

            if loading() {
                div { class: "loading", "鎺掔洏涓?.." }
            }

            if let Some(ref err) = *error.read() {
                div { class: "error-message", "{err}" }
            }

            if let Some(ref data) = *result.read() {
                div { class: "result-card",
                    h3 { "鍏瓧鎺掔洏缁撴灉" }

                    // 鍥涙煴
                    div { class: "bazi-pillars",
                        h4 { "鍥涙煴" }
                        div { class: "pillar-grid",
                            for pillar_key in ["year", "month", "day", "hour"] {
                                div { class: "pillar-item",
                                    div { class: "pillar-label",
                                        {match pillar_key {
                                            "year" => "骞存煴",
                                            "month" => "鏈堟煴",
                                            "day" => "鏃ユ煴",
                                            "hour" => "鏃舵煴",
                                            _ => "",
                                        }}
                                    }
                                    if let Some(pillar) = data.get(pillar_key) {
                                        div { class: "pillar-tg",
                                            {pillar.get("tian_gan").and_then(|v| v.as_str()).unwrap_or("?")}
                                        }
                                        div { class: "pillar-dz",
                                            {pillar.get("di_zhi").and_then(|v| v.as_str()).unwrap_or("?")}
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 鏃ヤ富
                    if let Some(dm) = data.get("day_master").and_then(|v| v.as_str()) {
                        div { class: "day-master",
                            span { "鏃ヤ富: " }
                            strong { "{dm}" }
                        }
                    }
                    if let Some(adj_hour) = data.get("adjusted_hour").and_then(|v| v.as_f64()) {
                        div { class: "adjusted-hour",
                            span { "鏍℃鏃? " }
                            span { "{adj_hour:.2}鏃? }
                        }
                    }

                    // 鍗佺
                    if let Some(ten_gods) = data.get("ten_gods") {
                        div { class: "ten-gods",
                            h4 { "鍗佺" }
                            div { class: "ten-god-grid",
                                for (key, label) in [("year", "骞?), ("month", "鏈?), ("day", "鏃?), ("hour", "鏃?)] {
                                    div { class: "ten-god-item",
                                        span { "{label}: " }
                                        span { class: "ten-god-value",
                                            {ten_gods.get(key).and_then(|v| v.as_str()).unwrap_or("?")}
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 闀跨敓鍗佷簩绁?                    if let Some(chang_sheng) = data.get("chang_sheng") {
                        div { class: "chang-sheng",
                            h4 { "闀跨敓鍗佷簩绁? }
                            div { class: "chang-sheng-grid",
                                for (key, label) in [("year", "骞?), ("month", "鏈?), ("day", "鏃?), ("hour", "鏃?)] {
                                    div { class: "chang-sheng-item",
                                        span { "{label}: " }
                                        span { class: "chang-sheng-value",
                                            {chang_sheng.get(key).and_then(|v| v.as_str()).unwrap_or("?")}
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 绾抽煶
                    if let Some(na_yin) = data.get("na_yin") {
                        div { class: "na-yin",
                            h4 { "绾抽煶" }
                            div { class: "na-yin-grid",
                                for (key, label) in [("year", "骞?), ("month", "鏈?), ("day", "鏃?), ("hour", "鏃?)] {
                                    div { class: "na-yin-item",
                                        span { "{label}: " }
                                        span { {na_yin.get(key).and_then(|v| v.as_str()).unwrap_or("?")} }
                                    }
                                }
                            }
                        }
                    }

                    // 钘忓共
                    if let Some(hidden) = data.get("hidden_stems") {
                        div { class: "hidden-stems",
                            h4 { "钘忓共" }
                            div { class: "ten-god-grid",
                                for (key, label) in [("year", "骞?), ("month", "鏈?), ("day", "鏃?), ("hour", "鏃?)] {
                                    div { class: "ten-god-item",
                                        span { "{label}: " }
                                        if let Some(arr) = hidden.get(key).and_then(|v| v.as_array()) {
                                            span { class: "hidden-value",
                                                {arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("銆?)}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 骞叉敮鍒戝啿鍚堝
                    if let Some(relations) = data.get("relations").and_then(|v| v.as_array()) {
                        if !relations.is_empty() {
                            div { class: "relations",
                                h4 { "骞叉敮鍒戝啿鍚堝" }
                                table { class: "data-table",
                                    thead {
                                        tr {
                                            th { "绫诲瀷" }
                                            th { "娑夊強鏌? }
                                            th { "璇︽儏" }
                                        }
                                    }
                                    tbody {
                                        for rel in relations {
                                            tr {
                                                td { {rel.get("relation_type").and_then(|v| v.as_str()).unwrap_or("?")} }
                                                td {
                                                    if let Some(pillars) = rel.get("pillars").and_then(|v| v.as_array()) {
                                                        {pillars.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("銆?)}
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
                            div { class: "shen-sha",
                                h4 { "绁炵厼" }
                                table { class: "data-table",
                                    thead {
                                        tr {
                                            th { "绁炵厼" }
                                            th { "浣嶇疆" }
                                            th { "璇存槑" }
                                        }
                                    }
                                    tbody {
                                        for ss in shen_sha {
                                            tr {
                                                td { {ss.get("name").and_then(|v| v.as_str()).unwrap_or("?")} }
                                                td { {ss.get("pillar").and_then(|v| v.as_str()).unwrap_or("?")} }
                                                td { {ss.get("description").and_then(|v| v.as_str()).unwrap_or("?")} }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 澶ц繍
                    if let Some(qi_yun) = data.get("qi_yun_time").and_then(|v| v.as_str()) {
                        div { class: "qi-yun",
                            h4 { "璧疯繍鏃堕棿" }
                            p { "{qi_yun}" }
                        }
                    }

                    if let Some(da_yun) = data.get("da_yun").and_then(|v| v.as_array()) {
                        if !da_yun.is_empty() {
                            div { class: "da-yun",
                                h4 { "澶ц繍" }
                                table { class: "data-table",
                                    thead {
                                        tr {
                                            th { "骞撮緞" }
                                            th { "澶╁共" }
                                            th { "鍦版敮" }
                                            th { "鍗佺" }
                                            th { "骞翠唤" }
                                        }
                                    }
                                    tbody {
                                        for dy in da_yun {
                                            tr {
                                                td { {format!("{}-{}宀?", dy.get("start_age").and_then(|v| v.as_u64()).unwrap_or(0), dy.get("end_age").and_then(|v| v.as_u64()).unwrap_or(0))} }
                                                td {
                                                    {dy.get("pillar").and_then(|v| v.get("tian_gan")).and_then(|v| v.as_str()).unwrap_or("?")}
                                                }
                                                td {
                                                    {dy.get("pillar").and_then(|v| v.get("di_zhi")).and_then(|v| v.as_str()).unwrap_or("?")}
                                                }
                                                td { {dy.get("ten_god").and_then(|v| v.as_str()).unwrap_or("?")} }
                                                td {
                                                    {format!("{}-{}", dy.get("start_year").and_then(|v| v.as_i64()).unwrap_or(0), dy.get("end_year").and_then(|v| v.as_i64()).unwrap_or(0))}
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
                        div { class: "options-display",
                            h4 { "閫夐」" }
                            div { class: "options-display-grid",
                                span { "鐪熷お闃虫椂: {options.get("use_true_solar_time").and_then(|v| v.as_bool()).unwrap_or(false)}" }
                                span { "鏃╂櫄瀛愭椂: {options.get("use_early_late_zi").and_then(|v| v.as_bool()).unwrap_or(false)}" }
                                span { "瀹氭皵娉? {options.get("use_ding_qi").and_then(|v| v.as_bool()).unwrap_or(true)}" }
                                span { "缁忓害: {options.get("longitude").and_then(|v| v.as_f64()).unwrap_or(0.0)}" }
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
        let req = serde_json::json!({
            "datetime": datetime(),
            "gender": gender(),
        });
        let fut = services::ziwei::calculate(&req);
        spawn(async move {
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
        div { class: "page",
            h2 { "绱井鏂楁暟" }
            p { class: "page-desc", "杈撳叆鍑虹敓鏃ユ湡鏃堕棿锛屾帓绱井鏂楁暟鍛界洏" }

            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group",
                        label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input {
                            r#type: "datetime-local",
                            value: "{datetime}",
                            oninput: move |evt| datetime.set(evt.value()),
                        }
                    }
                    div { class: "form-group",
                        label { "鎬у埆" }
                        select {
                            value: "{gender}",
                            onchange: move |evt| gender.set(evt.value()),
                            option { value: "male", "鐢? }
                            option { value: "female", "濂? }
                        }
                    }
                }
                button {
                    class: "submit-btn",
                    onclick: on_submit,
                    disabled: loading(),
                    "鎺掔洏"
                }
            }

            if loading() {
                div { class: "loading", "鎺掔洏涓?.." }
            }

            if let Some(ref err) = *error.read() {
                div { class: "error-message", "{err}" }
            }

            if let Some(ref data) = *result.read() {
                div { class: "result-card",
                    h3 { "绱井鏂楁暟鍛界洏" }

                    if let Some(ming_zhu) = data.get("ming_zhu").and_then(|v| v.as_str()) {
                        div { class: "zw-info",
                            span { "鍛戒富: " }
                            strong { "{ming_zhu}" }
                        }
                    }
                    if let Some(shen_zhu) = data.get("shen_zhu").and_then(|v| v.as_str()) {
                        div { class: "zw-info",
                            span { "韬富: " }
                            strong { "{shen_zhu}" }
                        }
                    }
                    if let Some(qi_yun) = data.get("qi_yun_age").and_then(|v| v.as_u64()) {
                        div { class: "zw-info",
                            span { "璧疯繍骞撮緞: " }
                            strong { "{qi_yun}宀? }
                        }
                    }

                    // 鍥涘寲
                    if let Some(si_hua) = data.get("si_hua") {
                        div { class: "si-hua",
                            h4 { "鍥涘寲" }
                            div { class: "si-hua-grid",
                                for (key, label) in [("hua_lu", "鍖栫"), ("hua_quan", "鍖栨潈"), ("hua_ke", "鍖栫"), ("hua_ji", "鍖栧繉")] {
                                    if let Some(item) = si_hua.get(key).and_then(|v| v.as_array()) {
                                        if item.len() >= 2 {
                                            div { class: "si-hua-item",
                                                span { class: "si-hua-label", "{label}: " }
                                                span { {item[0].as_str().unwrap_or("?")} }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 鍗佷簩瀹?                    if let Some(gongs) = data.get("gongs").and_then(|v| v.as_array()) {
                        div { class: "zw-gongs",
                            h4 { "鍗佷簩瀹? }
                            table { class: "data-table",
                                thead {
                                    tr {
                                        th { "瀹綅" }
                                        th { "鍦版敮" }
                                        th { "涓绘槦" }
                                        th { "杈呮槦" }
                                    }
                                }
                                tbody {
                                    for gong in gongs {
                                        tr {
                                            td { {gong.get("name").and_then(|v| v.as_str()).unwrap_or("?")} }
                                            td { {gong.get("di_zhi").and_then(|v| v.as_str()).unwrap_or("?")} }
                                            td {
                                                if let Some(arr) = gong.get("zhu_xing").and_then(|v| v.as_array()) {
                                                    {arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("銆?)}
                                                }
                                            }
                                            td {
                                                if let Some(arr) = gong.get("fu_xing").and_then(|v| v.as_array()) {
                                                    {arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("銆?)}
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
                            div { class: "da-xian",
                                h4 { "澶ч檺" }
                                table { class: "data-table",
                                    thead {
                                        tr {
                                            th { "瀹綅" }
                                            th { "骞撮緞" }
                                            th { "涓绘槦" }
                                        }
                                    }
                                    tbody {
                                        for dx in da_xian {
                                            tr {
                                                td { {dx.get("gong_name").and_then(|v| v.as_str()).unwrap_or("?")} }
                                                td { {format!("{}-{}宀?", dx.get("start_age").and_then(|v| v.as_u64()).unwrap_or(0), dx.get("end_age").and_then(|v| v.as_u64()).unwrap_or(0))} }
                                                td {
                                                    if let Some(arr) = dx.get("zhu_xing").and_then(|v| v.as_array()) {
                                                        {arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join("銆?)}
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
        ("shaozi", "閭靛瓙绁炴暟"),
        ("tieban", "閾佹澘绁炴暟"),
        ("beiji", "鍖楁瀬绁炴暟"),
        ("nanji", "鍗楁瀬绁炴暟"),
        ("cetian", "绛栧ぉ"),
        ("chunzi", "鏄ュ瓙"),
        ("fendjing", "鍒嗙粡"),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let endpoint = match active_tab().as_str() {
            "shaozi" => "/shaozi/calculate",
            "tieban" => "/tieban/calculate",
            "beiji" => "/beiji/calculate",
            "nanji" => "/nanji/calculate",
            "cetian" => "/cetian/calculate",
            "chunzi" => "/chunzi/calculate",
            "fendjing" => "/fendjing/calculate",
            _ => "/shaozi/calculate",
        };
        let fut = services::astro::api_request("POST", endpoint, Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鏁扮畻 路 绁炴暟鎺掔洏" }
            p { class: "page-desc", "閭靛瓙绁炴暟銆侀搧鏉跨鏁般€佸寳鏋佺鏁般€佸崡鏋佺鏁般€佺瓥澶┿€佹槬瀛愩€佸垎缁? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons",
                    for (key, label) in &tabs {
                        button { class: if active_tab() == *key { "tab-btn active" } else { "tab-btn" },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn", onclick: on_calc, disabled: loading(), "鎺掔洏" }
            }
            if loading() { div { class: "loading", "鎺掔洏涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鎺掔洏缁撴灉" } pre { "{data}" } }
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
        ("qimen", "濂囬棬閬佺敳"),
        ("taiyi", "澶箼绁炴暟"),
        ("liuren", "鍏，"),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let endpoint = match active_tab().as_str() {
            "qimen" => "/qimen/calculate",
            "taiyi" => "/taiyi/calculate",
            "liuren" => "/liuren/calculate",
            _ => "/qimen/calculate",
        };
        let fut = services::astro::api_request("POST", endpoint, Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "涓夊紡鍚堜竴" }
            p { class: "page-desc", "濂囬棬銆佸お涔欍€佸叚澹笁寮忔暣鍚堟帓鐩? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "tab-buttons",
                    for (key, label) in &tabs {
                        button { class: if active_tab() == *key { "tab-btn active" } else { "tab-btn" },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                button { class: "submit-btn", onclick: on_calc, disabled: loading(), "鎺掔洏" }
            }
            if loading() { div { class: "loading", "鎺掔洏涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "涓夊紡缁撴灉" } pre { "{data}" } }
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
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::sanshi::qimen(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(e) => { error.set(Some(e)); loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "濂囬棬閬佺敳" }
            p { class: "page-desc", "杈撳叆鏃ユ湡鏃堕棿锛屾帓濂囬棬閬佺敳鐩? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group",
                        label { "鏃ユ湡鏃堕棿" }
                        input {
                            r#type: "datetime-local",
                            value: "{datetime}",
                            oninput: move |evt| datetime.set(evt.value()),
                        }
                    }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺掔洏" }
            }
            if loading() { div { class: "loading", "鎺掔洏涓?.." } }
            if let Some(ref err) = *error.read() { div { class: "error-message", "{err}" } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card",
                    h3 { "濂囬棬鐩? }
                    if let Some(ju) = data.get("ju") {
                        p { "鐢ㄥ眬: {ju}" }
                    }
                    if let Some(gongs) = data.get("gongs").and_then(|v| v.as_array()) {
                        table { class: "data-table",
                            thead { tr { th { "瀹? } th { "鍏崷" } th { "澶╃洏" } th { "鍦扮洏" } th { "鍏棬" } th { "涔濇槦" } th { "鍏" } } }
                            tbody {
                                for gong in gongs {
                                    tr {
                                        td { {gong.get("number").and_then(|v| v.as_u64()).unwrap_or(0)} }
                                        td { {gong.get("ba_gua").and_then(|v| v.as_str()).unwrap_or("?")} }
                                        td { {gong.get("tian_pan_gan").and_then(|v| v.as_str()).unwrap_or("?")} }
                                        td { {gong.get("di_pan_gan").and_then(|v| v.as_str()).unwrap_or("?")} }
                                        td { {gong.get("ba_men").and_then(|v| v.as_str()).unwrap_or("?")} }
                                        td { {gong.get("jiu_xing").and_then(|v| v.as_str()).unwrap_or("?")} }
                                        td { {gong.get("ba_shen").and_then(|v| v.as_str()).unwrap_or("?")} }
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
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::sanshi::taiyi(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "澶箼绁炴暟" }
            p { class: "page-desc", "澶箼绁炴暟鎺掔洏锛氬お涔欏崄鍏銆佽绁炪€佹枃鏄屻€佸鍑汇€佷富瀹㈠ぇ灏? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_calc, disabled: loading(), "鎺掔洏" }
            }
            if loading() { div { class: "loading", "鎺掔洏涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "澶箼鐩? } pre { "{data}" } }
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
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::sanshi::liuren(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍏，" }
            p { class: "page-desc", "澶у叚澹帓鐩橈細澶╁湴鐩樸€佸洓璇俱€佷笁浼犮€侀亖骞层€佽吹浜? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_calc, disabled: loading(), "鎺掔洏" }
            }
            if loading() { div { class: "loading", "鎺掔洏涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍏，鐩? } pre { "{data}" } }
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
        let req = serde_json::json!({ "coins": coins() });
        let fut = services::astro::api_request("POST", "/liuyao/cast", Some(&req));
        spawn(async move {
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
        div { class: "page",
            h2 { "鍏埢" }
            p { class: "page-desc", "鍏埢璧峰崷锛氶摐閽辨憞鍗︼紝杈撳叆鍏鏁板€硷紙6/7/8/9锛夛紝閫楀彿鍒嗛殧" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group",
                        label { "鍏閾滈挶鏁? }
                        input { r#type: "text", placeholder: "濡? 6,7,8,6,9,7", value: "{coins}", oninput: move |evt| coins.set(evt.value()) }
                    }
                }
                div { class: "form-row",
                    button { class: "submit-btn", onclick: on_cast, disabled: loading(), "璧峰崷" }
                    button { class: "submit-btn secondary", onclick: on_random, "闅忔満" }
                }
            }
            if loading() { div { class: "loading", "璧峰崷涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍗﹁薄" } pre { "{data}" } }
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
        let fut = services::calendar::get_jieqi(y);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鑺傛皵鐩? }
            p { class: "page-desc", "鏌ヨ浜屽崄鍥涜妭姘旂簿纭椂鍒? }

            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group",
                        label { "骞翠唤" }
                        input {
                            r#type: "number",
                            value: "{year}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<i32>() { year.set(v); }
                            },
                        }
                    }
                }
                button { class: "submit-btn", onclick: on_query, disabled: loading(), "鏌ヨ鑺傛皵" }
            }

            if loading() { div { class: "loading", "鏌ヨ涓?.." } }

            if let Some(ref data) = *result.read() {
                if let Some(list) = data.as_array() {
                    div { class: "result-card",
                        h3 { "{year()}骞?浜屽崄鍥涜妭姘? }
                        div { class: "jieqi-grid",
                            for (i, jq) in list.iter().enumerate() {
                                div { class: "jieqi-item",
                                    div { class: "jieqi-name",
                                        {jq.get("name_zh").and_then(|v| v.as_str()).unwrap_or("?")}
                                    }
                                    div { class: "jieqi-date",
                                        {jq.get("datetime").and_then(|v| v.as_str()).unwrap_or("?")}
                                    }
                                    div { class: "jieqi-type",
                                        {if jq.get("is_jie").and_then(|v| v.as_bool()).unwrap_or(false) { "鑺? } else { "姘? }}
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
        let endpoint = if active_tab() == "ming_gua" { "/fengshui/ming-gua" } else { "/fengshui/flying-stars" };
        let req = if active_tab() == "ming_gua" {
            serde_json::json!({ "year": year(), "gender": gender() })
        } else {
            serde_json::json!({ "build_year": build_year(), "facing": facing() })
        };
        let fut = services::astro::api_request("POST", endpoint, Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "椋庢按" }
            p { class: "page-desc", "鍏畢鍛藉崷銆佺巹绌洪鏄熴€佷笁鍏冧節杩? }
            div { class: "form-card",
                div { class: "tab-buttons",
                    button { class: if active_tab() == "ming_gua" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| active_tab.set("ming_gua".to_string()), "鍏畢鍛藉崷" }
                    button { class: if active_tab() == "flying_stars" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| active_tab.set("flying_stars".to_string()), "鐜勭┖椋炴槦" }
                }
                if active_tab() == "ming_gua" {
                    div { class: "form-row",
                        div { class: "form-group", label { "鍑虹敓骞翠唤" }
                            input { r#type: "number", value: "{year}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<i32>() { year.set(v); } } } }
                        div { class: "form-group", label { "鎬у埆" }
                            select { value: "{gender}", onchange: move |evt| gender.set(evt.value()),
                                option { value: "male", "鐢? } option { value: "female", "濂? } } }
                    }
                } else {
                    div { class: "form-row",
                        div { class: "form-group", label { "寤烘埧骞翠唤" }
                            input { r#type: "number", value: "{build_year}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<i32>() { build_year.set(v); } } } }
                        div { class: "form-group", label { "鏈濆悜(搴?" }
                            input { r#type: "number", step: "0.1", value: "{facing}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { facing.set(v); } } } }
                    }
                }
                button { class: "submit-btn", onclick: on_calc, disabled: loading(), "璁＄畻" }
            }
            if loading() { div { class: "loading", "璁＄畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "椋庢按缁撴灉" } pre { "{data}" } }
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
        ("jinkou", "閲戝彛璇€"), ("jingjue", "鑽嗚瘈"), ("shenyishu", "绁炴槗鏁?),
        ("wuzhao", "浜斿厗"), ("taixuan", "澶巹"), ("xianqin", "鍏堢Е鍗犲崪"),
    ];

    let on_calc = move |_| {
        loading.set(true);
        let (endpoint, req) = match active_tab().as_str() {
            "jinkou" => ("/jinkou/calculate", serde_json::json!({ "datetime": datetime(), "di_fen": di_fen() })),
            "jingjue" => ("/jingjue/calculate", serde_json::json!({ "birth": { "datetime": datetime() }, "query_year": chrono::Local::now().year() })),
            "shenyishu" => ("/shenyishu/calculate", serde_json::json!({ "num1": num1(), "num2": num2(), "num3": num3() })),
            "wuzhao" => ("/wuzhao/calculate", serde_json::json!({ "question": question() })),
            "taixuan" => ("/taixuan/calculate", serde_json::json!({ "seed": seed() })),
            "xianqin" => ("/xianqin/divination", serde_json::json!({ "seed": seed(), "method": "钃嶈崏" })),
            _ => ("/jinkou/calculate", serde_json::json!({})),
        };
        let fut = services::astro::api_request("POST", endpoint, Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍏朵粬鍗滄硶" }
            p { class: "page-desc", "閲戝彛璇€銆佽崋璇€銆佺鏄撴暟銆佷簲鍏嗐€佸お鐜勩€佸厛绉﹀崰鍗? }
            div { class: "form-card",
                div { class: "tab-buttons",
                    for (key, label) in &tabs {
                        button { class: if active_tab() == *key { "tab-btn active" } else { "tab-btn" },
                            onclick: move |_| active_tab.set(key.to_string()), "{label}" }
                    }
                }
                if active_tab() == "jinkou" {
                    div { class: "form-row",
                        div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                            input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                        div { class: "form-group", label { "鍦板垎" }
                            input { r#type: "text", value: "{di_fen}", oninput: move |evt| di_fen.set(evt.value()) } }
                    }
                } else if active_tab() == "shenyishu" {
                    div { class: "form-row",
                        div { class: "form-group", label { "鏁颁竴" }
                            input { r#type: "number", value: "{num1}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num1.set(v); } } } }
                        div { class: "form-group", label { "鏁颁簩" }
                            input { r#type: "number", value: "{num2}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num2.set(v); } } } }
                        div { class: "form-group", label { "鏁颁笁" }
                            input { r#type: "number", value: "{num3}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num3.set(v); } } } }
                    }
                } else if active_tab() == "wuzhao" {
                    div { class: "form-row",
                        div { class: "form-group", label { "闂簨" }
                            input { r#type: "text", value: "{question}", oninput: move |evt| question.set(evt.value()) } }
                    }
                } else if active_tab() == "taixuan" || active_tab() == "xianqin" {
                    div { class: "form-row",
                        div { class: "form-group", label { "绉嶅瓙鏁? }
                            input { r#type: "number", value: "{seed}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { seed.set(v); } } } }
                    }
                } else {
                    div { class: "form-row",
                        div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                            input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                    }
                }
                button { class: "submit-btn", onclick: on_calc, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鎺ㄧ畻缁撴灉" } pre { "{data}" } }
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
        let req = serde_json::json!({ "message": message() });
        let fut = services::ai::chat(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "AI 鍒嗘瀽" }
            p { class: "page-desc", "澶氭ā鍨嬫帴鍏ャ€佹祦寮忓璇濄€佸懡鐞嗚В璇? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鎻愰棶" }
                        textarea { value: "{message}", oninput: move |evt| message.set(evt.value()),
                            placeholder: "杈撳叆鍛界悊鍒嗘瀽闂...", rows: "4" } }
                }
                button { class: "submit-btn", onclick: on_send, disabled: loading(), "鍙戦€? }
            }
            if loading() { div { class: "loading", "AI鎬濊€冧腑..." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "AI 鍥炲" } pre { "{data}" } }
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
        let req = serde_json::json!({ "latitude": latitude(), "longitude": longitude() });
        let fut = services::astro::planetarium_current(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "澶╂枃棣? }
            p { class: "page-desc", "瀹炴椂澶╄薄锛氬お闃虫槦搴с€佹湀鐩搞€佸彲瑙佽鏄熶綅缃? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "绾害" }
                        input { r#type: "number", step: "0.0001", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group", label { "缁忓害" }
                        input { r#type: "number", step: "0.0001", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                }
                button { class: "submit-btn", onclick: on_query, disabled: loading(), "鏌ヨ澶╄薄" }
            }
            if loading() { div { class: "loading", "鏌ヨ涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "褰撳墠澶╄薄" } pre { "{data}" } }
            }
        }
    }
}

// ============ 涓囧勾鍘嗭紙榛勫巻锛?============

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
        let fut = services::calendar::solar_to_lunar(y, m, d);
        spawn(async move {
            match fut.await {
                Ok(data) => { lunar_result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    let on_eclipses = move |_| {
        loading.set(true);
        let y = year();
        let fut = services::calendar::get_eclipses(y);
        spawn(async move {
            match fut.await {
                Ok(data) => { eclipse_result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    let on_ganzhi = move |_| {
        loading.set(true);
        let y = year(); let m = month(); let d = day();
        let fut = services::calendar::get_ganzhi(y, m, d);
        spawn(async move {
            match fut.await {
                Ok(data) => { ganzhi_result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "榛勫巻 路 涓囧勾鍘? }
            p { class: "page-desc", "瀵挎槦澶╂枃鍘?鈥斺€?鍏巻/鍐滃巻/鍥炲巻涓夊巻杞崲銆佹棩鏈堥銆佸共鏀妭姘? }

            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group",
                        label { "骞? }
                        input {
                            r#type: "number",
                            value: "{year}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<i32>() { year.set(v); }
                            },
                        }
                    }
                    div { class: "form-group",
                        label { "鏈? }
                        input {
                            r#type: "number",
                            min: "1",
                            max: "12",
                            value: "{month}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<u32>() { month.set(v); }
                            },
                        }
                    }
                    div { class: "form-group",
                        label { "鏃? }
                        input {
                            r#type: "number",
                            min: "1",
                            max: "31",
                            value: "{day}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<u32>() { day.set(v); }
                            },
                        }
                    }
                }

                div { class: "tab-buttons",
                    button {
                        class: if active_tab() == "lunar" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| { active_tab.set("lunar".to_string()); },
                        "鍏巻杞啘鍘?
                    }
                    button {
                        class: if active_tab() == "ganzhi" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| { active_tab.set("ganzhi".to_string()); },
                        "骞叉敮鏌ヨ"
                    }
                    button {
                        class: if active_tab() == "eclipse" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| { active_tab.set("eclipse".to_string()); },
                        "鏃ユ湀椋?
                    }
                }

                div { class: "tab-content",
                    if active_tab() == "lunar" {
                        div {
                            button {
                                class: "submit-btn",
                                onclick: on_solar_to_lunar,
                                disabled: loading(),
                                "鏌ヨ鍐滃巻"
                            }
                            if loading() { div { class: "loading", "鏌ヨ涓?.." } }
                            if let Some(ref data) = *lunar_result.read() {
                                div { class: "result-card lunar-card",
                                    h3 { "鍐滃巻杞崲缁撴灉" }
                                    div { class: "lunar-info",
                                        div { class: "lunar-row",
                                            span { class: "lunar-label", "鍐滃巻鏃ユ湡: " }
                                            span { class: "lunar-value",
                                                {data.get("year").and_then(|v| v.as_i64()).unwrap_or(0)} "骞?
                                                {data.get("month_name_zh").and_then(|v| v.as_str()).unwrap_or("?")}
                                                {data.get("day_name_zh").and_then(|v| v.as_str()).unwrap_or("?")}
                                            }
                                        }
                                        div { class: "lunar-row",
                                            span { class: "lunar-label", "骞村共鏀? " }
                                            span { class: "lunar-value", {data.get("year_ganzhi").and_then(|v| v.as_str()).unwrap_or("?")} }
                                        }
                                        div { class: "lunar-row",
                                            span { class: "lunar-label", "鐢熻倴: " }
                                            span { class: "lunar-value", {data.get("zodiac_animal").and_then(|v| v.as_str()).unwrap_or("?")} }
                                        }
                                        if let Some(leap) = data.get("is_leap_month").and_then(|v| v.as_bool()) {
                                            if leap {
                                                div { class: "lunar-row",
                                                    span { class: "lunar-label lunar-leap", "锛堥棸鏈堬級" }
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
                                class: "submit-btn",
                                onclick: on_ganzhi,
                                disabled: loading(),
                                "鏌ヨ骞叉敮"
                            }
                            if loading() { div { class: "loading", "鏌ヨ涓?.." } }
                            if let Some(ref data) = *ganzhi_result.read() {
                                div { class: "result-card",
                                    h3 { "骞叉敮淇℃伅" }
                                    table { class: "data-table",
                                        tbody {
                                            tr { td { "骞村共鏀? } td { {data.get("year_ganzhi").and_then(|v| v.as_str()).unwrap_or("?")} } }
                                            tr { td { "鐢熻倴" } td { {data.get("zodiac").and_then(|v| v.as_str()).unwrap_or("?")} } }
                                            tr { td { "骞村彿" } td { {data.get("nianhao").and_then(|v| v.as_str()).unwrap_or("?")} } }
                                        }
                                    }
                                }
                            }
                        }
                    } else if active_tab() == "eclipse" {
                        div {
                            button {
                                class: "submit-btn",
                                onclick: on_eclipses,
                                disabled: loading(),
                                "鏌ヨ鏃ユ湀椋?
                            }
                            if loading() { div { class: "loading", "鏌ヨ涓?.." } }
                            if let Some(ref data) = *eclipse_result.read() {
                                if let Some(list) = data.as_array() {
                                    div { class: "result-card",
                                        h3 { "{year()}骞?鏃ユ湀椋? }
                                        if list.is_empty() {
                                            p { class: "empty-state", "璇ュ勾鏃犳棩鏈堥" }
                                        } else {
                                            table { class: "data-table",
                                                thead {
                                                    tr {
                                                        th { "鏃ユ湡" }
                                                        th { "绫诲瀷" }
                                                        th { "椋熷垎" }
                                                    }
                                                }
                                                tbody {
                                                    for eclipse in list {
                                                        tr {
                                                            td { {eclipse.get("date").and_then(|v| v.as_str()).unwrap_or("?")} }
                                                            td { {eclipse.get("eclipse_type").and_then(|v| v.as_str()).unwrap_or("?")} }
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
        div { class: "page",
            h2 { "杈呭姪鍙傝€? }
            p { class: "page-desc", "鍏崷绫昏薄銆佸崄浜屽銆佽鍒欓€熸煡" }
            div { class: "result-card",
                h3 { "鍏崄鍥涘崷" }
                div { class: "ref-grid",
                    for gua in &["涔?, "鍧?, "灞?, "钂?, "闇€", "璁?, "甯?, "姣?, "灏忕暅", "灞?, "娉?, "鍚?, "鍚屼汉", "澶ф湁", "璋?, "璞?, "闅?, "铔?, "涓?, "瑙?, "鍣棏", "璐?, "鍓?, "澶?, "鏃犲", "澶х暅", "棰?, "澶ц繃", "鍧?, "绂?, "鍜?, "鎭?, "閬?, "澶у．", "鏅?, "鏄庡し", "瀹朵汉", "鐫?, "韫?, "瑙?, "鎹?, "鐩?, "澶?, "濮?, "钀?, "鍗?, "鍥?, "浜?, "闈?, "榧?, "闇?, "鑹?, "娓?, "褰掑", "涓?, "鏃?, "宸?, "鍏?, "娑?, "鑺?, "涓瓪", "灏忚繃", "鏃㈡祹", "鏈祹"] {
                        div { class: "ref-item", "{gua}" }
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
        div { class: "page",
            h2 { "璁剧疆" }
            p { class: "page-desc", "搴旂敤璁剧疆" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "涓婚" }
                        select { value: "{theme}", onchange: move |evt| theme.set(evt.value()),
                            option { value: "light", "娴呰壊" } option { value: "dark", "娣辫壊" } } }
                    div { class: "form-group", label { "璇█" }
                        select { value: "{language}", onchange: move |evt| language.set(evt.value()),
                            option { value: "zh", "涓枃" } option { value: "en", "English" } } }
                }
                button { class: "submit-btn", onclick: on_save, "淇濆瓨璁剧疆" }
                if saved() { div { class: "success-msg", "璁剧疆宸蹭繚瀛? } }
            }
        }
    }
}

#[component]
pub fn GuoLao() -> Element {
    rsx! {
        div { class: "page",
            h2 { "鏋滆€佹槦瀹? }
            p { class: "page-desc", "鏋滆€佹槦瀹楁帹婕斻€佷簩鍗佸叓瀹垮懡搴﹁韩搴? }
            p { "璇蜂娇鐢?涓冩斂鍥涗綑 椤甸潰杩涜鎺掔洏锛屾灉鑰佹槦瀹椾笌涓冩斂鍥涗綑鍏辩敤鍚屼竴璁＄畻寮曟搸銆? }
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
        let req = if active_tab() == "meihua" {
            serde_json::json!({ "num1": num1(), "num2": num2(), "num3": num3() })
        } else {
            serde_json::json!({ "datetime": datetime() })
        };
        let endpoint = if active_tab() == "meihua" { "/gua/meihua" } else { "/gua/meiyi" };
        let fut = services::astro::api_request("POST", endpoint, Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍗﹀崰" }
            p { class: "page-desc", "姊呰姳鏄撴暟銆佸叚鐖诲崷鍗? }
            div { class: "form-card",
                div { class: "tab-buttons",
                    button { class: if active_tab() == "meihua" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| active_tab.set("meihua".to_string()), "姊呰姳鏄撴暟" }
                    button { class: if active_tab() == "meiyi" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| active_tab.set("meiyi".to_string()), "鍏埢鍗? }
                }
                if active_tab() == "meihua" {
                    div { class: "form-row",
                        div { class: "form-group", label { "鏁颁竴" }
                            input { r#type: "number", value: "{num1}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num1.set(v); } } } }
                        div { class: "form-group", label { "鏁颁簩" }
                            input { r#type: "number", value: "{num2}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num2.set(v); } } } }
                        div { class: "form-group", label { "鏁颁笁" }
                            input { r#type: "number", value: "{num3}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num3.set(v); } } } }
                    }
                } else {
                    div { class: "form-row",
                        div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                            input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                    }
                }
                button { class: "submit-btn", onclick: on_calc, disabled: loading(), "璧峰崷" }
            }
            if loading() { div { class: "loading", "璧峰崷涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍗﹁薄" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn DunJia() -> Element {
    rsx! {
        div { class: "page",
            h2 { "閬佺敳" }
            p { class: "page-desc", "鍏敳閬併€侀潚榫欓亖銆佺櫧铏庨亖绛? }
            p { "璇蜂娇鐢?濂囬棬閬佺敳 椤甸潰杩涜鎺掔洏锛岄亖鐢蹭笌濂囬棬鍏辩敤鍚屼竴璁＄畻寮曟搸銆? }
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
        let req = serde_json::json!({ "seq": gua_seq() });
        let fut = services::astro::api_request("POST", "/gua/desc", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍗﹁薄" }
            p { class: "page-desc", "鍏崄鍥涘崷銆佸崷璞″叧绯汇€佸崷杈炵埢杈? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍗﹀簭 (0-63)" }
                        input { r#type: "number", min: "0", max: "63", value: "{gua_seq}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { gua_seq.set(v); } } } }
                }
                button { class: "submit-btn", onclick: on_query, disabled: loading(), "鏌ヨ" }
            }
            if loading() { div { class: "loading", "鏌ヨ涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍗﹁薄璇︽儏" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "page about-page",
            h2 { "鍏充簬Divines" }
            p { "鐗堟湰: 0.1.0 (绾?Rust 閲嶅啓)" }
            p { "Divines 鏄竴濂楁闈㈢鐨勭巹瀛﹀伐浣滅珯銆? }
            p { "瑗挎柟鍗犳槦鐨勬湰鍛姐€佹帹杩愩€佸叧绯荤洏锛岃繛鍚屽叓瀛椼€佺传寰€佸闂ㄣ€佸叚澹€佸お涔欒繖浜涗腑鍥戒紶缁熸湳鏁帮紝琚斁杩涘悓涓€涓簲鐢ㄩ噷銆? }
            p { "鏈増鏈娇鐢?Rust 鍏ㄦ爤閲嶅啓锛屽墠绔娇鐢?Dioxus 0.7.9銆? }
            p { "鍘熼」鐩湴鍧€: https://github.com/Horace-Maxwell/divines-Web-App-comprehensively-improved-MacOS" }
            p { "涓囧勾鍘嗗弬鑰? 瀵挎槦澶╂枃鍘?(sxwnl)" }
            p { "璁稿彲: AGPL-3.0" }
        }
    }
}

// ============ 浼犵粺鏈暟 路 鏁扮畻涓庣鏁?============

#[component]
pub fn HuangJi() -> Element {
    let mut year = use_signal(|| chrono::Local::now().year());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "year": year() });
        let fut = services::astro::huangji_yuan_hui(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鐨囨瀬缁忎笘" }
            p { class: "page-desc", "鐨囨瀬缁忎笘鍏冧細杩愪笘鎺ㄧ畻锛屽€煎勾鍗︺€佸€间簨鍗? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group",
                        label { "骞翠唤" }
                        input {
                            r#type: "number",
                            value: "{year}",
                            oninput: move |evt| {
                                if let Ok(v) = evt.value().parse::<i32>() { year.set(v); }
                            },
                        }
                    }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card",
                    h3 { "鐨囨瀬缁忎笘缁撴灉" }
                    pre { "{data}" }
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
        let req = serde_json::json!({ "birth": { "datetime": datetime() }, "query_year": query_year() });
        let fut = services::astro::jingjue_calculate(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鑽嗚瘈" }
            p { class: "page-desc", "鑽嗚瘈娴佸勾鎺ㄦ紨锛氫互鍑虹敓鏃堕棿鎺ㄧ畻鍚勫勾杩愬娍" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                    div { class: "form-group", label { "鏌ヨ骞翠唤" }
                        input { r#type: "number", value: "{query_year}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<i32>() { query_year.set(v); } } } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鑽嗚瘈鎺ㄦ紨缁撴灉" } pre { "{data}" } }
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
        let req = serde_json::json!({ "datetime": datetime(), "di_fen": di_fen() });
        let fut = services::astro::jinkou_calculate(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "閲戝彛璇€" }
            p { class: "page-desc", "閲戝彛璇€鎺掔洏锛氭湀灏嗐€佸湴鍒嗐€佸皢绁炪€佽吹绁炪€佷汉鍏? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                    div { class: "form-group", label { "鍦板垎" }
                        input { r#type: "text", placeholder: "瀛?涓?瀵?鍗?杈?宸?鍗?鏈?鐢?閰?鎴?浜?, value: "{di_fen}", oninput: move |evt| di_fen.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺掔洏" }
            }
            if loading() { div { class: "loading", "鎺掔洏涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "閲戝彛璇€鎺掔洏缁撴灉" } pre { "{data}" } }
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
        let req = serde_json::json!({ "num1": num1(), "num2": num2(), "num3": num3() });
        let fut = services::astro::shenyishu_calculate(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "绁炴槗鏁? }
            p { class: "page-desc", "绁炴槗鏁颁笁鏁拌捣鍗︼細浠ヤ笁涓暟瀛楄捣鍗︽帹鏂悏鍑? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏁颁竴" }
                        input { r#type: "number", value: "{num1}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num1.set(v); } } } }
                    div { class: "form-group", label { "鏁颁簩" }
                        input { r#type: "number", value: "{num2}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num2.set(v); } } } }
                    div { class: "form-group", label { "鏁颁笁" }
                        input { r#type: "number", value: "{num3}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { num3.set(v); } } } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "璧峰崷" }
            }
            if loading() { div { class: "loading", "璧峰崷涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "绁炴槗鏁扮粨鏋? } pre { "{data}" } }
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
        let req = serde_json::json!({ "question": question() });
        let fut = services::astro::wuzhao_calculate(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "浜斿厗" }
            p { class: "page-desc", "浜斿厗浜旇鍗犲崪锛氫互闂簨涓哄紩锛屾帹婕斾簲琛屽厗璞? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "闂簨" }
                        input { r#type: "text", placeholder: "杈撳叆鎮ㄦ兂闂殑浜?..", value: "{question}", oninput: move |evt| question.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鍗犲崪" }
            }
            if loading() { div { class: "loading", "鍗犲崪涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "浜斿厗鍗犲崪缁撴灉" } pre { "{data}" } }
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
        let req = serde_json::json!({ "seed": seed() });
        let fut = services::astro::taixuan_calculate(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "澶巹" }
            p { class: "page-desc", "澶巹绛硶锛氶璧炴帹绠楋紝81棣?29璧? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "绉嶅瓙鏁? }
                        input { r#type: "number", value: "{seed}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { seed.set(v); } } } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "澶巹缁撴灉" } pre { "{data}" } }
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
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::beiji_calculate(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍖楁瀬绁炴暟" }
            p { class: "page-desc", "鍖楁瀬绁炴暟鎺掔洏锛氬叓瀛楁帹绠椼€佸叓鍗﹀畾浣嶃€佺鏁版潯鏂? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺掔洏" }
            }
            if loading() { div { class: "loading", "鎺掔洏涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍖楁瀬绁炴暟鎺掔洏缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn CeTian() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::cetian_calculate(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "绛栧ぉ" }
            p { class: "page-desc", "绛栧ぉ鏄熷懡鎺掔洏锛?8鏄熷銆佷竷鏀夸綅缃€佷簲琛屽厓绱? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺掔洏" }
            }
            if loading() { div { class: "loading", "鎺掔洏涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "绛栧ぉ鎺掔洏缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn ChunZi() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::chunzi_calculate(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鏄ュ瓙" }
            p { class: "page-desc", "鏄ュ瓙鍛界悊鎺掔洏锛氬洓鏌辨帹绠椼€佷簲琛屽垎鏋? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺掔洏" }
            }
            if loading() { div { class: "loading", "鎺掔洏涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鏄ュ瓙鎺掔洏缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn FenJing() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::fendjing_calculate(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍒嗙粡" }
            p { class: "page-desc", "鍒嗙粡鍏崷瀹氫綅锛氬叚鍗佸洓鍗﹀睘缁忓垎缁忔帹绠? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍒嗙粡鎺ㄧ畻缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn NanJi() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::nanji_calculate(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍗楁瀬绁炴暟" }
            p { class: "page-desc", "鍗楁瀬绁炴暟鏉℃枃鎺ㄧ畻锛氫互鍑虹敓鏃堕棿鎺ㄧ畻绁炴暟鏉℃枃" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍗楁瀬绁炴暟缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn ShaoZi() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::shaozi_calculate(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "閭靛瓙绁炴暟" }
            p { class: "page-desc", "閭靛瓙绁炴暟锛氬厓浼氳繍涓栥€?4鍗﹀瘑閽ャ€佹潯鏂囨帹绠? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "閭靛瓙绁炴暟缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn TieBan() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::tieban_calculate(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "閾佹澘绁炴暟" }
            p { class: "page-desc", "閾佹澘绁炴暟锛氳€冩潯鏂囨帹绠楋紝12000鏉℃潯鏂囧搴? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "閾佹澘绁炴暟缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn XianQin() -> Element {
    let mut seed = use_signal(|| 0u32);
    let mut method = use_signal(|| "钃嶈崏".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "seed": seed(), "method": method() });
        let fut = services::astro::xianqin_divination(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍏堢Е鍗犲崪" }
            p { class: "page-desc", "鍏堢Е榫熷崪銆佽搷鑽夊崰銆佸叓鍗︿箣鍗? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "绉嶅瓙鏁? }
                        input { r#type: "number", value: "{seed}", oninput: move |evt| { if let Ok(v) = evt.value().parse::<u32>() { seed.set(v); } } } }
                    div { class: "form-group", label { "鍗犳硶" }
                        select { value: "{method}", onchange: move |evt| method.set(evt.value()),
                            option { value: "钃嶈崏", "钃嶈崏鍗? }
                            option { value: "榫熷崪", "榫熷崪" }
                            option { value: "鍏崷", "鍏崷涔嬪崰" }
                        } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鍗犲崪" }
            }
            if loading() { div { class: "loading", "鍗犲崪涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍏堢Е鍗犲崪缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

// ============ 瑗挎柟鍗犳槦 路 涓撻」 ============

#[component]
pub fn AstroHellenistic() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut timezone = use_signal(|| 8.0_f64);
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({
            "datetime": datetime(), "latitude": latitude(),
            "longitude": longitude(), "timezone": timezone(),
        });
        let fut = services::astro::api_request("POST", "/astro/hellenistic", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "甯岃厞鏄熸湳" }
            p { class: "page-desc", "鏁村鍒躲€佺晫銆佸瑙傜瓑甯岃厞鏄熸湳鍒嗘瀽" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "绾害" }
                        input { r#type: "number", step: "0.0001", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group", label { "缁忓害" }
                        input { r#type: "number", step: "0.0001", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                    div { class: "form-group", label { "鏃跺尯" }
                        input { r#type: "number", step: "0.5", value: "{timezone}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { timezone.set(v); } } } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鍒嗘瀽" }
            }
            if loading() { div { class: "loading", "鍒嗘瀽涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "甯岃厞鏄熸湳鍒嗘瀽缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn AstroHorary() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut question = use_signal(|| String::new());
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({
            "datetime": datetime(), "question": question(),
            "latitude": latitude(), "longitude": longitude(),
        });
        let fut = services::astro::api_request("POST", "/astro/horary", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍗滃崷鍗犳槦" }
            p { class: "page-desc", "鍗滃崷鐩樺垎鏋愶細鐢ㄤ簨寰佽薄鏄熴€佹湀浜┖浜°€佸厜绾夸紶閫? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "闂簨鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "闂" }
                        input { r#type: "text", placeholder: "杈撳叆浣犵殑闂...", value: "{question}", oninput: move |evt| question.set(evt.value()) } }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "绾害" }
                        input { r#type: "number", step: "0.0001", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group", label { "缁忓害" }
                        input { r#type: "number", step: "0.0001", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "璧峰崷鍒嗘瀽" }
            }
            if loading() { div { class: "loading", "鍒嗘瀽涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍗滃崷鍗犳槦缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn AstroElectional() -> Element {
    let mut start_date = use_signal(|| String::new());
    let mut end_date = use_signal(|| String::new());
    let mut purpose = use_signal(|| String::new());
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({
            "start_date": start_date(), "end_date": end_date(),
            "purpose": purpose(), "latitude": latitude(), "longitude": longitude(),
        });
        let fut = services::astro::api_request("POST", "/astro/electional", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鎷╂椂鍗犳槦" }
            p { class: "page-desc", "鍚夋椂鎷╅€夛細鏍规嵁鐩殑閫夋嫨鏈€浣虫椂闂? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "寮€濮嬫棩鏈? }
                        input { r#type: "datetime-local", value: "{start_date}", oninput: move |evt| start_date.set(evt.value()) } }
                    div { class: "form-group", label { "缁撴潫鏃ユ湡" }
                        input { r#type: "datetime-local", value: "{end_date}", oninput: move |evt| end_date.set(evt.value()) } }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "鎷╂椂鐩殑" }
                        input { r#type: "text", placeholder: "濡傦細缁撳銆佸紑涓氥€佸嚭琛?..", value: "{purpose}", oninput: move |evt| purpose.set(evt.value()) } }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "绾害" }
                        input { r#type: "number", step: "0.0001", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group", label { "缁忓害" }
                        input { r#type: "number", step: "0.0001", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎷╂椂" }
            }
            if loading() { div { class: "loading", "鎷╂椂涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鎷╂椂缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn AstroMundane() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut place = use_signal(|| String::new());
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut active_tab = use_signal(|| "mundane".to_string());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({
            "datetime": datetime(), "place": place(),
            "latitude": latitude(), "longitude": longitude(),
        });
        let endpoint = if active_tab() == "ingress" { "/astro/aries-ingress" } else { "/astro/mundane" };
        let fut = services::astro::api_request("POST", endpoint, Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "涓栦織鍗犳槦" }
            p { class: "page-desc", "涓栬繍鐩樸€佸浗瀹剁洏銆丄ries Ingress" }
            div { class: "form-card",
                div { class: "tab-buttons",
                    button { class: if active_tab() == "mundane" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| active_tab.set("mundane".to_string()), "涓栬繍鐩? }
                    button { class: if active_tab() == "ingress" { "tab-btn active" } else { "tab-btn" },
                        onclick: move |_| active_tab.set("ingress".to_string()), "Aries Ingress" }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                    div { class: "form-group", label { "鍦扮偣" }
                        input { r#type: "text", placeholder: "濡傦細鍖椾含", value: "{place}", oninput: move |evt| place.set(evt.value()) } }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "绾害" }
                        input { r#type: "number", step: "0.0001", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group", label { "缁忓害" }
                        input { r#type: "number", step: "0.0001", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鍒嗘瀽" }
            }
            if loading() { div { class: "loading", "鍒嗘瀽涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "涓栦織鍗犳槦缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn AstroGermany() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut timezone = use_signal(|| 8.0_f64);
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({
            "datetime": datetime(), "latitude": latitude(),
            "longitude": longitude(), "timezone": timezone(),
        });
        let fut = services::astro::germany_calculate(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "寰峰浗鍗犳槦" }
            p { class: "page-desc", "姹夊牎瀛︽淳銆佸畤瀹欑敓鐗╁銆佷腑鐐圭粨鏋勩€佸绉扮偣鍒嗘瀽" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "绾害" }
                        input { r#type: "number", step: "0.0001", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group", label { "缁忓害" }
                        input { r#type: "number", step: "0.0001", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                    div { class: "form-group", label { "鏃跺尯" }
                        input { r#type: "number", step: "0.5", value: "{timezone}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { timezone.set(v); } } } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鍒嗘瀽" }
            }
            if loading() { div { class: "loading", "鍒嗘瀽涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "寰峰浗鍗犳槦鍒嗘瀽缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn AstroSynastry() -> Element {
    let mut inner_datetime = use_signal(|| String::new());
    let mut outer_datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({
            "inner": { "datetime": inner_datetime() },
            "outer": { "datetime": outer_datetime() },
        });
        let fut = services::astro::synastry_chart(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍚堢洏" }
            p { class: "page-desc", "姣旇緝鐩樺垎鏋愶細瀹綅鍙犲姞銆佺浉浣嶄簰鍔? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group",
                        label { "鍐呯洏鍑虹敓鏃堕棿" }
                        input { r#type: "datetime-local", value: "{inner_datetime}", oninput: move |evt| inner_datetime.set(evt.value()) }
                    }
                }
                div { class: "form-row",
                    div { class: "form-group",
                        label { "澶栫洏鍑虹敓鏃堕棿" }
                        input { r#type: "datetime-local", value: "{outer_datetime}", oninput: move |evt| outer_datetime.set(evt.value()) }
                    }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "姣旇緝鍚堢洏" }
            }
            if loading() { div { class: "loading", "鍒嗘瀽涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card",
                    h3 { "鍚堢洏缁撴灉" }
                    pre { "{data}" }
                }
            }
        }
    }
}

#[component]
pub fn AstroAcg() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::acg_lines(&req);
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "ACG 鏄熶綋鍦板浘" }
            p { class: "page-desc", "鍗犳槦鍦扮悊瀹氫綅(ACG)锛氭槦浣撳湪涓栫晫鍦板浘涓婄殑澶╅《/澶╁簳/涓婂崌/涓嬮檷绾? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "璁＄畻ACG" }
            }
            if loading() { div { class: "loading", "璁＄畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "ACG缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn AstroRectification() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut events = use_signal(|| String::new());
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({
            "approx_datetime": datetime(), "events": events(),
            "latitude": latitude(), "longitude": longitude(),
        });
        let fut = services::astro::api_request("POST", "/astro/rectification", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鐢熸椂鏍℃" }
            p { class: "page-desc", "Trutine of Hermes銆佷汉鐢熶簨浠跺弽鎺ㄧ敓鏃舵牎姝? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "澶ц嚧鍑虹敓鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "浜虹敓浜嬩欢锛堟瘡琛屼竴涓紝鏍煎紡锛氭棩鏈?浜嬩欢鎻忚堪锛? }
                        textarea { value: "{events}", oninput: move |evt| events.set(evt.value()),
                            placeholder: "濡傦細\n2000-01-01,缁撳\n2005-06-15,鐢熷瓙", rows: "4" } }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "绾害" }
                        input { r#type: "number", step: "0.0001", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group", label { "缁忓害" }
                        input { r#type: "number", step: "0.0001", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鏍℃" }
            }
            if loading() { div { class: "loading", "鏍℃涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鐢熸椂鏍℃缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

// ============ 宸ュ叿 路 楠板瓙鍗犲崪 / 浜屽崄鍏 ============

#[component]
pub fn Dice() -> Element {
    let mut question = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_roll = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "question": question() });
        let fut = services::astro::api_request("POST", "/dice/roll", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "楠板瓙鍗犲崪" }
            p { class: "page-desc", "鍗犳槦楠板瓙銆佸崄浜屽鑹插瓙锛氶殢鏈烘幏楠拌В璇? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "闂锛堝彲閫夛級" }
                        input { r#type: "text", placeholder: "榛樺康浣犵殑闂...", value: "{question}", oninput: move |evt| question.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_roll, disabled: loading(), "鎺烽瀛? }
            }
            if loading() { div { class: "loading", "鎺烽涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "楠板瓙缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn Su28() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::api_request("POST", "/su28/calculate", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "浜屽崄鍏" }
            p { class: "page-desc", "浜屽崄鍏鎺ㄦ紨锛氬綋鍓嶆椂鍒讳簩鍗佸叓瀹垮害鏁般€佸鐩? }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鏌ヨ" }
            }
            if loading() { div { class: "loading", "鏌ヨ涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "浜屽崄鍏" } pre { "{data}" } }
            }
        }
    }
}

// ============ 閭靛瓙绯诲垪 ============

#[component]
pub fn SzBaGua() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::api_request("POST", "/sz/bagua", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "閭靛瓙鍏崷" }
            p { class: "page-desc", "閭靛瓙鍏崷鏂逛綅锛氬厛澶╁叓鍗︽帓甯冧笌鎺ㄧ畻" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "閭靛瓙鍏崷缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn SzDunJia() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::api_request("POST", "/sz/dunjia", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "閭靛瓙閬佺敳" }
            p { class: "page-desc", "閭靛瓙閬佺敳鎺掔洏锛氶偟搴疯妭閬佺敳鎺ㄦ紨" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺掔洏" }
            }
            if loading() { div { class: "loading", "鎺掔洏涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "閭靛瓙閬佺敳缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn SzTaiYi() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::api_request("POST", "/sz/taiyi", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "閭靛瓙澶箼" }
            p { class: "page-desc", "閭靛瓙澶箼鎺掔洏锛氶偟搴疯妭澶箼绁炴暟鎺ㄦ紨" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺掔洏" }
            }
            if loading() { div { class: "loading", "鎺掔洏涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "閭靛瓙澶箼缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

// ============ 閭靛瓙鎵╁睍 ============

#[component]
pub fn SzFangWei() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::api_request("POST", "/sz/fangwei", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "閭靛瓙鏂逛綅" }
            p { class: "page-desc", "閭靛瓙鏂逛綅鎺ㄦ紨锛氶偟搴疯妭鏂逛綅绯荤粺鎺ㄦ紨" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "閭靛瓙鏂逛綅缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn SzFengYe() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::api_request("POST", "/sz/fengye", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "閭靛瓙鍒嗛噹" }
            p { class: "page-desc", "閭靛瓙鍒嗛噹鎺ㄦ紨锛氶偟搴疯妭鍒嗛噹绯荤粺鎺ㄦ紨" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "閭靛瓙鍒嗛噹缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn SzNiXiang() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::api_request("POST", "/sz/nixiang", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "閭靛瓙閫嗚薄" }
            p { class: "page-desc", "閭靛瓙閫嗚薄鎺ㄦ紨锛氶偟搴疯妭閫嗚薄绯荤粺鎺ㄦ紨" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "閭靛瓙閫嗚薄缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

#[component]
pub fn SzSign() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::api_request("POST", "/sz/sign", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "閭靛瓙鏄熷骇" }
            p { class: "page-desc", "閭靛瓙鏄熷骇鎺ㄦ紨锛氶偟搴疯妭鏄熷骇绯荤粺鎺ㄦ紨" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "閭靛瓙鏄熷骇缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

// ============ 鍛界悊鍏朵粬 ============

#[component]
pub fn MingOther() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut timezone = use_signal(|| 8.0_f64);
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({
            "datetime": datetime(),
            "latitude": latitude(),
            "longitude": longitude(),
            "timezone": timezone(),
        });
        let fut = services::astro::api_request("POST", "/mingother/calculate", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍛界悊鍏朵粬" }
            p { class: "page-desc", "寤剁Е銆佸綕鍗滅瓑鍛界悊鏈暟鎺ㄦ紨" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鍑虹敓鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "绾害" }
                        input { r#type: "number", step: "0.0001", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group", label { "缁忓害" }
                        input { r#type: "number", step: "0.0001", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                    div { class: "form-group", label { "鏃跺尯" }
                        input { r#type: "number", step: "0.5", value: "{timezone}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { timezone.set(v); } } } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍛界悊鍏朵粬缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

// ============ 瀹垮崰 ============

#[component]
pub fn SuZhan() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut latitude = use_signal(|| 39.9042_f64);
    let mut longitude = use_signal(|| 116.4074_f64);
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({
            "datetime": datetime(),
            "latitude": latitude(),
            "longitude": longitude(),
        });
        let fut = services::astro::api_request("POST", "/suzhan/calculate", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "瀹垮崰" }
            p { class: "page-desc", "浜屽崄鍏鍗犲崪锛氫互浜屽崄鍏鎺ㄦ紨鍚夊嚩" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                div { class: "form-row",
                    div { class: "form-group", label { "绾害" }
                        input { r#type: "number", step: "0.0001", value: "{latitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { latitude.set(v); } } } }
                    div { class: "form-group", label { "缁忓害" }
                        input { r#type: "number", step: "0.0001", value: "{longitude}",
                            oninput: move |evt| { if let Ok(v) = evt.value().parse::<f64>() { longitude.set(v); } } } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鍗犲崪" }
            }
            if loading() { div { class: "loading", "鍗犲崪涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "瀹垮崰缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

// ============ 閫氳娉?============

#[component]
pub fn TongSheFa() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::api_request("POST", "/tongshefa/calculate", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "閫氳娉? }
            p { class: "page-desc", "閫氳娉曟帹婕旓細浼犵粺鏈暟閫氳鎺ㄦ紨" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鎺ㄧ畻" }
            }
            if loading() { div { class: "loading", "鎺ㄧ畻涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "閫氳娉曠粨鏋? } pre { "{data}" } }
            }
        }
    }
}

// ============ 鍏朵粬鍗?============

#[component]
pub fn OtherBu() -> Element {
    let mut datetime = use_signal(|| String::new());
    let mut result = use_signal(|| None::<serde_json::Value>);
    let mut loading = use_signal(|| false);

    let on_submit = move |_| {
        loading.set(true);
        let req = serde_json::json!({ "datetime": datetime() });
        let fut = services::astro::api_request("POST", "/otherbu/calculate", Some(&req));
        spawn(async move {
            match fut.await {
                Ok(data) => { result.set(Some(data)); loading.set(false); }
                Err(_) => { loading.set(false); }
            }
        });
    };

    rsx! {
        div { class: "page",
            h2 { "鍏朵粬鍗滄硶" }
            p { class: "page-desc", "鍏朵粬鍗滄硶鎺ㄦ紨锛氶笩鍗溿€佸吔鍗溿€佺鍗滅瓑浼犵粺鍗滄硶" }
            div { class: "form-card",
                div { class: "form-row",
                    div { class: "form-group", label { "鏃ユ湡鏃堕棿" }
                        input { r#type: "datetime-local", value: "{datetime}", oninput: move |evt| datetime.set(evt.value()) } }
                }
                button { class: "submit-btn", onclick: on_submit, disabled: loading(), "鍗犲崪" }
            }
            if loading() { div { class: "loading", "鍗犲崪涓?.." } }
            if let Some(ref data) = *result.read() {
                div { class: "result-card", h3 { "鍏朵粬鍗滄硶缁撴灉" } pre { "{data}" } }
            }
        }
    }
}

// ============ 404 ============

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "page not-found",
            h2 { "404 - 椤甸潰鏈壘鍒? }
            p { "璺緞: {route.join("/")}" }
        }
    }
}