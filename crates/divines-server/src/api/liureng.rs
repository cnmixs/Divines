// Divines - 六壬 API
// 参考原项目: astrostudysrv/astrostudycn/controller/LiuRengController.java

use axum::{extract::State, Json};
use std::sync::Arc;
use crate::AppState;
use divines_core::*;

/// 六壬排盘
pub async fn calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LiuRengRequest>,
) -> Json<serde_json::Value> {
    let chart = state.liuren_full.calculate(
        req.year_gan,
        req.year_zhi,
        req.month_gan,
        req.month_zhi,
        req.day_gan,
        req.day_zhi,
        req.hour_gan,
        req.hour_zhi,
        req.is_diurnal.unwrap_or(true),
    );
    Json(serde_json::to_value(chart).unwrap_or_default())
}

/// 六壬月将查询
pub async fn month_jiang(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MonthJiangRequest>,
) -> Json<serde_json::Value> {
    let mj = state.liuren_full.determine_month_jiang(&req.month_zhi);
    Json(serde_json::json!({
        "month_zhi": req.month_zhi,
        "month_jiang": mj,
    }))
}

/// 六壬占时查询
pub async fn zhan_shi(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ZhanShiRequest>,
) -> Json<serde_json::Value> {
    let zs = state.liuren_full.determine_zhan_shi(req.hour);
    Json(serde_json::json!({
        "hour": req.hour,
        "zhan_shi": zs,
    }))
}

/// 六壬贵人查询
pub async fn gui_ren(
    State(state): State<Arc<AppState>>,
    Json(req): Json<GuiRenRequest>,
) -> Json<serde_json::Value> {
    let day_gan_str = req.day_gan.name_zh();
    let (pos, yang_gui) = state.liuren_full.arrange_gui_ren(day_gan_str, req.is_diurnal.unwrap_or(true));

    let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
    Json(serde_json::json!({
        "day_gan": day_gan_str,
        "is_diurnal": req.is_diurnal.unwrap_or(true),
        "gui_ren_zhi": di_zhi[pos],
        "gui_ren_position": pos,
        "yang_gui": yang_gui,
    }))
}

/// 六壬神将排布
pub async fn shen_jiang(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ShenJiangRequest>,
) -> Json<serde_json::Value> {
    let (shen_jiang, tian_jiang) = state.liuren_full.arrange_shen_jiang(req.gui_ren_position, req.yang_gui.unwrap_or(true));
    Json(serde_json::json!({
        "shen_jiang": shen_jiang,
        "tian_jiang": tian_jiang,
    }))
}

/// 六壬四课
pub async fn si_ke(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SiKeRequest>,
) -> Json<serde_json::Value> {
    let day_gan_str = req.day_gan.name_zh();
    let day_zhi_str = req.day_zhi.name_zh();

    let month_jiang = state.liuren_full.determine_month_jiang(req.month_zhi.name_zh());
    let zhan_shi = state.liuren_full.determine_zhan_shi(req.hour);
    let tian_pan = state.liuren_full.arrange_tian_pan(&month_jiang, &zhan_shi);
    let si_ke = state.liuren_full.calculate_si_ke(day_gan_str, day_zhi_str, &tian_pan);

    Json(serde_json::json!({
        "tian_pan": tian_pan,
        "si_ke": si_ke,
    }))
}

/// 六壬三传
pub async fn san_chuan(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SiKeRequest>,
) -> Json<serde_json::Value> {
    let day_gan_str = req.day_gan.name_zh();
    let day_zhi_str = req.day_zhi.name_zh();

    let month_jiang = state.liuren_full.determine_month_jiang(req.month_zhi.name_zh());
    let zhan_shi = state.liuren_full.determine_zhan_shi(req.hour);
    let tian_pan = state.liuren_full.arrange_tian_pan(&month_jiang, &zhan_shi);
    let si_ke = state.liuren_full.calculate_si_ke(day_gan_str, day_zhi_str, &tian_pan);
    let san_chuan = state.liuren_full.calculate_san_chuan(&si_ke);

    Json(serde_json::json!({
        "si_ke": si_ke,
        "san_chuan": san_chuan,
    }))
}

/// 六壬遁干 (五鼠遁)
pub async fn dun_gan(
    State(state): State<Arc<AppState>>,
    Json(req): Json<DunGanRequest>,
) -> Json<serde_json::Value> {
    let day_gan_str = req.day_gan.name_zh();
    let dun_gan = state.liuren_full.calculate_dun_gan(day_gan_str);
    Json(serde_json::json!({
        "day_gan": day_gan_str,
        "dun_gan": dun_gan,
    }))
}

/// 六壬长生十二神
pub async fn zhang_sheng(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ZhangShengRequest>,
) -> Json<serde_json::Value> {
    let day_gan_str = req.day_gan.name_zh();
    let zhang_sheng = state.liuren_full.get_zhang_sheng(day_gan_str);
    Json(serde_json::json!({
        "day_gan": day_gan_str,
        "zhang_sheng": zhang_sheng,
    }))
}

// ============ 请求类型 ============

#[derive(serde::Deserialize)]
pub struct LiuRengRequest {
    pub year_gan: TianGan,
    pub year_zhi: DiZhi,
    pub month_gan: TianGan,
    pub month_zhi: DiZhi,
    pub day_gan: TianGan,
    pub day_zhi: DiZhi,
    pub hour_gan: TianGan,
    pub hour_zhi: DiZhi,
    pub is_diurnal: Option<bool>,
}

#[derive(serde::Deserialize)]
pub struct MonthJiangRequest {
    pub month_zhi: String,
}

#[derive(serde::Deserialize)]
pub struct ZhanShiRequest {
    pub hour: u32,
}

#[derive(serde::Deserialize)]
pub struct GuiRenRequest {
    pub day_gan: TianGan,
    pub is_diurnal: Option<bool>,
}

#[derive(serde::Deserialize)]
pub struct ShenJiangRequest {
    pub gui_ren_position: usize,
    pub yang_gui: Option<bool>,
}

#[derive(serde::Deserialize)]
pub struct SiKeRequest {
    pub day_gan: TianGan,
    pub day_zhi: DiZhi,
    pub month_zhi: DiZhi,
    pub hour: u32,
}

#[derive(serde::Deserialize)]
pub struct DunGanRequest {
    pub day_gan: TianGan,
}

#[derive(serde::Deserialize)]
pub struct ZhangShengRequest {
    pub day_gan: TianGan,
}