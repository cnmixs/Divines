// Divines - 七政四余/果老星宗类型定义
// 参考原项目: vendor/kinastro/astro/qizheng/calculator.py, constants.py
// 参考原项目: astrostudysrv/astrostudycn/QizhengMoiraRuleService.java

use serde::{Deserialize, Serialize};

use super::chart::BirthInfo;

/// 七政四余星盘
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QizhengChart {
    /// 出生信息
    pub birth: BirthInfo,
    /// 行星列表 (七政 + 四余，共11颗)
    pub planets: Vec<QizhengPlanetPosition>,
    /// 28宿列表
    pub su28: Vec<Su28Info>,
    /// 12宫位
    pub houses: [QizhengHouse; 12],
    /// 命度（黄经）
    pub life_degree: f64,
    /// 命度宿名
    pub life_su: String,
    /// 身度（黄经）
    pub body_degree: Option<f64>,
    /// 身度宿名
    pub body_su: Option<String>,
    /// 命宫索引 (0-11)
    pub ming_gong: usize,
    /// 身宫索引 (0-11)
    pub shen_gong: usize,
    /// 洞微大限
    pub dong_wei: Vec<DongWeiLimit>,
    /// 大运
    pub da_yun: Vec<QizhengDaYun>,
    /// 星盘格局
    pub patterns: Vec<String>,
    /// 神煞
    pub shen_sha: Vec<QizhengShenSha>,
    /// Moira规则（果老星宗评断）
    pub moira_rules: Vec<MoiraRuleResult>,
    /// 上升点（黄经）
    pub ascendant: f64,
    /// 中天（黄经）
    pub midheaven: f64,
    /// 节气月 (1-12)
    pub solar_month: i32,
    /// 时辰地支索引 (0-11)
    pub hour_branch: usize,
    /// 性别
    pub gender: String,
    /// 儒略日
    pub julian_day: f64,
}

/// 行星位置（黄经/赤经/赤纬）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QizhengPlanetPosition {
    /// 行星ID: Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn, Rahu, Ketu, ZiQi, YueBo
    pub id: String,
    /// 中文名: 太阳, 太阴, 水星, 金星, 火星, 木星, 土星, 罗睺, 计都, 紫气, 月孛
    pub name: String,
    /// 类型: qizheng / siyu
    pub planet_type: String,
    /// 黄经（度）
    pub lon: f64,
    /// 黄纬（度）
    pub lat: f64,
    /// 赤经（度，小时制）
    pub ra: f64,
    /// 赤纬（度）
    pub decl: f64,
    /// 星座/地支
    pub sign: String,
    /// 星座内度数
    pub sign_lon: f64,
    /// 所在宫位名
    pub house: String,
    /// 运行速度（度/日）
    pub speed: f64,
    /// 是否逆行
    pub is_retrograde: bool,
    /// 所在28宿
    pub su28: Option<String>,
    /// 入宿度
    pub su28_degree: f64,
    /// 五行属性
    pub element: String,
    /// 所在星座五行
    pub sign_element: String,
    /// 是否岐度
    pub is_qidu: bool,
    /// 高度角
    pub altitude: f64,
}

/// 12宫位
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QizhengHouse {
    /// 宫位编号 (0-11)
    pub number: u8,
    /// 宫名: 命宫, 财帛宫, 兄弟宫, 田宅宫, 男女宫, 奴仆宫, 夫妻宫, 疾厄宫, 迁移宫, 官禄宫, 福德宫, 相貌宫
    pub name: String,
    /// 宫头黄经
    pub lon: f64,
    /// 地支
    pub sign: String,
    /// 地支索引
    pub branch: usize,
    /// 28宿主宿
    pub su28: String,
    /// 宿度
    pub su28_degree: f64,
    /// 宫内行星ID列表
    pub planets: Vec<String>,
}

/// 28宿信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Su28Info {
    /// 宿名: 角, 亢, 氐...
    pub name: String,
    /// 五行属性
    pub element: String,
    /// 动物
    pub animal: String,
    /// 方位组: 东方青龙, 北方玄武, 西方白虎, 南方朱雀
    pub group: String,
    /// 赤经（度）
    pub ra: f64,
    /// 赤纬（度）
    pub decl: f64,
    /// 黄经（距星起始度）
    pub lon: f64,
    /// 所在星座
    pub sign: String,
    /// 宿宽（度）
    pub width: f64,
}

/// 洞微大限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DongWeiLimit {
    /// 起始年龄
    pub age_start: u8,
    /// 结束年龄
    pub age_end: u8,
    /// 对应宫位
    pub house: usize,
    /// 主星
    pub planet: Option<String>,
    /// 描述
    pub description: String,
}

/// 七政四余大运
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QizhengDaYun {
    /// 主星
    pub planet: String,
    /// 起始年龄
    pub age_start: u8,
    /// 结束年龄
    pub age_end: u8,
    /// 年限
    pub years: u8,
    /// 宫位名
    pub palace_name: String,
    /// 地支名
    pub branch_name: String,
    /// 起始年份
    pub start_year: i32,
    /// 结束年份
    pub end_year: i32,
}

/// 神煞
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QizhengShenSha {
    /// 神煞名
    pub name: String,
    /// 类别: 吉/凶/中
    pub category: String,
    /// 所在地支索引
    pub branch: usize,
    /// 所在地支名
    pub branch_name: String,
    /// 所在宫位
    pub house: Option<usize>,
    /// 关联行星
    pub planet: Option<String>,
    /// 来源: 地支/天干/月支/干支/月时/纳音
    pub source: String,
    /// 描述
    pub description: String,
}

/// Moira规则结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoiraRuleResult {
    /// 规则名
    pub rule_name: String,
    /// 判断: 吉/凶/平
    pub signal: String,
    /// 相关行星
    pub planets: Vec<String>,
    /// 描述
    pub description: String,
}

/// 果老星宗格局
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZhangguoPattern {
    /// 格局ID
    pub pattern_id: u32,
    /// 格局名
    pub name: String,
    /// 类型: 合格/忌格/次格/总论
    pub pattern_type: String,
    /// 分类: 日月/五星/四余/政余/...
    pub category: String,
    /// 成格条件
    pub condition: String,
    /// 说明
    pub note: String,
}

/// 星曜落宫断语
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZhangguoReading {
    /// 条目ID
    pub entry_id: u32,
    /// 星曜名
    pub star: String,
    /// 地支
    pub branch: String,
    /// 性别: both/male/female
    pub gender: String,
    /// 类型: 合格/忌格
    pub reading_type: String,
    /// 断语
    pub description: String,
    /// 出处/格名
    pub note: String,
}

/// 七政四余计算请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QizhengChartRequest {
    /// 年份
    pub year: i32,
    /// 月份
    pub month: u8,
    /// 日
    pub day: u8,
    /// 时
    pub hour: u8,
    /// 分
    pub minute: u8,
    /// 秒
    pub second: Option<u8>,
    /// 时区偏移（小时）
    pub timezone: f64,
    /// 纬度
    pub latitude: f64,
    /// 经度
    pub longitude: f64,
    /// 海拔
    pub altitude: Option<f64>,
    /// 地名
    pub place_name: Option<String>,
    /// 性别
    pub gender: Option<String>,
    /// 姓名
    pub name: Option<String>,
}

/// 星格（星体状态）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarStatus {
    /// 星名
    pub star: String,
    /// 入垣: 是否在本垣
    pub is_ru_yuan: bool,
    /// 升殿: 是否在升殿宿
    pub is_sheng_dian: bool,
    /// 庙旺
    pub is_miao_wang: bool,
    /// 喜乐
    pub is_xi_le: bool,
    /// 是否逆行
    pub is_retrograde: bool,
    /// 是否伏（距日太近）
    pub is_fu: bool,
    /// 是否疾（速度异常）
    pub is_ji: bool,
    /// 是否迟（速度慢）
    pub is_chi: bool,
    /// 是否留（即将逆行/顺行）
    pub is_liu: bool,
    /// 被伤（被克）
    pub is_bei_shang: bool,
    /// 被生（得生）
    pub is_bei_sheng: bool,
    /// 所在宿
    pub su28: String,
    /// 所在宫位
    pub house: String,
    /// 所在星座
    pub sign: String,
}