// Divines - 占星相关类型
// 参考原项目: flatlib-ctrad2/flatlib/const.py, astropy/astrostudy/

use super::chart::*;
use serde::{Deserialize, Serialize};

/// 星盘主题/配色
/// 参考原项目: AstroColor0.js - AstroColor8.js
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartTheme {
    pub name: String,
    pub background: String,
    pub foreground: String,
    pub accent: String,
    pub planet_colors: std::collections::HashMap<String, String>,
    pub sign_colors: std::collections::HashMap<String, String>,
    pub aspect_colors: std::collections::HashMap<String, String>,
}

/// 阿拉伯点（希腊星术）
/// 参考原项目: flatlib-ctrad2/recipes/arabicparts.py
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArabicPart {
    pub name: String,
    pub name_zh: String,
    pub longitude: f64,
    pub sign: ZodiacSign,
    pub degree: f64,
    pub formula: String,
}

/// 界限（Terms / Bounds）
/// 参考原项目: flatlib-ctrad2/flatlib/dignities/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Term {
    pub sign: ZodiacSign,
    pub degree_start: f64,
    pub degree_end: f64,
    pub ruler: Planet,
}

/// 法达星限
/// 参考原项目: astropy/astrostudy/firdaria.py
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirdariaPeriod {
    pub planet: Planet,
    pub start_date: String,
    pub end_date: String,
    pub sub_periods: Vec<FirdariaSubPeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirdariaSubPeriod {
    pub planet: Planet,
    pub start_date: String,
    pub end_date: String,
}

/// 主限法推运
/// 参考原项目: astropy/astrostudy/pd_engine.py
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryDirection {
    pub significator: Planet,
    pub promissor: Planet,
    pub direction_type: String,
    pub arc: f64,
    pub date: String,
    pub age: f64,
}

/// 黄道星释
/// 参考原项目: astropy/astrostudy/zreleasing.py
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZodiacalReleasing {
    pub lot: String,
    pub level: u8,
    pub sign: ZodiacSign,
    pub start_date: String,
    pub end_date: String,
    pub peak_period: Option<(String, String)>,
}

/// 太阳弧
/// 参考原项目: astropy/astrostudy/solararc.py
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarArc {
    pub planet: Planet,
    pub arc: f64,
    pub directed_planet: Planet,
    pub aspect_type: AspectType,
    pub date: String,
}

/// 小限
/// 参考原项目: flatlib-ctrad2/recipes/profections.py
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profection {
    pub age: u8,
    pub house: u8,
    pub sign: ZodiacSign,
    pub lord_of_year: Planet,
    pub start_date: String,
    pub end_date: String,
}

/// 返照盘
/// 参考原项目: flatlib-ctrad2/recipes/solarreturn.py
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnChart {
    pub return_type: ReturnType,
    pub chart: AstroChart,
    pub return_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReturnType {
    SolarReturn,
    LunarReturn,
}

/// 印度占星盘
/// 参考原项目: astropy/astrostudy/india/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedicChart {
    /// 恒星黄道偏移（Ayanamsa）
    pub ayanamsa: f64,
    /// 出生信息
    pub birth: BirthInfo,
    /// 行星位置（恒星黄道）
    pub planets: Vec<PlanetPosition>,
    /// 宫位
    pub houses: Vec<House>,
    /// 上升点
    pub ascendant: Ascendant,
    /// 每个星体的星宿
    pub nakshatras: std::collections::HashMap<String, NakshatraInfo>,
    /// 当前大运序列
    pub dashas: Vec<VimshottariDasha>,
    /// 发现的瑜伽
    pub yogas: Vec<String>,
    /// 分盘 D-1/D-9 等
    pub vargas: std::collections::HashMap<u8, VargaChart>,
    /// 星体尊严
    pub dignities: std::collections::HashMap<String, PlanetaryDignity>,
    /// 八层占
    pub ashtakavarga: AshtakavargaData,
    /// 五支
    pub panchanga: Panchanga,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VedicChartType {
    NorthIndian,
    SouthIndian,
    EastIndian,
}

/// 二十七宿（印度）详细信息
/// 参考原项目: astropy/astrostudy/india/jyotish_engine.py nakshatra_from_lon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NakshatraInfo {
    /// 序号 1-27
    pub index: u8,
    /// 英文名
    pub name: String,
    /// 中文名
    pub name_zh: String,
    /// 主星
    pub lord: Planet,
    /// 足 1-4
    pub pada: u8,
    /// 起始度数（恒星黄道）
    pub degree_start: f64,
    /// 结束度数（恒星黄道）
    pub degree_end: f64,
    /// 在星宿中的进度 0.0-1.0
    pub progress: f64,
    /// 剩余比例 1.0 - progress
    pub remaining_ratio: f64,
}

/// 二十七宿（旧版，保留兼容）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nakshatra {
    pub name: String,
    pub name_zh: String,
    pub degree_start: f64,
    pub degree_end: f64,
    pub lord: Planet,
    pub pada: u8,
}

/// 大运（旧版，保留兼容）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dasha {
    pub system: String,
    pub planet: Planet,
    pub start_date: String,
    pub end_date: String,
    pub sub_periods: Vec<Dasha>,
}

/// 维摩沙里大运期
/// 参考原项目: astropy/astrostudy/india/jyotish_engine.py vimshottari
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VimshottariDasha {
    /// 主星
    pub lord: Planet,
    /// 主星名称
    pub lord_name: String,
    /// 大运年数
    pub years: f64,
    /// 开始日期
    pub start_date: String,
    /// 结束日期
    pub end_date: String,
    /// 开始年龄
    pub start_age: f64,
    /// 结束年龄
    pub end_age: f64,
    /// 是否为出生平衡期
    pub is_birth_balance: bool,
    /// 是否为当前活跃期
    pub is_active: bool,
    /// 小运（Bhukti/Antardasha）
    pub bhuktis: Vec<BhuktiPeriod>,
}

/// 小运期（Bhukti/Antardasha）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BhuktiPeriod {
    /// 主星
    pub lord: Planet,
    /// 主星名称
    pub lord_name: String,
    /// 开始日期
    pub start_date: String,
    /// 结束日期
    pub end_date: String,
    /// 小运年数
    pub years: f64,
}

/// 分盘（Varga Chart）
/// 参考原项目: 印度占星分盘系统 D-1/D-2/D-3/D-9/D-12
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VargaChart {
    /// 分盘编号
    pub division: u8,
    /// 分盘名称
    pub name: String,
    /// 分盘中文名
    pub name_zh: String,
    /// 行星在分盘中的位置
    pub planets: Vec<VargaPlanetPosition>,
    /// 上升点
    pub ascendant_sign: ZodiacSign,
}

/// 分盘行星位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VargaPlanetPosition {
    pub planet: Planet,
    pub sign: ZodiacSign,
    pub degree: f64,
}

/// 八层占（Ashtakavarga）数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AshtakavargaData {
    pub available: bool,
    /// 各星体 Bhinna Ashtakavarga
    pub bhinna: std::collections::HashMap<String, std::collections::HashMap<String, u8>>,
    /// Sarva Ashtakavarga
    pub sarva: std::collections::HashMap<String, u8>,
}

/// 五支（Panchanga）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panchanga {
    pub tithi: TithiInfo,
    pub vara: VaraInfo,
    pub nakshatra: NakshatraInfo,
    pub yoga: PanchangaYogaInfo,
    pub karana: KaranaInfo,
}

/// 太阴日（Tithi）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TithiInfo {
    /// 序号 1-30
    pub index: u8,
    /// 名称
    pub name: String,
    /// 半月（Shukla白月/Krishna黑月）
    pub paksha: String,
    /// 日月角度差
    pub angle: f64,
    /// 在当日的进度
    pub progress: f64,
}

/// 星期（Vara）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaraInfo {
    /// 0=Sunday
    pub index: u8,
    /// 星期名
    pub name: String,
    /// 中文名
    pub name_zh: String,
    /// 主星
    pub lord: Planet,
}

/// 星宿瑜伽（Panchanga 中的 Nitya Yoga，非命盘瑜伽）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanchangaYogaInfo {
    /// 序号 1-27
    pub index: u8,
    /// 名称
    pub name: String,
    /// 进度
    pub progress: f64,
}

/// 半太阴日（Karana）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaranaInfo {
    /// 序号 1-60
    pub index: u8,
    /// 名称
    pub name: String,
}

/// 星体尊严（Planetary Dignity）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetaryDignity {
    pub planet: Planet,
    /// 尊严状态：own_sign, exaltation, deep_exaltation, moolatrikona, debilitation, neutral
    pub status: String,
    /// 中文描述
    pub status_zh: String,
    /// 所在星座
    pub sign: ZodiacSign,
    /// 在星座中的度数
    pub degree_in_sign: f64,
}

/// 汉堡学派 - 中点树
/// 参考原项目: astropy/astrostudy/germany/midpoint.py
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidpointTree {
    pub planet: Planet,
    pub midpoints: Vec<Midpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Midpoint {
    pub planet1: Planet,
    pub planet2: Planet,
    pub longitude: f64,
    pub sign: ZodiacSign,
    pub degree: f64,
}

/// 星体地图（占星地理定位）
/// 参考原项目: astropy/astrostudy/acg/ACGraph.py
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcgLine {
    pub planet: Planet,
    pub line_type: AcgLineType,
    pub coordinates: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcgLineType {
    Ascendant,
    Descendant,
    Midheaven,
    ImumCoeli,
}