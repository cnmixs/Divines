// 寿星天文历 - 朔望与节气计算
// 参考原项目: SharpSxwnl/SSQ.cs
//
// 计算实朔(新月)和实气(二十四节气)的精确时刻
// 算法基于 VSOP87 行星理论和 ELP/MPP02 月球理论

use super::julian::JulianDay;
use super::vsop87;
use divines_core::*;

/// 朔望与节气计算器
pub struct SyzygyCalc;

impl SyzygyCalc {
    pub fn new() -> Self {
        Self
    }

    /// 计算某年的所有节气时刻
    ///
    /// 返回 24 个节气的 (名称, 儒略日) 列表
    /// 参考原项目: SSQ.calcY()
    pub fn calc_year_jieqi(&self, year: i32) -> Vec<JieQi> {
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

        for (i, term) in terms.iter().enumerate() {
            // 第 i 个节气的角度 = i * 15°
            let angle = (i * 15) as f64;
            let jd = self.calc_qi_jd(year, angle);

            let (y, m, d) = JulianDay::from_jd(jd);
            let hour = (d.fract() * 24.0) as u32;
            let min = (d.fract() * 24.0 * 60.0) as u32 % 60;
            let day_int = d.floor() as u32;

            jieqi_list.push(JieQi {
                name: format!("{:?}", term),
                name_zh: term.name_zh().to_string(),
                datetime: format!("{}-{:02}-{:02}T{:02}:{:02}:00", y, m, day_int, hour, min),
                solar_longitude: angle,
                is_jie: i % 2 == 0,
            });
        }

        jieqi_list
    }

    /// 计算某年的所有朔日（农历初一）
    ///
    /// 返回每月朔日的儒略日列表
    /// 参考原项目: SSQ.calcS()
    pub fn calc_year_shuo(&self, year: i32) -> Vec<f64> {
        let mut shuo_list: Vec<f64> = Vec::new();

        // 从上年冬至开始搜索
        let start_jd = JulianDay::to_jd(year - 1, 12, 1.0);
        let end_jd = JulianDay::to_jd(year + 1, 1, 31.0);

        let mut jd = start_jd;
        while jd < end_jd {
            let shuo_jd = self.calc_shuo_near(jd);
            if shuo_jd > start_jd && shuo_jd < end_jd {
                // 避免重复添加
                if shuo_list.last().map_or(true, |&last| (shuo_jd as f64 - last).abs() > 10.0) {
                    shuo_list.push(shuo_jd);
                }
            }
            jd += 25.0; // 朔望月约29.5天，搜索步长25天
        }

        shuo_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
        shuo_list
    }

    /// 计算指定角度附近的节气儒略日
    ///
    /// angle: 太阳黄经度数 (0, 15, 30, ..., 345)
    fn calc_qi_jd(&self, year: i32, angle: f64) -> f64 {
        // 估算节气日期
        let est_month = (angle / 30.0 + 2.0) as u32;
        let est_day = if (angle as i32) % 30 == 0 { 21.0 } else { 7.0 };

        let mut jd = JulianDay::to_jd(year, est_month.min(12).max(1), est_day);
        let mut jd = jd - JulianDay::delta_t(year, est_month.min(12).max(1)) / 86400.0;

        // 牛顿迭代法精算
        for _ in 0..10 {
            let (sun_lon, _, _) = vsop87::calc_sun_position(jd);
            let d = Self::angle_diff(sun_lon, angle);
            if d.abs() < 0.00001 {
                break;
            }
            // 太阳每天移动约0.9856度
            jd += d / 0.9856;
        }

        jd
    }

    /// 计算指定日期附近的朔日（新月）儒略日
    ///
    /// 参考原项目: SSQ.calcS()
    fn calc_shuo_near(&self, jd: f64) -> f64 {
        let mut jd = jd;
        let dt = JulianDay::delta_t(
            JulianDay::from_jd(jd).0,
            JulianDay::from_jd(jd).1,
        ) / 86400.0;

        // 计算太阳和月亮的黄经
        let (sun_lon, _, _) = vsop87::calc_sun_position(jd - dt);
        let (moon_lon, _, _) = vsop87::calc_moon_position(jd - dt);

        // 日月黄经差
        let diff = Self::angle_diff(moon_lon, sun_lon);

        // 朔日 = 日月黄经相等时
        // 月球相对太阳每天移动约12.19度
        jd -= diff / 12.19;

        // 牛顿迭代
        for _ in 0..8 {
            let (sun_lon, _, _) = vsop87::calc_sun_position(jd);
            let (moon_lon, _, _) = vsop87::calc_moon_position(jd);
            let d = Self::angle_diff(moon_lon, sun_lon);
            if d.abs() < 0.0001 {
                break;
            }
            jd -= d / 12.19;
        }

        jd
    }

    /// 计算两个角度的差值（归一化到 [-180, 180]）
    fn angle_diff(a: f64, b: f64) -> f64 {
        let mut diff = (a - b) % 360.0;
        if diff > 180.0 {
            diff -= 360.0;
        } else if diff < -180.0 {
            diff += 360.0;
        }
        diff
    }

    /// 计算指定角度附近的合朔时刻（精确版）
    ///
    /// 用于日月食计算
    pub fn calc_shuo_exact(&self, jd_approx: f64) -> f64 {
        let mut jd = jd_approx;

        for _ in 0..15 {
            let (sun_lon, _, sun_r) = vsop87::calc_sun_position(jd);
            let (moon_lon, _, moon_r) = vsop87::calc_moon_position(jd);

            let d = Self::angle_diff(moon_lon, sun_lon);
            if d.abs() < 0.000001 {
                break;
            }

            // 月球相对太阳的角速度
            let rate = 12.190749; // 度/天
            jd -= d / rate;
        }

        jd
    }

    /// 计算指定角度附近的望日（满月）时刻
    pub fn calc_wang_exact(&self, jd_approx: f64) -> f64 {
        let mut jd = jd_approx;

        for _ in 0..15 {
            let (sun_lon, _, _) = vsop87::calc_sun_position(jd);
            let (moon_lon, _, _) = vsop87::calc_moon_position(jd);

            // 望 = 日月黄经差180度
            let d = Self::angle_diff(moon_lon, sun_lon + 180.0);
            if d.abs() < 0.000001 {
                break;
            }

            let rate = 12.190749;
            jd -= d / rate;
        }

        jd
    }
}

impl Default for SyzygyCalc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jieqi_2024() {
        let calc = SyzygyCalc::new();
        let jieqi = calc.calc_year_jieqi(2024);
        assert_eq!(jieqi.len(), 24);
        // 立春大约在2月4日
        assert!(jieqi[0].datetime.contains("-02-0"));
    }
}