// 星阙 Horosa - 命盘核心类型
// 参考原项目: Horosa-Web/astrostudyui/src/models/astro.js

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 性别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    #[default]
    Male,
    Female,
}

/// 命盘类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ChartType {
    /// 本命盘
    #[default]
    Natal,
    /// 推运盘
    Predictive,
    /// 合盘
    Relationship,
    /// 辅盘
    Specialty,
    /// 印度占星盘
    Vedic,
    /// 七政四余
    Qizheng,
}

/// 地理坐标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPosition {
    /// 纬度（度）
    pub latitude: f64,
    /// 经度（度）
    pub longitude: f64,
    /// 海拔（米）
    pub altitude: f64,
    /// 时区偏移（小时）
    pub timezone_offset: f64,
    /// 地名
    pub place_name: Option<String>,
    /// 国家
    pub country: Option<String>,
}

impl Default for GeoPosition {
    fn default() -> Self {
        Self {
            latitude: 39.9042,    // 北京
            longitude: 116.4074,
            altitude: 50.0,
            timezone_offset: 8.0,
            place_name: Some("北京".to_string()),
            country: Some("中国".to_string()),
        }
    }
}

/// 出生信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirthInfo {
    /// 出生日期时间（UTC）
    pub datetime: DateTime<Utc>,
    /// 本地日期时间
    pub local_datetime: String,
    /// 出生地点
    pub location: GeoPosition,
    /// 性别
    pub gender: Gender,
    /// 姓名
    pub name: Option<String>,
}

/// 命盘信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartInfo {
    /// 唯一标识
    pub id: Uuid,
    /// 命盘类型
    pub chart_type: ChartType,
    /// 出生信息
    pub birth: BirthInfo,
    /// 标签
    pub tags: Vec<String>,
    /// 备注
    pub notes: Option<String>,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
    /// 快照数据（JSON）
    pub snapshot: Option<serde_json::Value>,
}

/// 宫位制
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum HouseSystem {
    #[default]
    Placidus,
    Koch,
    Equal,
    WholeSign,
    Regiomontanus,
    Campanus,
    Porphyry,
    Alcabitius,
    Morinus,
    Topocentric,
}

/// 行星
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Planet {
    Sun,
    Moon,
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
    NorthNode,
    SouthNode,
    Chiron,
    Lilith,
    PartOfFortune,
    Vertex,
}

/// 星座
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZodiacSign {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

/// 星座信息
impl ZodiacSign {
    pub fn name_zh(&self) -> &'static str {
        match self {
            ZodiacSign::Aries => "白羊座",
            ZodiacSign::Taurus => "金牛座",
            ZodiacSign::Gemini => "双子座",
            ZodiacSign::Cancer => "巨蟹座",
            ZodiacSign::Leo => "狮子座",
            ZodiacSign::Virgo => "处女座",
            ZodiacSign::Libra => "天秤座",
            ZodiacSign::Scorpio => "天蝎座",
            ZodiacSign::Sagittarius => "射手座",
            ZodiacSign::Capricorn => "摩羯座",
            ZodiacSign::Aquarius => "水瓶座",
            ZodiacSign::Pisces => "双鱼座",
        }
    }

    pub fn degree_range(&self) -> (f64, f64) {
        let start = *self as u8 as f64 * 30.0;
        (start, start + 30.0)
    }
}

/// 相位类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectType {
    Conjunction,     // 0° 合
    Opposition,      // 180° 冲
    Trine,           // 120° 拱
    Square,          // 90° 刑
    Sextile,         // 60° 六合
    Quincunx,        // 150° 补
    SemiSextile,     // 30° 半六合
    SemiSquare,      // 45° 半刑
    Sesquiquadrate,  // 135° 补八分
    Quintile,        // 72° 五分
    BiQuintile,      // 144° 倍五分
}

/// 相位
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aspect {
    pub aspect_type: AspectType,
    pub planet1: Planet,
    pub planet2: Planet,
    /// 精确角度
    pub angle: f64,
    /// 容许度
    pub orb: f64,
    /// 是否精确
    pub is_exact: bool,
}

/// 行星位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetPosition {
    pub planet: Planet,
    /// 黄经（度）
    pub longitude: f64,
    /// 黄纬（度）
    pub latitude: f64,
    /// 赤经（度）
    pub right_ascension: f64,
    /// 赤纬（度）
    pub declination: f64,
    /// 所在星座
    pub sign: ZodiacSign,
    /// 在星座中的度数
    pub degree_in_sign: f64,
    /// 所在宫位（1-12）
    pub house: u8,
    /// 是否逆行
    pub is_retrograde: bool,
    /// 距离
    pub distance: f64,
    /// 速度
    pub speed: f64,
}

/// 宫位
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct House {
    /// 宫位编号（1-12）
    pub number: u8,
    /// 宫头黄经（度）
    pub cusp: f64,
    /// 宫头星座
    pub sign: ZodiacSign,
    /// 宫头度数
    pub degree: f64,
    /// 宫位大小
    pub size: f64,
}

/// 上升点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ascendant {
    pub longitude: f64,
    pub sign: ZodiacSign,
    pub degree: f64,
}

/// 中天
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Midheaven {
    pub longitude: f64,
    pub sign: ZodiacSign,
    pub degree: f64,
}

/// 完整星盘
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstroChart {
    /// 命盘基本信息
    pub info: ChartInfo,
    /// 宫位制
    pub house_system: HouseSystem,
    /// 行星位置列表
    pub planets: Vec<PlanetPosition>,
    /// 宫位列表
    pub houses: Vec<House>,
    /// 上升点
    pub ascendant: Ascendant,
    /// 中天
    pub midheaven: Midheaven,
    /// 相位列表
    pub aspects: Vec<Aspect>,
    /// 恒星时
    pub sidereal_time: f64,
    /// 黄赤交角
    pub obliquity: f64,
}