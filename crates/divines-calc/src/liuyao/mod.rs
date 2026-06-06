// Divines - 六爻排盘计算引擎
// 参考原项目: divines-Web/astrostudyui/src/components/gua/GuaConst.js
// 参考原项目: divines-original/modules/reference/xuan-utils-pro-master/src/main/java/xuan/core/liuyao/

use divines_core::liuyao::*;
use divines_core::chart::BirthInfo;
use crate::sxwnl::Sxwnl;
use chrono::Datelike;
use serde::{Serialize, Deserialize};

use rand::Rng;

// ============================================================
// 数据表
// ============================================================

/// 八卦宫位五行
const PALACE_ELEMENT: &[(&str, &str)] = &[
    ("乾", "金"), ("兑", "金"), ("离", "火"), ("震", "木"),
    ("巽", "木"), ("坎", "水"), ("艮", "土"), ("坤", "土"),
];

/// 地支五行
const DI_ZHI_ELEMENT: &[(&str, &str)] = &[
    ("子", "水"), ("丑", "土"), ("寅", "木"), ("卯", "木"),
    ("辰", "土"), ("巳", "火"), ("午", "火"), ("未", "土"),
    ("申", "金"), ("酉", "金"), ("戌", "土"), ("亥", "水"),
];

/// 六亲映射: [宫位五行][爻地支五行] → 六亲名称
/// 顺序: 金, 木, 水, 火, 土
const LIU_QIN_TABLE: &[[&str; 5]; 5] = &[
    // 金宫: 金=兄弟, 木=妻财, 水=子孙, 火=官鬼, 土=父母
    ["兄弟", "妻财", "子孙", "官鬼", "父母"],
    // 木宫: 金=官鬼, 木=兄弟, 水=父母, 火=子孙, 土=妻财
    ["官鬼", "兄弟", "父母", "子孙", "妻财"],
    // 水宫: 金=父母, 木=子孙, 水=兄弟, 火=妻财, 土=官鬼
    ["父母", "子孙", "兄弟", "妻财", "官鬼"],
    // 火宫: 金=妻财, 木=父母, 水=官鬼, 火=兄弟, 土=子孙
    ["妻财", "父母", "官鬼", "兄弟", "子孙"],
    // 土宫: 金=子孙, 木=官鬼, 水=妻财, 火=父母, 土=兄弟
    ["子孙", "官鬼", "妻财", "父母", "兄弟"],
];

/// 六兽: 按日干排列 (从初爻到上爻)
/// 甲乙日: 青龙、朱雀、勾陈、螣蛇、白虎、玄武
/// 丙丁日: 朱雀、勾陈、螣蛇、白虎、玄武、青龙
/// 戊日:   勾陈、螣蛇、白虎、玄武、青龙、朱雀
/// 己日:   螣蛇、白虎、玄武、青龙、朱雀、勾陈
/// 庚辛日: 白虎、玄武、青龙、朱雀、勾陈、螣蛇
/// 壬癸日: 玄武、青龙、朱雀、勾陈、螣蛇、白虎
const LIU_SHOU_TABLE: &[(&str, &[&str; 6])] = &[
    ("甲", &["青龙", "朱雀", "勾陈", "螣蛇", "白虎", "玄武"]),
    ("乙", &["青龙", "朱雀", "勾陈", "螣蛇", "白虎", "玄武"]),
    ("丙", &["朱雀", "勾陈", "螣蛇", "白虎", "玄武", "青龙"]),
    ("丁", &["朱雀", "勾陈", "螣蛇", "白虎", "玄武", "青龙"]),
    ("戊", &["勾陈", "螣蛇", "白虎", "玄武", "青龙", "朱雀"]),
    ("己", &["螣蛇", "白虎", "玄武", "青龙", "朱雀", "勾陈"]),
    ("庚", &["白虎", "玄武", "青龙", "朱雀", "勾陈", "螣蛇"]),
    ("辛", &["白虎", "玄武", "青龙", "朱雀", "勾陈", "螣蛇"]),
    ("壬", &["玄武", "青龙", "朱雀", "勾陈", "螣蛇", "白虎"]),
    ("癸", &["玄武", "青龙", "朱雀", "勾陈", "螣蛇", "白虎"]),
];

/// 六十四卦数据
/// 格式: (二进制key, 卦名, 宫位, 六爻干支六亲世应)
const GUA_64_DATA: &[(&str, &str, &str, &[&str; 6])] = &[
    // ===== 乾宫 (金) =====
    ("111111", "乾为天", "乾", &["子水子孙", "寅木妻财", "辰土父母应", "午火官鬼", "申金兄弟", "戌土父母世"]),
    ("011111", "天风姤", "乾", &["丑土父母世", "亥水子孙", "酉金兄弟", "午火官鬼应", "申金兄弟", "戌土父母"]),
    ("001111", "天山遁", "乾", &["辰土父母", "午火官鬼世", "申金兄弟", "午火官鬼", "申金兄弟应", "戌土父母"]),
    ("000111", "天地否", "乾", &["未土父母", "巳火官鬼", "卯木妻财世", "午火官鬼", "申金兄弟", "戌土父母应"]),
    ("000011", "风地观", "乾", &["未土父母应", "巳火官鬼", "卯木妻财", "未土父母世", "巳火官鬼", "卯木妻财"]),
    ("000001", "山地剥", "乾", &["未土父母", "巳火官鬼应", "卯木妻财", "戌土父母", "子水子孙世", "寅木妻财"]),
    ("000101", "火地晋", "乾", &["未土父母应", "巳火官鬼", "卯木妻财", "酉金兄弟世", "未土父母", "巳火官鬼"]),
    ("111101", "火天大有", "乾", &["子水子孙", "寅木妻财", "辰土父母世", "酉金兄弟", "未土父母", "巳火官鬼应"]),

    // ===== 坎宫 (水) =====
    ("010010", "坎为水", "坎", &["寅木子孙", "辰土官鬼", "午火妻财应", "申金父母", "戌土官鬼", "子水兄弟世"]),
    ("110010", "水泽节", "坎", &["巳火妻财世", "卯木子孙", "丑土官鬼", "申金父母应", "戌土官鬼", "子水兄弟"]),
    ("100010", "水雷屯", "坎", &["子水兄弟", "寅木子孙世", "辰土官鬼", "申金父母", "戌土官鬼应", "子水兄弟"]),
    ("101010", "水火既济", "坎", &["卯木子孙", "丑土官鬼", "亥水兄弟世", "申金父母", "戌土官鬼", "子水兄弟应"]),
    ("101110", "泽火革", "坎", &["卯木子孙应", "丑土官鬼", "亥水兄弟", "亥水兄弟世", "酉金父母", "未土官鬼"]),
    ("101100", "雷火丰", "坎", &["卯木子孙", "丑土官鬼应", "亥水兄弟", "午火妻财", "申金父母世", "戌土官鬼"]),
    ("101000", "地火明夷", "坎", &["卯木子孙应", "丑土官鬼", "亥水兄弟", "丑土官鬼世", "亥水兄弟", "酉金父母"]),
    ("010000", "地水师", "坎", &["寅木子孙", "辰土官鬼", "午火妻财世", "丑土官鬼", "亥水兄弟", "酉金父母应"]),

    // ===== 艮宫 (土) =====
    ("001001", "艮为山", "艮", &["辰土兄弟", "午火父母", "申金子孙应", "戌土兄弟", "子水妻财", "寅木官鬼世"]),
    ("101001", "山火贲", "艮", &["卯木官鬼世", "丑土兄弟", "亥水妻财", "戌土兄弟应", "子水妻财", "寅木官鬼"]),
    ("111001", "山天大畜", "艮", &["子水妻财", "寅木官鬼世", "辰土兄弟", "戌土兄弟", "子水妻财应", "寅木官鬼"]),
    ("110001", "山泽损", "艮", &["巳火父母", "卯木官鬼", "丑土兄弟世", "戌土兄弟", "子水妻财", "寅木官鬼应"]),
    ("110101", "火泽睽", "艮", &["巳火父母应", "卯木官鬼", "丑土兄弟", "酉金子孙世", "未土兄弟", "巳火父母"]),
    ("110111", "天泽履", "艮", &["巳火父母", "卯木官鬼应", "丑土兄弟", "午火父母", "申金子孙世", "戌土兄弟"]),
    ("110011", "风泽中孚", "艮", &["巳火父母应", "卯木官鬼", "丑土兄弟", "未土兄弟世", "巳火父母", "卯木官鬼"]),
    ("001011", "风山渐", "艮", &["辰土兄弟", "午火父母", "申金子孙世", "未土兄弟", "巳火父母", "卯木官鬼应"]),

    // ===== 震宫 (木) =====
    ("100100", "震为雷", "震", &["子水父母", "寅木兄弟", "辰土妻财应", "午火子孙", "申金官鬼", "戌土妻财世"]),
    ("000100", "雷地豫", "震", &["未土妻财世", "巳火子孙", "卯木兄弟", "午火子孙应", "申金官鬼", "戌土妻财"]),
    ("010100", "雷水解", "震", &["寅木兄弟", "辰土妻财世", "午火子孙", "午火子孙", "申金官鬼应", "戌土妻财"]),
    ("011100", "雷风恒", "震", &["丑土妻财", "亥水父母", "酉金官鬼世", "午火子孙", "申金官鬼", "戌土妻财应"]),
    ("011000", "地风升", "震", &["丑土妻财应", "亥水父母", "酉金官鬼", "丑土妻财世", "亥水父母", "酉金官鬼"]),
    ("011010", "水风井", "震", &["丑土妻财", "亥水父母应", "酉金官鬼", "申金官鬼", "戌土妻财世", "子水父母"]),
    ("011110", "泽风大过", "震", &["丑土妻财应", "亥水父母", "酉金官鬼", "亥水父母世", "酉金官鬼", "未土妻财"]),
    ("100110", "泽雷随", "震", &["子水父母", "寅木兄弟", "辰土妻财世", "亥水父母", "酉金官鬼", "未土妻财应"]),

    // ===== 巽宫 (木) =====
    ("011011", "巽为风", "巽", &["丑土妻财", "亥水父母", "酉金官鬼应", "未土妻财", "巳火子孙", "卯木兄弟世"]),
    ("111011", "风天小畜", "巽", &["子水父母世", "寅木兄弟", "辰土妻财", "未土妻财应", "巳火子孙", "卯木兄弟"]),
    ("101011", "风火家人", "巽", &["卯木兄弟", "丑土妻财世", "亥水父母", "未土妻财", "巳火子孙应", "卯木兄弟"]),
    ("100011", "风雷益", "巽", &["子水父母", "寅木兄弟", "辰土妻财世", "未土妻财", "巳火子孙", "卯木兄弟应"]),
    ("100111", "天雷无妄", "巽", &["子水父母应", "寅木兄弟", "辰土妻财", "午火子孙世", "申金官鬼", "戌土妻财"]),
    ("100101", "火雷噬嗑", "巽", &["子水父母", "寅木兄弟应", "辰土妻财", "酉金官鬼", "未土妻财世", "巳火子孙"]),
    ("100001", "山雷颐", "巽", &["子水父母应", "寅木兄弟", "辰土妻财", "戌土妻财世", "子水父母", "寅木兄弟"]),
    ("011001", "山风蛊", "巽", &["丑土妻财", "亥水父母", "酉金官鬼世", "戌土妻财", "子水父母", "寅木兄弟应"]),

    // ===== 离宫 (火) =====
    ("101101", "离为火", "离", &["卯木父母", "丑土子孙", "亥水官鬼应", "酉金妻财", "未土子孙", "巳火兄弟世"]),
    ("001101", "火山旅", "离", &["辰土子孙世", "午火兄弟", "申金妻财", "酉金妻财应", "未土子孙", "巳火兄弟"]),
    ("011101", "火风鼎", "离", &["丑土子孙", "亥水官鬼世", "酉金妻财", "酉金妻财", "未土子孙应", "巳火兄弟"]),
    ("010101", "火水未济", "离", &["寅木父母", "辰土子孙", "午火兄弟世", "酉金妻财", "未土子孙", "巳火兄弟应"]),
    ("010001", "山水蒙", "离", &["寅木父母应", "辰土子孙", "午火兄弟", "戌土子孙世", "子水官鬼", "寅木父母"]),
    ("010011", "风水涣", "离", &["寅木父母", "辰土子孙应", "午火兄弟", "未土子孙", "巳火兄弟世", "卯木父母"]),
    ("010111", "天水讼", "离", &["寅木父母应", "辰土子孙", "午火兄弟", "午火兄弟世", "申金妻财", "戌土子孙"]),
    ("101111", "天火同人", "离", &["卯木父母", "丑土子孙", "亥水官鬼世", "午火兄弟", "申金妻财", "戌土子孙应"]),

    // ===== 坤宫 (土) =====
    ("000000", "坤为地", "坤", &["未土兄弟", "巳火父母", "卯木官鬼应", "丑土兄弟", "亥水妻财", "酉金子孙世"]),
    ("100000", "地雷复", "坤", &["子水妻财世", "寅木官鬼", "辰土兄弟", "丑土兄弟应", "亥水妻财", "酉金子孙"]),
    ("110000", "地泽临", "坤", &["巳火父母", "卯木官鬼世", "丑土兄弟", "丑土兄弟", "亥水妻财应", "酉金子孙"]),
    ("111000", "地天泰", "坤", &["子水妻财", "寅木官鬼", "辰土兄弟世", "丑土兄弟", "亥水妻财", "酉金子孙应"]),
    ("111100", "雷天大壮", "坤", &["子水妻财应", "寅木官鬼", "辰土兄弟", "午火父母世", "申金子孙", "戌土兄弟"]),
    ("111110", "泽天夬", "坤", &["子水妻财", "寅木官鬼应", "辰土兄弟", "亥水妻财", "酉金子孙世", "未土兄弟"]),
    ("111010", "水天需", "坤", &["子水妻财应", "寅木官鬼", "辰土兄弟", "申金子孙世", "戌土兄弟", "子水妻财"]),
    ("000010", "水地比", "坤", &["未土兄弟", "巳火父母", "卯木官鬼世", "申金子孙", "戌土兄弟", "子水妻财应"]),

    // ===== 兑宫 (金) =====
    ("110110", "兑为泽", "兑", &["巳火官鬼", "卯木妻财", "丑土父母应", "亥水子孙", "酉金兄弟", "未土父母世"]),
    ("010110", "泽水困", "兑", &["寅木妻财世", "辰土父母", "午火官鬼", "亥水子孙应", "酉金兄弟", "未土父母"]),
    ("000110", "泽地萃", "兑", &["未土父母", "巳火官鬼世", "卯木妻财", "亥水子孙", "酉金兄弟应", "未土父母"]),
    ("001110", "泽山咸", "兑", &["辰土父母", "午火官鬼", "亥水子孙世", "亥水子孙", "酉金兄弟", "未土父母应"]),
    ("001010", "水山蹇", "兑", &["辰土父母应", "午火官鬼", "申金兄弟", "申金兄弟世", "戌土父母", "子水子孙"]),
    ("001000", "地山谦", "兑", &["辰土父母", "午火官鬼应", "申金兄弟", "丑土父母", "亥水子孙世", "酉金兄弟"]),
    ("001100", "雷山小过", "兑", &["辰土父母应", "午火官鬼", "申金兄弟", "午火官鬼世", "申金兄弟", "戌土父母"]),
    ("110100", "雷泽归妹", "兑", &["巳火官鬼", "卯木妻财", "丑土父母世", "午火官鬼", "申金兄弟", "戌土父母应"]),
];

/// 用神关键词映射
const YONG_SHEN_KEYWORDS: &[(&str, &[&str])] = &[
    ("妻财", &["财", "财运", "财富", "求财", "生意", "投资", "金钱", "盈利", "收入", "赚钱"]),
    ("官鬼", &["官", "事业", "工作", "官运", "功名", "职位", "升迁", "求职", "官司", "诉讼"]),
    ("子孙", &["子", "子女", "孩子", "后代", "健康", "疾病", "身体", "出行", "旅行", "娱乐"]),
    ("父母", &["父母", "长辈", "学业", "考试", "文书", "证书", "房屋", "车辆", "合同"]),
    ("兄弟", &["兄弟", "朋友", "合作", "合伙", "竞争", "同事", "姐妹"]),
];

// ============================================================
// 辅助类型
// ============================================================

/// 单次铜钱摇卦结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YaoCastItem {
    /// 0=阴爻, 1=阳爻
    pub value: u8,
    /// 是否为动爻
    pub change: bool,
}

/// 六次铜钱摇卦结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiuyaoCastResult {
    /// 从初爻(0)到上爻(5)
    pub yaos: [YaoCastItem; 6],
}

/// 解析后的爻信息
#[derive(Debug, Clone)]
struct ParsedYao {
    di_zhi: String,
    element: String,
    liu_qin: String,
    is_shi: bool,
    is_ying: bool,
}

/// 卦的完整信息
#[derive(Debug, Clone)]
struct GuaInfo {
    name: String,
    palace: String,
    yaos: [ParsedYao; 6],
}

// ============================================================
// 六爻计算器
// ============================================================

/// 六爻排盘计算器
pub struct LiuyaoCalc {
    sxwnl: Sxwnl,
}

impl LiuyaoCalc {
    pub fn new() -> Self {
        Self {
            sxwnl: Sxwnl::new(),
        }
    }

    // ============================================================
    // 铜钱起卦
    // ============================================================

    /// 模拟三枚铜钱摇六次，每次得出一爻
    pub fn cast_coins(&self) -> LiuyaoCastResult {
        let mut rng = rand::thread_rng();
        let mut yaos: [YaoCastItem; 6] = [
            YaoCastItem { value: 0, change: false },
            YaoCastItem { value: 0, change: false },
            YaoCastItem { value: 0, change: false },
            YaoCastItem { value: 0, change: false },
            YaoCastItem { value: 0, change: false },
            YaoCastItem { value: 0, change: false },
        ];

        for i in 0..6 {
            // 三枚铜钱，每枚 0=背面(阴) 1=正面(阳)
            let c1: u8 = rng.gen_range(0..2);
            let c2: u8 = rng.gen_range(0..2);
            let c3: u8 = rng.gen_range(0..2);
            let yang_count = c1 + c2 + c3;

            match yang_count {
                3 => {
                    // 三个正面 = 老阳 (动爻, 阳变阴)
                    yaos[i].value = 1;
                    yaos[i].change = true;
                }
                2 => {
                    // 两正一反 = 少阳 (静爻)
                    yaos[i].value = 1;
                    yaos[i].change = false;
                }
                1 => {
                    // 一正两反 = 少阴 (静爻)
                    yaos[i].value = 0;
                    yaos[i].change = false;
                }
                0 => {
                    // 三个反面 = 老阴 (动爻, 阴变阳)
                    yaos[i].value = 0;
                    yaos[i].change = true;
                }
                _ => unreachable!(),
            }
        }

        LiuyaoCastResult { yaos }
    }

    // ============================================================
    // 地支五行查询
    // ============================================================

    fn di_zhi_to_element(di_zhi: &str) -> &str {
        for &(dz, elem) in DI_ZHI_ELEMENT {
            if dz == di_zhi {
                return elem;
            }
        }
        "土"
    }

    // ============================================================
    // 宫位五行查询
    // ============================================================

    fn palace_to_element(palace: &str) -> &str {
        for &(p, e) in PALACE_ELEMENT {
            if p == palace {
                return e;
            }
        }
        "土"
    }

    // ============================================================
    // 元素索引 (用于查表)
    // ============================================================

    fn element_index(element: &str) -> usize {
        match element {
            "金" => 0,
            "木" => 1,
            "水" => 2,
            "火" => 3,
            "土" => 4,
            _ => 4,
        }
    }

    // ============================================================
    // 六亲
    // ============================================================

    /// 根据卦宫五行和爻地支，返回六亲名称
    pub fn get_liu_qin(&self, gua_element: &str, yao_di_zhi: &str) -> String {
        let palace_idx = Self::element_index(gua_element);
        let yao_elem = Self::di_zhi_to_element(yao_di_zhi);
        let yao_idx = Self::element_index(yao_elem);
        LIU_QIN_TABLE[palace_idx][yao_idx].to_string()
    }

    // ============================================================
    // 六兽
    // ============================================================

    /// 根据日干和爻位(0=初爻 ... 5=上爻)，返回六兽名称
    pub fn get_liu_shou(&self, day_gan: &str, position: usize) -> String {
        let seq = Self::liu_shou_sequence(day_gan);
        if position < 6 {
            seq[position].to_string()
        } else {
            "青龙".to_string()
        }
    }

    fn liu_shou_sequence(day_gan: &str) -> &[&str; 6] {
        for &(gan, seq) in LIU_SHOU_TABLE {
            if gan == day_gan {
                return seq;
            }
        }
        // 默认返回青龙序列
        &["青龙", "朱雀", "勾陈", "螣蛇", "白虎", "玄武"]
    }

    // ============================================================
    // 卦数据查找
    // ============================================================

    /// 根据6爻二进制串查找卦信息
    fn find_gua(binary_key: &str) -> Option<GuaInfo> {
        for &(key, name, palace, yaoname) in GUA_64_DATA {
            if key == binary_key {
                let mut yaos: [ParsedYao; 6] = [
                    ParsedYao { di_zhi: String::new(), element: String::new(), liu_qin: String::new(), is_shi: false, is_ying: false },
                    ParsedYao { di_zhi: String::new(), element: String::new(), liu_qin: String::new(), is_shi: false, is_ying: false },
                    ParsedYao { di_zhi: String::new(), element: String::new(), liu_qin: String::new(), is_shi: false, is_ying: false },
                    ParsedYao { di_zhi: String::new(), element: String::new(), liu_qin: String::new(), is_shi: false, is_ying: false },
                    ParsedYao { di_zhi: String::new(), element: String::new(), liu_qin: String::new(), is_shi: false, is_ying: false },
                    ParsedYao { di_zhi: String::new(), element: String::new(), liu_qin: String::new(), is_shi: false, is_ying: false },
                ];

                for (i, &yn_str) in yaoname.iter().enumerate() {
                    yaos[i] = Self::parse_yaoname(yn_str);
                }

                return Some(GuaInfo {
                    name: name.to_string(),
                    palace: palace.to_string(),
                    yaos,
                });
            }
        }
        None
    }

    /// 解析 yaoname 字符串，如 "子水子孙"、"辰土父母应"、"戌土父母世"
    fn parse_yaoname(yn: &str) -> ParsedYao {
        // 格式: [di_zhi][element][liu_qin][optional shi/ying]
        // di_zhi 是第一个汉字字符 (3字节 UTF-8)
        // element 是第二个汉字字符
        // 后面是六亲名称，可能带有 "世" 或 "应" 后缀

        let chars: Vec<char> = yn.chars().collect();
        if chars.len() < 3 {
            return ParsedYao {
                di_zhi: String::new(),
                element: String::new(),
                liu_qin: String::new(),
                is_shi: false,
                is_ying: false,
            };
        }

        let di_zhi = chars[0].to_string();
        let element = chars[1].to_string();
        let rest: String = chars[2..].iter().collect();

        let (liu_qin, is_shi, is_ying) = if rest.ends_with('世') {
            let lq = rest[..rest.len() - '世'.len_utf8()].to_string();
            (lq, true, false)
        } else if rest.ends_with('应') {
            let lq = rest[..rest.len() - '应'.len_utf8()].to_string();
            (lq, false, true)
        } else {
            (rest, false, false)
        };

        // 标准化六亲名称: "妻才" → "妻财"
        let liu_qin = match liu_qin.as_str() {
            "妻才" => "妻财".to_string(),
            _ => liu_qin,
        };

        ParsedYao {
            di_zhi,
            element,
            liu_qin,
            is_shi,
            is_ying,
        }
    }

    // ============================================================
    // 从卦数据中获取世应位置
    // ============================================================

    fn extract_shi_ying(gua: &GuaInfo) -> ShiYing {
        let mut shi_pos = 1;
        let mut ying_pos = 4;

        for (i, yao) in gua.yaos.iter().enumerate() {
            if yao.is_shi {
                shi_pos = i + 1;
            }
            if yao.is_ying {
                ying_pos = i + 1;
            }
        }

        // 如果数据中没有明确的世应标记，通过八卦宫规则推算
        if shi_pos == 1 && ying_pos == 4 && !gua.yaos[0].is_shi && !gua.yaos[0].is_ying {
            // 使用首卦对比法推算世应
            let shi = Self::calc_shi_position(gua);
            let ying = if shi <= 3 { shi + 3 } else { shi - 3 };
            return ShiYing { shi, ying };
        }

        ShiYing { shi: shi_pos, ying: ying_pos }
    }

    /// 通过八卦宫规则推算世爻位置
    /// 规则: 对比本卦与首卦(本宫卦)的爻差异
    fn calc_shi_position(gua: &GuaInfo) -> usize {
        // 找到该卦所在宫的首卦
        let palace = &gua.palace;

        // 首卦的二进制key: 每个宫的首卦上下卦相同
        let palace_gua_key = match palace.as_str() {
            "乾" => "111111",
            "坎" => "010010",
            "艮" => "001001",
            "震" => "100100",
            "巽" => "011011",
            "离" => "101101",
            "坤" => "000000",
            "兑" => "110110",
            _ => return 1,
        };

        // 获取当前卦的二进制key
        let current_key = Self::gua_to_binary_key(gua);

        let palace_chars: Vec<char> = palace_gua_key.chars().collect();
        let current_chars: Vec<char> = current_key.chars().collect();

        // 从初爻(索引0)开始，数有多少个连续的爻与首卦相同
        let mut same_count = 0;
        for i in 0..6 {
            if palace_chars[i] == current_chars[i] {
                same_count += 1;
            } else {
                break;
            }
        }

        match same_count {
            6 => 6, // 本宫卦: 世在6爻
            0 => {
                // 归魂卦或游魂卦
                // 游魂卦: 第4爻与首卦不同，世在4爻
                // 归魂卦: 第3爻与首卦不同，世在3爻
                // 判断方法: 看下卦是否完整变化
                let lower_same = (0..3).all(|i| palace_chars[i] == current_chars[i]);
                if lower_same {
                    // 下卦同首卦，为归魂卦，世在3爻
                    3
                } else {
                    // 游魂卦，世在4爻
                    4
                }
            }
            n => {
                // n世卦: 世在n爻位置
                // same_count 表示前n个爻与首卦相同，第n+1个开始不同
                // 实际世爻位置: 如果n=0, 一世卦(世在1爻); n=1, 二世卦(世在2爻); etc.
                // 但要注意: same_count=0时，一世卦; same_count=1时，二世卦
                if n == 0 {
                    1 // 一世卦
                } else if n >= 1 && n <= 5 {
                    // 检查是否前n个爻相同，如果是游魂/归魂则另算
                    // 对于正常的一世到五世卦，世在 n+1
                    // 但 n=5 时可能是五世卦(世在5爻)也可能是游魂卦
                    n + 1
                } else {
                    1
                }
            }
        }
    }

    /// 从卦信息生成二进制key
    fn gua_to_binary_key(gua: &GuaInfo) -> String {
        // 从卦名反查二进制key
        for &(key, name, _, _) in GUA_64_DATA {
            if name == gua.name {
                return key.to_string();
            }
        }
        // fallback: 从 yaoname 的地支不能直接反推，返回空
        String::from("000000")
    }

    // ============================================================
    // 伏神
    // ============================================================

    /// 获取伏神: 本宫首卦中与当前卦不同的爻
    fn get_fu_shen(gua: &GuaInfo) -> Vec<String> {
        let palace = &gua.palace;

        // 找到本宫首卦
        let palace_gua = Self::find_palace_head_gua(palace);
        if palace_gua.is_none() {
            return Vec::new();
        }
        let palace_gua = palace_gua.unwrap();

        let mut fu_shen: Vec<String> = Vec::new();

        for i in 0..6 {
            let current_di_zhi = &gua.yaos[i].di_zhi;
            let palace_di_zhi = &palace_gua.yaos[i].di_zhi;

            if current_di_zhi != palace_di_zhi {
                fu_shen.push(format!(
                    "{}伏{}{}",
                    palace_di_zhi,
                    current_di_zhi,
                    palace_gua.yaos[i].liu_qin
                ));
            }
        }

        fu_shen
    }

    /// 查找本宫首卦
    fn find_palace_head_gua(palace: &str) -> Option<GuaInfo> {
        let key = match palace {
            "乾" => "111111",
            "坎" => "010010",
            "艮" => "001001",
            "震" => "100100",
            "巽" => "011011",
            "离" => "101101",
            "坤" => "000000",
            "兑" => "110110",
            _ => return None,
        };
        Self::find_gua(key)
    }

    // ============================================================
    // 用神
    // ============================================================

    /// 根据占问事项判断用神
    fn determine_yong_shen(query: &str) -> String {
        let query_lower = query.to_lowercase();

        for &(yong_shen, keywords) in YONG_SHEN_KEYWORDS {
            for kw in keywords {
                if query_lower.contains(kw) || query.contains(kw) {
                    return yong_shen.to_string();
                }
            }
        }

        // 默认: 如果查询包含感情/婚姻，根据情况判断
        if query.contains('婚') || query.contains('恋') || query.contains('情') || query.contains('姻') {
            return "妻财".to_string();
        }

        // 默认用神: 妻财
        "妻财".to_string()
    }

    // ============================================================
    // 获取日干
    // ============================================================

    fn get_day_gan(&self, birth: &BirthInfo) -> String {
        let dt = birth.datetime;
        let year = dt.year();
        let month = dt.month();
        let day = dt.day();

        let ganzhi = crate::sxwnl::calendar::CalendarCalc::get_day_ganzhi(year, month, day);
        if ganzhi.len() >= 3 {
            // 日干是第一个汉字 (3字节 UTF-8)
            ganzhi[..3].to_string()
        } else {
            "甲".to_string()
        }
    }

    // ============================================================
    // 主排盘方法
    // ============================================================

    /// 完整六爻排盘
    pub fn divine(&self, birth: &BirthInfo, query: &str) -> LiuyaoGua {
        // 1. 铜钱起卦
        let cast = self.cast_coins();

        // 2. 获取日干
        let day_gan = self.get_day_gan(birth);

        // 3. 构建本卦 binary key
        let mut ben_gua_key = String::with_capacity(6);
        for i in 0..6 {
            ben_gua_key.push(if cast.yaos[i].value == 1 { '1' } else { '0' });
        }

        // 4. 查找本卦信息
        let ben_gua = Self::find_gua(&ben_gua_key);

        // 5. 确定变卦
        let mut bian_gua_key = ben_gua_key.clone();
        let mut dong_bian: Vec<(usize, String)> = Vec::new();

        for i in 0..6 {
            if cast.yaos[i].change {
                // 动爻: 阳变阴, 阴变阳
                let new_val = if cast.yaos[i].value == 1 { '0' } else { '1' };
                bian_gua_key.replace_range(i..i + 1, &new_val.to_string());
            }
        }

        // 记录变卦名称
        if bian_gua_key != ben_gua_key {
            if let Some(bian_gua) = Self::find_gua(&bian_gua_key) {
                dong_bian.push((0, bian_gua.name.clone()));
            }
        }

        // 6. 构建 Yao 数组
        let mut yaos: [Yao; 6] = [
            Yao { position: 1, yao_type: YaoType::Yin, di_zhi: String::new(), liu_qin: String::new(), liu_shou: String::new() },
            Yao { position: 2, yao_type: YaoType::Yin, di_zhi: String::new(), liu_qin: String::new(), liu_shou: String::new() },
            Yao { position: 3, yao_type: YaoType::Yin, di_zhi: String::new(), liu_qin: String::new(), liu_shou: String::new() },
            Yao { position: 4, yao_type: YaoType::Yin, di_zhi: String::new(), liu_qin: String::new(), liu_shou: String::new() },
            Yao { position: 5, yao_type: YaoType::Yin, di_zhi: String::new(), liu_qin: String::new(), liu_shou: String::new() },
            Yao { position: 6, yao_type: YaoType::Yin, di_zhi: String::new(), liu_qin: String::new(), liu_shou: String::new() },
        ];

        let mut liu_qin_arr: [String; 6] = Default::default();
        let mut liu_shou_arr: [String; 6] = Default::default();

        for i in 0..6 {
            let yao_type = match (cast.yaos[i].value, cast.yaos[i].change) {
                (1, false) => YaoType::Yang,
                (0, false) => YaoType::Yin,
                (1, true) => YaoType::LaoYang,
                (0, true) => YaoType::Laoyin,
                _ => YaoType::Yin,
            };

            yaos[i].yao_type = yao_type;
            yaos[i].position = (i + 1) as u8;

            // 六兽: 从初爻开始排列
            let liu_shou = self.get_liu_shou(&day_gan, i);
            yaos[i].liu_shou = liu_shou.clone();
            liu_shou_arr[i] = liu_shou;

            if let Some(ref gua) = ben_gua {
                let parsed = &gua.yaos[i];
                yaos[i].di_zhi = parsed.di_zhi.clone();

                let palace_elem = Self::palace_to_element(&gua.palace);
                let liu_qin = self.get_liu_qin(palace_elem, &parsed.di_zhi);
                yaos[i].liu_qin = liu_qin.clone();
                liu_qin_arr[i] = liu_qin;
            }
        }

        // 7. 世应
        let shi_ying = if let Some(ref gua) = ben_gua {
            Self::extract_shi_ying(gua)
        } else {
            ShiYing { shi: 1, ying: 4 }
        };

        // 8. 伏神
        let fu_shen = if let Some(ref gua) = ben_gua {
            Self::get_fu_shen(gua)
        } else {
            Vec::new()
        };

        // 9. 用神
        let yong_shen = Self::determine_yong_shen(query);

        // 10. 卦名和符号
        let (name, symbol) = if let Some(ref gua) = ben_gua {
            let sym = Self::yaos_to_symbol(&yaos);
            (gua.name.clone(), sym)
        } else {
            (String::from("未知"), String::new())
        };

        // 11. 动变详情（记录每个动爻位置）
        for i in 0..6 {
            if cast.yaos[i].change {
                let pos = i + 1;
                let desc = format!("爻{}动", pos);
                dong_bian.push((pos, desc));
            }
        }

        LiuyaoGua {
            name,
            symbol,
            yaos,
            shi_ying,
            liu_qin: liu_qin_arr,
            liu_shou: liu_shou_arr,
            fu_shen,
            dong_bian,
            yong_shen,
        }
    }

    /// 将6个Yao转为卦象符号
    fn yaos_to_symbol(yaos: &[Yao; 6]) -> String {
        let mut sym = String::with_capacity(12);
        for yao in yaos.iter() {
            match yao.yao_type {
                YaoType::Yang => sym.push_str("— "),
                YaoType::Yin => sym.push_str("- - "),
                YaoType::LaoYang => sym.push_str("—o "),
                YaoType::Laoyin => sym.push_str("- -x "),
            }
        }
        sym.trim().to_string()
    }
}

impl Default for LiuyaoCalc {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cast_coins() {
        let calc = LiuyaoCalc::new();
        let result = calc.cast_coins();
        assert_eq!(result.yaos.len(), 6);
        for yao in &result.yaos {
            assert!(yao.value == 0 || yao.value == 1);
        }
    }

    #[test]
    fn test_get_liu_qin() {
        let calc = LiuyaoCalc::new();
        // 乾宫(金): 子水 → 子孙
        assert_eq!(calc.get_liu_qin("金", "子"), "子孙");
        // 乾宫(金): 寅木 → 妻财
        assert_eq!(calc.get_liu_qin("金", "寅"), "妻财");
        // 乾宫(金): 戌土 → 父母
        assert_eq!(calc.get_liu_qin("金", "戌"), "父母");
        // 坎宫(水): 子水 → 兄弟
        assert_eq!(calc.get_liu_qin("水", "子"), "兄弟");
    }

    #[test]
    fn test_get_liu_shou() {
        let calc = LiuyaoCalc::new();
        // 甲乙日: 青龙在初爻
        assert_eq!(calc.get_liu_shou("甲", 0), "青龙");
        assert_eq!(calc.get_liu_shou("甲", 1), "朱雀");
        // 丙丁日: 朱雀在初爻
        assert_eq!(calc.get_liu_shou("丙", 0), "朱雀");
        // 庚辛日: 白虎在初爻
        assert_eq!(calc.get_liu_shou("庚", 0), "白虎");
    }

    #[test]
    fn test_find_gua() {
        let gua = LiuyaoCalc::find_gua("111111");
        assert!(gua.is_some());
        let gua = gua.unwrap();
        assert_eq!(gua.name, "乾为天");
        assert_eq!(gua.palace, "乾");
        assert_eq!(gua.yaos[0].di_zhi, "子");
        assert_eq!(gua.yaos[0].liu_qin, "子孙");
    }

    #[test]
    fn test_parse_yaoname() {
        let yao = LiuyaoCalc::parse_yaoname("子水子孙");
        assert_eq!(yao.di_zhi, "子");
        assert_eq!(yao.element, "水");
        assert_eq!(yao.liu_qin, "子孙");
        assert!(!yao.is_shi);
        assert!(!yao.is_ying);

        let yao = LiuyaoCalc::parse_yaoname("戌土父母世");
        assert_eq!(yao.di_zhi, "戌");
        assert_eq!(yao.liu_qin, "父母");
        assert!(yao.is_shi);

        let yao = LiuyaoCalc::parse_yaoname("辰土父母应");
        assert_eq!(yao.liu_qin, "父母");
        assert!(yao.is_ying);
    }

    #[test]
    fn test_determine_yong_shen() {
        assert_eq!(LiuyaoCalc::determine_yong_shen("求财"), "妻财");
        assert_eq!(LiuyaoCalc::determine_yong_shen("事业运"), "官鬼");
        assert_eq!(LiuyaoCalc::determine_yong_shen("健康如何"), "子孙");
        assert_eq!(LiuyaoCalc::determine_yong_shen("考试能过吗"), "父母");
        assert_eq!(LiuyaoCalc::determine_yong_shen("合作怎么样"), "兄弟");
    }
}