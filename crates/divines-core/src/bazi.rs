// Divines - 八字相关类型
// 参考原项目: astrostudysrv/astrostudycn/BaZiBirth.java, PaiBaZi.java

use serde::{Deserialize, Serialize};

/// 天干
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TianGan {
    #[serde(rename = "甲")]
    Jia,
    #[serde(rename = "乙")]
    Yi,
    #[serde(rename = "丙")]
    Bing,
    #[serde(rename = "丁")]
    Ding,
    #[serde(rename = "戊")]
    Wu,
    #[serde(rename = "己")]
    Ji,
    #[serde(rename = "庚")]
    Geng,
    #[serde(rename = "辛")]
    Xin,
    #[serde(rename = "壬")]
    Ren,
    #[serde(rename = "癸")]
    Gui,
}

impl TianGan {
    pub fn name_zh(&self) -> &'static str {
        match self {
            TianGan::Jia => "甲", TianGan::Yi => "乙",
            TianGan::Bing => "丙", TianGan::Ding => "丁",
            TianGan::Wu => "戊", TianGan::Ji => "己",
            TianGan::Geng => "庚", TianGan::Xin => "辛",
            TianGan::Ren => "壬", TianGan::Gui => "癸",
        }
    }
}

/// 地支
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiZhi {
    #[serde(rename = "子")]
    Zi,
    #[serde(rename = "丑")]
    Chou,
    #[serde(rename = "寅")]
    Yin,
    #[serde(rename = "卯")]
    Mao,
    #[serde(rename = "辰")]
    Chen,
    #[serde(rename = "巳")]
    Si,
    #[serde(rename = "午")]
    Wu,
    #[serde(rename = "未")]
    Wei,
    #[serde(rename = "申")]
    Shen,
    #[serde(rename = "酉")]
    You,
    #[serde(rename = "戌")]
    Xu,
    #[serde(rename = "亥")]
    Hai,
}

impl DiZhi {
    pub fn name_zh(&self) -> &'static str {
        match self {
            DiZhi::Zi => "子", DiZhi::Chou => "丑",
            DiZhi::Yin => "寅", DiZhi::Mao => "卯",
            DiZhi::Chen => "辰", DiZhi::Si => "巳",
            DiZhi::Wu => "午", DiZhi::Wei => "未",
            DiZhi::Shen => "申", DiZhi::You => "酉",
            DiZhi::Xu => "戌", DiZhi::Hai => "亥",
        }
    }
}

/// 五行
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WuXing {
    #[serde(rename = "木")]
    Wood,
    #[serde(rename = "火")]
    Fire,
    #[serde(rename = "土")]
    Earth,
    #[serde(rename = "金")]
    Metal,
    #[serde(rename = "水")]
    Water,
}

impl WuXing {
    pub fn name_zh(&self) -> &'static str {
        match self {
            WuXing::Wood => "木", WuXing::Fire => "火",
            WuXing::Earth => "土", WuXing::Metal => "金",
            WuXing::Water => "水",
        }
    }
}

/// 阴阳
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum YinYang {
    #[serde(rename = "阴")]
    Yin,
    #[serde(rename = "阳")]
    Yang,
}

/// 天干地支组合（一柱）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pillar {
    pub tian_gan: TianGan,
    pub di_zhi: DiZhi,
}

/// 四柱八字
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziChart {
    /// 年柱
    pub year: Pillar,
    /// 月柱
    pub month: Pillar,
    /// 日柱
    pub day: Pillar,
    /// 时柱
    pub hour: Pillar,
    /// 日干
    pub day_master: TianGan,
    /// 日干五行
    pub day_master_wuxing: WuXing,
    /// 日干阴阳
    pub day_master_yinyang: YinYang,
    /// 藏干
    pub hidden_stems: HiddenStems,
    /// 十神
    pub ten_gods: TenGods,
    /// 纳音
    pub na_yin: NaYin,
    /// 空亡
    pub kong_wang: [DiZhi; 2],
    /// 神煞
    pub shen_sha: Vec<ShenSha>,
    /// 大运
    pub da_yun: Vec<DaYun>,
    /// 起运时间
    pub qi_yun_time: String,
    /// 八字格局
    pub pattern: BaziPattern,
    /// 长生十二神状态
    pub chang_sheng: ChangShengState,
    /// 干支刑冲合害关系
    pub relations: Vec<GanZhiRelation>,
    /// 排盘选项
    pub options: BaziOptions,
    /// 调整后的小时（真太阳时校正后）
    pub adjusted_hour: f64,
    /// 季节旺衰（五行在月支上的旺衰状态）
    #[serde(default)]
    pub season: std::collections::HashMap<String, String>,
    /// 调候用神
    #[serde(default)]
    pub tiaohou: Vec<String>,
    /// 十二宫（流年神煞）
    #[serde(default)]
    pub gong12: serde_json::Value,
}

/// 藏干
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiddenStems {
    pub year: Vec<TianGan>,
    pub month: Vec<TianGan>,
    pub day: Vec<TianGan>,
    pub hour: Vec<TianGan>,
}

/// 十神
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenGods {
    pub year: TenGod,
    pub month: TenGod,
    pub day: TenGod,
    pub hour: TenGod,
}

/// 十神类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TenGod {
    #[serde(rename = "正印")]
    ZhengYin,
    #[serde(rename = "偏印")]
    PianYin,
    #[serde(rename = "正官")]
    ZhengGuan,
    #[serde(rename = "七杀")]
    PianGuan,
    #[serde(rename = "正财")]
    ZhengCai,
    #[serde(rename = "偏财")]
    PianCai,
    #[serde(rename = "食神")]
    ShiShen,
    #[serde(rename = "伤官")]
    ShangGuan,
    #[serde(rename = "比肩")]
    BiJian,
    #[serde(rename = "劫财")]
    JieCai,
}

impl TenGod {
    pub fn name_zh(&self) -> &'static str {
        match self {
            TenGod::ZhengYin => "正印", TenGod::PianYin => "偏印",
            TenGod::ZhengGuan => "正官", TenGod::PianGuan => "七杀",
            TenGod::ZhengCai => "正财", TenGod::PianCai => "偏财",
            TenGod::ShiShen => "食神", TenGod::ShangGuan => "伤官",
            TenGod::BiJian => "比肩", TenGod::JieCai => "劫财",
        }
    }
}

/// 纳音
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaYin {
    pub year: String,
    pub month: String,
    pub day: String,
    pub hour: String,
}

/// 神煞
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShenSha {
    pub name: String,
    pub pillar: String,
    pub description: String,
}

/// 大运
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaYun {
    pub pillar: Pillar,
    pub start_age: u8,
    pub end_age: u8,
    pub start_year: i32,
    pub end_year: i32,
    pub ten_god: TenGod,
    pub liu_nian: Vec<LiuNian>,
}

/// 流年
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiuNian {
    pub year: i32,
    pub pillar: Pillar,
    pub ten_god: TenGod,
}

/// 八字格局
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziPattern {
    pub name: String,
    pub description: String,
    pub strength: PatternStrength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternStrength {
    Weak,
    Normal,
    Strong,
    VeryStrong,
}

// ============ 长生十二神 ============

/// 长生十二神
///
/// 天干在地支上的十二种状态，反映五行力量强弱
/// 参考: RedSC1/bazi_core
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangSheng12 {
    #[serde(rename = "长生")]
    ChangSheng,
    #[serde(rename = "沐浴")]
    MuYu,
    #[serde(rename = "冠带")]
    GuanDai,
    #[serde(rename = "临官")]
    LinGuan,
    #[serde(rename = "帝旺")]
    DiWang,
    #[serde(rename = "衰")]
    Shuai,
    #[serde(rename = "病")]
    Bing,
    #[serde(rename = "死")]
    Si,
    #[serde(rename = "墓")]
    Mu,
    #[serde(rename = "绝")]
    Jue,
    #[serde(rename = "胎")]
    Tai,
    #[serde(rename = "养")]
    Yang,
}

impl ChangSheng12 {
    pub fn name_zh(&self) -> &'static str {
        match self {
            ChangSheng12::ChangSheng => "长生",
            ChangSheng12::MuYu => "沐浴",
            ChangSheng12::GuanDai => "冠带",
            ChangSheng12::LinGuan => "临官",
            ChangSheng12::DiWang => "帝旺",
            ChangSheng12::Shuai => "衰",
            ChangSheng12::Bing => "病",
            ChangSheng12::Si => "死",
            ChangSheng12::Mu => "墓",
            ChangSheng12::Jue => "绝",
            ChangSheng12::Tai => "胎",
            ChangSheng12::Yang => "养",
        }
    }
}

/// 四柱长生十二神状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangShengState {
    pub year: ChangSheng12,
    pub month: ChangSheng12,
    pub day: ChangSheng12,
    pub hour: ChangSheng12,
}

// ============ 干支刑冲合害 ============

/// 干支关系类型（15种复杂关系）
/// 参考: RedSC1/bazi_core
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationType {
    #[serde(rename = "天干合")]
    TianGanHe,
    #[serde(rename = "天干冲")]
    TianGanChong,
    #[serde(rename = "地支六合")]
    DiZhiLiuHe,
    #[serde(rename = "地支三合")]
    DiZhiSanHe,
    #[serde(rename = "地支半合")]
    DiZhiBanHe,
    #[serde(rename = "地支六冲")]
    DiZhiLiuChong,
    #[serde(rename = "地支六害")]
    DiZhiLiuHai,
    #[serde(rename = "地支相刑")]
    DiZhiXiangXing,
    #[serde(rename = "地支自刑")]
    DiZhiZiXing,
    #[serde(rename = "地支三会")]
    DiZhiSanHui,
    #[serde(rename = "地支相破")]
    DiZhiPo,
    #[serde(rename = "地支相绝")]
    DiZhiJue,
    #[serde(rename = "三合局")]
    SanHeJu,
    #[serde(rename = "三会局")]
    SanHuiJu,
    #[serde(rename = "拱合")]
    GongHe,
    #[serde(rename = "暗合")]
    AnHe,
}

impl RelationType {
    pub fn name_zh(&self) -> &'static str {
        match self {
            RelationType::TianGanHe => "天干合",
            RelationType::TianGanChong => "天干冲",
            RelationType::DiZhiLiuHe => "地支六合",
            RelationType::DiZhiSanHe => "地支三合",
            RelationType::DiZhiBanHe => "地支半合",
            RelationType::DiZhiLiuChong => "地支六冲",
            RelationType::DiZhiLiuHai => "地支六害",
            RelationType::DiZhiXiangXing => "地支相刑",
            RelationType::DiZhiZiXing => "地支自刑",
            RelationType::DiZhiSanHui => "地支三会",
            RelationType::DiZhiPo => "地支相破",
            RelationType::DiZhiJue => "地支相绝",
            RelationType::SanHeJu => "三合局",
            RelationType::SanHuiJu => "三会局",
            RelationType::GongHe => "拱合",
            RelationType::AnHe => "暗合",
        }
    }
}

/// 干支关系描述
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GanZhiRelation {
    /// 关系类型
    pub relation_type: RelationType,
    /// 涉及的四柱位置（"year", "month", "day", "hour"）
    pub pillars: Vec<String>,
    /// 涉及的干支详情
    pub detail: String,
    /// 关系描述
    pub description: String,
}

// ============ 排盘选项 ============

/// 八字排盘选项
///
/// 支持真太阳时、早晚子时配置、平气/定气选择
/// 参考: hkargc/JavaScript-For-Paipan, tiandirenwx/libsxwnl
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziOptions {
    /// 是否使用真太阳时
    pub use_true_solar_time: bool,
    /// 出生地经度（用于真太阳时计算）
    pub longitude: f64,
    /// 是否区分早晚子时
    pub use_early_late_zi: bool,
    /// 使用定气法（true）还是平气法（false）
    pub use_ding_qi: bool,
}

impl Default for BaziOptions {
    fn default() -> Self {
        Self {
            use_true_solar_time: false,
            longitude: 116.4074, // 默认北京
            use_early_late_zi: false,
            use_ding_qi: true,
        }
    }
}