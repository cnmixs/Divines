// Divines - 卦象系统类型 (梅花易数 / 六爻)
// 参考原项目: astrostudysrv/astrostudycn/helper/GuaHelper.java

use serde::{Deserialize, Serialize};

// ============ 八卦 ============

/// 八卦 (BaGua)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BaGua {
    #[serde(rename = "乾")]
    Qian,   // ☰ 天
    #[serde(rename = "兑")]
    Dui,    // ☱ 泽
    #[serde(rename = "离")]
    Li,     // ☲ 火
    #[serde(rename = "震")]
    Zhen,   // ☳ 雷
    #[serde(rename = "巽")]
    Xun,    // ☴ 风
    #[serde(rename = "坎")]
    Kan,    // ☵ 水
    #[serde(rename = "艮")]
    Gen,    // ☶ 山
    #[serde(rename = "坤")]
    Kun,    // ☷ 地
}

impl BaGua {
    pub fn name_zh(&self) -> &'static str {
        match self {
            BaGua::Qian => "乾", BaGua::Dui => "兑",
            BaGua::Li => "离",   BaGua::Zhen => "震",
            BaGua::Xun => "巽",  BaGua::Kan => "坎",
            BaGua::Gen => "艮",  BaGua::Kun => "坤",
        }
    }

    pub fn abr_name(&self) -> &'static str {
        match self {
            BaGua::Qian => "天", BaGua::Dui => "泽",
            BaGua::Li => "火",   BaGua::Zhen => "雷",
            BaGua::Xun => "风",  BaGua::Kan => "水",
            BaGua::Gen => "山",  BaGua::Kun => "地",
        }
    }

    /// 八卦的爻 (从下往上: 0=阴, 1=阳)
    pub fn yao(&self) -> [u8; 3] {
        match self {
            BaGua::Qian => [1, 1, 1],
            BaGua::Dui  => [0, 1, 1],
            BaGua::Li   => [1, 0, 1],
            BaGua::Zhen => [0, 0, 1],
            BaGua::Xun  => [1, 1, 0],
            BaGua::Kan  => [0, 1, 0],
            BaGua::Gen  => [1, 0, 0],
            BaGua::Kun  => [0, 0, 0],
        }
    }

    /// 从3爻二进制字符串获取八卦 (小端序: 最低位=初爻)
    pub fn from_yao_str(yao: &str) -> Option<BaGua> {
        match yao {
            "111" => Some(BaGua::Qian),
            "011" => Some(BaGua::Dui),
            "101" => Some(BaGua::Li),
            "001" => Some(BaGua::Zhen),
            "110" => Some(BaGua::Xun),
            "010" => Some(BaGua::Kan),
            "100" => Some(BaGua::Gen),
            "000" => Some(BaGua::Kun),
            _ => None,
        }
    }

    /// 从3爻数组获取八卦 (0=阴, 1=阳)
    pub fn from_yao_array(yao: &[u8; 3]) -> Option<BaGua> {
        let s = format!("{}{}{}", yao[0], yao[1], yao[2]);
        BaGua::from_yao_str(&s)
    }

    /// 八卦的先天数 (乾1 兑2 离3 震4 巽5 坎6 艮7 坤8)
    pub fn xian_tian_number(&self) -> u8 {
        match self {
            BaGua::Qian => 1, BaGua::Dui => 2,
            BaGua::Li => 3,   BaGua::Zhen => 4,
            BaGua::Xun => 5,  BaGua::Kan => 6,
            BaGua::Gen => 7,  BaGua::Kun => 8,
        }
    }

    /// 从先天数获取八卦
    pub fn from_xian_tian_number(n: u8) -> Option<BaGua> {
        match n {
            1 => Some(BaGua::Qian), 2 => Some(BaGua::Dui),
            3 => Some(BaGua::Li),   4 => Some(BaGua::Zhen),
            5 => Some(BaGua::Xun),  6 => Some(BaGua::Kan),
            7 => Some(BaGua::Gen),  8 => Some(BaGua::Kun),
            _ => None,
        }
    }
}

// ============ 六十四卦 ============

/// 六十四卦的爻辞
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YaoCi {
    pub position: u8,       // 1-6 (初爻到上爻)
    pub text: String,       // 爻辞文字
    pub xiang: String,      // 爻象解释
}

/// 六十四卦
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiuShiSiGua {
    /// 序号 (1-64)
    pub ord: u32,
    /// 卦名 (如 "乾")
    pub name: String,
    /// 卦全名 (如 "乾乾" 即上乾下乾)
    pub gua_name: String,
    /// 简称 (如 "天天")
    pub abr_name: String,
    /// 描述 (如 "乾为天，乾上乾下")
    pub desc: String,
    /// 六爻 (从初爻到上爻, 0=阴, 1=阳)
    pub yao: [u8; 6],
    /// 卦辞
    pub gua_ci: String,
    /// 爻辞 (含用九/用六)
    pub yao_ci: Vec<YaoCi>,
    /// 彖辞
    pub tuan_ci: String,
    /// 大象辞
    pub xiang_ci: String,
    /// 文言 (乾坤二卦有)
    pub wen_yan: Option<String>,
    /// 参考资料 URL
    pub url: Option<String>,
    /// 卦象象征
    pub symbolize: Vec<String>,
}

/// 梅花易数八卦属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeiYiGua {
    /// 序号 (1-8)
    pub ord: u32,
    /// 卦名
    pub name: String,
    /// 简称
    pub abr_name: String,
    /// 描述
    pub desc: String,
    /// 三爻
    pub yao: [u8; 3],
    /// 梅花易数分类
    pub mei_yi: Option<MeiYiCategory>,
    /// 象征
    pub symbolize: Vec<String>,
}

/// 梅花易数分类占
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeiYiCategory {
    pub ren_wu: Vec<String>,    // 人物
    pub shen_ti: Vec<String>,   // 身体
    pub dong_wu: Vec<String>,   // 动物
    pub wu_pin: Vec<String>,    // 物品
    pub chang_suo: Vec<String>, // 场所
    pub tian_xiang: Vec<String>,// 天象
    pub shi_jian: Vec<String>,  // 时间
    pub shu_zi: Vec<String>,    // 数字
    pub fang_wei: Vec<String>,  // 方位
    pub gan_zhi: Vec<String>,   // 干支
    pub wei: Vec<String>,       // 味
    pub se: Vec<String>,        // 色
}

/// 梅花易数盘
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeiHuaYiChart {
    /// 上卦 (本卦)
    pub upper_gua: BaGua,
    /// 下卦 (本卦)
    pub lower_gua: BaGua,
    /// 本卦 (六十四卦)
    pub ben_gua: LiuShiSiGua,
    /// 动爻位置 (1-6, None表示无动爻)
    pub changing_yao: Option<u8>,
    /// 互卦
    pub hu_gua: Option<LiuShiSiGua>,
    /// 互卦上卦
    pub hu_upper: Option<BaGua>,
    /// 互卦下卦
    pub hu_lower: Option<BaGua>,
    /// 变卦
    pub bian_gua: Option<LiuShiSiGua>,
    /// 体卦
    pub ti_gua: BaGua,
    /// 用卦
    pub yong_gua: BaGua,
}

/// 卦象关系 (错卦、综卦、互卦)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuaRelation {
    /// 错卦 (阴阳爻全部相反)
    pub cuo_gua: Option<LiuShiSiGua>,
    /// 综卦 (上下颠倒)
    pub zong_gua: Option<LiuShiSiGua>,
    /// 互卦 (234爻+345爻)
    pub hu_gua: Option<LiuShiSiGua>,
}

/// 四象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiXiang {
    pub yao: [u8; 2],
    pub name: String,
    pub season: String,
    pub xiang: String,
}

/// 干支卦映射表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GanZhiGuaMap {
    pub map: std::collections::HashMap<String, String>,
}

// ============ 六爻排盘 ============

/// 爻类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum YaoType {
    #[serde(rename = "—")]
    Yang,       // 阳爻 —
    #[serde(rename = "- -")]
    Yin,        // 阴爻 - -
    #[serde(rename = "—o")]
    LaoYang,    // 老阳 —o (动爻阳变阴)
    #[serde(rename = "- -x")]
    Laoyin,     // 老阴 - -x (动爻阴变阳)
}

/// 六爻摇卦结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiuyaoGua {
    /// 卦名
    pub name: String,
    /// 卦象符号
    pub symbol: String,
    /// 六爻详情
    pub yaos: [Yao; 6],
    /// 世应
    pub shi_ying: ShiYing,
    /// 六亲
    pub liu_qin: [String; 6],
    /// 六兽
    pub liu_shou: [String; 6],
    /// 伏神
    pub fu_shen: Vec<String>,
    /// 动变
    pub dong_bian: Vec<(usize, String)>,
    /// 用神
    pub yong_shen: String,
}

/// 单个爻
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Yao {
    pub position: u8,   // 1-6 (初爻到上爻)
    pub yao_type: YaoType,
    pub di_zhi: String,
    pub liu_qin: String,
    pub liu_shou: String,
}

/// 世应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiYing {
    pub shi: usize,   // 世爻位置 (1-6)
    pub ying: usize,  // 应爻位置 (1-6)
}