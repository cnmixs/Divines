// Divines - 八字 API
// 参考原项目: astrostudysrv/astrostudycn/ BaZiBirthController.java

use axum::{extract::State, Json};
use std::sync::Arc;
use crate::AppState;
use divines_core::*;

/// 八字计算
pub async fn calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BaziRequest>,
) -> Json<serde_json::Value> {
    // 解析日期时间
    let dt = parse_datetime(&req.datetime);
    let birth = BirthInfo {
        datetime: dt,
        local_datetime: req.datetime.clone(),
        location: GeoPosition {
            latitude: req.latitude.unwrap_or(39.9042),
            longitude: req.longitude.unwrap_or(116.4074),
            ..Default::default()
        },
        gender: req.gender.unwrap_or(Gender::Male),
        name: req.name,
    };

    let options = BaziOptions {
        use_true_solar_time: req.use_true_solar_time.unwrap_or(false),
        longitude: req.longitude.unwrap_or(116.4074),
        use_early_late_zi: req.use_early_late_zi.unwrap_or(false),
        use_ding_qi: req.use_ding_qi.unwrap_or(true),
    };

    let chart = state.bazi.calculate(&birth, &options);
    Json(serde_json::to_value(chart).unwrap_or_default())
}

/// 排盘（八字排盘）
pub async fn paipan(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BaziRequest>,
) -> Json<serde_json::Value> {
    let dt = parse_datetime(&req.datetime);
    let birth = BirthInfo {
        datetime: dt,
        local_datetime: req.datetime.clone(),
        location: GeoPosition {
            latitude: req.latitude.unwrap_or(39.9042),
            longitude: req.longitude.unwrap_or(116.4074),
            ..Default::default()
        },
        gender: req.gender.unwrap_or(Gender::Male),
        name: req.name,
    };

    let options = BaziOptions {
        use_true_solar_time: req.use_true_solar_time.unwrap_or(false),
        longitude: req.longitude.unwrap_or(116.4074),
        use_early_late_zi: req.use_early_late_zi.unwrap_or(false),
        use_ding_qi: req.use_ding_qi.unwrap_or(true),
    };

    let chart = state.bazi.calculate(&birth, &options);
    Json(serde_json::to_value(chart).unwrap_or_default())
}

/// 解析日期时间字符串为 UTC
fn parse_datetime(dt: &str) -> chrono::DateTime<chrono::Utc> {
    use chrono::{NaiveDate, NaiveTime, NaiveDateTime, TimeZone, FixedOffset};

    // 尝试多种格式
    // 格式1: "2024-06-15T14:30"
    if let Ok(naive) = NaiveDateTime::parse_from_str(dt, "%Y-%m-%dT%H:%M") {
        let offset = FixedOffset::east_opt(8 * 3600).unwrap();
        return offset.from_utc_datetime(&naive).with_timezone(&chrono::Utc);
    }
    // 格式2: "2024-06-15T14:30:00"
    if let Ok(naive) = NaiveDateTime::parse_from_str(dt, "%Y-%m-%dT%H:%M:%S") {
        let offset = FixedOffset::east_opt(8 * 3600).unwrap();
        return offset.from_utc_datetime(&naive).with_timezone(&chrono::Utc);
    }
    // 格式3: "2024-06-15 14:30"
    if let Ok(naive) = NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M") {
        let offset = FixedOffset::east_opt(8 * 3600).unwrap();
        return offset.from_utc_datetime(&naive).with_timezone(&chrono::Utc);
    }

    // 回退：当前时间
    chrono::Utc::now()
}

#[derive(serde::Deserialize)]
pub struct BaziRequest {
    pub datetime: String,
    pub name: Option<String>,
    pub gender: Option<Gender>,
    pub birthplace: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    /// 是否使用真太阳时
    pub use_true_solar_time: Option<bool>,
    /// 是否区分早晚子时
    pub use_early_late_zi: Option<bool>,
    /// 使用定气法（true）还是平气法（false）
    pub use_ding_qi: Option<bool>,
}