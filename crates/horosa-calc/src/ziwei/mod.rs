// 星阙 Horosa - 紫微斗数计算引擎
// 参考原项目: astrostudysrv/astrostudycn/model/ZiWeiChart.java, helper/ZiWeiHelper.java
// 参考原项目: vendor/kinastro/astro/ziwei.py

pub mod data;

use horosa_core::*;
use data::*;
use std::collections::HashMap;

/// 紫微斗数计算器
pub struct ZiWeiCalc;

impl Default for ZiWeiCalc {
    fn default() -> Self {
        Self
    }
}

impl ZiWeiCalc {
    /// 计算完整的紫微斗数命盘
    pub fn calculate(&self, input: &ZiWeiInput) -> ZiWeiChart {
        let mut chart = ZiWeiChart {
            nongli: None,
            houses: (0..12).map(|_| ZiWeiHouse::new()).collect(),
            life_house_index: 0,
            body_house_index: 0,
            wuxing_ju: 2,
            wuxing_ju_text: String::new(),
            ziwei_index: 0,
            gender: input.gender,
            year_gan: String::new(),
            year_zhi: String::new(),
            year_polar: String::new(),
            time_zhi: String::new(),
            life_master: String::new(),
            body_master: String::new(),
            zidou: String::new(),
            doujun: String::new(),
            stars_house_index: HashMap::new(),
            birth: input.birth.clone(),
            zone: input.zone.clone(),
            lon: input.lon.clone(),
            lat: input.lat.clone(),
            ad: input.ad,
            time_alg: input.time_alg,
            bazi: serde_json::Value::Null,
            patterns: Vec::new(),
        };

        // 从 birth 字符串解析出生信息
        // 格式: "YYYY-MM-DD HH:MM:SS" 或 "YYYY-MM-DD HH:MM"
        let (nongli, year_gan, year_zhi, time_zhi) = self.parse_birth(&input.birth);

        chart.year_gan = year_gan.clone();
        chart.year_zhi = year_zhi.clone();
        chart.time_zhi = time_zhi.clone();
        chart.year_polar = stem_polar(&year_gan).to_string();

        // 1. 安命宫、身宫
        let (life_idx, body_idx) = self.setup_life_body(
            &mut chart,
            nongli.month_int,
            &time_zhi,
            nongli.leap,
            nongli.day_int,
        );

        // 2. 定十二宫干支
        self.setup_house_ganzi(&mut chart);

        // 3. 定五行局并安十二宫名称、长生
        let wuxing_ju = self.setup_wuxing_ju_and_houses(
            &mut chart,
            life_idx,
            &year_gan,
            &year_zhi,
            input.gender,
        );
        chart.wuxing_ju = wuxing_ju;
        chart.wuxing_ju_text = wuxing_ju_text(wuxing_ju);

        // 4. 安紫微星
        chart.ziwei_index = self.setup_ziwei_pos(nongli.day_int, wuxing_ju);

        // 5. 安十四主星
        self.setup_stars_main(&mut chart);

        // 6. 安年干星
        self.setup_stars_by_year_gan(&mut chart);

        // 7. 安年支星
        self.setup_stars_by_year_zhi(&mut chart);

        // 8. 安月星
        self.setup_stars_by_month(&mut chart, &nongli.month);

        // 9. 安时星
        self.setup_stars_by_time_zhi(&mut chart);

        // 10. 安火铃星
        self.setup_stars_huo_lin(&mut chart);

        // 11. 安日星
        self.setup_stars_by_days(&mut chart, nongli.day_int);

        // 12. 安旬空
        self.setup_stars_xun_empty(&mut chart);

        // 13. 安天才天寿
        self.setup_tian_cai_shou(&mut chart);

        // 14. 安博士十二神
        self.setup_stars_bosi(&mut chart);

        // 15. 安岁前十二神
        self.setup_stars_tai_sui(&mut chart);

        // 16. 安将星
        self.setup_stars_jiang(&mut chart);

        // 17. 定命主、身主
        chart.life_master = life_master(&year_zhi).to_string();
        chart.body_master = body_master(&year_zhi).to_string();

        // 18. 安斗君
        let zidou = self.get_dou_jun(&nongli.month, &time_zhi);
        chart.zidou = zidou.clone();
        let doujun_idx = (branch_index(&zidou).unwrap_or(0) + branch_index(&year_zhi).unwrap_or(0)) % 12;
        chart.doujun = BRANCHES[doujun_idx].to_string();

        // 19. 安小限
        self.setup_small_direction(&mut chart);

        // 20. 定格局
        chart.patterns = self.detect_patterns(&chart);

        chart.nongli = Some(nongli);

        chart
    }

    /// 从出生日期字符串解析信息
    fn parse_birth(&self, birth: &str) -> (NongLiZiWei, String, String, String) {
        // 简化解析：按 "YYYY-MM-DD HH:MM:SS" 格式
        let parts: Vec<&str> = birth.split_whitespace().collect();
        let date_part = parts.first().unwrap_or(&"2000-01-01");
        let time_part = parts.get(1).unwrap_or(&"12:00:00");

        let date_components: Vec<&str> = date_part.split('-').collect();
        let year: i32 = date_components.first().and_then(|s| s.parse().ok()).unwrap_or(2000);
        let month: i32 = date_components.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
        let day: i32 = date_components.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);

        let time_components: Vec<&str> = time_part.split(':').collect();
        let hour: i32 = time_components.first().and_then(|s| s.parse().ok()).unwrap_or(12);

        // 获取时辰地支
        let time_zhi = self.hour_to_zhi(hour).to_string();

        // 获取农历月份名
        let month_name = if month >= 1 && month <= 12 {
            LUNAR_MONTHS[(month - 1) as usize].to_string()
        } else {
            "正月".to_string()
        };

        // 简单日干支计算（基于2000年基准）
        let day_gan_zi = self.simple_day_ganzhi(year, month, day);

        // 年干支简化计算
        let year_gan = STEMS[((year - 4).rem_euclid(10)) as usize].to_string();
        let year_zhi = BRANCHES[((year - 4).rem_euclid(12)) as usize].to_string();

        let nongli = NongLiZiWei {
            year: format!("{}{}", year_gan, year_zhi),
            month: month_name,
            month_int: month,
            day_int: day,
            time: format!("子{}", time_zhi),
            day_gan_zi,
            leap: false,
        };

        (nongli, year_gan, year_zhi, time_zhi)
    }

    /// 将小时转为时辰地支
    fn hour_to_zhi(&self, hour: i32) -> &'static str {
        match hour {
            23 | 0 => "子",
            1 | 2 => "丑",
            3 | 4 => "寅",
            5 | 6 => "卯",
            7 | 8 => "辰",
            9 | 10 => "巳",
            11 | 12 => "午",
            13 | 14 => "未",
            15 | 16 => "申",
            17 | 18 => "酉",
            19 | 20 => "戌",
            21 | 22 => "亥",
            _ => "子",
        }
    }

    /// 简化日干支计算（基于已知基准日期）
    fn simple_day_ganzhi(&self, year: i32, month: i32, day: i32) -> String {
        // 使用简化的日期到干支算法
        // 基于 2000-01-01 = 戊午
        let days = self.days_since_2000(year, month, day);
        let gan_idx = ((days + 4).rem_euclid(10)) as usize; // 2000-01-01 是戊(4)
        let zhi_idx = ((days + 6).rem_euclid(12)) as usize; // 2000-01-01 是午(6)
        format!("{}{}", STEMS[gan_idx], BRANCHES[zhi_idx])
    }

    /// 计算从2000年1月1日起的天数
    fn days_since_2000(&self, year: i32, month: i32, day: i32) -> i64 {
        let mut total: i64 = 0;
        // 年差
        for y in 2000..year {
            total += if self.is_leap_year(y) { 366 } else { 365 };
        }
        for y in year..2000 {
            total -= if self.is_leap_year(y) { 366 } else { 365 };
        }
        // 月差
        let month_days = if self.is_leap_year(year) {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };
        for m in 1..month {
            total += month_days[(m - 1) as usize] as i64;
        }
        total += day as i64 - 1;
        total
    }

    fn is_leap_year(&self, year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    // ============================================================
    // 1. 安命宫/身宫
    // ============================================================

    /// 安命宫、身宫
    /// 命宫 = (寅 + month - 1) - time_zhi_index
    /// 身宫 = (寅 + month - 1) + time_zhi_index
    fn setup_life_body(
        &self,
        chart: &mut ZiWeiChart,
        month: i32,
        time_zhi: &str,
        leap: bool,
        day_int: i32,
    ) -> (usize, usize) {
        let mut m = month;
        if leap && day_int >= 16 {
            m += 1;
        }
        // 寅起正月，寅支index=2
        let loc = (2 + m - 1) % 12;
        let tm_idx = branch_index(time_zhi).unwrap_or(0) as i32;

        let life_idx = ((loc as i32 - tm_idx + 24) % 12) as usize;
        let body_idx = ((loc as i32 + tm_idx) % 12) as usize;

        chart.houses[life_idx].is_life = true;
        chart.houses[body_idx].is_body = true;
        chart.life_house_index = life_idx;
        chart.body_house_index = body_idx;

        (life_idx, body_idx)
    }

    // ============================================================
    // 2. 定十二宫干支（五虎遁）
    // ============================================================

    /// 宫位干支（五虎遁法）
    fn setup_house_ganzi(&self, chart: &mut ZiWeiChart) {
        let start_gan = house_gan_start(&chart.year_gan);
        let start_gan_idx = stem_index(start_gan).unwrap_or(0);

        // 寅宫干支（地支index=2）
        chart.houses[2].ganzi = format!("{}{}", STEMS[start_gan_idx], BRANCHES[2]);
        // 卯宫干支（地支index=3）
        chart.houses[3].ganzi = format!("{}{}", STEMS[(start_gan_idx + 1) % 10], BRANCHES[3]);

        // 其余宫位
        for i in 0..12 {
            if i == 2 || i == 3 {
                continue;
            }
            let offset = if i < 2 { i + 12 - 2 } else { i - 2 };
            let gan_idx = (start_gan_idx + offset) % 10;
            chart.houses[i].ganzi = format!("{}{}", STEMS[gan_idx], BRANCHES[i]);
        }
    }

    // ============================================================
    // 3. 定五行局并安十二宫名称、长生
    // ============================================================

    /// 定五行局、十二宫名称、长生十二神、大限
    fn setup_wuxing_ju_and_houses(
        &self,
        chart: &mut ZiWeiChart,
        life_idx: usize,
        year_gan: &str,
        year_zhi: &str,
        gender: Gender,
    ) -> i32 {
        // 命宫干支纳音 → 五行局数
        let life_ganzi = &chart.houses[life_idx].ganzi;
        let gan = ganzi_first(life_ganzi);
        let zhi = ganzi_second(life_ganzi);
        let nayin_wx = get_nayin_wuxing(gan, zhi);
        let wuxing_ju = wuxing_ju_num(nayin_wx);

        // 长生十二神起始
        // 水二局长生在申(8), 木三局长生在亥(11), 金四局长生在巳(5), 土五局长生在申(8), 火六局长生在寅(2)
        let changsheng_start = match nayin_wx {
            "水" | "土" => 8, // 申
            "木" => 11,       // 亥
            "金" => 5,        // 巳
            "火" => 2,        // 寅
            _ => 8,
        };

        // 顺时针/逆时针判断
        let year_polar_val = stem_polar(year_gan);
        let is_clockwise = (gender == Gender::Male && year_polar_val == "阳")
            || (gender == Gender::Female && year_polar_val == "阴");

        for i in 0..12 {
            // 长生十二神
            let phase_idx = if is_clockwise {
                ((i as i32 - changsheng_start as i32 + 24) % 12) as usize
            } else {
                ((changsheng_start as i32 - i as i32 + 24) % 12) as usize
            };
            chart.houses[i].phase = Some(CHANG_SHENG_12[phase_idx].to_string());

            // 宫位名称
            let delta = i as i32 - life_idx as i32;
            let name_idx = if delta >= 0 {
                (12 - delta) % 12
            } else {
                (-delta) % 12
            };
            let final_idx = if name_idx == 0 { 0 } else { 12 - name_idx };
            chart.houses[i].name = HOUSE_NAMES[final_idx as usize].to_string();

            // 大限
            if is_clockwise {
                if i == life_idx {
                    chart.houses[i].direction = vec![wuxing_ju, wuxing_ju + 9];
                } else {
                    let idx = ((life_idx as i32 - i as i32 + 12) % 12) as i32;
                    chart.houses[i].direction = vec![10 * idx + wuxing_ju, 10 * idx + wuxing_ju + 9];
                }
            } else {
                if i == life_idx {
                    chart.houses[i].direction = vec![wuxing_ju, wuxing_ju + 9];
                } else {
                    let idx = ((i as i32 - life_idx as i32 + 12) % 12) as i32;
                    chart.houses[i].direction = vec![10 * idx + wuxing_ju, 10 * idx + wuxing_ju + 9];
                }
            }
        }

        wuxing_ju
    }

    // ============================================================
    // 4. 安紫微星
    // ============================================================

    /// 安紫微星位置
    fn setup_ziwei_pos(&self, day: i32, wuxing_ju: i32) -> usize {
        let rest = day % wuxing_ju;
        let div = day / wuxing_ju;

        if rest == 0 {
            ((1 + div) % 12) as usize
        } else {
            let div = div + 1;
            let rest = wuxing_ju * div - day;
            if rest % 2 == 0 {
                ((1 + div + rest) % 12) as usize
            } else {
                ((1 + div - rest + 24) % 12) as usize
            }
        }
    }

    // ============================================================
    // 5. 安十四主星
    // ============================================================

    /// 安十四主星（紫微系 + 天府系）
    fn setup_stars_main(&self, chart: &mut ZiWeiChart) {
        let ziwei_idx = chart.ziwei_index;

        // 天府位置
        let tianfu_idx = if ziwei_idx >= 2 && ziwei_idx <= 8 {
            let delta = ziwei_idx as i32 - 2;
            ((2 - delta + 24) % 12) as usize
        } else {
            let delta = ziwei_idx as i32 - 8;
            ((8 - delta + 24) % 12) as usize
        };

        // 紫微系（逆布）
        let north_stars = [
            ("紫微", 0),
            ("天机", -1),
            ("太阳", -3),
            ("武曲", -4),
            ("天同", -5),
            ("廉贞", -8),
        ];

        for (name, step) in &north_stars {
            let idx = ((ziwei_idx as i32 + step + 12) % 12) as usize;
            let house_zi = ganzi_second(&chart.houses[idx].ganzi);
            let mut star = self.create_star(name, &chart.year_gan, house_zi);
            self.setup_sihua_for_star(&mut star, &chart.year_gan);
            chart.houses[idx].add_star(star, StarType::Main);
            chart.stars_house_index.insert(name.to_string(), idx);
        }

        // 天府系（顺布）
        let south_stars = [
            ("天府", 0),
            ("太阴", 1),
            ("贪狼", 2),
            ("巨门", 3),
            ("天相", 4),
            ("天梁", 5),
            ("七杀", 6),
            ("破军", 10),
        ];

        for (name, step) in &south_stars {
            let idx = ((tianfu_idx as i32 + step) % 12) as usize;
            let house_zi = ganzi_second(&chart.houses[idx].ganzi);
            let mut star = self.create_star(name, &chart.year_gan, house_zi);
            self.setup_sihua_for_star(&mut star, &chart.year_gan);
            chart.houses[idx].add_star(star, StarType::Main);
            chart.stars_house_index.insert(name.to_string(), idx);
        }
    }

    // ============================================================
    // 6. 安年干星
    // ============================================================

    /// 安年干星（天魁,天钺,禄存,擎羊,陀罗,天官,天福,天厨）
    fn setup_stars_by_year_gan(&self, chart: &mut ZiWeiChart) {
        let year_gan = &chart.year_gan.clone();
        let year_polar_val = chart.year_polar.clone();

        // 从JSON数据中读取年干星
        let year_gan_data = &DATA.year_gan;
        if let Some(obj) = year_gan_data.as_object() {
            for (star_name, star_def) in obj {
                let type_code = star_def.get("type").and_then(|v| v.as_i64()).unwrap_or(5) as i32;
                let star_type = StarType::from_code(type_code);

                if let Some(pos) = star_def.get("pos").and_then(|p| p.as_object()) {
                    if let Some(zhi_str) = pos.get(year_gan.as_str()).and_then(|v| v.as_str()) {
                        // 可能有两个字符（如"丑未"表示天魁天钺一对）
                        if zhi_str.chars().count() == 2 {
                            // 第一个字
                            let z1 = ganzi_first(zhi_str);
                            if let Some(idx) = branch_index(z1) {
                                let house_zi = ganzi_second(&chart.houses[idx].ganzi);
                                let house_pol = branch_polar(house_zi);
                                let sname = if house_pol != year_polar_val.as_str() {
                                    format!("副{}", star_name)
                                } else {
                                    star_name.clone()
                                };
                                let mut star = self.create_star(&sname, year_gan, house_zi);
                                self.setup_sihua_for_star(&mut star, year_gan);
                                chart.houses[idx].add_star(star, star_type);
                                chart.stars_house_index.insert(star_name.clone(), idx);
                            }
                            // 第二个字
                            let z2 = ganzi_second(zhi_str);
                            if let Some(idx) = branch_index(z2) {
                                let house_zi = ganzi_second(&chart.houses[idx].ganzi);
                                let mut star = self.create_star(star_name, year_gan, house_zi);
                                self.setup_sihua_for_star(&mut star, year_gan);
                                chart.houses[idx].add_star(star, star_type);
                                chart.stars_house_index.insert(star_name.clone(), idx);
                            }
                        } else {
                            let idx = branch_index(zhi_str).unwrap_or(0);
                            let house_zi = ganzi_second(&chart.houses[idx].ganzi);
                            let mut star = self.create_star(star_name, year_gan, house_zi);
                            self.setup_sihua_for_star(&mut star, year_gan);
                            chart.houses[idx].add_star(star, star_type);
                            chart.stars_house_index.insert(star_name.clone(), idx);
                        }
                    }
                }
            }
        }
    }

    // ============================================================
    // 7. 安年支星
    // ============================================================

    /// 安年支星（天马,天空,天哭,天虚,龙池,凤阁,红鸾,天喜,孤辰,寡宿等）
    fn setup_stars_by_year_zhi(&self, chart: &mut ZiWeiChart) {
        let year_zhi = &chart.year_zhi.clone();

        let year_zhi_data = &DATA.year_zhi;
        if let Some(obj) = year_zhi_data.as_object() {
            for (star_name, star_def) in obj {
                let type_code = star_def.get("type").and_then(|v| v.as_i64()).unwrap_or(5) as i32;
                let star_type = StarType::from_code(type_code);

                if let Some(pos) = star_def.get("pos").and_then(|p| p.as_object()) {
                    if let Some(zhi_str) = pos.get(year_zhi.as_str()).and_then(|v| v.as_str()) {
                        let idx = branch_index(zhi_str).unwrap_or(0);
                        let house_zi = ganzi_second(&chart.houses[idx].ganzi);
                        let mut star = self.create_star(star_name, &chart.year_gan, house_zi);
                        self.setup_sihua_for_star(&mut star, &chart.year_gan);
                        chart.houses[idx].add_star(star, star_type);
                        chart.stars_house_index.insert(star_name.clone(), idx);
                    }
                }
            }
        }
    }

    // ============================================================
    // 8. 安月星
    // ============================================================

    /// 安月星（左辅,右弼,天刑,天姚,解神,天巫,天月,阴煞,天马）
    fn setup_stars_by_month(&self, chart: &mut ZiWeiChart, month: &str) {
        let month_data = &DATA.month;
        if let Some(obj) = month_data.as_object() {
            for (star_name, star_def) in obj {
                let type_code = star_def.get("type").and_then(|v| v.as_i64()).unwrap_or(5) as i32;
                let star_type = StarType::from_code(type_code);

                if let Some(pos) = star_def.get("pos").and_then(|p| p.as_object()) {
                    if let Some(zhi_str) = pos.get(month).and_then(|v| v.as_str()) {
                        let idx = branch_index(zhi_str).unwrap_or(0);
                        let house_zi = ganzi_second(&chart.houses[idx].ganzi);
                        let mut star = self.create_star(star_name, &chart.year_gan, house_zi);
                        self.setup_sihua_for_star(&mut star, &chart.year_gan);
                        chart.houses[idx].add_star(star, star_type);
                        chart.stars_house_index.insert(star_name.clone(), idx);
                    }
                }
            }
        }
    }

    // ============================================================
    // 9. 安时星
    // ============================================================

    /// 安时星（文昌,文曲,地劫,地空,台辅,封诰）
    fn setup_stars_by_time_zhi(&self, chart: &mut ZiWeiChart) {
        let time_zhi = &chart.time_zhi.clone();

        let time_data = &DATA.time_zhi;
        if let Some(obj) = time_data.as_object() {
            for (star_name, star_def) in obj {
                let type_code = star_def.get("type").and_then(|v| v.as_i64()).unwrap_or(5) as i32;
                let star_type = StarType::from_code(type_code);

                if let Some(pos) = star_def.get("pos").and_then(|p| p.as_object()) {
                    if let Some(zhi_str) = pos.get(time_zhi.as_str()).and_then(|v| v.as_str()) {
                        let idx = branch_index(zhi_str).unwrap_or(0);
                        let house_zi = ganzi_second(&chart.houses[idx].ganzi);
                        let mut star = self.create_star(star_name, &chart.year_gan, house_zi);
                        self.setup_sihua_for_star(&mut star, &chart.year_gan);
                        chart.houses[idx].add_star(star, star_type);
                        chart.stars_house_index.insert(star_name.clone(), idx);
                    }
                }
            }
        }
    }

    // ============================================================
    // 10. 安火铃星
    // ============================================================

    /// 安火星、铃星（年支 + 时支）
    fn setup_stars_huo_lin(&self, chart: &mut ZiWeiChart) {
        let year_zhi = &chart.year_zhi.clone();
        let time_zhi = &chart.time_zhi.clone();
        let group = year_zhi_group(year_zhi);

        let huo_lin_data = &DATA.huo_lin;
        if let Some(group_data) = huo_lin_data.get(group) {
            if let Some(obj) = group_data.as_object() {
                for (star_name, star_data) in obj {
                    if let Some(pos) = star_data.get(time_zhi.as_str()).and_then(|v| v.as_str()) {
                        let idx = branch_index(pos).unwrap_or(0);
                        let house_zi = ganzi_second(&chart.houses[idx].ganzi);
                        let mut star = self.create_star(star_name, &chart.year_gan, house_zi);
                        self.setup_sihua_for_star(&mut star, &chart.year_gan);
                        chart.houses[idx].add_star(star, StarType::Evil);
                        chart.stars_house_index.insert(star_name.clone(), idx);
                    }
                }
            }
        }
    }

    // ============================================================
    // 11. 安日星
    // ============================================================

    /// 安日星（三台,八座,恩光,天贵）
    fn setup_stars_by_days(&self, chart: &mut ZiWeiChart, day: i32) {
        // 三台：从左辅宫起，顺数日数
        if let Some(&zuofu_idx) = chart.stars_house_index.get("左辅") {
            let idx = (zuofu_idx as i32 + day - 1) as usize % 12;
            let house_zi = ganzi_second(&chart.houses[idx].ganzi);
            let mut star = self.create_star("三台", &chart.year_gan, house_zi);
            self.setup_sihua_for_star(&mut star, &chart.year_gan);
            chart.houses[idx].add_star(star, StarType::OtherGood);
            chart.stars_house_index.insert("三台".to_string(), idx);
        }

        // 八座：从右弼宫起，逆数日数
        if let Some(&youbi_idx) = chart.stars_house_index.get("右弼") {
            let idx = ((youbi_idx as i32 - day + 1).rem_euclid(12)) as usize;
            let house_zi = ganzi_second(&chart.houses[idx].ganzi);
            let mut star = self.create_star("八座", &chart.year_gan, house_zi);
            self.setup_sihua_for_star(&mut star, &chart.year_gan);
            chart.houses[idx].add_star(star, StarType::OtherGood);
            chart.stars_house_index.insert("八座".to_string(), idx);
        }

        // 恩光：从文昌宫起，顺数日数
        if let Some(&wenchang_idx) = chart.stars_house_index.get("文昌") {
            let idx = (wenchang_idx as i32 + day - 1) as usize % 12;
            let house_zi = ganzi_second(&chart.houses[idx].ganzi);
            let mut star = self.create_star("恩光", &chart.year_gan, house_zi);
            self.setup_sihua_for_star(&mut star, &chart.year_gan);
            chart.houses[idx].add_star(star, StarType::OtherGood);
            chart.stars_house_index.insert("恩光".to_string(), idx);
        }

        // 天贵：从文曲宫起，顺数日数
        if let Some(&wenqu_idx) = chart.stars_house_index.get("文曲") {
            let idx = (wenqu_idx as i32 + day - 1) as usize % 12;
            let house_zi = ganzi_second(&chart.houses[idx].ganzi);
            let mut star = self.create_star("天贵", &chart.year_gan, house_zi);
            self.setup_sihua_for_star(&mut star, &chart.year_gan);
            chart.houses[idx].add_star(star, StarType::OtherGood);
            chart.stars_house_index.insert("天贵".to_string(), idx);
        }
    }

    // ============================================================
    // 12. 安旬空
    // ============================================================

    /// 安旬空
    fn setup_stars_xun_empty(&self, chart: &mut ZiWeiChart) {
        let year_ganzhi = format!("{}{}", chart.year_gan, chart.year_zhi);
        let empty_set = xun_empty_set(&year_ganzhi);
        let year_polar_val = chart.year_polar.clone();

        for i in 0..12 {
            let zi = ganzi_second(&chart.houses[i].ganzi);
            if empty_set.contains(&zi) {
                let house_pol = branch_polar(zi);
                let starname = if house_pol != year_polar_val.as_str() {
                    "副旬空".to_string()
                } else {
                    "旬空".to_string()
                };
                let star = self.create_star(&starname, &chart.year_gan, zi);
                chart.houses[i].add_star(star, StarType::OtherBad);
                chart.stars_house_index.insert("旬空".to_string(), i);
            }
        }
    }

    // ============================================================
    // 13. 安天才天寿
    // ============================================================

    /// 安天才、天寿
    fn setup_tian_cai_shou(&self, chart: &mut ZiWeiChart) {
        let year_zhi = &chart.year_zhi.clone();

        // 天才：年支对冲宫
        let idx = branch_index(year_zhi).unwrap_or(0);
        let tiancai_idx = if idx == 0 { 11 } else { 11 - idx };
        let house_zi = ganzi_second(&chart.houses[tiancai_idx].ganzi);
        let mut star = self.create_star("天才", &chart.year_gan, house_zi);
        self.setup_sihua_for_star(&mut star, &chart.year_gan);
        chart.houses[tiancai_idx].add_star(star, StarType::OtherGood);
        chart.stars_house_index.insert("天才".to_string(), tiancai_idx);

        // 天寿：身宫 + 年支
        let tianshou_idx = (chart.body_house_index + branch_index(year_zhi).unwrap_or(0)) % 12;
        let house_zi = ganzi_second(&chart.houses[tianshou_idx].ganzi);
        let mut star = self.create_star("天寿", &chart.year_gan, house_zi);
        self.setup_sihua_for_star(&mut star, &chart.year_gan);
        chart.houses[tianshou_idx].add_star(star, StarType::OtherGood);
        chart.stars_house_index.insert("天寿".to_string(), tianshou_idx);
    }

    // ============================================================
    // 14. 安博士十二神
    // ============================================================

    /// 安博士十二神（从禄存宫起，顺/逆时针取决于性别和年干阴阳）
    fn setup_stars_bosi(&self, chart: &mut ZiWeiChart) {
        let lucun_idx = match chart.stars_house_index.get("禄存") {
            Some(&idx) => idx,
            None => return,
        };

        let year_polar_val = chart.year_polar.clone();
        let is_clockwise = (chart.gender == Gender::Male && year_polar_val == "阳")
            || (chart.gender == Gender::Female && year_polar_val == "阴");

        for i in 0..12 {
            let star_name = BOSHI_12[i];
            let house_idx = if is_clockwise {
                (i + lucun_idx) % 12
            } else {
                (lucun_idx + 12 - i) % 12
            };
            let house_zi = ganzi_second(&chart.houses[house_idx].ganzi);
            let mut star = self.create_star(star_name, &chart.year_gan, house_zi);
            self.setup_sihua_for_star(&mut star, &chart.year_gan);
            chart.houses[house_idx].add_star(star, StarType::Small);
            chart.stars_house_index.insert(format!("Y{}", star_name), house_idx);
        }
    }

    // ============================================================
    // 15. 安岁前十二神
    // ============================================================

    /// 安岁前十二神（以年支宫为岁建，顺布十二神）
    fn setup_stars_tai_sui(&self, chart: &mut ZiWeiChart) {
        let year_idx = branch_index(&chart.year_zhi).unwrap_or(0);

        for i in 0..12 {
            let idx = (i + year_idx) % 12;
            let star_name = TAISUI_12[i];
            let house_zi = ganzi_second(&chart.houses[idx].ganzi);
            let mut star = self.create_star(star_name, &chart.year_gan, house_zi);
            self.setup_sihua_for_star(&mut star, &chart.year_gan);
            chart.houses[idx].add_star(star, StarType::Small);
            chart.stars_house_index.insert(format!("Y{}", star_name), idx);
        }
    }

    // ============================================================
    // 16. 安将星
    // ============================================================

    /// 安将星（将星,攀鞍,岁驿,息神,华盖,劫煞,灾煞,天煞,指背,咸池,月煞,亡神）
    fn setup_stars_jiang(&self, chart: &mut ZiWeiChart) {
        let group = year_zhi_group(&chart.year_zhi);

        let jiang_data = &DATA.jiang;
        if let Some(group_data) = jiang_data.get(group) {
            if let Some(obj) = group_data.as_object() {
                for (star_name, zhi_str) in obj {
                    if let Some(zhi) = zhi_str.as_str() {
                        let idx = branch_index(zhi).unwrap_or(0);
                        let house_zi = ganzi_second(&chart.houses[idx].ganzi);
                        let mut star = self.create_star(star_name, &chart.year_gan, house_zi);
                        self.setup_sihua_for_star(&mut star, &chart.year_gan);
                        chart.houses[idx].add_star(star, StarType::Small);
                        chart.stars_house_index.insert(format!("Y{}", star_name), idx);
                    }
                }
            }
        }
    }

    // ============================================================
    // 17. 安小限
    // ============================================================

    /// 安小限（男顺女逆，从生年支小限起始宫起）
    fn setup_small_direction(&self, chart: &mut ZiWeiChart) {
        for age in 0..100 {
            let idx = self.get_small_direction_house(age, &chart.year_zhi, chart.gender);
            chart.houses[idx].small_direction.push(age + 1);
        }
    }

    /// 计算小限宫位
    fn get_small_direction_house(&self, age: i32, year_zhi: &str, gender: Gender) -> usize {
        let idx = age % 12;
        let start_zhi = xiao_xian_start(year_zhi);
        let start_idx = branch_index(start_zhi).unwrap_or(0) as i32;

        if gender == Gender::Male {
            ((idx + start_idx) % 12) as usize
        } else {
            ((start_idx - idx + 12) % 12) as usize
        }
    }

    // ============================================================
    // 斗君
    // ============================================================

    /// 获取斗君（子斗）
    fn get_dou_jun(&self, month: &str, time_zhi: &str) -> String {
        let dou_data = &DATA.dou_jun;
        if let Some(month_data) = dou_data.get(month) {
            if let Some(zhi) = month_data.get(time_zhi).and_then(|v| v.as_str()) {
                return zhi.to_string();
            }
        }
        "子".to_string()
    }

    // ============================================================
    // 18. 定格局
    // ============================================================

    /// 检测格局
    fn detect_patterns(&self, chart: &ZiWeiChart) -> Vec<ZiWeiPattern> {
        let mut patterns = Vec::new();
        let life_idx = chart.life_house_index;

        // 三方四正：命宫、财帛(命-4)、官禄(命+4)、迁移(命+6)
        let trine: Vec<usize> = vec![
            life_idx,
            (life_idx + 8) % 12, // 命-4 = 命+8
            (life_idx + 4) % 12,
            (life_idx + 6) % 12,
        ];

        let ge_data = &DATA.ge;
        if let Some(rules) = ge_data.as_object() {
            for (name, rule) in rules {
                let category = rule.get("category").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let duanyi = rule.get("duanyi").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let source_ref = rule.get("source_ref").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let logic = rule.get("logic").and_then(|v| v.as_str()).unwrap_or("AND");

                let conditions = rule.get("conditions");
                let hit = self.eval_conditions(conditions, logic, chart, &trine);

                if hit {
                    let mut broken = false;
                    if let Some(breakers) = rule.get("breakers").and_then(|v| v.as_array()) {
                        for breaker in breakers {
                            if self.eval_single_condition(breaker, chart, &trine) {
                                broken = true;
                                break;
                            }
                        }
                    }
                    patterns.push(ZiWeiPattern {
                        name: name.clone(),
                        category,
                        duanyi,
                        source_ref,
                        broken,
                    });
                }
            }
        }

        patterns
    }

    fn eval_conditions(
        &self,
        conditions: Option<&serde_json::Value>,
        logic: &str,
        chart: &ZiWeiChart,
        trine: &[usize],
    ) -> bool {
        let conds = match conditions.and_then(|v| v.as_array()) {
            Some(c) => c,
            None => return false,
        };

        if logic == "OR" {
            conds.iter().any(|c| self.eval_single_condition(c, chart, trine))
        } else {
            conds.iter().all(|c| self.eval_single_condition(c, chart, trine))
        }
    }

    fn eval_single_condition(
        &self,
        cond: &serde_json::Value,
        chart: &ZiWeiChart,
        trine: &[usize],
    ) -> bool {
        let obj = match cond.as_object() {
            Some(o) => o,
            None => return false,
        };

        let op = obj.get("op").and_then(|v| v.as_str()).unwrap_or("");

        match op {
            "and" => {
                let conds = obj.get("conditions");
                self.eval_conditions(conds, "AND", chart, trine)
            }
            "or" => {
                let conds = obj.get("conditions");
                self.eval_conditions(conds, "OR", chart, trine)
            }
            "inMing" => {
                let star = obj.get("star").and_then(|v| v.as_str()).unwrap_or("");
                self.star_index(chart, star) == Some(chart.life_house_index)
            }
            "inTrine" => {
                let star = obj.get("star").and_then(|v| v.as_str()).unwrap_or("");
                match self.star_index(chart, star) {
                    Some(idx) => trine.contains(&idx),
                    None => false,
                }
            }
            "inTrineAny" => {
                let stars = self.get_str_array(obj, "stars");
                let at_least = obj.get("atLeast").and_then(|v| v.as_i64()).unwrap_or(1) as usize;
                let count = stars.iter().filter(|s| {
                    match self.star_index(chart, s) {
                        Some(idx) => trine.contains(&idx),
                        None => false,
                    }
                }).count();
                count >= at_least
            }
            "same" => {
                let stars = self.get_str_array(obj, "stars");
                if stars.is_empty() { return false; }
                let first = match self.star_index(chart, &stars[0]) {
                    Some(i) => i,
                    None => return false,
                };
                stars.iter().all(|s| self.star_index(chart, s) == Some(first))
            }
            "sameAnyOf" => {
                let star = obj.get("star").and_then(|v| v.as_str()).unwrap_or("");
                let base = match self.star_index(chart, star) {
                    Some(i) => i,
                    None => return false,
                };
                let others = self.get_str_array(obj, "others");
                others.iter().any(|s| self.star_index(chart, s) == Some(base))
            }
            "mingZhi" => {
                let branches = self.get_str_array(obj, "branches");
                let life_zhi = ganzi_second(&chart.houses[chart.life_house_index].ganzi);
                branches.contains(&life_zhi.to_string())
            }
            "inZhi" => {
                let star = obj.get("star").and_then(|v| v.as_str()).unwrap_or("");
                let branches = self.get_str_array(obj, "branches");
                match self.star_index(chart, star) {
                    Some(idx) => {
                        let zhi = ganzi_second(&chart.houses[idx].ganzi);
                        branches.contains(&zhi.to_string())
                    }
                    None => false,
                }
            }
            "sandwichMing" => {
                let stars = self.get_str_array(obj, "stars");
                if stars.len() != 2 { return false; }
                let left = (chart.life_house_index + 11) % 12;
                let right = (chart.life_house_index + 1) % 12;
                let a = self.star_index(chart, &stars[0]);
                let b = self.star_index(chart, &stars[1]);
                (a == Some(left) && b == Some(right)) || (a == Some(right) && b == Some(left))
            }
            "sandwichStarMix" => {
                let target = obj.get("target").and_then(|v| v.as_str()).unwrap_or("");
                let star = obj.get("star").and_then(|v| v.as_str()).unwrap_or("");
                let hua = obj.get("hua").and_then(|v| v.as_str()).unwrap_or("");
                let t = match self.star_index(chart, target) {
                    Some(i) => i,
                    None => return false,
                };
                let left = (t + 11) % 12;
                let right = (t + 1) % 12;
                let si = self.star_index(chart, star);
                let hi = self.hua_house_index(chart, hua);
                (si == Some(left) && hi == Some(right)) || (si == Some(right) && hi == Some(left))
            }
            "bright" => {
                let star = obj.get("star").and_then(|v| v.as_str()).unwrap_or("");
                let levels = self.get_str_array(obj, "levels");
                match self.star_index(chart, star) {
                    Some(idx) => {
                        let zhi = ganzi_second(&chart.houses[idx].ganzi);
                        let light = self.get_starlight(star, zhi);
                        match light {
                            Some(l) => levels.contains(&l.to_string()),
                            None => false,
                        }
                    }
                    None => false,
                }
            }
            "mingNoMainStar" => {
                chart.houses[chart.life_house_index].stars_main.is_empty()
            }
            "huaMing" => {
                let hua = obj.get("hua").and_then(|v| v.as_str()).unwrap_or("");
                self.hua_house_index(chart, hua) == Some(chart.life_house_index)
            }
            "huaTrineAll" => {
                let huas = self.get_str_array(obj, "huas");
                huas.iter().all(|h| {
                    match self.hua_house_index(chart, h) {
                        Some(idx) => trine.contains(&idx),
                        None => false,
                    }
                })
            }
            "huaWithStar" => {
                let star = obj.get("star").and_then(|v| v.as_str()).unwrap_or("");
                let hua = obj.get("hua").and_then(|v| v.as_str()).unwrap_or("");
                self.star_index(chart, star) == self.hua_house_index(chart, hua)
            }
            "breakBy" => {
                let stars = self.get_str_array(obj, "stars");
                let qian = (chart.life_house_index + 6) % 12;
                stars.iter().any(|s| {
                    match self.star_index(chart, s) {
                        Some(idx) => idx == chart.life_house_index || idx == qian,
                        None => false,
                    }
                })
            }
            _ => false,
        }
    }

    fn get_str_array(&self, obj: &serde_json::Map<String, serde_json::Value>, key: &str) -> Vec<String> {
        match obj.get(key) {
            Some(arr) => arr.as_array().map(|a| {
                a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
            }).unwrap_or_default(),
            None => Vec::new(),
        }
    }

    fn star_index(&self, chart: &ZiWeiChart, star: &str) -> Option<usize> {
        chart.stars_house_index.get(star).copied()
    }

    fn hua_house_index(&self, chart: &ZiWeiChart, hua: &str) -> Option<usize> {
        let stars = gan_sihua_stars(&chart.year_gan);
        let hua_idx = match hua {
            "禄" => 0, "权" => 1, "科" => 2, "忌" => 3,
            _ => return None,
        };
        if hua_idx < stars.len() {
            self.star_index(chart, stars[hua_idx])
        } else {
            None
        }
    }

    // ============================================================
    // 辅助方法
    // ============================================================

    /// 创建星曜
    fn create_star(&self, name: &str, gan: &str, zi: &str) -> ZiWeiStar {
        let mut star = ZiWeiStar::new(name);
        let light = self.get_starlight(name, zi);
        star.starlight = light;
        star
    }

    /// 获取星曜亮度
    fn get_starlight(&self, star: &str, zi: &str) -> Option<String> {
        let light_data = &DATA.starlight;
        light_data
            .get(star)
            .and_then(|s| s.get(zi))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// 设置星曜四化
    fn setup_sihua_for_star(&self, star: &mut ZiWeiStar, gan: &str) {
        // 从JSON数据获取四化
        let sihua_data = &DATA.sihua;
        if let Some(gan_data) = sihua_data.get("gan").and_then(|g| g.get(gan)) {
            if let Some(arr) = gan_data.as_array() {
                let hua_names = ["禄", "权", "科", "忌"];
                for (i, hua) in hua_names.iter().enumerate() {
                    if let Some(s) = arr.get(i).and_then(|v| v.as_str()) {
                        if s == star.name {
                            star.sihua = Some(hua.to_string());
                        }
                    }
                }
            }
        }
    }

    // ============================================================
    // 流年/流月/小限计算
    // ============================================================

    /// 计算大限列表
    pub fn calc_da_xian(&self, chart: &ZiWeiChart) -> Vec<DaXian> {
        let mut result = Vec::new();
        for i in 0..12 {
            let house = &chart.houses[i];
            if house.direction.len() >= 2 {
                result.push(DaXian {
                    house_index: i,
                    house_name: house.name.clone(),
                    ganzi: house.ganzi.clone(),
                    gan: ganzi_first(&house.ganzi).to_string(),
                    start_age: house.direction[0],
                    end_age: house.direction[1],
                    sihua: self.calc_sihua_for_gan(ganzi_first(&house.ganzi), chart),
                });
            }
        }
        result
    }

    /// 计算流年
    pub fn calc_liu_nian(&self, chart: &ZiWeiChart, year: i32) -> LiuNianInfo {
        let year_ganzi = self.simple_day_ganzhi(year, 1, 1);
        // 更准确地：使用年干支
        let year_gan = STEMS[((year - 4).rem_euclid(10)) as usize].to_string();
        let year_zhi = BRANCHES[((year - 4).rem_euclid(12)) as usize].to_string();
        let ganzi = format!("{}{}", year_gan, year_zhi);

        let ming = branch_index(&year_zhi).unwrap_or(0);
        let sihua = self.calc_sihua_for_gan(&year_gan, chart);
        let flow_stars = self.calc_flow_stars(&year_gan, &year_zhi);
        let doujun_idx = (branch_index(&chart.zidou).unwrap_or(0) + branch_index(&year_zhi).unwrap_or(0)) % 12;

        LiuNianInfo {
            year,
            ganzi,
            gan: year_gan,
            ming_zhi_index: ming,
            sihua,
            flow_stars,
            doujun_index: doujun_idx,
        }
    }

    /// 计算流曜
    fn calc_flow_stars(&self, gan: &str, zhi: &str) -> Vec<FlowStar> {
        let mut stars = Vec::new();

        // 流禄
        stars.push(FlowStar {
            name: "流禄".to_string(),
            zhi_index: lucun_branch(gan) as i32,
        });
        // 流羊
        stars.push(FlowStar {
            name: "流羊".to_string(),
            zhi_index: qingyang_branch(gan) as i32,
        });
        // 流陀
        stars.push(FlowStar {
            name: "流陀".to_string(),
            zhi_index: tuoluo_branch(gan) as i32,
        });
        // 流魁
        let (kui, _) = tiankui_tianyue_branch(gan);
        stars.push(FlowStar {
            name: "流魁".to_string(),
            zhi_index: kui as i32,
        });
        // 流钺
        let (_, yue) = tiankui_tianyue_branch(gan);
        stars.push(FlowStar {
            name: "流钺".to_string(),
            zhi_index: yue as i32,
        });
        // 流马
        stars.push(FlowStar {
            name: "流马".to_string(),
            zhi_index: tianma_branch(zhi) as i32,
        });

        // 流昌/流曲
        let liu_data = &DATA.liu_chang_qu;
        if let Some(liu_chang) = liu_data.get("流昌").and_then(|v| v.as_object()) {
            if let Some(z) = liu_chang.get(gan).and_then(|v| v.as_str()) {
                stars.push(FlowStar {
                    name: "流昌".to_string(),
                    zhi_index: branch_index(z).unwrap_or(0) as i32,
                });
            }
        }
        if let Some(liu_qu) = liu_data.get("流曲").and_then(|v| v.as_object()) {
            if let Some(z) = liu_qu.get(gan).and_then(|v| v.as_str()) {
                stars.push(FlowStar {
                    name: "流曲".to_string(),
                    zhi_index: branch_index(z).unwrap_or(0) as i32,
                });
            }
        }

        stars
    }

    /// 计算小限
    pub fn calc_xiao_xian(&self, chart: &ZiWeiChart, age: i32) -> XiaoXianInfo {
        let idx = self.get_small_direction_house(age - 1, &chart.year_zhi, chart.gender);
        let ganzi = chart.houses[idx].ganzi.clone();
        let gan = ganzi_first(&ganzi).to_string();
        let sihua = self.calc_sihua_for_gan(&gan, chart);

        XiaoXianInfo {
            age,
            ganzi,
            gan,
            ming_zhi_index: idx,
            sihua,
        }
    }

    /// 计算某个天干对应的四化
    fn calc_sihua_for_gan(&self, gan: &str, chart: &ZiWeiChart) -> Vec<SiHuaItem> {
        let stars = gan_sihua_stars(gan);
        let hua_names = ["禄", "权", "科", "忌"];
        let mut result = Vec::new();

        for i in 0..4 {
            let star = stars[i];
            let hua = hua_names[i];
            let zhi_index = chart.stars_house_index.get(star).copied().map(|v| v as i32).unwrap_or(-1);
            result.push(SiHuaItem {
                star: star.to_string(),
                hua: hua.to_string(),
                zhi_index,
            });
        }

        result
    }
}