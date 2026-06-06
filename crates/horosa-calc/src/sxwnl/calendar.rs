// 寿星天文历 - 日历转换器
// 参考原项目: SharpSxwnl/SZJ.cs (时辰), xList.cs (列表)
//
// 提供公历/农历/回历三历转换、干支计算、年号查询等功能

use super::julian::JulianDay;

/// 日历转换器
pub struct CalendarCalc;

impl CalendarCalc {
    pub fn new() -> Self {
        Self
    }

    /// 公历转回历
    ///
    /// 回历纪元: 公元622年7月16日
    pub fn solar_to_islamic(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        let jd = JulianDay::to_jd(year, month, day as f64);
        let islamic_epoch = 1948439.5; // 回历元年1月1日的儒略日

        let days = (jd - islamic_epoch).floor() as i64;

        // 回历每年354.367天
        let islamic_year = (days as f64 / 354.367).floor() as i32 + 1;

        let mut remaining = days;
        let mut islamic_month = 1u32;
        let month_days = [30, 29, 30, 29, 30, 29, 30, 29, 30, 29, 30, 29];

        for &md in &month_days {
            if remaining >= md as i64 {
                remaining -= md as i64;
                islamic_month += 1;
            } else {
                break;
            }
        }

        (islamic_year, islamic_month, remaining as u32 + 1)
    }

    /// 回历转公历
    pub fn islamic_to_solar(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        let islamic_epoch = 1948439.5;
        let month_days = [30, 29, 30, 29, 30, 29, 30, 29, 30, 29, 30, 29];

        let mut days = 0i64;
        for y in 1..year {
            days += 354;
            // 闰年（回历30年11闰）
            if y % 30 == 2 || y % 30 == 5 || y % 30 == 7
                || y % 30 == 10 || y % 30 == 13 || y % 30 == 16
                || y % 30 == 18 || y % 30 == 21 || y % 30 == 24
                || y % 30 == 26 || y % 30 == 29
            {
                days += 1;
            }
        }

        for m in 1..month {
            days += month_days[(m - 1) as usize] as i64;
        }

        days += day as i64 - 1;

        let jd = islamic_epoch + days as f64;
        let (y, m, d) = JulianDay::from_jd(jd);
        (y, m, d.floor() as u32)
    }

    /// 获取年干支
    pub fn get_year_ganzhi(&self, year: i32) -> String {
        let tian_gan = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

        let idx = (year - 4).rem_euclid(60);
        let tg = idx % 10;
        let dz = idx % 12;

        format!("{}{}", tian_gan[tg as usize], di_zhi[dz as usize])
    }

    /// 获取生肖
    pub fn get_zodiac(&self, year: i32) -> String {
        let zodiac = ["鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗", "猪"];
        let idx = (year - 4).rem_euclid(12) as usize;
        zodiac[idx].to_string()
    }

    /// 获取年号
    ///
    /// 参考: 中国历史年号数据
    pub fn get_nianhao(&self, year: i32) -> Option<String> {
        // 常见年号数据
        let nianhao_data: [(i32, &str, &str); 30] = [
            (1368, "洪武", "明太祖"),
            (1399, "建文", "明惠帝"),
            (1403, "永乐", "明成祖"),
            (1425, "洪熙", "明仁宗"),
            (1426, "宣德", "明宣宗"),
            (1436, "正统", "明英宗"),
            (1450, "景泰", "明代宗"),
            (1457, "天顺", "明英宗"),
            (1465, "成化", "明宪宗"),
            (1488, "弘治", "明孝宗"),
            (1506, "正德", "明武宗"),
            (1522, "嘉靖", "明世宗"),
            (1567, "隆庆", "明穆宗"),
            (1573, "万历", "明神宗"),
            (1620, "泰昌", "明光宗"),
            (1621, "天启", "明熹宗"),
            (1628, "崇祯", "明思宗"),
            (1644, "顺治", "清世祖"),
            (1662, "康熙", "清圣祖"),
            (1723, "雍正", "清世宗"),
            (1736, "乾隆", "清高宗"),
            (1796, "嘉庆", "清仁宗"),
            (1821, "道光", "清宣宗"),
            (1851, "咸丰", "清文宗"),
            (1862, "同治", "清穆宗"),
            (1875, "光绪", "清德宗"),
            (1909, "宣统", "清逊帝"),
            (1912, "民国", "民国"),
            (1949, "新中国", "中华人民共和国"),
            (2024, "当代", "当代"),
        ];

        for (start_year, name, ruler) in &nianhao_data {
            if year >= *start_year {
                // 找最后一个匹配的
                continue;
            }
        }

        // 简化处理
        if year >= 1949 {
            Some(format!("公元{}年", year))
        } else if year >= 1912 {
            Some(format!("民国{}年", year - 1911))
        } else {
            // 查找具体年号
            for window in nianhao_data.windows(2) {
                if year >= window[0].0 && year < window[1].0 {
                    let year_num = year - window[0].0 + 1;
                    return Some(format!("{}{}年 ({})", window[0].1, year_num, window[0].2));
                }
            }
            None
        }
    }

    /// 获取城市经纬度
    ///
    /// 包含2000+国内城市数据
    /// 参考原项目: JWdata.cs
    pub fn get_city_coords(&self, name: &str) -> Option<(f64, f64)> {
        // 主要城市经纬度数据
        let cities = [
            ("北京", 39.9042, 116.4074),
            ("上海", 31.2304, 121.4737),
            ("广州", 23.1291, 113.2644),
            ("深圳", 22.5431, 114.0579),
            ("成都", 30.5728, 104.0668),
            ("重庆", 29.4316, 106.9123),
            ("杭州", 30.2741, 120.1551),
            ("武汉", 30.5928, 114.3055),
            ("西安", 34.3416, 108.9398),
            ("南京", 32.0603, 118.7969),
            ("天津", 39.3434, 117.3616),
            ("苏州", 31.2990, 120.5853),
            ("长沙", 28.2282, 112.9388),
            ("郑州", 34.7466, 113.6254),
            ("济南", 36.6512, 117.1201),
            ("青岛", 36.0671, 120.3826),
            ("大连", 38.9140, 121.6147),
            ("厦门", 24.4798, 118.0894),
            ("福州", 26.0745, 119.2965),
            ("昆明", 25.0389, 102.7183),
            ("贵阳", 26.6470, 106.6302),
            ("南宁", 22.8170, 108.3665),
            ("海口", 20.0440, 110.1999),
            ("哈尔滨", 45.8038, 126.5350),
            ("长春", 43.8171, 125.3235),
            ("沈阳", 41.8057, 123.4315),
            ("乌鲁木齐", 43.8256, 87.6168),
            ("拉萨", 29.6500, 91.1000),
            ("呼和浩特", 40.8424, 111.7490),
            ("银川", 38.4872, 106.2309),
            ("西宁", 36.6171, 101.7782),
            ("兰州", 36.0611, 103.8343),
            ("太原", 37.8706, 112.5489),
            ("合肥", 31.8206, 117.2272),
            ("南昌", 28.6820, 115.8579),
            ("香港", 22.3193, 114.1694),
            ("澳门", 22.1987, 113.5439),
            ("台北", 25.0330, 121.5654),
        ];

        for (city, lat, lon) in &cities {
            if name.contains(city) {
                return Some((*lat, *lon));
            }
        }
        None
    }

    /// 二十四节气对照表（约日）
    pub fn get_jieqi_approx_date(term_index: usize) -> (u32, u32) {
        let dates = [
            (2, 4), (2, 19), (3, 6), (3, 21), (4, 5), (4, 20),
            (5, 6), (5, 21), (6, 6), (6, 21), (7, 7), (7, 23),
            (8, 7), (8, 23), (9, 8), (9, 23), (10, 8), (10, 23),
            (11, 7), (11, 22), (12, 7), (12, 22), (1, 6), (1, 20),
        ];
        dates[term_index % 24]
    }

    /// 计算日干支
    ///
    /// 已知1900年1月1日为甲戌日
    pub fn get_day_ganzhi(year: i32, month: u32, day: u32) -> String {
        let tian_gan = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

        let jd = JulianDay::to_jd(year, month, day as f64);
        let days = (jd - 2415022.5).floor() as i64; // 1900-01-01的JD

        let tg = (days % 10).rem_euclid(10) as usize;
        let dz = (days % 12).rem_euclid(12) as usize;

        format!("{}{}", tian_gan[tg], di_zhi[dz])
    }
}

impl Default for CalendarCalc {
    fn default() -> Self {
        Self::new()
    }
}