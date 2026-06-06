// 星阙 Horosa - 紫微斗数相关类型
// 参考原项目: astrostudysrv/astrostudycn/model/ZiWeiChart.java, ZiWeiHouse.java, ZiWeiStar.java, ZiWeiLuck.java, ZiWeiPattern.java

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::Gender;

// ============================================================
// 星曜类型 (StarType)
// ============================================================

/// 星曜类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StarType {
    /// 主星
    Main,
    /// 辅星
    Assist,
    /// 煞星
    Evil,
    /// 吉星（其他）
    OtherGood,
    /// 凶星（其他）
    OtherBad,
    /// 小星
    Small,
}

impl StarType {
    pub fn from_code(code: i32) -> Self {
        match code {
            0 => StarType::Main,
            1 => StarType::Assist,
            2 => StarType::Evil,
            3 => StarType::OtherGood,
            4 => StarType::OtherBad,
            5 => StarType::Small,
            _ => StarType::Small,
        }
    }
}

// ============================================================
// 星曜 (ZiWeiStar)
// ============================================================

/// 紫微斗数星曜
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZiWeiStar {
    /// 星名
    pub name: String,
    /// 星曜亮度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starlight: Option<String>,
    /// 四化（禄/权/科/忌）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sihua: Option<String>,
    /// 四化亮度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sihua_starlight: Option<String>,
    /// 四化天干
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub sihua_gan: HashMap<String, Vec<String>>,
}

impl ZiWeiStar {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            starlight: None,
            sihua: None,
            sihua_starlight: None,
            sihua_gan: HashMap::new(),
        }
    }
}

// ============================================================
// 宫位 (ZiWeiHouse)
// ============================================================

/// 紫微斗数宫位
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZiWeiHouse {
    /// 是否命宫
    #[serde(default)]
    pub is_life: bool,
    /// 是否身宫
    #[serde(default)]
    pub is_body: bool,
    /// 宫位名称
    pub name: String,
    /// 宫位干支
    pub ganzi: String,
    /// 长生十二神状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// 主星列表
    #[serde(default)]
    pub stars_main: Vec<ZiWeiStar>,
    /// 辅星列表
    #[serde(default)]
    pub stars_assist: Vec<ZiWeiStar>,
    /// 煞星列表
    #[serde(default)]
    pub stars_evil: Vec<ZiWeiStar>,
    /// 其他吉星列表
    #[serde(default)]
    pub stars_others_good: Vec<ZiWeiStar>,
    /// 其他凶星列表
    #[serde(default)]
    pub stars_others_bad: Vec<ZiWeiStar>,
    /// 小星列表
    #[serde(default)]
    pub stars_small: Vec<ZiWeiStar>,
    /// 大限范围 [start, end]
    #[serde(default)]
    pub direction: Vec<i32>,
    /// 小限年龄列表
    #[serde(default)]
    pub small_direction: Vec<i32>,
}

impl ZiWeiHouse {
    pub fn new() -> Self {
        Self {
            is_life: false,
            is_body: false,
            name: String::new(),
            ganzi: String::new(),
            phase: None,
            stars_main: Vec::new(),
            stars_assist: Vec::new(),
            stars_evil: Vec::new(),
            stars_others_good: Vec::new(),
            stars_others_bad: Vec::new(),
            stars_small: Vec::new(),
            direction: Vec::new(),
            small_direction: Vec::new(),
        }
    }

    /// 根据类型添加星曜
    pub fn add_star(&mut self, star: ZiWeiStar, star_type: StarType) {
        match star_type {
            StarType::Main => self.stars_main.push(star),
            StarType::Assist => self.stars_assist.push(star),
            StarType::Evil => self.stars_evil.push(star),
            StarType::OtherGood => self.stars_others_good.push(star),
            StarType::OtherBad => self.stars_others_bad.push(star),
            StarType::Small => self.stars_small.push(star),
        }
    }
}

impl Default for ZiWeiHouse {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// 命盘 (ZiWeiChart)
// ============================================================

/// 紫微斗数命盘
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZiWeiChart {
    /// 农历信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nongli: Option<NongLiZiWei>,
    /// 十二宫位
    pub houses: Vec<ZiWeiHouse>,
    /// 命宫索引
    pub life_house_index: usize,
    /// 身宫索引
    pub body_house_index: usize,
    /// 五行局数
    pub wuxing_ju: i32,
    /// 五行局文本
    pub wuxing_ju_text: String,
    /// 紫微星位置索引
    pub ziwei_index: usize,
    /// 性别
    pub gender: Gender,
    /// 年干
    pub year_gan: String,
    /// 年支
    pub year_zhi: String,
    /// 年干支阴阳
    pub year_polar: String,
    /// 时支
    pub time_zhi: String,
    /// 命主
    pub life_master: String,
    /// 身主
    pub body_master: String,
    /// 斗君（子斗）
    pub zidou: String,
    /// 斗君（流年斗君）
    pub doujun: String,
    /// 星曜宫位索引
    #[serde(default)]
    pub stars_house_index: HashMap<String, usize>,
    /// 出生日期
    pub birth: String,
    /// 时区
    pub zone: String,
    /// 经度
    pub lon: String,
    /// 纬度
    pub lat: String,
    /// 公元
    pub ad: i32,
    /// 时辰算法
    pub time_alg: i32,
    /// 八字信息
    #[serde(default)]
    pub bazi: serde_json::Value,
    /// 格局
    #[serde(default)]
    pub patterns: Vec<ZiWeiPattern>,
}

// ============================================================
// 农历信息 (NongLiZiWei)
// ============================================================

/// 紫微斗数所需的农历信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NongLiZiWei {
    /// 年干支
    pub year: String,
    /// 月（中文名）
    pub month: String,
    /// 月（数字）
    pub month_int: i32,
    /// 日
    pub day_int: i32,
    /// 时
    pub time: String,
    /// 日干支
    pub day_gan_zi: String,
    /// 是否闰月
    pub leap: bool,
}

// ============================================================
// 四化 (SiHua)
// ============================================================

/// 四化星
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiHuaItem {
    pub star: String,
    pub hua: String,
    pub zhi_index: i32,
}

/// 四化映射
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiHua {
    pub hua_lu: SiHuaItem,
    pub hua_quan: SiHuaItem,
    pub hua_ke: SiHuaItem,
    pub hua_ji: SiHuaItem,
}

// ============================================================
// 五行局 (WuXingJu)
// ============================================================

/// 五行局
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WuXingJu {
    /// 水二局
    Water2 = 2,
    /// 木三局
    Wood3 = 3,
    /// 金四局
    Metal4 = 4,
    /// 土五局
    Earth5 = 5,
    /// 火六局
    Fire6 = 6,
}

impl WuXingJu {
    pub fn from_number(n: i32) -> Option<Self> {
        match n {
            2 => Some(WuXingJu::Water2),
            3 => Some(WuXingJu::Wood3),
            4 => Some(WuXingJu::Metal4),
            5 => Some(WuXingJu::Earth5),
            6 => Some(WuXingJu::Fire6),
            _ => None,
        }
    }

    pub fn name_zh(&self) -> &'static str {
        match self {
            WuXingJu::Water2 => "水二局",
            WuXingJu::Wood3 => "木三局",
            WuXingJu::Metal4 => "金四局",
            WuXingJu::Earth5 => "土五局",
            WuXingJu::Fire6 => "火六局",
        }
    }
}

// ============================================================
// 大限 (DaXian)
// ============================================================

/// 大限（10年运势）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaXian {
    /// 宫位序号
    pub house_index: usize,
    /// 宫位名称
    pub house_name: String,
    /// 宫位干支
    pub ganzi: String,
    /// 天干
    pub gan: String,
    /// 起运年龄
    pub start_age: i32,
    /// 结束年龄
    pub end_age: i32,
    /// 四化
    #[serde(default)]
    pub sihua: Vec<SiHuaItem>,
}

// ============================================================
// 流年 (LiuNian)
// ============================================================

/// 流年
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiuNianInfo {
    /// 年份
    pub year: i32,
    /// 干支
    pub ganzi: String,
    /// 天干
    pub gan: String,
    /// 命宫地支索引
    pub ming_zhi_index: usize,
    /// 四化
    #[serde(default)]
    pub sihua: Vec<SiHuaItem>,
    /// 流曜
    #[serde(default)]
    pub flow_stars: Vec<FlowStar>,
    /// 斗君宫索引
    pub doujun_index: usize,
}

/// 流曜
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowStar {
    pub name: String,
    pub zhi_index: i32,
}

// ============================================================
// 小限 (XiaoXian)
// ============================================================

/// 小限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XiaoXianInfo {
    /// 年龄
    pub age: i32,
    /// 干支
    pub ganzi: String,
    /// 天干
    pub gan: String,
    /// 命宫地支索引
    pub ming_zhi_index: usize,
    /// 四化
    #[serde(default)]
    pub sihua: Vec<SiHuaItem>,
}

// ============================================================
// 格局 (ZiWeiPattern)
// ============================================================

/// 紫微格局
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZiWeiPattern {
    pub name: String,
    pub category: String,
    pub duanyi: String,
    pub source_ref: String,
    #[serde(default)]
    pub broken: bool,
}

// ============================================================
// 命主/身主 (MingZhu / ShenZhu)
// ============================================================

/// 命主
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MingShenZhu {
    pub ming_zhu: String,
    pub shen_zhu: String,
}

// ============================================================
// 输入参数 (ZiWeiInput)
// ============================================================

/// 紫微斗数计算输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZiWeiInput {
    /// 公元年（公元前为负数）
    pub ad: i32,
    /// 性别
    pub gender: Gender,
    /// 出生日期时间字符串（如 "1976-10-01 01:50"）
    pub birth: String,
    /// 时区
    pub zone: String,
    /// 经度
    pub lon: String,
    /// 纬度
    pub lat: String,
    /// 23点后是否算次日
    #[serde(default = "default_true")]
    pub after_23_new_day: bool,
    /// 自定义四化
    #[serde(default)]
    pub sihua: Option<HashMap<String, HashMap<String, String>>>,
    /// 时辰算法（0=真太阳时, 1=平太阳时）
    #[serde(default)]
    pub time_alg: i32,
    /// 是否调整节气
    #[serde(default)]
    pub adjust_jieqi: bool,
    /// 晚子时是否用次日
    #[serde(default = "default_true")]
    pub late_zi_hour_use_next_day: bool,
}

fn default_true() -> bool {
    true
}

impl Default for ZiWeiInput {
    fn default() -> Self {
        Self {
            ad: 1,
            gender: Gender::Male,
            birth: String::new(),
            zone: "+08:00".to_string(),
            lon: "116e28".to_string(),
            lat: "39n54".to_string(),
            after_23_new_day: true,
            sihua: None,
            time_alg: 0,
            adjust_jieqi: false,
            late_zi_hour_use_next_day: true,
        }
    }
}