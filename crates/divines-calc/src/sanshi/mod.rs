// Divines - 三式计算引擎（奇门、太乙、六壬）
// 参考原项目: vendor/kinqimen/, vendor/kintaiyi/, vendor/kinastro/astro/sanshi/

use chrono::{Datelike, Timelike};
use divines_core::*;

// ============ 奇门遁甲 ============

/// 奇门遁甲常量数据
mod constants {
    use divines_core::sanshi::{DunType, GongInfo};
    /// 十天干
    pub const TIAN_GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
    /// 十二地支
    pub const DI_ZHI: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
    /// 六十甲子
    pub const JIA_ZI: [&str; 60] = [
        "甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未", "壬申", "癸酉",
        "甲戌", "乙亥", "丙子", "丁丑", "戊寅", "己卯", "庚辰", "辛巳", "壬午", "癸未",
        "甲申", "乙酉", "丙戌", "丁亥", "戊子", "己丑", "庚寅", "辛卯", "壬辰", "癸巳",
        "甲午", "乙未", "丙申", "丁酉", "戊戌", "己亥", "庚子", "辛丑", "壬寅", "癸卯",
        "甲辰", "乙巳", "丙午", "丁未", "戊申", "己酉", "庚戌", "辛亥", "壬子", "癸丑",
        "甲寅", "乙卯", "丙辰", "丁巳", "戊午", "己未", "庚申", "辛酉", "壬戌", "癸亥",
    ];
    /// 六甲旬首
    pub const LIU_JIA: [&str; 6] = ["甲子", "甲戌", "甲申", "甲午", "甲辰", "甲寅"];
    /// 六甲旬首对应天干
    pub const LIU_JIA_GAN: [&str; 6] = ["戊", "己", "庚", "辛", "壬", "癸"];
    /// 八卦宫位（顺时针：坎艮震巽离坤兑乾）
    pub const EIGHT_GUA_CLOCKWISE: [&str; 8] = ["坎", "艮", "震", "巽", "離", "坤", "兌", "乾"];
    /// 八卦宫位（后天八卦序：坎坤震巽中乾兑艮离）
    pub const EIGHT_GUA: [&str; 9] = ["坎", "坤", "震", "巽", "中", "乾", "兌", "艮", "離"];
    /// 中文数字（一至九）
    pub const CNUMBER: [&str; 9] = ["一", "二", "三", "四", "五", "六", "七", "八", "九"];
    /// 八门名称
    pub const DOOR_R: [&str; 8] = ["休", "生", "傷", "杜", "景", "死", "驚", "開"];
    /// 九星名称
    pub const STAR_R: [&str; 9] = ["蓬", "任", "沖", "輔", "英", "禽", "柱", "心", "芮"];
    /// 八神名称（阳遁序）
    pub const GOD_YANG: [&str; 8] = ["符", "蛇", "陰", "合", "勾", "雀", "地", "天"];
    /// 八神全称（阳遁序）
    pub const GOD_YANG_FULL: [&str; 8] = ["值符", "螣蛇", "太陰", "六合", "勾陳", "朱雀", "九地", "九天"];
    /// 八神全称（阴遁序）
    pub const GOD_YIN_FULL: [&str; 8] = ["值符", "螣蛇", "太陰", "六合", "白虎", "玄武", "九地", "九天"];
    /// 24节气
    pub const JIEQI_NAME: [&str; 24] = [
        "春分", "清明", "穀雨", "立夏", "小滿", "芒種",
        "夏至", "小暑", "大暑", "立秋", "處暑", "白露",
        "秋分", "寒露", "霜降", "立冬", "小雪", "大雪",
        "冬至", "小寒", "大寒", "立春", "雨水", "驚蟄",
    ];

    /// 节气奇门局数：上中下三元
    /// 格式: 节气名 -> (上元局, 中元局, 下元局)
    pub fn jieqi_jushu(ji: &str) -> (u8, u8, u8) {
        match ji {
            "冬至" | "驚蟄" => (1, 7, 4),
            "小寒" => (2, 8, 5),
            "大寒" | "春分" => (3, 9, 6),
            "立春" => (8, 5, 2),
            "雨水" => (9, 6, 3),
            "清明" | "立夏" => (4, 1, 7),
            "穀雨" | "小滿" => (5, 2, 8),
            "芒種" => (6, 3, 9),
            "夏至" | "白露" => (9, 3, 6),
            "小暑" => (8, 2, 5),
            "大暑" | "秋分" => (7, 1, 4),
            "立秋" => (2, 5, 8),
            "處暑" => (1, 4, 7),
            "霜降" | "小雪" => (5, 8, 2),
            "寒露" | "立冬" => (6, 9, 3),
            "大雪" => (4, 7, 1),
            _ => (1, 7, 4), // 默认冬至
        }
    }

    /// 节气对应阳遁/阴遁
    /// 冬至起阳遁，夏至起阴遁
    pub fn jieqi_dun_type(ji: &str) -> Option<DunType> {
        let yang_jieqi = ["冬至", "小寒", "大寒", "立春", "雨水", "驚蟄",
                          "春分", "清明", "穀雨", "立夏", "小滿", "芒種"];
        let yin_jieqi = ["夏至", "小暑", "大暑", "立秋", "處暑", "白露",
                          "秋分", "寒露", "霜降", "立冬", "小雪", "大雪"];
        if yang_jieqi.contains(&ji) {
            Some(DunType::YangDun)
        } else if yin_jieqi.contains(&ji) {
            Some(DunType::YinDun)
        } else {
            None
        }
    }

    /// 九宫信息
    pub fn gong_info(gong: u8) -> GongInfo {
        match gong {
            1 => GongInfo {
                name: "坎".to_string(), wuxing: "水".to_string(),
                fang_wei: "北".to_string(), color: "白".to_string(),
                number: 1, men_original: "休".to_string(),
            },
            2 => GongInfo {
                name: "坤".to_string(), wuxing: "土".to_string(),
                fang_wei: "西南".to_string(), color: "黑".to_string(),
                number: 2, men_original: "死".to_string(),
            },
            3 => GongInfo {
                name: "震".to_string(), wuxing: "木".to_string(),
                fang_wei: "東".to_string(), color: "碧".to_string(),
                number: 3, men_original: "傷".to_string(),
            },
            4 => GongInfo {
                name: "巽".to_string(), wuxing: "木".to_string(),
                fang_wei: "東南".to_string(), color: "綠".to_string(),
                number: 4, men_original: "杜".to_string(),
            },
            5 => GongInfo {
                name: "中".to_string(), wuxing: "土".to_string(),
                fang_wei: "中".to_string(), color: "黃".to_string(),
                number: 5, men_original: "".to_string(),
            },
            6 => GongInfo {
                name: "乾".to_string(), wuxing: "金".to_string(),
                fang_wei: "西北".to_string(), color: "白".to_string(),
                number: 6, men_original: "開".to_string(),
            },
            7 => GongInfo {
                name: "兌".to_string(), wuxing: "金".to_string(),
                fang_wei: "西".to_string(), color: "赤".to_string(),
                number: 7, men_original: "驚".to_string(),
            },
            8 => GongInfo {
                name: "艮".to_string(), wuxing: "土".to_string(),
                fang_wei: "東北".to_string(), color: "白".to_string(),
                number: 8, men_original: "生".to_string(),
            },
            9 => GongInfo {
                name: "離".to_string(), wuxing: "火".to_string(),
                fang_wei: "南".to_string(), color: "紫".to_string(),
                number: 9, men_original: "景".to_string(),
            },
            _ => GongInfo {
                name: "坎".to_string(), wuxing: "水".to_string(),
                fang_wei: "北".to_string(), color: "白".to_string(),
                number: 1, men_original: "休".to_string(),
            },
        }
    }
}

/// 工具函数：旋转列表，从指定元素开始
fn new_list<T: PartialEq + Clone>(list: &[T], start: &T) -> Vec<T> {
    if let Some(pos) = list.iter().position(|x| x == start) {
        let mut result = Vec::with_capacity(list.len());
        result.extend_from_slice(&list[pos..]);
        result.extend_from_slice(&list[..pos]);
        result
    } else {
        list.to_vec()
    }
}

/// 工具函数：逆序旋转列表
fn new_list_r<T: PartialEq + Clone>(list: &[T], start: &T) -> Vec<T> {
    let mut fwd = new_list(list, start);
    fwd.reverse();
    fwd
}

/// 获取六十甲子中某干支的索引
fn jiazi_index(ganzhi: &str) -> Option<usize> {
    constants::JIA_ZI.iter().position(|&x| x == ganzhi)
}

/// 根据日干支计算三元（上中下）
fn find_yuan(day_ganzhi: &str) -> SanYuan {
    let shang_yuan_fu = ["甲子", "甲午", "己卯", "己酉"];
    let zhong_yuan_fu = ["甲寅", "甲申", "己巳", "己亥"];
    let xia_yuan_fu = ["甲辰", "甲戌", "己丑", "己未"];

    // 找到该日干支所属的符头
    if let Some(idx) = jiazi_index(day_ganzhi) {
        let fu_idx = (idx / 5) * 5;
        let fu_head = constants::JIA_ZI[fu_idx];
        if shang_yuan_fu.contains(&fu_head) {
            SanYuan::ShangYuan
        } else if zhong_yuan_fu.contains(&fu_head) {
            SanYuan::ZhongYuan
        } else {
            SanYuan::XiaYuan
        }
    } else {
        SanYuan::ShangYuan
    }
}

/// 根据时辰干支获取旬首
fn get_xun_shou(ganzhi: &str) -> String {
    if let Some(idx) = jiazi_index(ganzhi) {
        let xun_idx = (idx / 10) * 10;
        constants::JIA_ZI[xun_idx].to_string()
    } else {
        "甲子".to_string()
    }
}

/// 根据旬首获取值符天干
/// 甲子->戊, 甲戌->己, 甲申->庚, 甲午->辛, 甲辰->壬, 甲寅->癸
fn xun_shou_to_gan(xun_shou: &str) -> String {
    if let Some(pos) = constants::LIU_JIA.iter().position(|&x| x == xun_shou) {
        constants::LIU_JIA_GAN[pos].to_string()
    } else {
        "戊".to_string()
    }
}

/// 计算节气（简化版：基于日期近似）
/// 返回当前日期所在的节气名称
fn get_jieqi(year: i32, month: u32, day: u32) -> String {
    // 24节气近似日期（每月两个）
    let jieqi_approx: [(u32, u32, &str); 24] = [
        (1, 5, "小寒"), (1, 20, "大寒"),
        (2, 4, "立春"), (2, 19, "雨水"),
        (3, 6, "驚蟄"), (3, 21, "春分"),
        (4, 5, "清明"), (4, 20, "穀雨"),
        (5, 6, "立夏"), (5, 21, "小滿"),
        (6, 6, "芒種"), (6, 21, "夏至"),
        (7, 7, "小暑"), (7, 23, "大暑"),
        (8, 7, "立秋"), (8, 23, "處暑"),
        (9, 8, "白露"), (9, 23, "秋分"),
        (10, 8, "寒露"), (10, 23, "霜降"),
        (11, 7, "立冬"), (11, 22, "小雪"),
        (12, 7, "大雪"), (12, 22, "冬至"),
    ];

    // 找到当前日期所在的节气区间
    for i in 0..24 {
        let (m, d, _name) = jieqi_approx[i];
        let (m_next, d_next, _name_next) = jieqi_approx[(i + 1) % 24];

        let current_md = month * 100 + day;
        let this_md = m * 100 + d;
        let next_md = if m_next < m { m_next * 100 + d_next + 1200 } else { m_next * 100 + d_next };

        let adj_current = if current_md < this_md && m > month { current_md + 1200 } else { current_md };

        if adj_current >= this_md && adj_current < next_md {
            return jieqi_approx[i].2.to_string();
        }
    }

    // 兜底：返回当前月对应的节气
    match month {
        1 => if day < 20 { "小寒".to_string() } else { "大寒".to_string() },
        2 => if day < 19 { "立春".to_string() } else { "雨水".to_string() },
        3 => if day < 21 { "驚蟄".to_string() } else { "春分".to_string() },
        4 => if day < 20 { "清明".to_string() } else { "穀雨".to_string() },
        5 => if day < 21 { "立夏".to_string() } else { "小滿".to_string() },
        6 => if day < 21 { "芒種".to_string() } else { "夏至".to_string() },
        7 => if day < 23 { "小暑".to_string() } else { "大暑".to_string() },
        8 => if day < 23 { "立秋".to_string() } else { "處暑".to_string() },
        9 => if day < 23 { "白露".to_string() } else { "秋分".to_string() },
        10 => if day < 23 { "寒露".to_string() } else { "霜降".to_string() },
        11 => if day < 22 { "立冬".to_string() } else { "小雪".to_string() },
        12 => if day < 22 { "大雪".to_string() } else { "冬至".to_string() },
        _ => "春分".to_string(),
    }
}

/// 计算日干支（简化版）
fn calc_day_ganzhi(year: i32, month: u32, day: u32) -> String {
    // 使用近似公式计算日干支
    // 已知 1900-01-01 = 甲戌 (索引 10)
    let mut days = 0i64;
    for y in 1900..year {
        if is_leap_year(y) { days += 366; } else { days += 365; }
    }
    let month_days = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for m in 1..month as usize {
        days += month_days[m] as i64;
        if m == 2 && is_leap_year(year) { days += 1; }
    }
    days += (day - 1) as i64;

    let base_idx = 10; // 1900-01-01 = 甲戌
    let idx = ((base_idx + days) % 60 + 60) % 60;
    constants::JIA_ZI[idx as usize].to_string()
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// 计算时干支
/// 根据日干和时辰地支计算时柱
fn calc_hour_ganzhi(day_ganzhi: &str, hour: u32) -> String {
    let di_zhi = constants::DI_ZHI;
    let tian_gan = constants::TIAN_GAN;

    // 时辰地支
    let dz_idx = if hour == 23 || hour == 0 { 0 } // 子时
        else if hour == 1 || hour == 2 { 1 } // 丑时
        else { ((hour as usize + 1) / 2) % 12 };

    // 五鼠遁：甲己日起甲子，乙庚日起丙子，丙辛日起戊子，丁壬日起庚子，戊癸日起壬子
    let day_gan = day_ganzhi.chars().next().unwrap_or('甲');
    let rat_start = match day_gan {
        '甲' | '己' => 0, // 甲子
        '乙' | '庚' => 2, // 丙子
        '丙' | '辛' => 4, // 戊子
        '丁' | '壬' => 6, // 庚子
        '戊' | '癸' => 8, // 壬子
        _ => 0,
    };

    let tg_idx = (rat_start + dz_idx) % 10;
    format!("{}{}", tian_gan[tg_idx], di_zhi[dz_idx])
}

/// 时柱地支
fn hour_to_dizhi(hour: u32) -> DiZhi {
    match hour {
        23 | 0 => DiZhi::Zi,
        1 | 2 => DiZhi::Chou,
        3 | 4 => DiZhi::Yin,
        5 | 6 => DiZhi::Mao,
        7 | 8 => DiZhi::Chen,
        9 | 10 => DiZhi::Si,
        11 | 12 => DiZhi::Wu,
        13 | 14 => DiZhi::Wei,
        15 | 16 => DiZhi::Shen,
        17 | 18 => DiZhi::You,
        19 | 20 => DiZhi::Xu,
        21 | 22 => DiZhi::Hai,
        _ => DiZhi::Zi,
    }
}

/// 值符排列（zhifu_pai）
/// 根据局数和阴阳遁，返回每个旬首对应的值符星宫信息
fn zhifu_pai(ju: u8, dun_type: DunType) -> Vec<(String, String, String)> {
    // 返回: (旬首, 星名, 宫位八卦)
    let cnumber = constants::CNUMBER;
    let eight_gua = constants::EIGHT_GUA;
    let star_names = ["蓬", "芮", "沖", "輔", "禽", "心", "柱", "任", "英"];

    let ju_str = cnumber[(ju - 1) as usize];

    // 阳遁排列
    let yang_pai: [&str; 9] = [
        "九八七一二三四五六", "一九八二三四五六七", "二一九三四五六七八",
        "三二一四五六七八九", "四三二五六七八九一", "五四三六七八九一二",
        "六五四七八九一二三", "七六五八九一二三四", "八七六九一二三四五",
    ];
    // 阴遁排列
    let yin_pai: [&str; 9] = [
        "一二三九八七六五四", "九一二八七六五四三", "八九一七六五四三二",
        "七八九六五四三二一", "六七八五四三二一九", "五六七四三二一九八",
        "四五六三二一九八七", "三四五二一九八七六", "二三四一九八七六五",
    ];

    let pai_str = match dun_type {
        DunType::YangDun => yang_pai[(ju - 1) as usize],
        DunType::YinDun => yin_pai[(ju - 1) as usize],
    };

    let liu_jia = constants::LIU_JIA;
    let mut result = Vec::new();

    let cnum_list: Vec<&str> = match dun_type {
        DunType::YangDun => new_list(&cnumber.to_vec(), &ju_str),
        DunType::YinDun => new_list_r(&cnumber.to_vec(), &ju_str),
    };

    let cnum_slice: Vec<&str> = cnum_list.iter().take(6).copied().collect();

    for i in 0..6 {
        let c = cnum_slice[i];
        let p = pai_str.chars().nth(cnumber.iter().position(|&x| x == c).unwrap()).unwrap_or('一');
        let p_str = cnumber.iter().position(|&x| x == p.to_string().as_str()).unwrap_or(0);
        let gong = eight_gua[p_str];
        let star = star_names[p_str];
        result.push((liu_jia[i].to_string(), star.to_string(), gong.to_string()));
    }

    result
}

/// 值使排列（zhishi_pai）
/// 返回每个旬首对应的值使门宫
fn zhishi_pai(ju: u8, dun_type: DunType) -> Vec<(String, String, String)> {
    let cnumber = constants::CNUMBER;
    let eight_gua = constants::EIGHT_GUA;
    let door_names = ["休", "死", "傷", "杜", "中", "開", "驚", "生", "景"];

    let ju_str = cnumber[(ju - 1) as usize];

    let cnum_list: Vec<&str> = match dun_type {
        DunType::YangDun => new_list(&cnumber.to_vec(), &ju_str),
        DunType::YinDun => new_list_r(&cnumber.to_vec(), &ju_str),
    };

    let cnum_slice: Vec<&str> = cnum_list.iter().take(6).copied().collect();
    let mut result = Vec::new();

    let mut extended: Vec<&str> = Vec::new();
    for _ in 0..3 {
        extended.extend(cnum_list.iter().copied());
    }

    for i in 0..6 {
        let c = cnum_slice[i];
        let pos = extended.iter().position(|&x| x == c).unwrap_or(0);
        let slice: Vec<&str> = extended[pos + 1..].iter().take(11).copied().collect();

        let mut door_gongs = String::new();
        door_gongs.push_str(c);
        for s in &slice {
            door_gongs.push_str(s);
        }

        // 第一个字符是门，第二个字符是宫
        let door_idx = cnumber.iter().position(|&x| x == c).unwrap_or(0);
        let door = door_names[door_idx];
        let gong = eight_gua[cnumber.iter().position(|&x| x == slice[0]).unwrap_or(0)];

        result.push((constants::LIU_JIA[i].to_string(), door.to_string(), gong.to_string()));
    }

    result
}

/// 确定值符值使
/// 返回 (值符星, 值符宫, 值符天干, 值使门, 值使宫)
fn determine_zhi_fu_zhi_shi(
    ju: u8,
    dun_type: DunType,
    shi_ganzhi: &str,
) -> (String, String, String, String, String) {
    let xun_shou = get_xun_shou(shi_ganzhi);
    let zftg = xun_shou_to_gan(&xun_shou);

    let zf_pai = zhifu_pai(ju, dun_type);
    let zs_pai = zhishi_pai(ju, dun_type);

    let mut zhi_fu_star = "蓬".to_string();
    let mut zhi_fu_gong = "坎".to_string();
    let mut zhi_shi_door = "休".to_string();
    let mut zhi_shi_gong = "坎".to_string();

    for (xun, star, gong) in &zf_pai {
        if xun == &xun_shou {
            zhi_fu_star = star.clone();
            zhi_fu_gong = gong.clone();
            break;
        }
    }

    for (xun, door, gong) in &zs_pai {
        if xun == &xun_shou {
            zhi_shi_door = door.clone();
            if door == "中" {
                zhi_shi_door = "死".to_string();
            }
            zhi_shi_gong = gong.clone();
            break;
        }
    }

    (zhi_fu_star, zhi_fu_gong, zftg, zhi_shi_door, zhi_shi_gong)
}

/// 排地盘
/// 阳遁顺排：戊己庚辛壬癸丁丙乙
/// 阴遁逆排：戊乙丙丁癸壬辛庚己
fn arrange_di_pan(dun_type: DunType, ju: u8) -> [String; 9] {
    let yang_earth = ["戊", "己", "庚", "辛", "壬", "癸", "丁", "丙", "乙"];
    let yin_earth = ["戊", "乙", "丙", "丁", "癸", "壬", "辛", "庚", "己"];

    let stems = match dun_type {
        DunType::YangDun => yang_earth,
        DunType::YinDun => yin_earth,
    };

    // 八卦宫位顺序：坎坤震巽中乾兑艮离
    let cnumber = constants::CNUMBER;
    let eight_gua = constants::EIGHT_GUA;

    // 从局数开始排列
    let ju_str = cnumber[(ju - 1) as usize];
    let reordered = new_list(&cnumber.to_vec(), &ju_str);

    let mut result: [String; 9] = Default::default();
    for i in 0..9 {
        let c = reordered[i];
        let gua = eight_gua[cnumber.iter().position(|&x| x == c).unwrap()];
        let stem = stems[i];
        let idx = eight_gua.iter().position(|&x| x == gua).unwrap();
        result[idx] = stem.to_string();
    }

    result
}

/// 排天盘
/// 天盘随值符星转动，以值符天干落宫为基准
fn arrange_tian_pan(
    di_pan: &[String; 9],
    dun_type: DunType,
    zhi_fu_gong: &str,
    zhi_fu_tian_gan: &str,
) -> [String; 9] {
    let eight_gua = ["坎", "坤", "震", "巽", "中", "乾", "兌", "艮", "離"];
    let clockwise = ["坎", "艮", "震", "巽", "離", "坤", "兌", "乾"];

    let rotate_order: Vec<&str> = match dun_type {
        DunType::YangDun => clockwise.iter().copied().collect(),
        DunType::YinDun => clockwise.iter().rev().copied().collect(),
    };

    // 值符天干在地盘上的位置
    let fu_head_pos = di_pan.iter().position(|x| x == zhi_fu_tian_gan);

    // 值符宫位在 rotating order 中的位置
    let zf_gong_pos = rotate_order.iter().position(|&x| x == zhi_fu_gong);

    let mut result: [String; 9] = Default::default();

    if let (Some(fu_pos), Some(zf_pos)) = (fu_head_pos, zf_gong_pos) {
        let fu_gua = eight_gua[fu_pos];

        // 如果值符宫是中宫，寄坤宫
        let actual_zf_gong = if zhi_fu_gong == "中" { "坤" } else { zhi_fu_gong };
        let actual_zf_pos = rotate_order.iter().position(|&x| x == actual_zf_gong).unwrap_or(0);

        // 生成天盘干序列
        let mut gan_sequence: Vec<String> = rotate_order.iter().map(|&g| {
            let idx = eight_gua.iter().position(|&x| x == g).unwrap_or(0);
            di_pan[idx].clone()
        }).collect();

        // 以值符天干所在宫为起点旋转
        let fu_gua_pos = rotate_order.iter().position(|&x| x == fu_gua).unwrap_or(0);
        gan_sequence = new_list(&gan_sequence, &di_pan[fu_pos]);

        // 将天干序列分配到各宫，从值符宫开始
        let reordered_gongs = new_list(&rotate_order, &rotate_order[actual_zf_pos]);
        for (i, gong) in reordered_gongs.iter().enumerate() {
            if let Some(idx) = eight_gua.iter().position(|&x| x == *gong) {
                if i < gan_sequence.len() {
                    result[idx] = gan_sequence[i].clone();
                }
            }
        }

        // 中宫特殊处理
        if let Some(mid_idx) = eight_gua.iter().position(|&x| x == "中") {
            result[mid_idx] = di_pan[mid_idx].clone();
        }
    } else {
        result.clone_from(di_pan);
    }

    result
}

/// 排八门
/// 阳遁顺排，阴遁逆排
fn arrange_ba_men(
    dun_type: DunType,
    zhi_shi_door: &str,
    zhi_shi_gong: &str,
) -> [String; 9] {
    let door_r = ["休", "生", "傷", "杜", "景", "死", "驚", "開"];
    let clockwise = ["坎", "艮", "震", "巽", "離", "坤", "兌", "乾"];
    let eight_gua = ["坎", "坤", "震", "巽", "中", "乾", "兌", "艮", "離"];

    let rotate_order: Vec<&str> = match dun_type {
        DunType::YangDun => clockwise.iter().copied().collect(),
        DunType::YinDun => clockwise.iter().rev().copied().collect(),
    };

    let actual_gong = if zhi_shi_gong == "中" { "坤" } else { zhi_shi_gong };
    let gong_reorder = new_list(&rotate_order, &actual_gong);

    let door_order: Vec<&str> = match dun_type {
        DunType::YangDun => new_list(&door_r.to_vec(), &zhi_shi_door),
        DunType::YinDun => {
            let mut reversed: Vec<&str> = door_r.iter().rev().copied().collect();
            // 在反转列表中从末尾开始找
            let pos = reversed.iter().position(|&x| x == zhi_shi_door).unwrap_or(0);
            let mut result: Vec<&str> = Vec::new();
            result.extend_from_slice(&reversed[pos..]);
            result.extend_from_slice(&reversed[..pos]);
            result
        }
    };

    let mut result: [String; 9] = Default::default();
    for (i, gong) in gong_reorder.iter().enumerate() {
        if let Some(idx) = eight_gua.iter().position(|&x| x == *gong) {
            if i < door_order.len() {
                result[idx] = door_order[i].to_string();
            }
        }
    }

    // 中宫空
    if let Some(mid_idx) = eight_gua.iter().position(|&x| x == "中") {
        result[mid_idx] = String::new();
    }

    result
}

/// 排九星
/// 阳遁顺排，阴遁逆排
fn arrange_jiu_xing(
    dun_type: DunType,
    zhi_fu_star: &str,
    zhi_fu_gong: &str,
) -> [String; 9] {
    let star_names = ["蓬", "任", "沖", "輔", "英", "禽", "柱", "心", "芮"];
    let clockwise = ["坎", "艮", "震", "巽", "離", "坤", "兌", "乾"];
    let eight_gua = ["坎", "坤", "震", "巽", "中", "乾", "兌", "艮", "離"];

    let rotate_order: Vec<&str> = match dun_type {
        DunType::YangDun => clockwise.iter().copied().collect(),
        DunType::YinDun => clockwise.iter().rev().copied().collect(),
    };

    let actual_gong = if zhi_fu_gong == "中" { "坤" } else { zhi_fu_gong };
    let gong_reorder = new_list(&rotate_order, &actual_gong);

    // 天芮星在值符时用天禽星
    let actual_star = if zhi_fu_star == "芮" { "禽" } else { zhi_fu_star };

    let star_order: Vec<&str> = match dun_type {
        DunType::YangDun => new_list(&star_names.to_vec(), &actual_star),
        DunType::YinDun => {
            let reversed: Vec<&str> = star_names.iter().rev().copied().collect();
            new_list(&reversed, &actual_star)
        }
    };

    let mut result: [String; 9] = Default::default();
    for (i, gong) in gong_reorder.iter().enumerate() {
        if let Some(idx) = eight_gua.iter().position(|&x| x == *gong) {
            if i < star_order.len() {
                result[idx] = format!("天{}", star_order[i]);
            }
        }
    }

    if let Some(mid_idx) = eight_gua.iter().position(|&x| x == "中") {
        result[mid_idx] = String::new();
    }

    result
}

/// 排八神
/// 阳遁顺排：值符->螣蛇->太阴->六合->白虎(勾陈)->玄武(朱雀)->九地->九天
/// 阴遁逆排
fn arrange_ba_shen(
    dun_type: DunType,
    zhi_fu_gong: &str,
) -> [String; 9] {
    let gods_yang = ["值符", "螣蛇", "太陰", "六合", "勾陳", "朱雀", "九地", "九天"];
    let gods_yin = ["值符", "螣蛇", "太陰", "六合", "白虎", "玄武", "九地", "九天"];

    let clockwise = ["坎", "艮", "震", "巽", "離", "坤", "兌", "乾"];
    let eight_gua = ["坎", "坤", "震", "巽", "中", "乾", "兌", "艮", "離"];

    let rotate_order: Vec<&str> = match dun_type {
        DunType::YangDun => clockwise.iter().copied().collect(),
        DunType::YinDun => clockwise.iter().rev().copied().collect(),
    };

    let actual_gong = if zhi_fu_gong == "中" { "坤" } else { zhi_fu_gong };
    let gong_reorder = new_list(&rotate_order, &actual_gong);

    let gods = match dun_type {
        DunType::YangDun => gods_yang,
        DunType::YinDun => gods_yin,
    };

    let mut result: [String; 9] = Default::default();
    for (i, gong) in gong_reorder.iter().enumerate() {
        if let Some(idx) = eight_gua.iter().position(|&x| x == *gong) {
            if i < gods.len() {
                result[idx] = gods[i].to_string();
            }
        }
    }

    if let Some(mid_idx) = eight_gua.iter().position(|&x| x == "中") {
        result[mid_idx] = String::new();
    }

    result
}

/// 排暗干
/// 值符宫起甲子戊，阳顺阴逆
fn arrange_an_gan(
    dun_type: DunType,
    zhi_fu_gong: &str,
) -> [String; 9] {
    let yang_earth = ["戊", "己", "庚", "辛", "壬", "癸", "丁", "丙", "乙"];
    let yin_earth = ["戊", "乙", "丙", "丁", "癸", "壬", "辛", "庚", "己"];

    let clockwise = ["坎", "艮", "震", "巽", "離", "坤", "兌", "乾"];
    let eight_gua = ["坎", "坤", "震", "巽", "中", "乾", "兌", "艮", "離"];

    let rotate_order: Vec<&str> = match dun_type {
        DunType::YangDun => clockwise.iter().copied().collect(),
        DunType::YinDun => clockwise.iter().rev().copied().collect(),
    };

    let actual_gong = if zhi_fu_gong == "中" { "坤" } else { zhi_fu_gong };
    let gong_reorder = new_list(&rotate_order, &actual_gong);

    let stems = match dun_type {
        DunType::YangDun => yang_earth,
        DunType::YinDun => yin_earth,
    };

    let mut result: [String; 9] = Default::default();
    for (i, gong) in gong_reorder.iter().enumerate() {
        if let Some(idx) = eight_gua.iter().position(|&x| x == *gong) {
            if i < stems.len() {
                result[idx] = stems[i].to_string();
            }
        }
    }

    if let Some(mid_idx) = eight_gua.iter().position(|&x| x == "中") {
        result[mid_idx] = String::new();
    }

    result
}

/// 获取马星
/// 亥卯未在巳, 申子辰在寅, 巳酉丑在亥, 寅午戌在申
fn get_ma_xing(shi_chen_zhi: DiZhi) -> String {
    match shi_chen_zhi {
        DiZhi::Hai | DiZhi::Mao | DiZhi::Wei => "巳".to_string(),
        DiZhi::Shen | DiZhi::Zi | DiZhi::Chen => "寅".to_string(),
        DiZhi::Si | DiZhi::You | DiZhi::Chou => "亥".to_string(),
        DiZhi::Yin | DiZhi::Wu | DiZhi::Xu => "申".to_string(),
    }
}

/// 获取旬空
/// 甲子旬戌亥空, 甲戌旬申酉空, 甲申旬午未空, 甲午旬辰巳空, 甲辰旬寅卯空, 甲寅旬子丑空
fn get_xun_kong(ganzhi: &str) -> [String; 2] {
    let xun_shou = get_xun_shou(ganzhi);
    match xun_shou.as_str() {
        "甲子" => ["戌".to_string(), "亥".to_string()],
        "甲戌" => ["申".to_string(), "酉".to_string()],
        "甲申" => ["午".to_string(), "未".to_string()],
        "甲午" => ["辰".to_string(), "巳".to_string()],
        "甲辰" => ["寅".to_string(), "卯".to_string()],
        "甲寅" => ["子".to_string(), "丑".to_string()],
        _ => ["戌".to_string(), "亥".to_string()],
    }
}

/// 奇门遁甲计算器
pub struct QimenCalc;

impl QimenCalc {
    /// 计算奇门遁甲盘
    /// 参考原项目: vendor/kinqimen/kinqimen.py
    pub fn calculate(&self, year: i32, month: u32, day: u32, hour: u32, minute: u32) -> QimenChart {
        // 计算日干支和时干支
        let day_ganzhi = calc_day_ganzhi(year, month, day);
        let shi_ganzhi = calc_hour_ganzhi(&day_ganzhi, hour);
        let shi_chen = hour_to_dizhi(hour);

        // 确定节气
        let jieqi = get_jieqi(year, month, day);

        // 确定三元
        let yuan = find_yuan(&day_ganzhi);

        // 确定阴阳遁
        let dun_type = constants::jieqi_dun_type(&jieqi).unwrap_or(DunType::YangDun);

        // 确定局数
        let (ju_shang, ju_zhong, ju_xia) = constants::jieqi_jushu(&jieqi);
        let ju = match yuan {
            SanYuan::ShangYuan => ju_shang,
            SanYuan::ZhongYuan => ju_zhong,
            SanYuan::XiaYuan => ju_xia,
        };

        // 用局字符串
        let yong_ju = format!("{}{}局{}元", dun_type.name_zh(), ju, yuan.name_zh());

        // 排地盘
        let di_pan = arrange_di_pan(dun_type, ju);

        // 确定值符值使
        let (zhi_fu_star, zhi_fu_gong, zhi_fu_tian_gan, zhi_shi_door, zhi_shi_gong) =
            determine_zhi_fu_zhi_shi(ju, dun_type, &shi_ganzhi);

        // 排天盘
        let tian_pan = arrange_tian_pan(&di_pan, dun_type, &zhi_fu_gong, &zhi_fu_tian_gan);

        // 排八门
        let ba_men_arr = arrange_ba_men(dun_type, &zhi_shi_door, &zhi_shi_gong);

        // 排九星
        let jiu_xing_arr = arrange_jiu_xing(dun_type, &zhi_fu_star, &zhi_fu_gong);

        // 排八神
        let ba_shen_arr = arrange_ba_shen(dun_type, &zhi_fu_gong);

        // 排暗干
        let an_gan_arr = arrange_an_gan(dun_type, &zhi_fu_gong);

        // 马星
        let ma_xing = get_ma_xing(shi_chen);

        // 旬空
        let xun_kong = get_xun_kong(&shi_ganzhi);

        // 构建九宫格
        let eight_gua = ["坎", "坤", "震", "巽", "中", "乾", "兌", "艮", "離"];
        let mut gongs: [QimenGong; 9] = [
            QimenGong {
                number: 1, di_pan_gan: String::new(), tian_pan_gan: String::new(),
                men: String::new(), xing: String::new(), shen: String::new(),
                an_gan: String::new(), di_shen: String::new(),
                gong_info: constants::gong_info(1), chang_sheng: None,
            },
            QimenGong {
                number: 2, di_pan_gan: String::new(), tian_pan_gan: String::new(),
                men: String::new(), xing: String::new(), shen: String::new(),
                an_gan: String::new(), di_shen: String::new(),
                gong_info: constants::gong_info(2), chang_sheng: None,
            },
            QimenGong {
                number: 3, di_pan_gan: String::new(), tian_pan_gan: String::new(),
                men: String::new(), xing: String::new(), shen: String::new(),
                an_gan: String::new(), di_shen: String::new(),
                gong_info: constants::gong_info(3), chang_sheng: None,
            },
            QimenGong {
                number: 4, di_pan_gan: String::new(), tian_pan_gan: String::new(),
                men: String::new(), xing: String::new(), shen: String::new(),
                an_gan: String::new(), di_shen: String::new(),
                gong_info: constants::gong_info(4), chang_sheng: None,
            },
            QimenGong {
                number: 5, di_pan_gan: String::new(), tian_pan_gan: String::new(),
                men: String::new(), xing: String::new(), shen: String::new(),
                an_gan: String::new(), di_shen: String::new(),
                gong_info: constants::gong_info(5), chang_sheng: None,
            },
            QimenGong {
                number: 6, di_pan_gan: String::new(), tian_pan_gan: String::new(),
                men: String::new(), xing: String::new(), shen: String::new(),
                an_gan: String::new(), di_shen: String::new(),
                gong_info: constants::gong_info(6), chang_sheng: None,
            },
            QimenGong {
                number: 7, di_pan_gan: String::new(), tian_pan_gan: String::new(),
                men: String::new(), xing: String::new(), shen: String::new(),
                an_gan: String::new(), di_shen: String::new(),
                gong_info: constants::gong_info(7), chang_sheng: None,
            },
            QimenGong {
                number: 8, di_pan_gan: String::new(), tian_pan_gan: String::new(),
                men: String::new(), xing: String::new(), shen: String::new(),
                an_gan: String::new(), di_shen: String::new(),
                gong_info: constants::gong_info(8), chang_sheng: None,
            },
            QimenGong {
                number: 9, di_pan_gan: String::new(), tian_pan_gan: String::new(),
                men: String::new(), xing: String::new(), shen: String::new(),
                an_gan: String::new(), di_shen: String::new(),
                gong_info: constants::gong_info(9), chang_sheng: None,
            },
        ];

        for i in 0..9 {
            gongs[i].di_pan_gan = di_pan[i].clone();
            gongs[i].tian_pan_gan = if !tian_pan[i].is_empty() { tian_pan[i].clone() } else { di_pan[i].clone() };
            gongs[i].men = ba_men_arr[i].clone();
            gongs[i].xing = jiu_xing_arr[i].clone();
            gongs[i].shen = ba_shen_arr[i].clone();
            gongs[i].an_gan = an_gan_arr[i].clone();
            gongs[i].di_shen = String::new();
        }

        let zhi_fu_full = format!("天{}", zhi_fu_star);
        let zhi_shi_full = format!("{}門", zhi_shi_door);

        QimenChart {
            dun_type,
            ju,
            jieqi,
            yuan,
            yong_ju,
            gongs,
            zhi_fu: zhi_fu_full,
            zhi_shi: zhi_shi_full,
            xun_kong,
            ma_xing,
            shi_chen: divines_core::sanshi::DiZhi::from_str(shi_chen.name_zh()).unwrap_or(divines_core::sanshi::DiZhi::Zi),
            shi_gan_zhi: shi_ganzhi,
            day_gan_zhi: day_ganzhi,
        }
    }

    /// 查询局数（不排盘）
    pub fn get_ju(&self, year: i32, month: u32, day: u32, hour: u32) -> serde_json::Value {
        let day_ganzhi = calc_day_ganzhi(year, month, day);
        let jieqi = get_jieqi(year, month, day);
        let yuan = find_yuan(&day_ganzhi);
        let dun_type = constants::jieqi_dun_type(&jieqi).unwrap_or(DunType::YangDun);
        let (ju_shang, ju_zhong, ju_xia) = constants::jieqi_jushu(&jieqi);
        let ju = match yuan {
            SanYuan::ShangYuan => ju_shang,
            SanYuan::ZhongYuan => ju_zhong,
            SanYuan::XiaYuan => ju_xia,
        };

        serde_json::json!({
            "dun_type": dun_type.name_zh(),
            "ju": ju,
            "jieqi": jieqi,
            "yuan": yuan.name_zh(),
            "yong_ju": format!("{}{}局{}元", dun_type.name_zh(), ju, yuan.name_zh()),
            "day_ganzhi": day_ganzhi,
        })
    }
}

impl Default for QimenCalc {
    fn default() -> Self { Self }
}

// ============ 太乙神数 ============

pub struct TaiyiCalc;

impl TaiyiCalc {
    pub fn calculate(&self, birth: &BirthInfo) -> TaiyiChart {
        let year = birth.datetime.year();

        TaiyiChart {
            taiyi_gong: (year % 16 + 1) as u8,
            shi_ji_gong: ((year + 1) % 16 + 1) as u8,
            wen_chang_gong: ((year + 2) % 16 + 1) as u8,
            zhu_da_jiang: "主大将".to_string(),
            zhu_can_jiang: "主参将".to_string(),
            ke_da_jiang: "客大将".to_string(),
            ke_can_jiang: "客参将".to_string(),
            wu_fu: "五福".to_string(),
            jun_ji: "君基".to_string(),
            chen_ji: "臣基".to_string(),
            min_ji: "民基".to_string(),
            ji_shen: "计神".to_string(),
            he_shen: "合神".to_string(),
            tai_sui: "太岁".to_string(),
            shi_liu_shen: std::array::from_fn(|i| TaiyiShen {
                name: format!("神{}", i + 1),
                gong: (i + 1) as u8,
                description: "".to_string(),
            }),
        }
    }
}

impl Default for TaiyiCalc {
    fn default() -> Self { Self }
}

// ============ 六壬 ============

pub struct LiurenCalc;

impl LiurenCalc {
    pub fn calculate(&self, birth: &BirthInfo) -> LiurenChart {
        let month = birth.datetime.month();
        let hour = birth.datetime.hour();

        let yue_jiang = self.get_yue_jiang(month);
        let tian_pan = self.arrange_tian_pan(yue_jiang, hour);
        let di_pan = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

        let hour_dz: usize = match hour {
            23 | 0 => 0, 1 | 2 => 1, 3 | 4 => 2, 5 | 6 => 3,
            7 | 8 => 4, 9 | 10 => 5, 11 | 12 => 6, 13 | 14 => 7,
            15 | 16 => 8, 17 | 18 => 9, 19 | 20 => 10, 21 | 22 => 11,
            _ => 0,
        };

        LiurenChart {
            month_jiang: yue_jiang.to_string(),
            zhan_shi: di_pan[hour_dz].to_string(),
            tian_pan: tian_pan.map(|s| s.to_string()),
            di_pan: di_pan.map(|s| s.to_string()),
            si_ke: SiKe {
                ke1: (String::new(), String::new()),
                ke2: (String::new(), String::new()),
                ke3: (String::new(), String::new()),
                ke4: (String::new(), String::new()),
            },
            san_chuan: SanChuan {
                chu_chuan: "初传".to_string(),
                zhong_chuan: "中传".to_string(),
                mo_chuan: "末传".to_string(),
            },
            dun_gan: ["甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未", "壬申", "癸酉", "甲戌", "乙亥"].map(|s| s.to_string()),
            shen_jiang: std::array::from_fn(|i| ShenJiang {
                name: ["贵人", "螣蛇", "朱雀", "六合", "勾陈", "青龙", "天空", "白虎", "太常", "玄武", "太阴", "天后"][i].to_string(),
                position: i,
                description: String::new(),
            }),
            gui_ren_position: 0,
            yang_gui: true,
            tian_jiang: ["贵人", "螣蛇", "朱雀", "六合", "勾陈", "青龙", "天空", "白虎", "太常", "玄武", "太阴", "天后"].map(|s| s.to_string()),
            liu_qin: ["父母", "兄弟", "妻财", "官鬼", "子孙", "父母", "兄弟", "妻财", "官鬼", "子孙", "父母", "兄弟"].map(|s| s.to_string()),
            de_shen: "".to_string(),
            he_shen: "".to_string(),
            gui: vec![],
            kong_wang: ["戌亥".to_string(), "".to_string()],
            zuo_shan: "".to_string(),
            xing_nian: "".to_string(),
            ben_ming: "".to_string(),
            four_pillars: LiurenPillars {
                year: String::new(),
                month: String::new(),
                day: String::new(),
                hour: String::new(),
            },
        }
    }

    fn get_yue_jiang(&self, month: u32) -> &str {
        match month {
            1 => "子", 2 => "亥", 3 => "戌",
            4 => "酉", 5 => "申", 6 => "未",
            7 => "午", 8 => "巳", 9 => "辰",
            10 => "卯", 11 => "寅", _ => "丑",
        }
    }

    fn arrange_tian_pan(&self, yue_jiang: &str, hour: u32) -> [&str; 12] {
        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
        let hour_dz = match hour {
            23 | 0 => 0, 1 | 2 => 1, 3 | 4 => 2, 5 | 6 => 3,
            7 | 8 => 4, 9 | 10 => 5, 11 | 12 => 6, 13 | 14 => 7,
            15 | 16 => 8, 17 | 18 => 9, 19 | 20 => 10, 21 | 22 => 11,
            _ => 0,
        };

        let yj_pos = di_zhi.iter().position(|&d| d == yue_jiang).unwrap_or(0);
        let offset = (hour_dz as i8 - yj_pos as i8).rem_euclid(12) as usize;

        let mut tian_pan = [""; 12];
        for i in 0..12 {
            tian_pan[i] = di_zhi[(i + offset) % 12];
        }
        tian_pan
    }
}

impl Default for LiurenCalc {
    fn default() -> Self { Self }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_day_ganzhi() {
        // 2024-01-14 should be specific day
        let gz = calc_day_ganzhi(2024, 1, 14);
        assert!(!gz.is_empty());
        println!("2024-01-14 day ganzhi: {}", gz);
    }

    #[test]
    fn test_get_jieqi() {
        let jq = get_jieqi(2024, 1, 14);
        assert!(!jq.is_empty());
        println!("2024-01-14 jieqi: {}", jq);
    }

    #[test]
    fn test_find_yuan() {
        let gz = calc_day_ganzhi(2024, 1, 14);
        let yuan = find_yuan(&gz);
        println!("2024-01-14 yuan: {:?}", yuan);
    }

    #[test]
    fn test_qimen_calculate() {
        let calc = QimenCalc;
        let chart = calc.calculate(2024, 1, 14, 23, 20);
        println!("=== Qimen Chart ===");
        println!("用局: {}", chart.yong_ju);
        println!("节气: {}", chart.jieqi);
        println!("值符: {}", chart.zhi_fu);
        println!("值使: {}", chart.zhi_shi);
        println!("时辰: {:?}", chart.shi_chen);
        println!("时辰干支: {}", chart.shi_gan_zhi);
        println!("日干支: {}", chart.day_gan_zhi);
        println!("旬空: {:?}", chart.xun_kong);
        println!("马星: {}", chart.ma_xing);
        println!();
        for gong in &chart.gongs {
            println!("宫{}: 地盘={}, 天盘={}, 门={}, 星={}, 神={}, 暗干={}, 八卦={}",
                gong.number, gong.di_pan_gan, gong.tian_pan_gan,
                gong.men, gong.xing, gong.shen, gong.an_gan,
                gong.gong_info.name);
        }
    }

    #[test]
    fn test_qimen_ju() {
        let calc = QimenCalc;
        let ju = calc.get_ju(2024, 1, 14, 23);
        println!("局数: {}", serde_json::to_string_pretty(&ju).unwrap());
    }
}