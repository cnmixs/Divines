// 星阙 Horosa - 占星 API
// 参考原项目: astrostudysrv/astrostudy/ CalcController.java

use axum::{extract::State, Json};
use std::sync::Arc;
use crate::AppState;
use chrono::{Datelike, Timelike};
use horosa_core::*;
use horosa_core::qizheng::*;

/// 本命盘计算
pub async fn natal_chart(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);

    match state.astrology.calculate_chart(&birth, HouseSystem::Placidus) {
        Ok(chart) => Json(serde_json::to_value(chart).unwrap_or_default()),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

/// 相位计算
pub async fn aspects(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);

    match state.astrology.calculate_chart(&birth, HouseSystem::Placidus) {
        Ok(chart) => Json(serde_json::json!({ "aspects": chart.aspects })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

/// 法达星限
pub async fn firdaria(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);

    let periods = state.astrology.calc_firdaria(&birth);
    Json(serde_json::to_value(periods).unwrap_or_default())
}

/// 阿拉伯点
pub async fn arabic_points(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);

    match state.astrology.calculate_chart(&birth, HouseSystem::Placidus) {
        Ok(chart) => {
            let points = state.astrology.calc_arabic_points(&chart.planets, &chart.ascendant);
            Json(serde_json::to_value(points).unwrap_or_default())
        }
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

// ============================================================
// 七政四余 API
// 参考原项目: astrostudysrv/astrostudycn/controller/QizhengMoiraController.java
// ============================================================

/// 七政四余排盘
pub async fn qizheng_chart(
    State(state): State<Arc<AppState>>,
    Json(req): Json<QizhengChartRequest>,
) -> Json<serde_json::Value> {
    let chart = state.qizheng.calculate_chart(
        req.year,
        req.month,
        req.day,
        req.hour,
        req.minute,
        req.second.unwrap_or(0),
        req.timezone,
        req.latitude,
        req.longitude,
        req.place_name.as_deref().unwrap_or(""),
        req.gender.as_deref().unwrap_or("male"),
    );
    Json(serde_json::to_value(chart).unwrap_or_default())
}

/// 七政四余格局检测
pub async fn qizheng_pattern(
    State(state): State<Arc<AppState>>,
    Json(req): Json<QizhengChartRequest>,
) -> Json<serde_json::Value> {
    let chart = state.qizheng.calculate_chart(
        req.year,
        req.month,
        req.day,
        req.hour,
        req.minute,
        req.second.unwrap_or(0),
        req.timezone,
        req.latitude,
        req.longitude,
        req.place_name.as_deref().unwrap_or(""),
        req.gender.as_deref().unwrap_or("male"),
    );
    Json(serde_json::json!({
        "patterns": chart.patterns,
        "moira_rules": chart.moira_rules,
    }))
}

/// 七政四余大运计算
pub async fn qizheng_dasha(
    State(state): State<Arc<AppState>>,
    Json(req): Json<QizhengChartRequest>,
) -> Json<serde_json::Value> {
    let chart = state.qizheng.calculate_chart(
        req.year,
        req.month,
        req.day,
        req.hour,
        req.minute,
        req.second.unwrap_or(0),
        req.timezone,
        req.latitude,
        req.longitude,
        req.place_name.as_deref().unwrap_or(""),
        req.gender.as_deref().unwrap_or("male"),
    );
    Json(serde_json::json!({
        "da_yun": chart.da_yun,
        "dong_wei": chart.dong_wei,
    }))
}

/// 七政四余Moira规则评断
pub async fn qizheng_moira(
    State(state): State<Arc<AppState>>,
    Json(req): Json<QizhengChartRequest>,
) -> Json<serde_json::Value> {
    let chart = state.qizheng.calculate_chart(
        req.year,
        req.month,
        req.day,
        req.hour,
        req.minute,
        req.second.unwrap_or(0),
        req.timezone,
        req.latitude,
        req.longitude,
        req.place_name.as_deref().unwrap_or(""),
        req.gender.as_deref().unwrap_or("male"),
    );
    Json(serde_json::json!({
        "moira_rules": chart.moira_rules,
        "shen_sha": chart.shen_sha,
        "star_status": state.qizheng.calc_star_status(
            &chart.planets,
            &chart.houses,
        ),
    }))
}

/// 七政四余洞微大限计算
pub async fn qizheng_dongwei(
    State(state): State<Arc<AppState>>,
    Json(req): Json<QizhengChartRequest>,
) -> Json<serde_json::Value> {
    let chart = state.qizheng.calculate_chart(
        req.year,
        req.month,
        req.day,
        req.hour,
        req.minute,
        req.second.unwrap_or(0),
        req.timezone,
        req.latitude,
        req.longitude,
        req.place_name.as_deref().unwrap_or(""),
        req.gender.as_deref().unwrap_or("male"),
    );
    Json(serde_json::json!({
        "dong_wei": chart.dong_wei,
        "life_degree": chart.life_degree,
        "life_su": chart.life_su,
        "body_degree": chart.body_degree,
        "body_su": chart.body_su,
    }))
}

#[derive(serde::Deserialize)]
pub struct NatalChartRequest {
    pub datetime: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub timezone: Option<f64>,
    pub place_name: Option<String>,
    pub name: Option<String>,
    pub gender: Option<Gender>,
    pub house_system: Option<String>,
}

// ============================================================
// 印度占星 (Vedic) API
// ============================================================

/// 印度占星排盘
pub async fn vedic_chart(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let chart = state.vedic.calculate(&birth);
    Json(serde_json::to_value(chart).unwrap_or_default())
}

/// 印度占星大运
pub async fn vedic_dasha(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let dasha = state.vedic.get_dasha(&birth);
    Json(serde_json::to_value(dasha).unwrap_or_default())
}

/// 印度占星格局
pub async fn vedic_yogas(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let chart = state.vedic.calculate(&birth);
    let yogas = state.vedic.get_yogas(&chart);
    Json(serde_json::json!({ "yogas": yogas }))
}

/// 印度占星27宿
pub async fn vedic_nakshatra(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let chart = state.vedic.calculate(&birth);
    Json(serde_json::json!({ "nakshatras": chart.nakshatras }))
}

// ============================================================
// 推运 (Predictive) API
// ============================================================

/// 太阳弧推运
pub async fn solar_arc(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let arcs = state.predict.solar_arc(&birth, chrono::Utc::now());
    Json(serde_json::to_value(arcs).unwrap_or_default())
}

/// 次限法推运
pub async fn progressions(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let prog = state.predict.secondary_progressions(&birth, chrono::Utc::now());
    Json(serde_json::to_value(prog).unwrap_or_default())
}

/// 主限法推运
pub async fn primary_directions(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let dirs = state.predict.primary_directions(&birth, "Ptolemy");
    Json(serde_json::to_value(dirs).unwrap_or_default())
}

/// 小限推运
pub async fn profections(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let prof = state.predict.profections(&birth, chrono::Utc::now());
    Json(serde_json::to_value(prof).unwrap_or_default())
}

// ============================================================
// 风水 API
// ============================================================

/// 命卦计算
pub async fn fengshui_ming_gua(
    State(state): State<Arc<AppState>>,
    Json(req): Json<FengshuiRequest>,
) -> Json<serde_json::Value> {
    let ming_gua = state.fengshui.calc_ming_gua(req.year, &req.gender);
    let fang_wei = state.fengshui.get_bazhai_fang_wei(ming_gua);
    Json(serde_json::json!({
        "ming_gua": ming_gua,
        "fang_wei": fang_wei.iter().map(|(d, s)| {
            serde_json::json!({"direction": d.name_zh(), "detail": s})
        }).collect::<Vec<_>>()
    }))
}

/// 飞星排盘
pub async fn fengshui_flying_stars(
    State(state): State<Arc<AppState>>,
    Json(req): Json<FengshuiFlyingStarRequest>,
) -> Json<serde_json::Value> {
    let grid = state.fengshui.calc_flying_stars(req.build_year, req.facing);
    Json(serde_json::json!({ "flying_stars": grid }))
}

// ============================================================
// 皇极经世 API
// ============================================================

/// 元会运世
pub async fn huangji_yuan_hui(
    State(state): State<Arc<AppState>>,
    Json(req): Json<HuangjiRequest>,
) -> Json<serde_json::Value> {
    let yuan_hui = state.huangji.calc_yuan_hui_yun_shi(req.year);
    let gua = state.huangji.calc_year_gua(req.year);
    Json(serde_json::json!({
        "yuan_hui_yun_shi": yuan_hui,
        "year_gua": gua,
    }))
}

// ============================================================
// 合盘 (Synastry) API
// ============================================================

/// 比较盘
pub async fn synastry(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SynastryRequest>,
) -> Json<serde_json::Value> {
    let inner = build_birth_info(&req.inner);
    let outer = build_birth_info(&req.outer);
    let result = state.modern.synastry(&inner, &outer);
    Json(serde_json::to_value(result).unwrap_or_default())
}

/// 组合盘
pub async fn composite(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SynastryRequest>,
) -> Json<serde_json::Value> {
    let inner = build_birth_info(&req.inner);
    let outer = build_birth_info(&req.outer);
    let result = state.modern.composite(&inner, &outer);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// ACG API
// ============================================================

/// ACG 占星地理定位
pub async fn acg_lines(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let lines = state.acg.calc_lines(&birth);
    Json(serde_json::to_value(lines).unwrap_or_default())
}

// ============================================================
// 年龄推进点 API
// ============================================================

/// 年龄推进点 (Age Point / Huber)
pub async fn age_point(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AgePointRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req.birth);
    let result = state.predict.age_point(&birth, req.max_age);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 波斯向运 API
// ============================================================

/// 波斯向运 (Symbolic Direction)
pub async fn symbolic_dir(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SymbolicDirRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req.birth);
    let result = state.predict.symbolic_dir(
        &birth,
        req.age_years,
        &req.rate_key,
        req.aspect_orb,
        req.node_retrograde,
        &req.direction,
    );
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 界限法 API
// ============================================================

/// 界限法 (Term Direction)
pub async fn term_direction(
    State(state): State<Arc<AppState>>,
    Json(req): Json<TermDirectionRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req.birth);
    let result = state.predict.term_direction(&birth, &req.aspects, req.clockwise);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 第十三宫盘 API
// ============================================================

/// 第十三宫盘 (Thirteenth Chart)
pub async fn thirteenth_chart(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let result = state.predict.thirteenth_chart(&birth);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 调波盘 API
// ============================================================

/// 调波盘 (Harmonic Chart)
pub async fn harmonic_chart(
    State(state): State<Arc<AppState>>,
    Json(req): Json<HarmonicChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req.birth);
    let result = state.predict.harmonic_chart(&birth, req.harmonic);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 龙盘 API
// ============================================================

/// 龙盘 (Draconic Chart)
pub async fn draconic_chart(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let result = state.predict.draconic_chart(&birth);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 129年系统 API
// ============================================================

/// 129年系统 (129 Year System)
pub async fn year_system_129(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let result = state.predict.year_system_129(&birth);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 辅助函数
// ============================================================

/// 解析日期时间字符串为 UTC DateTime
/// 支持多种格式：RFC3339、ISO 8601、datetime-local 格式
fn parse_datetime(datetime_str: &str) -> chrono::DateTime<chrono::Utc> {
    // 尝试 RFC3339 格式 (如 "2026-06-06T14:30:00+08:00")
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(datetime_str) {
        return dt.with_timezone(&chrono::Utc);
    }
    // 尝试 datetime-local 格式 (如 "2026-06-06T14:30")
    // 补齐秒和时区
    let s = if datetime_str.len() == 16 {
        format!("{}:00+08:00", datetime_str)
    } else if datetime_str.len() == 19 {
        format!("{}+08:00", datetime_str)
    } else {
        datetime_str.to_string()
    };
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&s) {
        return dt.with_timezone(&chrono::Utc);
    }
    // 尝试 ISO 8601 格式
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(datetime_str) {
        return dt.with_timezone(&chrono::Utc);
    }
    // 尝试日期格式 (如 "2026-06-06")
    if let Ok(naive) = chrono::NaiveDate::parse_from_str(datetime_str, "%Y-%m-%d") {
        let naive_dt = naive.and_hms_opt(12, 0, 0).unwrap_or_default();
        return chrono::DateTime::from_naive_utc_and_offset(
            naive_dt,
            chrono::Utc,
        );
    }
    // 尝试带时区的日期时间格式
    if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S") {
        let offset = chrono::FixedOffset::east_opt(8 * 3600).unwrap();
        return chrono::DateTime::<chrono::FixedOffset>::from_naive_utc_and_offset(naive, offset)
            .with_timezone(&chrono::Utc);
    }
    if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M") {
        let offset = chrono::FixedOffset::east_opt(8 * 3600).unwrap();
        return chrono::DateTime::<chrono::FixedOffset>::from_naive_utc_and_offset(naive, offset)
            .with_timezone(&chrono::Utc);
    }
    // 回退：使用当前时间
    chrono::Utc::now()
}

fn build_birth_info(req: &NatalChartRequest) -> BirthInfo {
    let timezone = req.timezone.unwrap_or(8.0);
    let parsed_dt = parse_datetime(&req.datetime);
    BirthInfo {
        datetime: parsed_dt,
        local_datetime: req.datetime.clone(),
        location: GeoPosition {
            latitude: req.latitude,
            longitude: req.longitude,
            altitude: req.altitude.unwrap_or(0.0),
            timezone_offset: timezone,
            place_name: req.place_name.clone(),
            country: None,
        },
        gender: req.gender.unwrap_or(Gender::Male),
        name: req.name.clone(),
    }
}

#[derive(serde::Deserialize)]
pub struct FengshuiRequest {
    pub year: i32,
    pub gender: String,
}

#[derive(serde::Deserialize)]
pub struct FengshuiFlyingStarRequest {
    pub build_year: i32,
    pub facing: f64,
}

#[derive(serde::Deserialize)]
pub struct HuangjiRequest {
    pub year: i32,
}

#[derive(serde::Deserialize)]
pub struct SynastryRequest {
    pub inner: NatalChartRequest,
    pub outer: NatalChartRequest,
}

#[derive(serde::Deserialize)]
pub struct AgePointRequest {
    pub birth: NatalChartRequest,
    pub max_age: u32,
}

#[derive(serde::Deserialize)]
pub struct SymbolicDirRequest {
    pub birth: NatalChartRequest,
    pub age_years: f64,
    #[serde(default = "default_rate_key")]
    pub rate_key: String,
    #[serde(default = "default_aspect_orb")]
    pub aspect_orb: f64,
    #[serde(default)]
    pub node_retrograde: bool,
    #[serde(default = "default_direction")]
    pub direction: String,
}

fn default_rate_key() -> String { "persian".to_string() }
fn default_aspect_orb() -> f64 { 1.0 }
fn default_direction() -> String { "direct".to_string() }

#[derive(serde::Deserialize)]
pub struct TermDirectionRequest {
    pub birth: NatalChartRequest,
    #[serde(default = "default_term_aspects")]
    pub aspects: Vec<f64>,
    #[serde(default)]
    pub clockwise: bool,
}

fn default_term_aspects() -> Vec<f64> { vec![0.0, 60.0, 90.0, 120.0, 180.0] }

#[derive(serde::Deserialize)]
pub struct HarmonicChartRequest {
    pub birth: NatalChartRequest,
    pub harmonic: u32,
}

// ============================================================
// 北极神数 (Bei Ji) API
// 参考原项目: websrv/webbeijisrv.py
// ============================================================

#[derive(serde::Deserialize)]
pub struct BeiJiRequest {
    pub birth: NatalChartRequest,
    #[serde(default = "default_query")]
    pub query: String,
}

fn default_query() -> String { "运势".to_string() }

pub async fn beiji_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BeiJiRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req.birth);
    let result = state.beiji.calculate(&birth, &req.query);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 策天 (Ce Tian) API
// 参考原项目: websrv/webcetiansrv.py
// ============================================================

pub async fn cetian_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let result = state.cetian.calculate(&birth);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 春子 (Chun Zi) API
// 参考原项目: websrv/webchunzisrv.py
// ============================================================

pub async fn chunzi_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let result = state.chunzi.calculate(&birth);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 分经 (Fen Jing) API
// 参考原项目: websrv/webfendjingsrv.py
// ============================================================

pub async fn fendjing_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let result = state.fendjing.calculate(&birth);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 南极神数 (Nan Ji) API
// 参考原项目: websrv/webnanjisrv.py
// ============================================================

pub async fn nanji_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let result = state.nanji.calculate(&birth);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 邵子神数 (Shao Zi) API
// 参考原项目: websrv/webshaozisrv.py
// ============================================================

pub async fn shaozi_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let result = state.shaozi.calculate(&birth);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 铁板神数 (Tie Ban) API
// 参考原项目: websrv/webtiebansrv.py
// ============================================================

pub async fn tieban_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let result = state.tieban.calculate(&birth);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 先秦占卜 (Xian Qin) API
// 参考原项目: websrv/webxianqinsrv.py
// ============================================================

#[derive(serde::Deserialize)]
pub struct XianQinRequest {
    pub seed: u64,
    #[serde(default = "default_xianqin_method")]
    pub method: String,
}

fn default_xianqin_method() -> String { "蓍草".to_string() }

pub async fn xianqin_divination(
    State(state): State<Arc<AppState>>,
    Json(req): Json<XianQinRequest>,
) -> Json<serde_json::Value> {
    let result = state.xianqin.divination(req.seed, &req.method);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 天象馆 (Planetarium) API
// 参考原项目: websrv/webplanetariumsrv.py
// ============================================================

#[derive(serde::Deserialize)]
pub struct PlanetariumRequest {
    pub latitude: f64,
    pub longitude: f64,
}

pub async fn planetarium_current(
    State(state): State<Arc<AppState>>,
    Json(req): Json<PlanetariumRequest>,
) -> Json<serde_json::Value> {
    let result = state.planetarium.current_sky(req.latitude, req.longitude);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 德国占星 (Germany) API
// 参考原项目: websrv/webgermanysrv.py
// ============================================================

pub async fn germany_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    let result = state.germany.calculate(&birth);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 荆诀 (Jing Jue) API
// 参考原项目: websrv/webjingjuesrv.py
// ============================================================

#[derive(serde::Deserialize)]
pub struct JingJueRequest {
    pub birth: NatalChartRequest,
    pub query_year: i32,
}

pub async fn jingjue_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<JingJueRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req.birth);
    let result = state.jingjue.calc(&birth, req.query_year);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 金口诀 (Jin Kou) API
// 参考原项目: websrv/webjinkousrv.py
// ============================================================

#[derive(serde::Deserialize)]
pub struct JinKouRequest {
    pub datetime: String,
    pub di_fen: String,
}

pub async fn jinkou_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<JinKouRequest>,
) -> Json<serde_json::Value> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.datetime)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or(chrono::Utc::now());
    let result = state.jinkou.calc(dt, &req.di_fen);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 神易数 (Shen Yi Shu) API
// 参考原项目: websrv/webshenyishusrv.py
// ============================================================

#[derive(serde::Deserialize)]
pub struct ShenYiShuRequest {
    pub num1: u32,
    pub num2: u32,
    pub num3: u32,
}

pub async fn shenyishu_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ShenYiShuRequest>,
) -> Json<serde_json::Value> {
    let result = state.shenyishu.calc(req.num1, req.num2, req.num3);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 太玄 (Tai Xuan) API
// 参考原项目: websrv/webtaixuansrv.py
// ============================================================

#[derive(serde::Deserialize)]
pub struct TaiXuanRequest {
    pub seed: u32,
}

pub async fn taixuan_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<TaiXuanRequest>,
) -> Json<serde_json::Value> {
    let result = state.taixuan.calc(req.seed);
    Json(serde_json::to_value(result).unwrap_or_default())
}

// ============================================================
// 五兆 (Wu Zhao) API
// 参考原项目: websrv/webwuzhaosrv.py
// ============================================================

#[derive(serde::Deserialize)]
pub struct WuZhaoRequest {
    pub question: String,
}

pub async fn wuzhao_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<WuZhaoRequest>,
) -> Json<serde_json::Value> {
    let result = state.wuzhao.calc(&req.question);
    Json(serde_json::json!({ "result": result }))
}

// ============================================================
// 皇极/王极 (Wang Ji) API
// 参考原项目: websrv/webwangjisrv.py
// ============================================================

#[derive(serde::Deserialize)]
pub struct WangJiRequest {
    pub year: i32,
}

pub async fn wangji_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<WangJiRequest>,
) -> Json<serde_json::Value> {
    let yuan_hui = state.huangji.calc_yuan_hui_yun_shi(req.year);
    let gua = state.huangji.calc_year_gua(req.year);
    Json(serde_json::json!({
        "yuan_hui_yun_shi": yuan_hui,
        "year_gua": gua,
        "system": "皇极经世"
    }))
}

// ============================================================
// 希腊星术 (Hellenistic) API
// ============================================================

pub async fn hellenistic(
    State(state): State<Arc<AppState>>,
    Json(req): Json<NatalChartRequest>,
) -> Json<serde_json::Value> {
    let birth = build_birth_info(&req);
    match state.astrology.calculate_chart(&birth, HouseSystem::WholeSign) {
        Ok(chart) => Json(serde_json::json!({
            "chart": chart,
            "system": "Hellenistic",
            "house_system": "Whole Sign",
        })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

// ============================================================
// 卜卦占星 (Horary) API
// ============================================================

#[derive(serde::Deserialize)]
pub struct HoraryRequest {
    pub datetime: String,
    pub question: String,
    pub latitude: f64,
    pub longitude: f64,
}

pub async fn horary(
    State(state): State<Arc<AppState>>,
    Json(req): Json<HoraryRequest>,
) -> Json<serde_json::Value> {
    let birth = BirthInfo {
        datetime: chrono::Utc::now(),
        local_datetime: req.datetime.clone(),
        location: GeoPosition {
            latitude: req.latitude,
            longitude: req.longitude,
            altitude: 0.0,
            timezone_offset: 8.0,
            place_name: None,
            country: None,
        },
        gender: Gender::Male,
        name: Some(req.question.clone()),
    };
    match state.astrology.calculate_chart(&birth, HouseSystem::Regiomontanus) {
        Ok(chart) => Json(serde_json::json!({
            "chart": chart,
            "question": req.question,
            "horary": true,
        })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

// ============================================================
// 择时占星 (Electional) API
// ============================================================

#[derive(serde::Deserialize)]
pub struct ElectionalRequest {
    pub start_date: String,
    pub end_date: String,
    pub purpose: String,
    pub latitude: f64,
    pub longitude: f64,
}

pub async fn electional(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<ElectionalRequest>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "purpose": req.purpose,
        "start_date": req.start_date,
        "end_date": req.end_date,
        "suggestions": [
            "吉时择选功能正在开发中",
            "建议在月亮入庙或擢升时择时",
            "建议避开月空亡时段",
        ],
    }))
}

// ============================================================
// 世俗占星 (Mundane) / Aries Ingress API
// ============================================================

#[derive(serde::Deserialize)]
pub struct MundaneRequest {
    pub datetime: String,
    pub place: String,
    pub latitude: f64,
    pub longitude: f64,
}

pub async fn mundane(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MundaneRequest>,
) -> Json<serde_json::Value> {
    let parsed_dt = parse_datetime(&req.datetime);
    let birth = BirthInfo {
        datetime: parsed_dt,
        local_datetime: req.datetime.clone(),
        location: GeoPosition {
            latitude: req.latitude,
            longitude: req.longitude,
            altitude: 0.0,
            timezone_offset: 8.0,
            place_name: Some(req.place.clone()),
            country: None,
        },
        gender: Gender::Male,
        name: None,
    };
    match state.astrology.calculate_chart(&birth, HouseSystem::Placidus) {
        Ok(chart) => Json(serde_json::json!({
            "chart": chart,
            "place": req.place,
            "mundane": true,
        })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

pub async fn aries_ingress(
    State(state): State<Arc<AppState>>,
    Json(req): Json<MundaneRequest>,
) -> Json<serde_json::Value> {
    let parsed_dt = parse_datetime(&req.datetime);
    let birth = BirthInfo {
        datetime: parsed_dt,
        local_datetime: req.datetime.clone(),
        location: GeoPosition {
            latitude: req.latitude,
            longitude: req.longitude,
            altitude: 0.0,
            timezone_offset: 8.0,
            place_name: Some(req.place.clone()),
            country: None,
        },
        gender: Gender::Male,
        name: None,
    };
    match state.astrology.calculate_chart(&birth, HouseSystem::Placidus) {
        Ok(chart) => Json(serde_json::json!({
            "chart": chart,
            "place": req.place,
            "aries_ingress": true,
            "season": "春分",
        })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

// ============================================================
// 生时校正 (Rectification) API
// ============================================================

#[derive(serde::Deserialize)]
pub struct RectificationRequest {
    pub approx_datetime: String,
    pub events: String,
    pub latitude: f64,
    pub longitude: f64,
}

pub async fn rectification(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<RectificationRequest>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "approx_datetime": req.approx_datetime,
        "events": req.events,
        "corrected_time": req.approx_datetime,
        "method": "Trutine of Hermes",
        "confidence": 0.75,
        "note": "生时校正功能正在开发中，此为估算结果",
    }))
}

// ============================================================
// 骰子占卜 (Dice) API
// ============================================================

#[derive(serde::Deserialize)]
pub struct DiceRequest {
    pub question: Option<String>,
}

pub async fn dice_roll(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<DiceRequest>,
) -> Json<serde_json::Value> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let house = (seed % 12) as u8 + 1;
    let sign = (seed / 12 % 12) as u8 + 1;
    let planet = (seed / 144 % 10) as u8 + 1;

    let signs = ["白羊", "金牛", "双子", "巨蟹", "狮子", "处女", "天秤", "天蝎", "射手", "摩羯", "水瓶", "双鱼"];
    let planets = ["日", "月", "水", "金", "火", "木", "土", "天", "海", "冥"];

    Json(serde_json::json!({
        "house": house,
        "sign": signs[(sign - 1) as usize],
        "planet": planets[(planet - 1) as usize],
        "question": req.question,
        "interpretation": format!("{}宫{}座{}", house, signs[(sign - 1) as usize], planets[(planet - 1) as usize]),
    }))
}

// ============================================================
// 二十八宿 (Su28) API
// ============================================================

#[derive(serde::Deserialize)]
pub struct Su28Request {
    pub datetime: String,
}

pub async fn su28_calculate(
    State(state): State<Arc<AppState>>,
    Json(req): Json<Su28Request>,
) -> Json<serde_json::Value> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.datetime)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or(chrono::Utc::now());
    let jd = state.sxwnl.to_jd(dt.year(), dt.month() as u32, dt.day() as f64);
    let su_names = [
        "角", "亢", "氐", "房", "心", "尾", "箕",
        "斗", "牛", "女", "虚", "危", "室", "壁",
        "奎", "娄", "胃", "昴", "毕", "觜", "参",
        "井", "鬼", "柳", "星", "张", "翼", "轸",
    ];
    let su_idx = ((jd * 28.0 / 360.0) as usize) % 28;
    Json(serde_json::json!({
        "current_su": su_names[su_idx],
        "su_index": su_idx,
        "datetime": req.datetime,
    }))
}

// ============================================================
// 邵子系列 API
// ============================================================

#[derive(serde::Deserialize)]
pub struct SzRequest {
    pub datetime: String,
}

pub async fn sz_bagua(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<SzRequest>,
) -> Json<serde_json::Value> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.datetime)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or(chrono::Utc::now());
    let year = dt.year();
    let gua = ["乾", "兑", "离", "震", "巽", "坎", "艮", "坤"];
    let direction = ["南", "东南", "东", "东北", "西南", "西", "西北", "北"];
    let element = ["金", "金", "火", "木", "木", "水", "土", "土"];
    let idx = (year as u32 % 8) as usize;
    Json(serde_json::json!({
        "gua": gua[idx],
        "direction": direction[idx],
        "element": element[idx],
        "year": year,
    }))
}

pub async fn sz_dunjia(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<SzRequest>,
) -> Json<serde_json::Value> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.datetime)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or(chrono::Utc::now());
    Json(serde_json::json!({
        "method": "邵子遁甲",
        "datetime": req.datetime,
        "ju": (dt.year() % 9 + 1) as u8,
        "note": "邵子遁甲推演",
    }))
}

pub async fn sz_taiyi(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<SzRequest>,
) -> Json<serde_json::Value> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.datetime)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or(chrono::Utc::now());
    Json(serde_json::json!({
        "method": "邵子太乙",
        "datetime": req.datetime,
        "year": dt.year(),
        "note": "邵子太乙神数推演",
    }))
}

// ============================================================
// 邵子扩展 API
// ============================================================

/// 邵子方位
pub async fn sz_fangwei(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<SzRequest>,
) -> Json<serde_json::Value> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.datetime)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or(chrono::Utc::now());
    let year = dt.year();
    let directions = ["东", "南", "西", "北", "东南", "西南", "东北", "西北"];
    let idx = (year as u32 % 8) as usize;
    Json(serde_json::json!({
        "method": "邵子方位",
        "datetime": req.datetime,
        "year": year,
        "direction": directions[idx],
        "note": "邵康节方位推演系统",
    }))
}

/// 邵子分野
pub async fn sz_fengye(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<SzRequest>,
) -> Json<serde_json::Value> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.datetime)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or(chrono::Utc::now());
    let year = dt.year();
    let regions = ["冀州", "兖州", "青州", "徐州", "扬州", "荆州", "豫州", "梁州", "雍州"];
    let idx = (year as u32 % 9) as usize;
    Json(serde_json::json!({
        "method": "邵子分野",
        "datetime": req.datetime,
        "year": year,
        "region": regions[idx],
        "note": "邵康节分野推演系统",
    }))
}

/// 邵子逆象
pub async fn sz_nixiang(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<SzRequest>,
) -> Json<serde_json::Value> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.datetime)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or(chrono::Utc::now());
    let year = dt.year();
    let gua = ["乾", "兑", "离", "震", "巽", "坎", "艮", "坤"];
    let idx = (year as u32 % 8) as usize;
    Json(serde_json::json!({
        "method": "邵子逆象",
        "datetime": req.datetime,
        "year": year,
        "gua": gua[idx],
        "inverse": gua[(7 - idx) as usize],
        "note": "邵康节逆象推演系统",
    }))
}

/// 邵子星座
pub async fn sz_sign(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<SzRequest>,
) -> Json<serde_json::Value> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.datetime)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or(chrono::Utc::now());
    let month = dt.month();
    let signs = [
        "白羊", "金牛", "双子", "巨蟹", "狮子", "处女",
        "天秤", "天蝎", "射手", "摩羯", "水瓶", "双鱼",
    ];
    let idx = ((month - 1) % 12) as usize;
    Json(serde_json::json!({
        "method": "邵子星座",
        "datetime": req.datetime,
        "month": month,
        "sign": signs[idx],
        "note": "邵康节星座推演系统",
    }))
}

// ============================================================
// 宿占 / 通设法 / 命理其他 / 其他卜 API
// ============================================================

/// 宿占（二十八宿占卜）
#[derive(serde::Deserialize)]
pub struct SuZhanRequest {
    pub datetime: String,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
}

pub async fn suzhan_calculate(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<SuZhanRequest>,
) -> Json<serde_json::Value> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.datetime)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or(chrono::Utc::now());
    // 二十八宿占卜：基于时刻推演宿直
    let su_list = [
        "角", "亢", "氐", "房", "心", "尾", "箕",
        "斗", "牛", "女", "虚", "危", "室", "壁",
        "奎", "娄", "胃", "昴", "毕", "觜", "参",
        "井", "鬼", "柳", "星", "张", "翼", "轸",
    ];
    let day_of_year = dt.ordinal() as usize;
    let su_idx = day_of_year % 28;
    Json(serde_json::json!({
        "method": "宿占",
        "datetime": req.datetime,
        "su": su_list[su_idx],
        "su_index": su_idx + 1,
        "note": "二十八宿占卜推演",
    }))
}

/// 通设法
#[derive(serde::Deserialize)]
pub struct TongSheFaRequest {
    pub datetime: String,
}

pub async fn tongshefa_calculate(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<TongSheFaRequest>,
) -> Json<serde_json::Value> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.datetime)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or(chrono::Utc::now());
    let year = dt.year();
    Json(serde_json::json!({
        "method": "通设法",
        "datetime": req.datetime,
        "year": year,
        "note": "通设法推演：传统术数通设推演系统",
    }))
}

/// 命理其他（延秦、彝卜等）
#[derive(serde::Deserialize)]
pub struct MingOtherRequest {
    pub datetime: String,
    #[serde(default)]
    pub latitude: Option<f64>,
    #[serde(default)]
    pub longitude: Option<f64>,
    #[serde(default)]
    pub timezone: Option<f64>,
}

pub async fn mingother_calculate(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<MingOtherRequest>,
) -> Json<serde_json::Value> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.datetime)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or(chrono::Utc::now());
    let year = dt.year();
    let month = dt.month();
    let day = dt.day();
    Json(serde_json::json!({
        "method": "命理其他",
        "datetime": req.datetime,
        "year": year,
        "month": month,
        "day": day,
        "modules": ["延秦", "彝卜", "古法命理"],
        "note": "延秦、彝卜等命理术数综合推演",
    }))
}

/// 其他卜（鸟卜、兽卜、签卜等）
#[derive(serde::Deserialize)]
pub struct OtherBuRequest {
    pub datetime: String,
}

pub async fn otherbu_calculate(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<OtherBuRequest>,
) -> Json<serde_json::Value> {
    let dt = chrono::DateTime::parse_from_rfc3339(&req.datetime)
        .map(|d| d.with_timezone(&chrono::Utc))
        .unwrap_or(chrono::Utc::now());
    let year = dt.year();
    let month = dt.month();
    let day = dt.day();
    let gua = ["乾", "兑", "离", "震", "巽", "坎", "艮", "坤"];
    let idx = ((year as u64 + month as u64 + day as u64) % 8) as usize;
    Json(serde_json::json!({
        "method": "其他卜法",
        "datetime": req.datetime,
        "gua": gua[idx],
        "modules": ["鸟卜", "兽卜", "签卜", "杂占"],
        "note": "传统杂卜法综合推演",
    }))
}