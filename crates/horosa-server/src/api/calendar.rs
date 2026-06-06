// 星阙 Horosa - 节气/黄历/万年历 API
// 参考原项目: astropy/websrv/webjieqisrv.py
// 寿星万年历: sxwnl

use axum::{extract::{State, Query}, Json};
use std::sync::Arc;
use crate::AppState;
use chrono::Datelike;

pub async fn jieqi(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CalendarQuery>,
) -> Json<serde_json::Value> {
    let year = params.year.unwrap_or_else(|| chrono::Utc::now().year());
    let jieqi_list = state.sxwnl.get_year_jieqi(year);
    Json(serde_json::to_value(jieqi_list).unwrap_or_default())
}

pub async fn almanac(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CalendarQuery>,
) -> Json<serde_json::Value> {
    let now = chrono::Utc::now();
    let year = params.year.unwrap_or(now.year());
    let month = params.month.unwrap_or(now.month());
    let day = params.day.unwrap_or(now.day());

    let almanac = state.jieqi.get_almanac(year, month, day);
    Json(serde_json::to_value(almanac).unwrap_or_default())
}

/// 公历转农历（寿星万年历）
pub async fn solar_to_lunar(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CalendarQuery>,
) -> Json<serde_json::Value> {
    let now = chrono::Utc::now();
    let year = params.year.unwrap_or(now.year());
    let month = params.month.unwrap_or(now.month());
    let day = params.day.unwrap_or(now.day());

    let lunar = state.sxwnl.solar_to_lunar(year, month, day);
    Json(serde_json::to_value(lunar).unwrap_or_default())
}

/// 农历转公历
pub async fn lunar_to_solar(
    State(state): State<Arc<AppState>>,
    Query(params): Query<LunarQuery>,
) -> Json<serde_json::Value> {
    let (y, m, d) = state.sxwnl.lunar_to_solar(
        params.lunar_year,
        params.lunar_month,
        params.lunar_day,
        params.is_leap.unwrap_or(false),
    );
    Json(serde_json::json!({ "year": y, "month": m, "day": d }))
}

/// 公历转回历
pub async fn solar_to_islamic(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CalendarQuery>,
) -> Json<serde_json::Value> {
    let now = chrono::Utc::now();
    let year = params.year.unwrap_or(now.year());
    let month = params.month.unwrap_or(now.month());
    let day = params.day.unwrap_or(now.day());

    let (y, m, d) = state.sxwnl.solar_to_islamic(year, month, day);
    Json(serde_json::json!({ "year": y, "month": m, "day": d }))
}

/// 干支查询
pub async fn ganzhi(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CalendarQuery>,
) -> Json<serde_json::Value> {
    let now = chrono::Utc::now();
    let year = params.year.unwrap_or(now.year());
    let month = params.month.unwrap_or(now.month());
    let day = params.day.unwrap_or(now.day());

    let year_ganzhi = state.sxwnl.get_year_ganzhi(year);
    let zodiac = state.sxwnl.get_zodiac(year);
    let nianhao = state.sxwnl.get_nianhao(year);

    Json(serde_json::json!({
        "year_ganzhi": year_ganzhi,
        "zodiac": zodiac,
        "nianhao": nianhao,
    }))
}

/// 日月食查询
pub async fn eclipses(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CalendarQuery>,
) -> Json<serde_json::Value> {
    let now = chrono::Utc::now();
    let year = params.year.unwrap_or(now.year());

    let eclipse_list = state.sxwnl.calc_eclipses(year);
    Json(serde_json::to_value(eclipse_list).unwrap_or_default())
}

/// 城市经纬度查询
pub async fn city_coords(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CityQuery>,
) -> Json<serde_json::Value> {
    match state.sxwnl.get_city_coords(&params.name) {
        Some((lat, lon)) => Json(serde_json::json!({ "latitude": lat, "longitude": lon })),
        None => Json(serde_json::json!({ "error": "未找到该城市" })),
    }
}

#[derive(serde::Deserialize)]
pub struct CalendarQuery {
    pub year: Option<i32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
}

#[derive(serde::Deserialize)]
pub struct LunarQuery {
    pub lunar_year: i32,
    pub lunar_month: u32,
    pub lunar_day: u32,
    pub is_leap: Option<bool>,
}

#[derive(serde::Deserialize)]
pub struct CityQuery {
    pub name: String,
}