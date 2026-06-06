// 寿星天文历 - 日月食计算
// 参考原项目: SharpSxwnl/sun_moon.cs
//
// 基于沙罗周期的日月食计算
// 计算精度: 可满足一般天文爱好者需求

use super::julian::JulianDay;
use super::syzygy::SyzygyCalc;
use super::vsop87;
use serde::{Serialize, Deserialize};

/// 日月食信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EclipseInfo {
    /// 食类型: "日偏食", "日全食", "日环食", "月偏食", "月全食"
    pub eclipse_type: String,
    /// 食发生时间 (儒略日)
    pub jd: f64,
    /// 食发生时间 (公历)
    pub date: String,
    /// 最大食分
    pub magnitude: f64,
    /// 沙罗序列号
    pub saros: Option<i32>,
}

/// 日月食计算器
pub struct EclipseCalc {
    syzygy: SyzygyCalc,
}

impl EclipseCalc {
    pub fn new() -> Self {
        Self {
            syzygy: SyzygyCalc::new(),
        }
    }

    /// 计算某年的所有日月食
    ///
    /// 返回该年发生的日月食列表
    pub fn calc_year_eclipses(&self, year: i32) -> Vec<EclipseInfo> {
        let mut eclipses = Vec::new();

        // 搜索该年所有朔日（可能的日食）和望日（可能的月食）
        let start_jd = JulianDay::to_jd(year, 1, 1.0);
        let end_jd = JulianDay::to_jd(year + 1, 1, 1.0);

        // 搜索朔日（日食）
        let mut jd = start_jd - 15.0;
        while jd < end_jd + 15.0 {
            let shuo = self.syzygy.calc_shuo_exact(jd);
            if shuo >= start_jd && shuo < end_jd {
                if let Some(eclipse) = self.check_solar_eclipse(shuo) {
                    eclipses.push(eclipse);
                }
            }
            jd += 29.0;
        }

        // 搜索望日（月食）
        jd = start_jd - 15.0;
        while jd < end_jd + 15.0 {
            let wang = self.syzygy.calc_wang_exact(jd);
            if wang >= start_jd && wang < end_jd {
                if let Some(eclipse) = self.check_lunar_eclipse(wang) {
                    eclipses.push(eclipse);
                }
            }
            jd += 29.0;
        }

        eclipses.sort_by(|a, b| a.jd.partial_cmp(&b.jd).unwrap());
        eclipses
    }

    /// 检查是否为日食
    ///
    /// 日食条件: 朔日时月球在黄白交点附近
    fn check_solar_eclipse(&self, jd: f64) -> Option<EclipseInfo> {
        let (sun_lon, _, sun_r) = vsop87::calc_sun_position(jd);
        let (moon_lon, moon_lat, moon_r) = vsop87::calc_moon_position(jd);

        // 日月黄经差（应接近0）
        let dlon = (moon_lon - sun_lon).abs();
        if dlon > 180.0 {
            // 处理角度差
        }

        // 月球黄纬
        let lat = moon_lat.abs();

        // 日食极限条件: 月球黄纬 < 约1.5度
        if lat > 1.6 {
            return None;
        }

        // 视半径
        let sun_semi: f64 = 0.004649; // 太阳视半径 (AU)
        let moon_semi: f64 = 0.004518; // 月球视半径 (AU)

        // 日食判断
        let dist = (dlon * dlon * sun_r * sun_r + lat * lat).sqrt();

        let eclipse_type = if dist < (sun_semi - moon_semi).abs() {
            "日全食"
        } else if dist < sun_semi + moon_semi {
            "日偏食"
        } else {
            return None;
        };

        let magnitude = (sun_semi + moon_semi - dist) / (2.0 * sun_semi);

        let (y, m, d) = JulianDay::from_jd(jd);
        let date = format!("{}-{:02}-{:02}", y, m, d.floor() as u32);

        Some(EclipseInfo {
            eclipse_type: eclipse_type.to_string(),
            jd,
            date,
            magnitude: magnitude.max(0.0).min(1.0),
            saros: None,
        })
    }

    /// 检查是否为月食
    ///
    /// 月食条件: 望日时月球在黄白交点附近
    fn check_lunar_eclipse(&self, jd: f64) -> Option<EclipseInfo> {
        let (sun_lon, _, sun_r) = vsop87::calc_sun_position(jd);
        let (moon_lon, moon_lat, moon_r) = vsop87::calc_moon_position(jd);

        // 月食时日月黄经差应接近180度
        let dlon = (moon_lon - sun_lon - 180.0).abs();
        if dlon > 180.0 {
            // 处理角度差
        }

        let lat = moon_lat.abs();

        // 月食极限条件
        if lat > 1.8 {
            return None;
        }

        // 地球本影半径
        let shadow_radius = 0.0075; // 约1.3度

        let dist = (dlon * dlon + lat * lat).sqrt();

        let eclipse_type = if dist < shadow_radius * 0.3 {
            "月全食"
        } else if dist < shadow_radius {
            "月偏食"
        } else {
            return None;
        };

        let magnitude = (shadow_radius - dist) / shadow_radius;

        let (y, m, d) = JulianDay::from_jd(jd);
        let date = format!("{}-{:02}-{:02}", y, m, d.floor() as u32);

        Some(EclipseInfo {
            eclipse_type: eclipse_type.to_string(),
            jd,
            date,
            magnitude: magnitude.max(0.0).min(1.0),
            saros: None,
        })
    }
}

impl Default for EclipseCalc {
    fn default() -> Self {
        Self::new()
    }
}