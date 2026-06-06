// Divines - 三式相关类型（奇门、太乙、六壬）
// 参考原项目: vendor/kinqimen/, vendor/kintaiyi/, vendor/kinastro/astro/sanshi/

use serde::{Deserialize, Serialize};

// ============ 奇门遁甲 ============

/// 阴阳遁
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DunType {
    YangDun,
    YinDun,
}

impl DunType {
    pub fn name_zh(&self) -> &'static str {
        match self {
            DunType::YangDun => "阳遁",
            DunType::YinDun => "阴遁",
        }
    }
}

/// 三元
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SanYuan {
    ShangYuan,
    ZhongYuan,
    XiaYuan,
}

impl SanYuan {
    pub fn name_zh(&self) -> &'static str {
        match self {
            SanYuan::ShangYuan => "上元",
            SanYuan::ZhongYuan => "中元",
            SanYuan::XiaYuan => "下元",
        }
    }
}

/// 十二地支
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiZhi {
    Zi, Chou, Yin, Mao, Chen, Si, Wu, Wei, Shen, You, Xu, Hai,
}

impl DiZhi {
    pub fn name_zh(&self) -> &'static str {
        match self {
            DiZhi::Zi => "子", DiZhi::Chou => "丑", DiZhi::Yin => "寅",
            DiZhi::Mao => "卯", DiZhi::Chen => "辰", DiZhi::Si => "巳",
            DiZhi::Wu => "午", DiZhi::Wei => "未", DiZhi::Shen => "申",
            DiZhi::You => "酉", DiZhi::Xu => "戌", DiZhi::Hai => "亥",
        }
    }

    pub fn from_usize(n: usize) -> DiZhi {
        match n % 12 {
            0 => DiZhi::Zi, 1 => DiZhi::Chou, 2 => DiZhi::Yin,
            3 => DiZhi::Mao, 4 => DiZhi::Chen, 5 => DiZhi::Si,
            6 => DiZhi::Wu, 7 => DiZhi::Wei, 8 => DiZhi::Shen,
            9 => DiZhi::You, 10 => DiZhi::Xu, 11 => DiZhi::Hai,
            _ => DiZhi::Zi,
        }
    }

    pub fn from_str(s: &str) -> Option<DiZhi> {
        match s {
            "子" => Some(DiZhi::Zi), "丑" => Some(DiZhi::Chou),
            "寅" => Some(DiZhi::Yin), "卯" => Some(DiZhi::Mao),
            "辰" => Some(DiZhi::Chen), "巳" => Some(DiZhi::Si),
            "午" => Some(DiZhi::Wu), "未" => Some(DiZhi::Wei),
            "申" => Some(DiZhi::Shen), "酉" => Some(DiZhi::You),
            "戌" => Some(DiZhi::Xu), "亥" => Some(DiZhi::Hai),
            _ => None,
        }
    }

    pub fn index(&self) -> usize {
        *self as usize
    }
}

/// 八门
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BaMen {
    XiuMen, ShengMen, ShangMen, DuMen, JingMen, SiMen, JingMen2, KaiMen,
}

impl BaMen {
    pub fn name_zh(&self) -> &'static str {
        match self {
            BaMen::XiuMen => "休门", BaMen::ShengMen => "生门",
            BaMen::ShangMen => "伤门", BaMen::DuMen => "杜门",
            BaMen::JingMen => "景门", BaMen::SiMen => "死门",
            BaMen::JingMen2 => "惊门", BaMen::KaiMen => "开门",
        }
    }
}

/// 九星
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JiuXing {
    TianPeng, TianRui, TianChong, TianFu, TianQin, TianXin, TianZhu, TianRen, TianYing,
}

impl JiuXing {
    pub fn name_zh(&self) -> &'static str {
        match self {
            JiuXing::TianPeng => "天蓬", JiuXing::TianRui => "天芮",
            JiuXing::TianChong => "天冲", JiuXing::TianFu => "天辅",
            JiuXing::TianQin => "天禽", JiuXing::TianXin => "天心",
            JiuXing::TianZhu => "天柱", JiuXing::TianRen => "天任",
            JiuXing::TianYing => "天英",
        }
    }
}

/// 八神（阳遁/阴遁顺序不同）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BaShen {
    ZhiFu, TengShe, TaiYin, LiuHe, BaiHu, XuanWu, JiuDi, JiuTian,
}

impl BaShen {
    pub fn name_zh(&self) -> &'static str {
        match self {
            BaShen::ZhiFu => "值符", BaShen::TengShe => "螣蛇",
            BaShen::TaiYin => "太阴", BaShen::LiuHe => "六合",
            BaShen::BaiHu => "白虎", BaShen::XuanWu => "玄武",
            BaShen::JiuDi => "九地", BaShen::JiuTian => "九天",
        }
    }
}

/// 奇门遁甲盘
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QimenChart {
    /// 阴阳遁
    pub dun_type: DunType,
    /// 局数 (1-9)
    pub ju: u8,
    /// 节气
    pub jieqi: String,
    /// 三元 (上元/中元/下元)
    pub yuan: SanYuan,
    /// 用局 (阳遁几局/阴遁几局)
    pub yong_ju: String,
    /// 九宫格 [宫位1..9]
    pub gongs: [QimenGong; 9],
    /// 值符
    pub zhi_fu: String,
    /// 值使
    pub zhi_shi: String,
    /// 旬空
    pub xun_kong: [String; 2],
    /// 马星
    pub ma_xing: String,
    /// 时辰
    pub shi_chen: DiZhi,
    /// 时辰干支
    pub shi_gan_zhi: String,
    /// 日干支
    pub day_gan_zhi: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QimenGong {
    /// 宫位编号 1-9
    pub number: u8,
    /// 地盘干
    pub di_pan_gan: String,
    /// 天盘干
    pub tian_pan_gan: String,
    /// 八门
    pub men: String,
    /// 九星
    pub xing: String,
    /// 八神
    pub shen: String,
    /// 暗干
    pub an_gan: String,
    /// 地盘八神
    pub di_shen: String,
    /// 九宫信息 (八卦/五行/颜色/方位)
    pub gong_info: GongInfo,
    /// 天盘干长生状态
    pub chang_sheng: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GongInfo {
    pub name: String,        // 八卦名
    pub wuxing: String,      // 五行
    pub fang_wei: String,    // 方位
    pub color: String,       // 颜色
    pub number: u8,          // 后天数
    pub men_original: String, // 原始门
}

// ============ 太乙神数 ============

/// 太乙盘
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaiyiChart {
    /// 太乙所在宫
    pub taiyi_gong: u8,
    /// 始击所在宫
    pub shi_ji_gong: u8,
    /// 文昌所在宫
    pub wen_chang_gong: u8,
    /// 主大将
    pub zhu_da_jiang: String,
    /// 主参将
    pub zhu_can_jiang: String,
    /// 客大将
    pub ke_da_jiang: String,
    /// 客参将
    pub ke_can_jiang: String,
    /// 五福
    pub wu_fu: String,
    /// 君基
    pub jun_ji: String,
    /// 臣基
    pub chen_ji: String,
    /// 民基
    pub min_ji: String,
    /// 计神
    pub ji_shen: String,
    /// 合神
    pub he_shen: String,
    /// 太岁
    pub tai_sui: String,
    /// 十六神
    pub shi_liu_shen: [TaiyiShen; 16],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaiyiShen {
    pub name: String,
    pub gong: u8,
    pub description: String,
}

// ============ 六壬 ============

/// 六壬盘
/// 参考原项目: astrostudysrv/astrostudycn/helper/LiuRengHelper.java, model/LiuReng.java
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiurenChart {
    /// 月将 (地盘月将位置)
    pub month_jiang: String,
    /// 占时 (时辰)
    pub zhan_shi: String,
    /// 天盘 (12地支)
    pub tian_pan: [String; 12],
    /// 地盘 (12地支)
    pub di_pan: [String; 12],
    /// 四课
    pub si_ke: SiKe,
    /// 三传
    pub san_chuan: SanChuan,
    /// 遁干 (12天干)
    pub dun_gan: [String; 12],
    /// 12神将
    pub shen_jiang: [ShenJiang; 12],
    /// 贵人位置
    pub gui_ren_position: usize,
    /// 阳贵/阴贵
    pub yang_gui: bool,
    /// 天将 (12天将名称)
    pub tian_jiang: [String; 12],
    /// 六亲
    pub liu_qin: [String; 12],
    /// 德神
    pub de_shen: String,
    /// 合神
    pub he_shen: String,
    /// 鬼
    pub gui: Vec<String>,
    /// 空亡
    pub kong_wang: [String; 2],
    /// 坐山
    pub zuo_shan: String,
    /// 行年
    pub xing_nian: String,
    /// 本命
    pub ben_ming: String,
    /// 年月日时四柱
    pub four_pillars: LiurenPillars,
}

/// 四课
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiKe {
    pub ke1: (String, String),
    pub ke2: (String, String),
    pub ke3: (String, String),
    pub ke4: (String, String),
}

/// 三传
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanChuan {
    pub chu_chuan: String,
    pub zhong_chuan: String,
    pub mo_chuan: String,
}

/// 神将
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShenJiang {
    pub name: String,
    pub position: usize,
    pub description: String,
}

/// 六壬所用四柱
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiurenPillars {
    pub year: String,
    pub month: String,
    pub day: String,
    pub hour: String,
}