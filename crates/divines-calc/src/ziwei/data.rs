// Divines - 紫微斗数数据加载
// 使用 include_str! 在编译时嵌入所有 JSON 数据文件

use std::collections::HashMap;
use std::sync::LazyLock;

/// 紫微斗数数据容器
pub struct ZiWeiData {
    /// 斗君表（月份 × 时辰 → 地支）
    pub dou_jun: serde_json::Value,
    /// 主星步进表（north: 紫微系, south: 天府系）
    pub stars_main: serde_json::Value,
    /// 四化表（gan → [禄,权,科,忌]）
    pub sihua: serde_json::Value,
    /// 格局表
    pub ge: serde_json::Value,
    /// 小星表（houses, changsheng, bosi, taisui）
    pub small_stars: serde_json::Value,
    /// 年干星表（天魁,天钺,禄存,擎羊,陀罗等）
    pub year_gan: serde_json::Value,
    /// 年支星表（天马,天空,天哭,天虚,龙池,凤阁等）
    pub year_zhi: serde_json::Value,
    /// 月星表（左辅,右弼,天刑,天姚,解神,天巫,天月,阴煞）
    pub month: serde_json::Value,
    /// 时支星表（文昌,文曲,地劫,地空,台辅,封诰）
    pub time_zhi: serde_json::Value,
    /// 星曜亮度表
    pub starlight: serde_json::Value,
    /// 火铃星表（火星,铃星 按年支分组 × 时辰）
    pub huo_lin: serde_json::Value,
    /// 流昌流曲表
    pub liu_chang_qu: serde_json::Value,
    /// 将星表（将星,攀鞍,岁驿,息神,华盖,劫煞,灾煞,天煞,指背,咸池,月煞,亡神）
    pub jiang: serde_json::Value,
    /// 命主身主表（life × 年支, body × 年支）
    pub zu: serde_json::Value,
    /// 小限表（年支三合局 → 起始地支）
    pub xiao_xian: serde_json::Value,
}

/// 全局数据实例
pub static DATA: LazyLock<ZiWeiData> = LazyLock::new(|| {
    ZiWeiData {
        dou_jun: parse_json(include_str!("data/ziweidou.json")),
        stars_main: parse_json(include_str!("data/ziweistarsmain.json")),
        sihua: parse_json(include_str!("data/ziweisihua.json")),
        ge: parse_json(include_str!("data/ziweige.json")),
        small_stars: parse_json(include_str!("data/ziweismallstars.json")),
        year_gan: parse_json(include_str!("data/ziweiyeargan.json")),
        year_zhi: parse_json(include_str!("data/ziweiyearzi.json")),
        month: parse_json(include_str!("data/ziweimonth.json")),
        time_zhi: parse_json(include_str!("data/ziweitimezi.json")),
        starlight: parse_json(include_str!("data/ziweistarlight.json")),
        huo_lin: parse_json(include_str!("data/ziweihuolin.json")),
        liu_chang_qu: parse_json(include_str!("data/ziweiliuchangqu.json")),
        jiang: parse_json(include_str!("data/ziweijiang.json")),
        zu: parse_json(include_str!("data/ziweizu.json")),
        xiao_xian: parse_json(include_str!("data/ziweixiaoxian.json")),
    }
});

fn parse_json(s: &str) -> serde_json::Value {
    serde_json::from_str(s).expect("Failed to parse ziwei data JSON")
}

// ============================================================
// 常量数据（编译时计算）
// ============================================================

/// 地支列表
pub const BRANCHES: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

/// 天干列表
pub const STEMS: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

/// 地支索引
pub fn branch_index(zhi: &str) -> Option<usize> {
    BRANCHES.iter().position(|&z| z == zhi)
}

/// 天干索引
pub fn stem_index(gan: &str) -> Option<usize> {
    STEMS.iter().position(|&g| g == gan)
}

/// 安全获取干支字符串的第一个字符（天干）
pub fn ganzi_first(s: &str) -> &str {
    s.get(..s.chars().next().map(|c| c.len_utf8()).unwrap_or(0)).unwrap_or("")
}

/// 安全获取干支字符串的第二个字符（地支）
pub fn ganzi_second(s: &str) -> &str {
    let first_len = s.chars().next().map(|c| c.len_utf8()).unwrap_or(0);
    let second_end = s.chars().nth(1).map(|c| first_len + c.len_utf8()).unwrap_or(first_len);
    s.get(first_len..second_end).unwrap_or("")
}

/// 天干阴阳: 甲丙戊庚壬为阳，乙丁己辛癸为阴
pub fn stem_polar(gan: &str) -> &'static str {
    match gan {
        "甲" | "丙" | "戊" | "庚" | "壬" => "阳",
        _ => "阴",
    }
}

/// 地支阴阳: 子寅辰午申戌为阳，丑卯巳未酉亥为阴
pub fn branch_polar(zhi: &str) -> &'static str {
    match zhi {
        "子" | "寅" | "辰" | "午" | "申" | "戌" => "阳",
        _ => "阴",
    }
}

/// 农历月份名
pub const LUNAR_MONTHS: [&str; 12] = [
    "正月", "二月", "三月", "四月", "五月", "六月",
    "七月", "八月", "九月", "十月", "冬月", "腊月",
];

/// 宫位名称（从命宫起逆时针）
pub const HOUSE_NAMES: [&str; 12] = [
    "命宫", "兄弟宫", "夫妻宫", "子女宫", "财帛宫", "疾厄宫",
    "迁移宫", "交友宫", "官禄宫", "田宅宫", "福德宫", "父母宫",
];

/// 长生十二神
pub const CHANG_SHENG_12: [&str; 12] = [
    "长生", "沐浴", "冠带", "临官", "帝旺", "衰", "病", "死", "墓", "绝", "胎", "养",
];

/// 博士十二神
pub const BOSHI_12: [&str; 12] = [
    "博士", "力士", "青龙", "小耗", "将军", "奏书", "飞廉", "喜神", "病符", "大耗", "伏兵", "官符",
];

/// 岁前十二神
pub const TAISUI_12: [&str; 12] = [
    "岁建", "晦气", "丧门", "贯索", "官符", "小耗", "岁破", "龙德", "白虎", "天德", "吊客", "病符",
];

/// 五行局数映射
pub fn wuxing_ju_num(element: &str) -> i32 {
    match element {
        "水" => 2,
        "木" => 3,
        "金" => 4,
        "土" => 5,
        "火" => 6,
        _ => 2,
    }
}

/// 五行局数文本
pub fn wuxing_ju_text(num: i32) -> String {
    match num {
        2 => "水二局".to_string(),
        3 => "木三局".to_string(),
        4 => "金四局".to_string(),
        5 => "土五局".to_string(),
        6 => "火六局".to_string(),
        _ => format!("{}局", num),
    }
}

/// 宫位干支起点（五虎遁）：年干 → 寅宫天干
pub fn house_gan_start(year_gan: &str) -> &'static str {
    match year_gan {
        "甲" | "己" => "丙",
        "乙" | "庚" => "戊",
        "丙" | "辛" => "庚",
        "丁" | "壬" => "壬",
        "戊" | "癸" => "甲",
        _ => "甲",
    }
}

/// 命宫月份起始地支（寅宫起正月）
pub fn life_house_start_month_zhi(month: i32) -> &'static str {
    BRANCHES[((2 + month - 1) % 12) as usize] // 寅起正月 = index 2
}

/// 纳音五行表（60甲子，每两个干支一组）
pub const NAYIN_WUXING: [&str; 30] = [
    "金", "火", "木", "土", "金",  // 甲子乙丑-海里金, 丙寅丁卯-炉中火, 戊辰己巳-大林木, 庚午辛未-路旁土, 壬申癸酉-剑锋金
    "火", "水", "土", "金", "木",  // 甲戌乙亥-山头火, 丙子丁丑-涧下水, 戊寅己卯-城头土, 庚辰辛巳-白蜡金, 壬午癸未-杨柳木
    "水", "土", "火", "木", "水",  // 甲申乙酉-泉中水, 丙戌丁亥-屋上土, 戊子己丑-霹雳火, 庚寅辛卯-松柏木, 壬辰癸巳-长流水
    "金", "火", "木", "土", "金",  // 甲午乙未-沙中金, 丙申丁酉-山下火, 戊戌己亥-平地木, 庚子辛丑-壁上土, 壬寅癸卯-金箔金
    "火", "水", "土", "金", "木",  // 甲辰乙巳-覆灯火, 丙午丁未-天河水, 戊申己酉-大驿土, 庚戌辛亥-钗环金, 壬子癸丑-桑柘木
    "水", "土", "火", "木", "水",  // 甲寅乙卯-大溪水, 丙辰丁巳-沙中土, 戊午己未-天上火, 庚申辛酉-石榴木, 壬戌癸亥-大海水
];

/// 通过干支获取纳音五行
pub fn get_nayin_wuxing(gan: &str, zhi: &str) -> &'static str {
    let g_idx = stem_index(gan).unwrap_or(0) as i32;
    let z_idx = branch_index(zhi).unwrap_or(0) as i32;
    // 六十甲子序数: (天干索引*6 + 地支索引*5) % 60 的简化公式
    // 更准确：直接使用 60甲子配对
    let idx = ((g_idx % 10) * 6 + (z_idx % 12) * 5) % 60;
    let pair_idx = (idx / 2) as usize;
    NAYIN_WUXING[pair_idx % 30]
}

/// 年支三合局分组
pub fn year_zhi_group(zhi: &str) -> &'static str {
    match zhi {
        "寅" | "午" | "戌" => "寅午戌",
        "申" | "子" | "辰" => "申子辰",
        "巳" | "酉" | "丑" => "巳酉丑",
        "亥" | "卯" | "未" => "亥卯未",
        _ => "寅午戌",
    }
}

/// 四化星
pub const SIHUA_NAMES: [&str; 4] = ["禄", "权", "科", "忌"];

/// 获取某天干对应的四化星数组 [禄, 权, 科, 忌]
pub fn gan_sihua_stars(gan: &str) -> [&'static str; 4] {
    match gan {
        "甲" => ["廉贞", "破军", "武曲", "太阳"],
        "乙" => ["天机", "天梁", "紫微", "太阴"],
        "丙" => ["天同", "天机", "文昌", "廉贞"],
        "丁" => ["太阴", "天同", "天机", "巨门"],
        "戊" => ["贪狼", "太阴", "右弼", "天机"],
        "己" => ["武曲", "贪狼", "天梁", "文曲"],
        "庚" => ["太阳", "武曲", "太阴", "天同"],
        "辛" => ["巨门", "太阳", "文曲", "文昌"],
        "壬" => ["天梁", "紫微", "左辅", "武曲"],
        "癸" => ["破军", "巨门", "太阴", "贪狼"],
        _ => ["", "", "", ""],
    }
}

/// 禄存表（年干 → 地支索引）
pub fn lucun_branch(gan: &str) -> usize {
    match gan {
        "甲" => 2, // 寅
        "乙" => 3, // 卯
        "丙" => 5, // 巳
        "丁" => 6, // 午
        "戊" => 5, // 巳
        "己" => 6, // 午
        "庚" => 8, // 申
        "辛" => 9, // 酉
        "壬" => 11, // 亥
        "癸" => 0, // 子
        _ => 2,
    }
}

/// 擎羊（禄存前一位）
pub fn qingyang_branch(gan: &str) -> usize {
    (lucun_branch(gan) + 1) % 12
}

/// 陀罗（禄存后一位）
pub fn tuoluo_branch(gan: &str) -> usize {
    (lucun_branch(gan) + 11) % 12
}

/// 天魁天钺通过天干（返回 (天魁位置, 天钺位置)）
pub fn tiankui_tianyue_branch(gan: &str) -> (usize, usize) {
    match gan {
        "甲" => (1, 7),   // 丑, 未
        "乙" => (0, 8),   // 子, 申
        "丙" => (11, 9),  // 亥, 酉
        "丁" => (11, 9),  // 亥, 酉
        "戊" => (1, 7),   // 丑, 未
        "己" => (0, 8),   // 子, 申
        "庚" => (1, 7),   // 丑, 未
        "辛" => (6, 2),   // 午, 寅
        "壬" => (3, 5),   // 卯, 巳
        "癸" => (3, 5),   // 卯, 巳
        _ => (0, 0),
    }
}

/// 天马通过年支（返回地支索引）
pub fn tianma_branch(zhi: &str) -> usize {
    match zhi {
        "子" => 2,  // 寅
        "丑" => 11, // 亥
        "寅" => 8,  // 申
        "卯" => 5,  // 巳
        "辰" => 2,  // 寅
        "巳" => 11, // 亥
        "午" => 8,  // 申
        "未" => 5,  // 巳
        "申" => 2,  // 寅
        "酉" => 11, // 亥
        "戌" => 8,  // 申
        "亥" => 5,  // 巳
        _ => 2,
    }
}

/// 命主（年支 → 星名）
pub fn life_master(zhi: &str) -> &'static str {
    match zhi {
        "子" => "贪狼", "丑" => "巨门", "寅" => "禄存", "卯" => "文曲",
        "辰" => "廉贞", "巳" => "武曲", "午" => "破军", "未" => "武曲",
        "申" => "廉贞", "酉" => "文曲", "戌" => "禄存", "亥" => "巨门",
        _ => "",
    }
}

/// 身主（年支 → 星名）
pub fn body_master(zhi: &str) -> &'static str {
    match zhi {
        "子" => "火星", "丑" => "天相", "寅" => "天梁", "卯" => "天同",
        "辰" => "文昌", "巳" => "天机", "午" => "火星", "未" => "天相",
        "申" => "天梁", "酉" => "天同", "戌" => "文昌", "亥" => "天机",
        _ => "",
    }
}

/// 生年干支的旬空
/// 旬空 = 地支天干配对中每旬空缺的两个地支
pub fn xun_empty_set(year_gz: &str) -> [&'static str; 2] {
    let gan = ganzi_first(year_gz);
    let gan_idx = stem_index(gan).unwrap_or(0) as i32;
    // 旬：天干从甲开始每10个为一旬，对应地支从某位开始
    // 每旬10个干支，空2个地支
    let start_zhi = ((10 - gan_idx) % 10) as usize;
    let empty1 = (start_zhi + 10) % 12;
    let empty2 = (start_zhi + 11) % 12;
    [BRANCHES[empty1], BRANCHES[empty2]]
}

/// 火星/铃星起始：通过年支确定起始地支索引
/// 返回 (火星起始, 铃星起始)
pub fn huoxing_lingxing_base(zhi: &str) -> (usize, usize) {
    match zhi {
        "寅" | "午" | "戌" => (1, 3),   // 火星起丑(1), 铃星起卯(3)
        "申" | "子" | "辰" => (2, 10),  // 火星起寅(2), 铃星起戌(10)
        "巳" | "酉" | "丑" => (3, 10),  // 火星起卯(3), 铃星起戌(10)
        "亥" | "卯" | "未" => (9, 10),  // 火星起酉(9), 铃星起戌(10)
        _ => (1, 3),
    }
}

/// 小限起始：通过年支三合局确定起始地支
pub fn xiao_xian_start(zhi: &str) -> &'static str {
    match zhi {
        "寅" | "午" | "戌" => "辰",
        "申" | "子" | "辰" => "戌",
        "巳" | "酉" | "丑" => "未",
        "亥" | "卯" | "未" => "丑",
        _ => "辰",
    }
}