// 星阙 Horosa - 十二宫（流年神煞）模块
// 参考原项目: astrostudysrv/astrostudy/helper gong12.json

use std::collections::HashMap;
use horosa_core::*;

use super::data;

/// 十二宫信息
#[derive(Debug, Clone)]
pub struct Gong12Info {
    /// 命宫地支
    pub ming_zhi: String,
    /// 命宫干支
    pub ming_ganzhi: String,
    /// 十二宫神煞映射
    pub palace_gods: HashMap<String, Vec<String>>,
    /// 星曜
    pub stars: HashMap<String, String>,
    /// 卦位
    pub gua: HashMap<String, String>,
}

/// 星曜信息
#[derive(Debug, Clone)]
pub struct StarInfo {
    pub name: String,
    pub event: String,
}

/// 十二宫辅助器
pub struct Gong12;

impl Gong12 {
    /// 计算命宫
    /// 命宫地支 = (14 - (月支索引 + 时支索引)) % 12
    /// 月支索引: 寅=0, 卯=1, ..., 子=10, 丑=11
    pub fn get_ming_gong(month_zhi: DiZhi, hour_zhi: DiZhi) -> Gong12Info {
        let month_idx = Self::zhi_to_lunar_index(month_zhi);
        let hour_idx = Self::zhi_to_lunar_index(hour_zhi);

        // 命宫地支索引: 从寅开始
        let ming_idx = (14 - (month_idx + hour_idx)).rem_euclid(12);
        let ming_zhi = Self::lunar_index_to_zhi(ming_idx);

        // 从命宫地支开始，按顺序排列十二宫
        let zhi_order = ["寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥", "子", "丑"];
        let mut start_idx = ming_idx as usize;

        let gong12_data = data::get_gong12_data();
        let gods_data = &gong12_data["gods"];

        let mut palace_gods: HashMap<String, Vec<String>> = HashMap::new();
        let mut stars: HashMap<String, String> = HashMap::new();
        let mut gua: HashMap<String, String> = HashMap::new();

        let palace_names = ["命宫", "兄弟", "夫妻", "子女", "财帛", "疾厄", "迁移", "交友", "官禄", "田宅", "福德", "父母"];

        for i in 0..12 {
            let zhi = zhi_order[(start_idx + i) % 12];
            let palace_name = palace_names[i].to_string();

            // 查找该地支对应的神煞
            for god_entry in gods_data.as_array().unwrap_or(&vec![]) {
                if god_entry["zhi"].as_str() == Some(zhi) {
                    let god_list: Vec<String> = god_entry["gods"]
                        .as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .filter_map(|g| g.as_str().map(String::from))
                        .collect();
                    palace_gods.insert(palace_name.clone(), god_list);

                    if let Some(star) = god_entry["star"].as_str() {
                        stars.insert(palace_name.clone(), star.to_string());
                    }
                    if let Some(g) = god_entry["gua"].as_str() {
                        gua.insert(palace_name.clone(), g.to_string());
                    }
                    break;
                }
            }
        }

        // 命宫干支（简化：使用五虎遁计算）
        // 这里需要根据年干推算命宫天干
        let ming_ganzhi = format!("{}", ming_zhi);

        Gong12Info {
            ming_zhi: ming_zhi.to_string(),
            ming_ganzhi,
            palace_gods,
            stars,
            gua,
        }
    }

    /// 获取年支对应的流年神煞
    pub fn get_year_charger(year_zhi: DiZhi, ming_zhi: DiZhi) -> Option<StarInfo> {
        let gong12_data = data::get_gong12_data();
        let year_zhi_str = year_zhi.name_zh();

        for god_entry in gong12_data["gods"].as_array().unwrap_or(&vec![]) {
            if god_entry["zhi"].as_str() == Some(year_zhi_str) {
                let name = god_entry["gods"][0].as_str().unwrap_or("").to_string();
                // 查找对应的事件描述
                let event = Self::find_star_event(&gong12_data["stars"], &name);
                return Some(StarInfo { name, event });
            }
        }

        None
    }

    fn find_star_event(stars: &serde_json::Value, name: &str) -> String {
        for star in stars.as_array().unwrap_or(&vec![]) {
            if star["name"].as_str() == Some(name) {
                return star["event"].as_str().unwrap_or("").to_string();
            }
        }
        String::new()
    }

    /// 将地支转换为农历月份索引（寅=0, 卯=1, ..., 子=10, 丑=11）
    fn zhi_to_lunar_index(dz: DiZhi) -> i32 {
        match dz {
            DiZhi::Yin => 0,
            DiZhi::Mao => 1,
            DiZhi::Chen => 2,
            DiZhi::Si => 3,
            DiZhi::Wu => 4,
            DiZhi::Wei => 5,
            DiZhi::Shen => 6,
            DiZhi::You => 7,
            DiZhi::Xu => 8,
            DiZhi::Hai => 9,
            DiZhi::Zi => 10,
            DiZhi::Chou => 11,
        }
    }

    /// 将农历月份索引转换回地支
    fn lunar_index_to_zhi(idx: i32) -> &'static str {
        match idx.rem_euclid(12) {
            0 => "寅", 1 => "卯", 2 => "辰",
            3 => "巳", 4 => "午", 5 => "未",
            6 => "申", 7 => "酉", 8 => "戌",
            9 => "亥", 10 => "子", 11 => "丑",
            _ => "寅",
        }
    }
}