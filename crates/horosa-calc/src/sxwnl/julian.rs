// 寿星天文历 - 儒略日计算
// 参考原项目: SharpSxwnl/JD.cs
//
// 提供儒略日(JD)与公历日期的相互转换
// 包含 TD(地球力学时) - UT1(世界时) 的 ΔT 修正

/// 儒略日计算器
pub struct JulianDay;

impl JulianDay {
    pub fn new() -> Self {
        Self
    }

    /// 公历转儒略日
    ///
    /// 支持公元前4712年到公元9999年
    /// 算法参考: Meeus《天文算法》
    pub fn to_jd(year: i32, month: u32, day: f64) -> f64 {
        let (y, m) = if month <= 2 {
            (year as f64 - 1.0, month as f64 + 12.0)
        } else {
            (year as f64, month as f64)
        };

        let a = (y / 100.0).floor();
        let b = 2.0 - a + (a / 4.0).floor();

        b + (365.25 * (y + 4716.0)).floor()
            + (30.6001 * (m + 1.0)).floor()
            + day
            - 1524.5
    }

    /// 儒略日转公历
    ///
    /// 返回 (年, 月, 日)
    pub fn from_jd(jd: f64) -> (i32, u32, f64) {
        let jd = jd + 0.5;
        let z = jd.floor();
        let f = jd - z;

        let a = if z < 2299161.0 {
            z
        } else {
            let alpha = ((z - 1867216.25) / 36524.25).floor();
            z + 1.0 + alpha - (alpha / 4.0).floor()
        };

        let b = a + 1524.0;
        let c = ((b - 122.1) / 365.25).floor();
        let d = (365.25 * c).floor();
        let e = ((b - d) / 30.6001).floor();

        let day = b - d - (30.6001 * e).floor() + f;
        let month = if e < 14.0 { e - 1.0 } else { e - 13.0 };
        let year = if month > 2.0 { c - 4716.0 } else { c - 4715.0 };

        (year as i32, month as u32, day)
    }

    /// 计算 ΔT (TD - UT1)
    ///
    /// TD = UT + ΔT
    /// 参考: NASA 多项式拟合公式
    /// 精度: 公元前4000年到公元3000年
    pub fn delta_t(year: i32, month: u32) -> f64 {
        let y = year as f64 + (month as f64 - 0.5) / 12.0;

        if year < -500 {
            // 公元前500年以前
            let u = (y - 1820.0) / 100.0;
            -20.0 + 32.0 * u * u
        } else if year < 500 {
            // 公元前500年到公元500年
            let u = y / 100.0;
            let dt_tab = [
                10583.6, -1014.41, 33.78311, -5.952053,
                -0.1798452, -0.022174192, 0.0090316521,
            ];
            let mut dt = 0.0;
            for (i, &c) in dt_tab.iter().enumerate() {
                dt += c * u.powi(i as i32);
            }
            dt
        } else if year < 1600 {
            // 500-1600年
            let u = (y - 1000.0) / 100.0;
            let dt_tab = [
                1574.2, -556.01, 71.23472, 0.319781,
                -0.8503463, -0.005050998, 0.0083572073,
            ];
            let mut dt = 0.0;
            for (i, &c) in dt_tab.iter().enumerate() {
                dt += c * u.powi(i as i32);
            }
            dt
        } else if year < 1700 {
            // 1600-1700年
            let t = y - 1600.0;
            let dt_tab = [120.0, -0.9808, -0.01532, 0.000140272128];
            let mut dt = 0.0;
            for (i, &c) in dt_tab.iter().enumerate() {
                dt += c * t.powi(i as i32);
            }
            dt
        } else if year < 1800 {
            // 1700-1800年
            let t = y - 1700.0;
            let dt_tab = [8.83, 0.1603, -0.0059285, 0.00013336, -0.000000011999];
            let mut dt = 0.0;
            for (i, &c) in dt_tab.iter().enumerate() {
                dt += c * t.powi(i as i32);
            }
            dt
        } else if year < 1860 {
            // 1800-1860年
            let t = y - 1800.0;
            let dt_tab = [13.72, -0.332447, 0.0068612, 0.0041116, -0.00037436,
                          0.0000121272, -0.0000001699, 0.000000000875];
            let mut dt = 0.0;
            for (i, &c) in dt_tab.iter().enumerate() {
                dt += c * t.powi(i as i32);
            }
            dt
        } else if year < 1900 {
            // 1860-1900年
            let t = y - 1860.0;
            let dt_tab = [7.62, 0.5737, -0.251754, 0.01680668, -0.0004473624,
                          0.000000011735];
            let mut dt = 0.0;
            for (i, &c) in dt_tab.iter().enumerate() {
                dt += c * t.powi(i as i32);
            }
            dt
        } else if year < 1920 {
            // 1900-1920年
            let t = y - 1900.0;
            let dt_tab = [-2.79, 1.494119, -0.0598939, 0.0061966, -0.000197];
            let mut dt = 0.0;
            for (i, &c) in dt_tab.iter().enumerate() {
                dt += c * t.powi(i as i32);
            }
            dt
        } else if year < 1941 {
            // 1920-1941年
            let t = y - 1920.0;
            let dt_tab = [21.20, 0.84493, -0.076100, 0.0020936];
            let mut dt = 0.0;
            for (i, &c) in dt_tab.iter().enumerate() {
                dt += c * t.powi(i as i32);
            }
            dt
        } else if year < 1961 {
            // 1941-1961年
            let t = y - 1950.0;
            let dt_tab = [29.07, 0.407, -0.000000025733, 0.000000003774];
            let mut dt = 0.0;
            for (i, &c) in dt_tab.iter().enumerate() {
                dt += c * t.powi(i as i32);
            }
            dt
        } else if year < 1986 {
            // 1961-1986年
            let t = y - 1975.0;
            let dt_tab = [45.45, 1.067, -0.000083333, -0.00000019709];
            let mut dt = 0.0;
            for (i, &c) in dt_tab.iter().enumerate() {
                dt += c * t.powi(i as i32);
            }
            dt
        } else if year < 2005 {
            // 1986-2005年
            let t = y - 2000.0;
            let dt_tab = [63.86, 0.3345, -0.060374, -0.0017275, 0.0006518, 0.00002373599];
            let mut dt = 0.0;
            for (i, &c) in dt_tab.iter().enumerate() {
                dt += c * t.powi(i as i32);
            }
            dt
        } else if year < 2050 {
            // 2005-2050年
            let t = y - 2000.0;
            let dt_tab = [62.92, 0.32217, 0.005589];
            let mut dt = 0.0;
            for (i, &c) in dt_tab.iter().enumerate() {
                dt += c * t.powi(i as i32);
            }
            dt
        } else if year < 2150 {
            // 2050-2150年
            let u = (y - 1820.0) / 100.0;
            -20.0 + 32.0 * u * u + 0.5628 * (2150.0 - y)
        } else {
            // 2150年以后
            let u = (y - 1820.0) / 100.0;
            -20.0 + 32.0 * u * u
        }
    }

    /// 计算星期（0=周日, 1=周一, ..., 6=周六）
    pub fn weekday(jd: f64) -> u8 {
        ((jd + 1.5).floor() as i64 % 7) as u8
    }

    /// 星期中文名称
    pub fn weekday_zh(jd: f64) -> &'static str {
        match Self::weekday(jd) {
            0 => "日", 1 => "一", 2 => "二",
            3 => "三", 4 => "四", 5 => "五",
            6 => "六", _ => "",
        }
    }

    /// 计算儒略世纪数 T（从 J2000.0 起算）
    pub fn julian_century(jd: f64) -> f64 {
        (jd - 2451545.0) / 36525.0
    }

    /// 计算儒略千年数
    pub fn julian_millennium(jd: f64) -> f64 {
        (jd - 2451545.0) / 365250.0
    }
}

impl Default for JulianDay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jd_conversion() {
        // 2024-01-01 的 JD 约为 2460310.5
        let jd = JulianDay::to_jd(2024, 1, 1.0);
        assert!((jd - 2460310.5).abs() < 1.0);

        let (y, m, d) = JulianDay::from_jd(2460310.5);
        assert_eq!(y, 2024);
        assert_eq!(m, 1);
        assert!((d - 1.0).abs() < 0.5);
    }

    #[test]
    fn test_delta_t() {
        let dt = JulianDay::delta_t(2024, 6);
        // 2024年的ΔT约为69秒
        assert!(dt > 60.0 && dt < 75.0);
    }
}