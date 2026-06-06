// 星阙 Horosa - 紫微斗数 API
// 参考原项目: astrostudysrv/astrostudycn/controller/ZiWeiController.java

use axum::{extract::State, Json};
use std::sync::Arc;
use crate::AppState;
use horosa_core::*;

/// 紫微斗数排盘
pub async fn calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ZiWeiRequest>,
) -> Json<serde_json::Value> {
    let input = ZiWeiInput {
        ad: req.ad.unwrap_or(1),
        gender: req.gender.unwrap_or(Gender::Male),
        birth: req.birth,
        zone: req.zone.unwrap_or_else(|| "+08:00".to_string()),
        lon: req.lon.unwrap_or_else(|| "116e28".to_string()),
        lat: req.lat.unwrap_or_else(|| "39n54".to_string()),
        after_23_new_day: req.after_23_new_day.unwrap_or(true),
        sihua: req.sihua,
        time_alg: req.time_alg.unwrap_or(0),
        adjust_jieqi: req.adjust_jieqi.unwrap_or(false),
        late_zi_hour_use_next_day: req.late_zi_hour_use_next_day.unwrap_or(true),
    };

    let chart = state.ziwei.calculate(&input);

    // 计算大限
    let da_xian = state.ziwei.calc_da_xian(&chart);

    Json(serde_json::json!({
        "chart": chart,
        "da_xian": da_xian,
    }))
}

/// 紫微斗数流年/流月/流日/流时/小限
pub async fn luck(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ZiWeiRequest>,
) -> Json<serde_json::Value> {
    let input = ZiWeiInput {
        ad: req.ad.unwrap_or(1),
        gender: req.gender.unwrap_or(Gender::Male),
        birth: req.birth,
        zone: req.zone.unwrap_or_else(|| "+08:00".to_string()),
        lon: req.lon.unwrap_or_else(|| "116e28".to_string()),
        lat: req.lat.unwrap_or_else(|| "39n54".to_string()),
        after_23_new_day: req.after_23_new_day.unwrap_or(true),
        sihua: req.sihua,
        time_alg: req.time_alg.unwrap_or(0),
        adjust_jieqi: req.adjust_jieqi.unwrap_or(false),
        late_zi_hour_use_next_day: req.late_zi_hour_use_next_day.unwrap_or(true),
    };

    let chart = state.ziwei.calculate(&input);

    let mut layers = serde_json::Map::new();

    // 大限
    if let Some(daxian_idx) = req.daxian_index {
        if daxian_idx < 12 {
            let da_xian = state.ziwei.calc_da_xian(&chart);
            if let Some(dx) = da_xian.get(daxian_idx as usize) {
                layers.insert("daxian".to_string(), serde_json::to_value(dx).unwrap_or_default());
            }
        }
    }

    // 流年
    if let Some(year) = req.year {
        let liu_nian = state.ziwei.calc_liu_nian(&chart, year);
        layers.insert("liunian".to_string(), serde_json::to_value(liu_nian).unwrap_or_default());
    }

    // 小限
    if let Some(age) = req.xiaoxian_age {
        let xiao_xian = state.ziwei.calc_xiao_xian(&chart, age);
        layers.insert("xiaoxian".to_string(), serde_json::to_value(xiao_xian).unwrap_or_default());
    }

    Json(serde_json::json!({
        "layers": layers,
    }))
}

/// 紫微斗数输入请求
#[derive(serde::Deserialize)]
pub struct ZiWeiRequest {
    /// 出生日期时间（如 "1976-10-01 01:50"）
    pub birth: String,
    /// 性别
    pub gender: Option<Gender>,
    /// 公元（1=公元，-1=公元前）
    pub ad: Option<i32>,
    /// 时区
    pub zone: Option<String>,
    /// 经度
    pub lon: Option<String>,
    /// 纬度
    pub lat: Option<String>,
    /// 23点后是否算次日
    pub after_23_new_day: Option<bool>,
    /// 自定义四化
    pub sihua: Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    /// 时辰算法
    pub time_alg: Option<i32>,
    /// 是否调整节气
    pub adjust_jieqi: Option<bool>,
    /// 晚子时是否用次日
    pub late_zi_hour_use_next_day: Option<bool>,
    // --- 运限参数 ---
    /// 大限宫位索引
    pub daxian_index: Option<usize>,
    /// 流年年份
    pub year: Option<i32>,
    /// 小限年龄
    pub xiaoxian_age: Option<i32>,
}