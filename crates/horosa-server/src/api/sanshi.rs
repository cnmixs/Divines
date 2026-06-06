// 星阙 Horosa - 三式 API（奇门、太乙、六壬）
// 参考原项目: astropy/websrv/webqimensrv.py, webtaiyisrv.py

use axum::{extract::State, Json};
use std::sync::Arc;
use crate::AppState;
use horosa_core::*;

/// 奇门排盘请求
#[derive(serde::Deserialize)]
pub struct QimenRequest {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: Option<u32>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub timezone: Option<f64>,
}

/// 奇门排盘响应
pub async fn qimen_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<QimenRequest>,
) -> Json<serde_json::Value> {
    let minute = req.minute.unwrap_or(0);
    let chart = state.qimen.calculate(req.year, req.month, req.day, req.hour, minute);
    Json(serde_json::to_value(chart).unwrap_or_default())
}

/// 查询局数
pub async fn qimen_ju(
    State(state): State<Arc<AppState>>,
    Json(req): Json<QimenRequest>,
) -> Json<serde_json::Value> {
    let ju = state.qimen.get_ju(req.year, req.month, req.day, req.hour);
    Json(ju)
}

/// 太乙请求
#[derive(serde::Deserialize)]
pub struct SanshiRequest {
    pub datetime: String,
    pub method: Option<String>,
}

/// 太乙计算
pub async fn taiyi(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SanshiRequest>,
) -> Json<serde_json::Value> {
    let birth = BirthInfo {
        datetime: chrono::Utc::now(),
        local_datetime: req.datetime,
        location: GeoPosition::default(),
        gender: Gender::Male,
        name: None,
    };

    let chart = state.taiyi.calculate(&birth);
    Json(serde_json::to_value(chart).unwrap_or_default())
}

/// 六壬计算
pub async fn liuren(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SanshiRequest>,
) -> Json<serde_json::Value> {
    let birth = BirthInfo {
        datetime: chrono::Utc::now(),
        local_datetime: req.datetime,
        location: GeoPosition::default(),
        gender: Gender::Male,
        name: None,
    };

    let chart = state.liuren.calculate(&birth);
    Json(serde_json::to_value(chart).unwrap_or_default())
}