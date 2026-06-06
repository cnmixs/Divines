// 星阙 Horosa - 卦象系统 API (梅花易数 / 六爻)
// 参考原项目: astrostudysrv/astrostudycn/controller/GuaController.java

use axum::{extract::State, Json};
use std::sync::Arc;
use crate::AppState;
use horosa_core::*;

/// 查询六十四卦详情
/// 支持: 卦名("乾"), 简称("天天"), 全名("乾乾"), 二进制("111111")
pub async fn gua_desc(
    State(state): State<Arc<AppState>>,
    Json(req): Json<GuaDescRequest>,
) -> Json<serde_json::Value> {
    let mut result = serde_json::Map::new();

    for name in &req.names {
        if name.is_empty() {
            continue;
        }
        if let Some(gua) = state.gua.get_gua_by_name(name) {
            result.insert(name.clone(), serde_json::to_value(gua).unwrap_or_default());
        }
    }

    Json(serde_json::Value::Object(result))
}

/// 查询梅花易数八卦
pub async fn gua_meiyi(
    State(state): State<Arc<AppState>>,
    Json(req): Json<GuaDescRequest>,
) -> Json<serde_json::Value> {
    let mut result = serde_json::Map::new();

    for name in &req.names {
        if name.is_empty() {
            continue;
        }
        if let Some(gua) = state.gua.get_meiyi_gua_by_name(name) {
            result.insert(name.clone(), serde_json::to_value(gua).unwrap_or_default());
        }
    }

    Json(serde_json::Value::Object(result))
}

/// 梅花易数起卦
pub async fn gua_meihua(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MeiHuaRequest>,
) -> Json<serde_json::Value> {
    let chart = state.gua.get_meihua_gua(&req.numbers);
    Json(serde_json::to_value(chart).unwrap_or_default())
}

/// 获取卦象关系 (错卦、综卦、互卦)
pub async fn gua_relation(
    State(state): State<Arc<AppState>>,
    Json(req): Json<GuaRelationRequest>,
) -> Json<serde_json::Value> {
    let gua = match state.gua.get_gua_by_name(&req.name) {
        Some(g) => g,
        None => {
            return Json(serde_json::json!({
                "error": "卦未找到",
                "name": req.name
            }))
        }
    };

    let cuo = state.gua.get_cuo_gua(gua);
    let zong = state.gua.get_zong_gua(gua);
    let hu = state.gua.get_mutual_gua(gua);

    Json(serde_json::json!({
        "name": req.name,
        "cuo_gua": cuo,
        "zong_gua": zong,
        "hu_gua": hu,
    }))
}

/// 由6爻构造卦
pub async fn gua_yao(
    State(state): State<Arc<AppState>>,
    Json(req): Json<YaoGuaRequest>,
) -> Json<serde_json::Value> {
    let yao: [u8; 6] = req.yao;
    let gua = state.gua.yao_to_gua(&yao);
    Json(serde_json::to_value(gua).unwrap_or_default())
}

/// 列出所有六十四卦
pub async fn gua_list_all(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let all = state.gua.list_all_gua();
    Json(serde_json::to_value(all).unwrap_or_default())
}

/// 四象查询
pub async fn gua_sixiang(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let si_xiang = state.gua.get_sixiang();
    Json(serde_json::to_value(si_xiang).unwrap_or_default())
}

// ============ 请求类型 ============

#[derive(serde::Deserialize)]
pub struct GuaDescRequest {
    #[serde(default)]
    pub names: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct MeiHuaRequest {
    pub numbers: Vec<u32>,
}

#[derive(serde::Deserialize)]
pub struct GuaRelationRequest {
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct YaoGuaRequest {
    pub yao: [u8; 6],
}

// ============ 六爻 API ============

/// 六爻摇卦（铜钱起卦）
pub async fn cast(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let result = state.liuyao.cast_coins();
    Json(serde_json::to_value(result).unwrap_or_default())
}

/// 六爻排盘（完整占卜）
pub async fn divine(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LiuyaoDivineRequest>,
) -> Json<serde_json::Value> {
    let birth = BirthInfo {
        datetime: chrono::Utc::now(),
        local_datetime: req.datetime.clone(),
        location: GeoPosition {
            latitude: req.latitude.unwrap_or(39.9042),
            longitude: req.longitude.unwrap_or(116.4074),
            altitude: 0.0,
            timezone_offset: 8.0,
            place_name: None,
            country: None,
        },
        gender: Gender::Male,
        name: None,
    };

    let gua = state.liuyao.divine(&birth, &req.query);
    Json(serde_json::to_value(gua).unwrap_or_default())
}

#[derive(serde::Deserialize)]
pub struct LiuyaoDivineRequest {
    pub datetime: String,
    pub query: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}