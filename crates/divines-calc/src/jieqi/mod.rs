// Divines - 节气/农历计算引擎
// 参考原项目: astropy/astrostudy/jieqi/, vendor/kinastro/ 中的节气计算

use divines_core::*;

/// 节气/农历计算器
pub struct JieQiCalc;

impl JieQiCalc {
    /// 获取全年节气列表
    /// 参考原项目: YearJieQi.py
    pub fn get_year_jieqi(&self, year: i32) -> Vec<JieQi> {
        let mut jieqi_list = Vec::new();
        let terms = [
            SolarTerm::LiChun, SolarTerm::YuShui, SolarTerm::JingZhe,
            SolarTerm::ChunFen, SolarTerm::QingMing, SolarTerm::GuYu,
            SolarTerm::LiXia, SolarTerm::XiaoMan, SolarTerm::MangZhong,
            SolarTerm::XiaZhi, SolarTerm::XiaoShu, SolarTerm::DaShu,
            SolarTerm::LiQiu, SolarTerm::ChuShu, SolarTerm::BaiLu,
            SolarTerm::QiuFen, SolarTerm::HanLu, SolarTerm::ShuangJiang,
            SolarTerm::LiDong, SolarTerm::XiaoXue, SolarTerm::DaXue,
            SolarTerm::DongZhi, SolarTerm::XiaoHan, SolarTerm::DaHan,
        ];

        // 简化的节气日期计算
        let base_dates = [
            (2, 4), (2, 19), (3, 6), (3, 21), (4, 5), (4, 20),
            (5, 6), (5, 21), (6, 6), (6, 21), (7, 7), (7, 23),
            (8, 7), (8, 23), (9, 8), (9, 23), (10, 8), (10, 23),
            (11, 7), (11, 22), (12, 7), (12, 22), (1, 6), (1, 20),
        ];

        for (i, term) in terms.iter().enumerate() {
            let (month, day) = base_dates[i];
            let year_adj = if month == 1 && i >= 22 { year + 1 } else { year };

            jieqi_list.push(JieQi {
                name: format!("{:?}", term),
                name_zh: term.name_zh().to_string(),
                datetime: format!("{}-{:02}-{:02}T00:00:00", year_adj, month, day),
                solar_longitude: (i * 15) as f64,
                is_jie: i % 2 == 0,
            });
        }

        jieqi_list
    }

    /// 公历转农历
    /// 参考原项目: NongLi.py
    pub fn solar_to_lunar(&self, year: i32, month: u32, day: u32) -> LunarDate {
        // 简化农历转换
        let tian_gan = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
        let zodiac = ["鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗", "猪"];

        let year_idx = (year - 4).rem_euclid(60);
        let tg_idx = (year_idx % 10) as usize;
        let dz_idx = (year_idx % 12) as usize;

        LunarDate {
            year,
            month: month as u8,
            is_leap_month: false,
            day: day as u8,
            year_ganzhi: format!("{}{}", tian_gan[tg_idx], di_zhi[dz_idx]),
            month_ganzhi: "".to_string(),
            day_ganzhi: "".to_string(),
            zodiac_animal: zodiac[dz_idx].to_string(),
            month_name_zh: format!("{}月", month),
            day_name_zh: format!("{}日", day),
        }
    }

    /// 获取黄历信息
    pub fn get_almanac(&self, year: i32, month: u32, day: u32) -> Almanac {
        let lunar = self.solar_to_lunar(year, month, day);
        let jieqi = self.get_year_jieqi(year);

        Almanac {
            solar_date: format!("{}-{:02}-{:02}", year, month, day),
            lunar_date: lunar,
            jie_qi: jieqi,
            yi: vec!["祭祀".to_string(), "祈福".to_string(), "出行".to_string()],
            ji: vec!["动土".to_string(), "破土".to_string(), "安葬".to_string()],
            chong_sha: "冲煞".to_string(),
            ji_shen_fang_wei: FangWei {
                xi_shen: "东北".to_string(),
                cai_shen: "正北".to_string(),
                fu_shen: "正南".to_string(),
                yang_gui: "西南".to_string(),
                yin_gui: "正东".to_string(),
            },
            tai_shen: "胎神".to_string(),
            peng_zu_bai_ji: "彭祖百忌".to_string(),
        }
    }

    /// 判断是否为闰年
    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }
}

impl Default for JieQiCalc {
    fn default() -> Self {
        Self
    }
}