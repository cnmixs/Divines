// Divines - 节气/农历/黄历相关类型
// 参考原项目: astropy/astrostudy/jieqi/, vendor/kinastro/astro/calendar/

use serde::{Deserialize, Serialize};

/// 节气信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JieQi {
    /// 节气名称
    pub name: String,
    /// 节气名称（中文）
    pub name_zh: String,
    /// 日期时间
    pub datetime: String,
    /// 太阳黄经
    pub solar_longitude: f64,
    /// 是否为"节"（非"气"）
    pub is_jie: bool,
}

/// 农历日期
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LunarDate {
    /// 农历年
    pub year: i32,
    /// 农历月
    pub month: u8,
    /// 是否为闰月
    pub is_leap_month: bool,
    /// 农历日
    pub day: u8,
    /// 农历年干支
    pub year_ganzhi: String,
    /// 农历月干支
    pub month_ganzhi: String,
    /// 农历日干支
    pub day_ganzhi: String,
    /// 生肖
    pub zodiac_animal: String,
    /// 农历月名称
    pub month_name_zh: String,
    /// 农历日名称
    pub day_name_zh: String,
}

/// 黄历
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Almanac {
    /// 公历日期
    pub solar_date: String,
    /// 农历日期
    pub lunar_date: LunarDate,
    /// 节气
    pub jie_qi: Vec<JieQi>,
    /// 宜
    pub yi: Vec<String>,
    /// 忌
    pub ji: Vec<String>,
    /// 冲煞
    pub chong_sha: String,
    /// 吉神方位
    pub ji_shen_fang_wei: FangWei,
    /// 胎神
    pub tai_shen: String,
    /// 彭祖百忌
    pub peng_zu_bai_ji: String,
}

/// 方位
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FangWei {
    pub xi_shen: String,  // 喜神方位
    pub cai_shen: String, // 财神方位
    pub fu_shen: String,  // 福神方位
    pub yang_gui: String, // 阳贵
    pub yin_gui: String,  // 阴贵
}

/// 二十四节气枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolarTerm {
    LiChun,       // 立春
    YuShui,       // 雨水
    JingZhe,      // 惊蛰
    ChunFen,      // 春分
    QingMing,     // 清明
    GuYu,         // 谷雨
    LiXia,        // 立夏
    XiaoMan,      // 小满
    MangZhong,    // 芒种
    XiaZhi,       // 夏至
    XiaoShu,      // 小暑
    DaShu,        // 大暑
    LiQiu,        // 立秋
    ChuShu,       // 处暑
    BaiLu,        // 白露
    QiuFen,       // 秋分
    HanLu,        // 寒露
    ShuangJiang,  // 霜降
    LiDong,       // 立冬
    XiaoXue,      // 小雪
    DaXue,        // 大雪
    DongZhi,      // 冬至
    XiaoHan,      // 小寒
    DaHan,        // 大寒
}

impl SolarTerm {
    pub fn name_zh(&self) -> &'static str {
        match self {
            SolarTerm::LiChun => "立春", SolarTerm::YuShui => "雨水",
            SolarTerm::JingZhe => "惊蛰", SolarTerm::ChunFen => "春分",
            SolarTerm::QingMing => "清明", SolarTerm::GuYu => "谷雨",
            SolarTerm::LiXia => "立夏", SolarTerm::XiaoMan => "小满",
            SolarTerm::MangZhong => "芒种", SolarTerm::XiaZhi => "夏至",
            SolarTerm::XiaoShu => "小暑", SolarTerm::DaShu => "大暑",
            SolarTerm::LiQiu => "立秋", SolarTerm::ChuShu => "处暑",
            SolarTerm::BaiLu => "白露", SolarTerm::QiuFen => "秋分",
            SolarTerm::HanLu => "寒露", SolarTerm::ShuangJiang => "霜降",
            SolarTerm::LiDong => "立冬", SolarTerm::XiaoXue => "小雪",
            SolarTerm::DaXue => "大雪", SolarTerm::DongZhi => "冬至",
            SolarTerm::XiaoHan => "小寒", SolarTerm::DaHan => "大寒",
        }
    }
}