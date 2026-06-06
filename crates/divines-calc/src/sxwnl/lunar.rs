// 寿星天文历 - 农历转换
// 参考原项目: SharpSxwnl/Lunar.cs, LunarHelper.cs
//
// 公历与农历之间的相互转换
// 农历数据范围: -3000年至+3000年

use super::julian::JulianDay;
use super::syzygy::SyzygyCalc;
use divines_core::*;

/// 农历转换器
pub struct LunarCalc {
    syzygy: SyzygyCalc,
}

impl LunarCalc {
    pub fn new() -> Self {
        Self {
            syzygy: SyzygyCalc::new(),
        }
    }

    /// 公历转农历
    ///
    /// 返回农历日期
    /// 参考原项目: LunarHelper.solarToLunar()
    pub fn solar_to_lunar(&self, year: i32, month: u32, day: u32) -> LunarDate {
        let jd = JulianDay::to_jd(year, month, day as f64);

        // 获取该年和上一年的朔日列表
        let shuo_prev = self.syzygy.calc_year_shuo(year - 1);
        let shuo_curr = self.syzygy.calc_year_shuo(year);
        let shuo_next = self.syzygy.calc_year_shuo(year + 1);

        let mut all_shuo: Vec<f64> = Vec::new();
        all_shuo.extend_from_slice(&shuo_prev);
        all_shuo.extend_from_slice(&shuo_curr);
        all_shuo.extend_from_slice(&shuo_next);
        all_shuo.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // 找到当前日期所在的农历月
        let mut lunar_year = year;
        let mut lunar_month = 1u32;
        let mut is_leap = false;
        let mut lunar_day = 1u32;

        // 找到当前日期之前的最近朔日
        let mut shuo_idx = 0;
        for (i, &s) in all_shuo.iter().enumerate() {
            if s <= jd && (i + 1 >= all_shuo.len() || all_shuo[i + 1] > jd) {
                shuo_idx = i;
                lunar_day = ((jd - s).floor() + 1.0) as u32;
                break;
            }
        }

        // 计算农历月
        // 通过节气确定月序
        let jieqi = self.syzygy.calc_year_jieqi(year);

        // 计算包含冬至的月为农历十一月
        // 简化处理：根据朔日序号推算月序
        if shuo_idx >= 1 {
            let mut month_count = 0u32;
            let mut current_jd = all_shuo[0];

            for i in 1..=shuo_idx {
                if i >= all_shuo.len() {
                    break;
                }
                let (y, _, _) = JulianDay::from_jd(all_shuo[i]);
                if y == year || (y == year - 1 && month_count == 0) {
                    month_count += 1;
                }
                current_jd = all_shuo[i];
            }

            lunar_month = ((month_count - 1) % 12 + 1) as u32;
            if lunar_month == 0 {
                lunar_month = 12;
            }
        }

        // 年干支
        let year_idx = (year - 4).rem_euclid(60);
        let tg_idx = (year_idx % 10) as usize;
        let dz_idx = (year_idx % 12) as usize;
        let tian_gan = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
        let zodiac = ["鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗", "猪"];

        let month_names = [
            "正", "二", "三", "四", "五", "六",
            "七", "八", "九", "十", "冬", "腊",
        ];

        LunarDate {
            year: lunar_year,
            month: lunar_month as u8,
            is_leap_month: is_leap,
            day: lunar_day as u8,
            year_ganzhi: format!("{}{}", tian_gan[tg_idx], di_zhi[dz_idx]),
            month_ganzhi: "".to_string(),
            day_ganzhi: "".to_string(),
            zodiac_animal: zodiac[dz_idx].to_string(),
            month_name_zh: format!("{}月", month_names[(lunar_month as usize - 1) % 12]),
            day_name_zh: format!("{}", lunar_day),
        }
    }

    /// 农历转公历
    ///
    /// 返回 (公历年, 公历月, 公历日)
    /// 参考原项目: LunarHelper.lunarToSolar()
    pub fn lunar_to_solar(
        &self,
        lunar_year: i32,
        lunar_month: u32,
        lunar_day: u32,
        _is_leap: bool,
    ) -> (i32, u32, u32) {
        // 获取该农历年的所有朔日
        let shuo_list = self.syzygy.calc_year_shuo(lunar_year);

        if lunar_month as usize > shuo_list.len() {
            // 超出范围，返回近似值
            return (lunar_year, lunar_month, lunar_day);
        }

        let shuo_jd = shuo_list[(lunar_month - 1) as usize];

        // 农历初一对应的公历日期
        let jd = shuo_jd + lunar_day as f64 - 1.0;
        let (y, m, d) = JulianDay::from_jd(jd);

        (y, m, d.floor() as u32)
    }

    /// 判断农历年是否有闰月
    ///
    /// 返回闰月月份（0表示无闰月）
    pub fn get_leap_month(&self, year: i32) -> u8 {
        let shuo_list = self.syzygy.calc_year_shuo(year);
        let jieqi = self.syzygy.calc_year_jieqi(year);

        // 正常情况下一年12个月，如果朔日有13个则存在闰月
        if shuo_list.len() <= 12 {
            return 0;
        }

        // 找到没有中气的月作为闰月
        // 简化处理：返回0表示无闰月，实际需要详细计算
        for i in 1..shuo_list.len() {
            let shuo_jd = shuo_list[i];
            let next_shuo = if i + 1 < shuo_list.len() {
                shuo_list[i + 1]
            } else {
                shuo_list[i] + 30.0
            };

            let mut has_zhongqi = false;
            for jq in &jieqi {
                let jq_date = self.parse_jq_date(&jq.datetime);
                let jq_jd = JulianDay::to_jd(jq_date.0, jq_date.1, jq_date.2 as f64);
                if jq_jd >= shuo_jd && jq_jd < next_shuo && !jq.is_jie {
                    has_zhongqi = true;
                    break;
                }
            }

            if !has_zhongqi {
                return i as u8;
            }
        }

        0
    }

    fn parse_jq_date(&self, dt: &str) -> (i32, u32, u32) {
        let parts: Vec<&str> = dt.split('T').collect();
        if parts.is_empty() {
            return (2000, 1, 1);
        }
        let date_parts: Vec<&str> = parts[0].split('-').collect();
        if date_parts.len() < 3 {
            return (2000, 1, 1);
        }
        (
            date_parts[0].parse().unwrap_or(2000),
            date_parts[1].parse().unwrap_or(1),
            date_parts[2].parse().unwrap_or(1),
        )
    }
}

impl Default for LunarCalc {
    fn default() -> Self {
        Self::new()
    }
}