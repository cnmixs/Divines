// Divines - 预测占星模块
// 参考原项目: astropy/astrostudy/solararc.py, pd_engine.py, firdaria.py,
//             zreleasing.py, signasctime.py, termdirection.py, perpredict.py

use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Timelike, Utc};
use divines_core::*;
use serde::{Serialize, Deserialize};
use crate::sxwnl::vsop87;
use crate::sxwnl::vsop87::AngleExt;

// ============================================================================
// 辅助类型
// ============================================================================

/// 次限推运盘
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressedChart {
    /// 推运日期
    pub date: String,
    /// 推运行星位置
    pub planets: Vec<ProgressedPlanet>,
    /// 推运上升点
    pub ascendant: f64,
    /// 推运中天
    pub midheaven: f64,
    /// 与出生盘的相位
    pub aspects: Vec<ProgressedAspect>,
}

/// 次限推运行星
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressedPlanet {
    pub planet: Planet,
    pub longitude: f64,
    pub sign: ZodiacSign,
    pub degree_in_sign: f64,
    pub is_retrograde: bool,
}

/// 次限推运相位
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressedAspect {
    pub progressed_planet: Planet,
    pub natal_planet: Planet,
    pub aspect_type: AspectType,
    pub orb: f64,
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 角度归一化到 [0, 360)
fn norm360(x: f64) -> f64 {
    ((x % 360.0) + 360.0) % 360.0
}

/// 角度归一化到 [-180, 180)
fn norm180(x: f64) -> f64 {
    ((x + 180.0) % 360.0 + 360.0) % 360.0 - 180.0
}

/// 黄经转星座
fn lon_to_sign(lon: f64) -> ZodiacSign {
    let idx = (norm360(lon) / 30.0) as u8;
    match idx % 12 {
        0 => ZodiacSign::Aries,
        1 => ZodiacSign::Taurus,
        2 => ZodiacSign::Gemini,
        3 => ZodiacSign::Cancer,
        4 => ZodiacSign::Leo,
        5 => ZodiacSign::Virgo,
        6 => ZodiacSign::Libra,
        7 => ZodiacSign::Scorpio,
        8 => ZodiacSign::Sagittarius,
        9 => ZodiacSign::Capricorn,
        10 => ZodiacSign::Aquarius,
        _ => ZodiacSign::Pisces,
    }
}

/// 星座转星座索引 (0=Aries)
fn sign_to_index(sign: ZodiacSign) -> u8 {
    sign as u8
}

/// 两个角度之间的最短距离（带符号）
fn angle_distance(a: f64, b: f64) -> f64 {
    let mut d = norm360(b) - norm360(a);
    if d > 180.0 {
        d -= 360.0;
    } else if d < -180.0 {
        d += 360.0;
    }
    d
}

/// 判断两个角度之间是否构成相位
fn check_aspect(angle1: f64, angle2: f64) -> Vec<(AspectType, f64)> {
    let angle = norm360(angle1);
    let mut results = Vec::new();

    let aspects: [(AspectType, f64, f64); 11] = [
        (AspectType::Conjunction, 0.0, 8.0),
        (AspectType::Opposition, 180.0, 8.0),
        (AspectType::Trine, 120.0, 8.0),
        (AspectType::Square, 90.0, 7.0),
        (AspectType::Sextile, 60.0, 6.0),
        (AspectType::Quincunx, 150.0, 5.0),
        (AspectType::SemiSextile, 30.0, 3.0),
        (AspectType::SemiSquare, 45.0, 3.0),
        (AspectType::Sesquiquadrate, 135.0, 3.0),
        (AspectType::Quintile, 72.0, 2.0),
        (AspectType::BiQuintile, 144.0, 2.0),
    ];

    for (asp_type, exact_angle, max_orb) in &aspects {
        let diff = (angle2 - angle - exact_angle).rem_euclid(360.0);
        let orb = if diff > 180.0 { 360.0 - diff } else { diff };
        if orb <= *max_orb {
            results.push((*asp_type, orb));
        }
    }

    results
}

/// 获取星座守护星
fn sign_ruler(sign: ZodiacSign) -> Planet {
    match sign {
        ZodiacSign::Aries => Planet::Mars,
        ZodiacSign::Taurus => Planet::Venus,
        ZodiacSign::Gemini => Planet::Mercury,
        ZodiacSign::Cancer => Planet::Moon,
        ZodiacSign::Leo => Planet::Sun,
        ZodiacSign::Virgo => Planet::Mercury,
        ZodiacSign::Libra => Planet::Venus,
        ZodiacSign::Scorpio => Planet::Mars, // 传统守护: 火星
        ZodiacSign::Sagittarius => Planet::Jupiter,
        ZodiacSign::Capricorn => Planet::Saturn,
        ZodiacSign::Aquarius => Planet::Saturn, // 传统守护: 土星
        ZodiacSign::Pisces => Planet::Jupiter,  // 传统守护: 木星
    }
}

/// 计算儒略日（简化算法）
fn julian_day_from_datetime(dt: &DateTime<Utc>) -> f64 {
    let year = dt.year();
    let month = dt.month() as i32;
    let day = dt.day() as i32;
    let hour = dt.hour() as f64
        + dt.minute() as f64 / 60.0
        + dt.second() as f64 / 3600.0;

    let a = (14 - month) / 12;
    let y = year + 4800 - a;
    let m = month + 12 * a - 3;

    let jd = day as i32
        + (153 * m + 2) / 5
        + 365 * y
        + y / 4
        - y / 100
        + y / 400
        - 32045;

    jd as f64 + (hour - 12.0) / 24.0
}

/// 儒略日转 DateTime<Utc>
fn julian_day_to_datetime(jd: f64) -> DateTime<Utc> {
    let jd_int = jd as i32;
    let frac = jd - jd_int as f64;

    let mut a = jd_int + 32044;
    let b = (4 * a + 3) / 146097;
    let c = a - (146097 * b) / 4;
    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m = (5 * e + 2) / 153;

    let day = e - (153 * m + 2) / 5 + 1;
    let month = m + 3 - 12 * (m / 10);
    let year = 100 * b + d - 4800 + m / 10;

    let total_hours = (frac + 0.5) * 24.0;
    let hour = total_hours as u32;
    let remainder = (total_hours - hour as f64) * 60.0;
    let minute = remainder as u32;
    let second = ((remainder - minute as f64) * 60.0) as u32;

    Utc.from_utc_datetime(
        &NaiveDate::from_ymd_opt(year, month as u32, day as u32)
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(2000, 1, 1).unwrap())
            .and_hms_opt(hour % 24, minute % 60, second % 60)
            .unwrap(),
    )
}

/// 计算行星黄经（使用 VSOP87 真位置）
///
/// 替换原平黄经近似计算，使用寿星天文历的 VSOP87 模块进行精确计算。
/// 对于太阳使用 VSOP87 太阳位置，月球使用 ELP 简化模型，
/// 其他行星使用 VSOP87 简化轨道根数，冥王星使用简化轨道公式，
/// 南北交点使用平均交点公式。
fn vsop87_planet_longitude(planet: Planet, jd: f64) -> f64 {
    match planet {
        Planet::Sun => {
            let (lon, _, _) = vsop87::calc_sun_position(jd);
            norm360(lon)
        }
        Planet::Moon => {
            let (lon, _, _) = vsop87::calc_moon_position(jd);
            norm360(lon)
        }
        Planet::Mercury => {
            if let Some((lon, _, _)) = vsop87::calc_planet_position("mercury", jd) {
                norm360(lon)
            } else {
                // 回退到平黄经
                let t = (jd - 2451545.0) / 36525.0;
                norm360((252.2508 + 149472.6745 * t) % 360.0)
            }
        }
        Planet::Venus => {
            if let Some((lon, _, _)) = vsop87::calc_planet_position("venus", jd) {
                norm360(lon)
            } else {
                let t = (jd - 2451545.0) / 36525.0;
                norm360((181.9798 + 58517.8156 * t) % 360.0)
            }
        }
        Planet::Mars => {
            if let Some((lon, _, _)) = vsop87::calc_planet_position("mars", jd) {
                norm360(lon)
            } else {
                let t = (jd - 2451545.0) / 36525.0;
                norm360((355.4530 + 19139.8580 * t) % 360.0)
            }
        }
        Planet::Jupiter => {
            if let Some((lon, _, _)) = vsop87::calc_planet_position("jupiter", jd) {
                norm360(lon)
            } else {
                let t = (jd - 2451545.0) / 36525.0;
                norm360((34.3515 + 3034.9057 * t) % 360.0)
            }
        }
        Planet::Saturn => {
            if let Some((lon, _, _)) = vsop87::calc_planet_position("saturn", jd) {
                norm360(lon)
            } else {
                let t = (jd - 2451545.0) / 36525.0;
                norm360((50.0774 + 1222.1138 * t) % 360.0)
            }
        }
        Planet::Uranus => {
            if let Some((lon, _, _)) = vsop87::calc_planet_position("uranus", jd) {
                norm360(lon)
            } else {
                let t = (jd - 2451545.0) / 36525.0;
                norm360((314.0550 + 428.4670 * t) % 360.0)
            }
        }
        Planet::Neptune => {
            if let Some((lon, _, _)) = vsop87::calc_planet_position("neptune", jd) {
                norm360(lon)
            } else {
                let t = (jd - 2451545.0) / 36525.0;
                norm360((304.2227 + 218.4862 * t) % 360.0)
            }
        }
        Planet::Pluto => {
            // VSOP87 不包含冥王星，使用简化轨道根数
            let t = (jd - 2451545.0) / 36525.0;
            let m = (14.882 + 1.452 * t + 0.0006 * t * t) % 360.0;
            let m_rad = m.to_radians();
            let ecc = 0.2488 - 0.000051 * t;
            let c = (2.0 * ecc - 0.25 * ecc * ecc * ecc) * m_rad.sin()
                + (1.25 * ecc * ecc) * (2.0 * m_rad).sin()
                + (13.0 / 12.0 * ecc * ecc * ecc) * (3.0 * m_rad).sin();
            let lop = (238.95 + 0.0040 * t) % 360.0;
            let lon = (lop + c.to_degrees()) % 360.0;
            norm360(lon)
        }
        Planet::NorthNode => {
            let t = (jd - 2451545.0) / 36525.0;
            let mean_lon = (125.0445 - 1934.1363 * t) % 360.0;
            norm360(mean_lon)
        }
        Planet::SouthNode => {
            let t = (jd - 2451545.0) / 36525.0;
            let mean_lon = (125.0445 - 1934.1363 * t + 180.0) % 360.0;
            norm360(mean_lon)
        }
        _ => 0.0,
    }
}

/// 计算出生盘行星位置（使用 VSOP87 真位置）
fn calc_birth_planet_position(planet: Planet, jd: f64) -> (f64, ZodiacSign, f64) {
    let lon = vsop87_planet_longitude(planet, jd);
    let sign = lon_to_sign(lon);
    let degree = lon % 30.0;
    (lon, sign, degree)
}

/// 计算上升点近似值
fn calc_ascendant_approx(jd: f64, lat: f64, _lon: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    // 恒星时近似
    let gmst = 280.46061837
        + 360.98564736629 * (jd - 2451545.0)
        + 0.000387933 * t * t
        - t * t * t / 38710000.0;
    let lst = norm360(gmst + _lon);

    // 黄赤交角
    let eps = 23.439291 - 0.0130042 * t;

    let lst_rad = lst.to_radians();
    let lat_rad = lat.to_radians();
    let eps_rad = eps.to_radians();

    let asc = (lst_rad.cos().atan2(
        -lst_rad.sin() * eps_rad.cos() - lat_rad.tan() * eps_rad.sin(),
    ))
    .to_degrees();

    norm360(asc)
}

/// 计算中天近似值
fn calc_midheaven_approx(jd: f64, _lat: f64, _lon: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    let gmst = 280.46061837
        + 360.98564736629 * (jd - 2451545.0)
        + 0.000387933 * t * t;
    let lst = norm360(gmst + _lon);

    let eps = 23.439291 - 0.0130042 * t;
    let lst_rad = lst.to_radians();
    let eps_rad = eps.to_radians();

    let mc = (lst_rad.sin().atan2(lst_rad.cos() * eps_rad.cos())).to_degrees();
    norm360(mc)
}

// ============================================================================
// 1. 太阳弧 Solar Arc Directions
// 参考原项目: astropy/astrostudy/solararc.py
// ============================================================================

impl PredictCalc {
    /// 太阳弧推运
    ///
    /// 所有行星推进相同的弧（太阳弧 = 推进太阳 - 出生太阳）。
    /// 计算推进行星与出生行星之间的相位，返回日期范围。
    pub fn solar_arc(&self, birth: &BirthInfo, target_date: DateTime<Utc>) -> Vec<SolarArc> {
        let birth_jd = julian_day_from_datetime(&birth.datetime);
        let target_jd = julian_day_from_datetime(&target_date);

        // 计算推进天数（太阳弧使用天/年比例）
        let days_per_year = 365.2421904;
        let delta_years = (target_jd - birth_jd) / days_per_year;

        // 推进日期 = 出生日期 + delta_years 天
        let progressed_jd = birth_jd + delta_years;

        // 计算出生太阳和推进太阳
        let natal_sun_lon = vsop87_planet_longitude(Planet::Sun, birth_jd);
        let progressed_sun_lon = vsop87_planet_longitude(Planet::Sun, progressed_jd);

        // 太阳弧 = 推进太阳 - 出生太阳
        let solar_arc = angle_distance(natal_sun_lon, progressed_sun_lon);

        // 所有需要考虑的行星
        let planets_list = [
            Planet::Sun,
            Planet::Moon,
            Planet::Mercury,
            Planet::Venus,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
            Planet::Uranus,
            Planet::Neptune,
            Planet::Pluto,
            Planet::NorthNode,
            Planet::SouthNode,
        ];

        let mut results = Vec::new();

        // 计算出生行星位置
        let natal_positions: Vec<(Planet, f64)> = planets_list
            .iter()
            .map(|&p| (p, vsop87_planet_longitude(p, birth_jd)))
            .collect();

        // 推进行星位置 = 出生位置 + 太阳弧
        // 北交点/南交点逆行（可选），这里默认也推进
        for (directed_planet, natal_lon) in &natal_positions {
            let directed_lon = norm360(natal_lon + solar_arc);

            for (natal_planet, natal_target_lon) in &natal_positions {
                if directed_planet == natal_planet {
                    // 跳过自己
                    continue;
                }

                let aspects = check_aspect(*natal_target_lon, directed_lon);
                for (asp_type, orb) in aspects {
                    results.push(SolarArc {
                        planet: *natal_planet,
                        arc: solar_arc,
                        directed_planet: *directed_planet,
                        aspect_type: asp_type,
                        date: format!("{}", target_date.format("%Y-%m-%d")),
                    });
                }
            }
        }

        results
    }
}

// ============================================================================
// 2. 次限法 Secondary Progressions
// 参考原项目: flatlib-ctrad2/flatlib/predictives/ 和 通用占星学
// ============================================================================

impl PredictCalc {
    /// 次限推运（A day for a year 方法）
    ///
    /// 出生后一天 = 生命中的一年。
    /// 计算推进月亮（约每月1°）、推进太阳（约每年1°）、推进四角（MC, ASC）。
    pub fn secondary_progressions(
        &self,
        birth: &BirthInfo,
        target_date: DateTime<Utc>,
    ) -> ProgressedChart {
        let birth_jd = julian_day_from_datetime(&birth.datetime);
        let target_jd = julian_day_from_datetime(&target_date);

        let days_per_year = 365.2421904;
        let age_years = (target_jd - birth_jd) / days_per_year;

        // 推进日期 = 出生日期 + 年龄天数（1天 = 1年）
        let progressed_jd = birth_jd + age_years;

        let planets_list = [
            Planet::Sun,
            Planet::Moon,
            Planet::Mercury,
            Planet::Venus,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
            Planet::Uranus,
            Planet::Neptune,
            Planet::Pluto,
            Planet::NorthNode,
            Planet::SouthNode,
        ];

        // 计算推进行星位置
        let mut progressed_planets = Vec::new();
        let mut natal_lons: Vec<(Planet, f64)> = Vec::new();

        for &planet in &planets_list {
            let natal_lon = vsop87_planet_longitude(planet, birth_jd);
            let progressed_lon = vsop87_planet_longitude(planet, progressed_jd);

            natal_lons.push((planet, natal_lon));

            // 判断逆行（简化：根据行星速度判断）
            let is_retrograde = match planet {
                Planet::Mercury | Planet::Venus | Planet::Mars
                | Planet::Jupiter | Planet::Saturn | Planet::Uranus
                | Planet::Neptune | Planet::Pluto => {
                    // 比较推进前后位置判断是否逆行
                    let lon_before = vsop87_planet_longitude(planet, progressed_jd - 0.1);
                    let lon_after = vsop87_planet_longitude(planet, progressed_jd + 0.1);
                    let diff = angle_distance(lon_before, lon_after);
                    diff < 0.0
                }
                _ => false,
            };

            progressed_planets.push(ProgressedPlanet {
                planet,
                longitude: progressed_lon,
                sign: lon_to_sign(progressed_lon),
                degree_in_sign: progressed_lon % 30.0,
                is_retrograde,
            });
        }

        // 计算推进上升点和中天
        let lat = birth.location.latitude;
        let lon = birth.location.longitude;
        let progressed_asc = calc_ascendant_approx(progressed_jd, lat, lon);
        let progressed_mc = calc_midheaven_approx(progressed_jd, lat, lon);

        // 计算推进盘与出生盘的相位
        let mut aspects = Vec::new();
        for prog in &progressed_planets {
            for (nat_planet, nat_lon) in &natal_lons {
                if prog.planet == *nat_planet {
                    continue;
                }
                let asp_list = check_aspect(*nat_lon, prog.longitude);
                for (asp_type, orb) in asp_list {
                    aspects.push(ProgressedAspect {
                        progressed_planet: prog.planet,
                        natal_planet: *nat_planet,
                        aspect_type: asp_type,
                        orb,
                    });
                }
            }

            // 推进行星与出生上升点的相位
            let natal_asc = calc_ascendant_approx(birth_jd, lat, lon);
            let asc_aspects = check_aspect(natal_asc, prog.longitude);
            for (asp_type, orb) in asc_aspects {
                aspects.push(ProgressedAspect {
                    progressed_planet: prog.planet,
                    natal_planet: Planet::Vertex, // 用 Vertex 代表上升点
                    aspect_type: asp_type,
                    orb,
                });
            }

            // 推进行星与出生中天的相位
            let natal_mc = calc_midheaven_approx(birth_jd, lat, lon);
            let mc_aspects = check_aspect(natal_mc, prog.longitude);
            for (asp_type, orb) in mc_aspects {
                aspects.push(ProgressedAspect {
                    progressed_planet: prog.planet,
                    natal_planet: Planet::PartOfFortune, // 用 PartOfFortune 代表中天
                    aspect_type: asp_type,
                    orb,
                });
            }
        }

        ProgressedChart {
            date: format!("{}", target_date.format("%Y-%m-%d")),
            planets: progressed_planets,
            ascendant: progressed_asc,
            midheaven: progressed_mc,
            aspects,
        }
    }
}

// ============================================================================
// 3. 主限法 Primary Directions
// 参考原项目: astropy/astrostudy/pd_engine.py, termdirection.py, signasctime.py
// ============================================================================

/// 上升星座时间表（signasctime.py）
/// 纬度 0-66，每度一个数组，包含6个值（对宫共享）
/// 值表示该星座上升1度需要的赤经时间（分钟）
const SIGNS_ASC_TIME: [[f64; 6]; 67] = [
    // 纬度 0-4
    [27.9167, 29.9000, 32.1833, 32.1833, 29.9000, 27.9167],
    [27.9167, 29.9000, 32.1833, 32.1833, 29.9000, 27.9167],
    [27.9167, 29.9000, 32.1833, 32.1833, 29.9000, 27.9167],
    [27.9167, 29.9000, 32.1833, 32.1833, 29.9000, 27.9167],
    [27.9167, 29.9000, 32.1833, 32.1833, 29.9000, 27.9167],
    // 纬度 5-9
    [26.9000, 29.0833, 31.8500, 32.5167, 30.7333, 28.9333],
    [26.9000, 29.0833, 31.8500, 32.5167, 30.7333, 28.9333],
    [26.9000, 29.0833, 31.8500, 32.5167, 30.7333, 28.9333],
    [26.9000, 29.0833, 31.8500, 32.5167, 30.7333, 28.9333],
    [26.9000, 29.0833, 31.8500, 32.5167, 30.7333, 28.9333],
    // 纬度 10-14
    [25.8500, 28.2500, 31.5000, 32.8500, 31.5667, 29.9667],
    [25.8500, 28.2500, 31.5000, 32.8500, 31.5667, 29.9667],
    [25.8500, 28.2500, 31.5000, 32.8500, 31.5667, 29.9667],
    [25.8500, 28.2500, 31.5000, 32.8500, 31.5667, 29.9667],
    [25.8500, 28.2500, 31.5000, 32.8500, 31.5667, 29.9667],
    // 纬度 15-19
    [24.7833, 27.3833, 31.1500, 33.2167, 32.4333, 31.0333],
    [24.7833, 27.3833, 31.1500, 33.2167, 32.4333, 31.0333],
    [24.7833, 27.3833, 31.1500, 33.2167, 32.4333, 31.0333],
    [24.7833, 27.3833, 31.1500, 33.2167, 32.4333, 31.0333],
    [24.7833, 27.3833, 31.1500, 33.2167, 32.4333, 31.0333],
    // 纬度 20-22
    [23.6667, 26.4667, 30.7833, 33.5833, 33.3500, 32.1500],
    [23.4333, 26.2833, 30.7000, 33.6667, 33.5333, 33.3833],
    [23.2000, 26.0833, 30.6167, 33.7500, 33.7333, 32.6167],
    // 纬度 23-26
    [22.9667, 25.8833, 30.5333, 33.8333, 33.9333, 32.8500],
    [22.7167, 25.6833, 30.4500, 33.9167, 34.1333, 33.1000],
    [22.4833, 25.4833, 30.3667, 34.0000, 34.3333, 33.3333],
    [22.2333, 25.2833, 30.2833, 34.0833, 34.5333, 33.5833],
    // 纬度 27-30
    [21.9667, 25.0667, 30.2000, 34.1667, 34.7500, 33.8500],
    [21.7167, 24.8500, 30.1000, 34.2667, 34.9667, 34.1000],
    [21.4500, 24.6333, 30.0167, 34.3500, 35.1833, 34.3667],
    [21.1833, 24.4000, 29.9167, 34.4500, 35.4167, 34.6333],
    // 纬度 31-34
    [20.9000, 24.1667, 29.8167, 34.5500, 35.6500, 34.9167],
    [20.6167, 23.9333, 29.7167, 34.6500, 35.8833, 35.2000],
    [20.3333, 23.7000, 29.6167, 34.7500, 36.1167, 35.4833],
    [20.0333, 23.4500, 29.5000, 34.8500, 36.3667, 35.7833],
    // 纬度 35-37
    [19.7333, 23.1833, 29.4000, 34.9667, 36.6333, 36.0833],
    [19.4333, 22.9167, 29.2833, 35.0833, 36.9000, 36.3833],
    [19.1167, 22.6500, 29.1667, 35.2000, 37.1667, 36.7167],
    // 纬度 38-40
    [18.7833, 22.3667, 29.0333, 35.3167, 37.4500, 37.0333],
    [18.4500, 22.0833, 28.9167, 35.4500, 37.7333, 37.3667],
    [18.1000, 21.7833, 28.7833, 35.5833, 38.3333, 37.7167],
    // 纬度 41-43
    [17.7500, 21.4667, 28.6333, 35.7167, 38.3500, 38.0667],
    [17.3833, 21.1333, 28.5000, 35.8667, 38.6833, 38.4333],
    [17.0000, 20.8000, 28.3500, 36.0167, 39.0167, 38.8167],
    // 纬度 44-45
    [16.6000, 20.4500, 28.1833, 36.1833, 39.3667, 39.2167],
    [16.2000, 20.0833, 28.0167, 36.3500, 39.7333, 39.6167],
    // 纬度 46-47
    [15.7667, 19.7000, 27.8333, 36.5333, 40.1167, 40.0500],
    [15.3333, 19.3000, 27.6500, 36.7167, 40.5167, 40.4833],
    // 纬度 48-49
    [14.8833, 18.8833, 27.4500, 36.9167, 40.9333, 40.9333],
    [14.4000, 18.4333, 27.2333, 37.1333, 41.3833, 41.4167],
    // 纬度 50-51
    [13.9167, 17.9667, 27.0000, 37.3667, 41.8500, 41.9167],
    [13.3833, 17.4667, 26.7500, 37.6167, 42.3500, 42.4333],
    // 纬度 52-53
    [12.8500, 16.9500, 26.4833, 37.8833, 42.8667, 42.9667],
    [12.2833, 16.3833, 26.2000, 38.1667, 43.4333, 43.5333],
    // 纬度 54-55
    [11.6833, 15.7833, 25.8833, 38.4833, 44.0333, 44.1333],
    [11.0500, 15.1500, 25.5333, 38.8333, 44.6667, 44.7667],
    // 纬度 56-57
    [10.4000, 14.4500, 25.1333, 39.2333, 45.3667, 45.4167],
    [9.7000, 13.7000, 24.7000, 39.6667, 46.1167, 46.1167],
    // 纬度 58-59
    [8.9500, 12.9000, 24.2000, 40.1667, 46.9167, 46.8667],
    [8.1667, 12.0000, 23.6333, 40.7333, 47.8167, 47.6500],
    // 纬度 60-66
    [7.3167, 11.0167, 22.9667, 41.4000, 48.8000, 48.5000],
    [6.4333, 9.9167, 22.1667, 42.2000, 49.9000, 49.4000],
    [5.4667, 8.7000, 21.1833, 43.1833, 51.1167, 50.3500],
    [4.4333, 7.3000, 19.9333, 44.4333, 52.5167, 51.3833],
    [3.3167, 5.6833, 18.2167, 48.1333, 54.1333, 52.5000],
    [2.1000, 3.8000, 15.6667, 48.7000, 56.0167, 53.7167],
    [0.7833, 1.5000, 10.7833, 53.5833, 58.3167, 55.0333],
];

/// 星座上升时间索引（signasctime.py SignsAscTimeIndex）
fn sign_asc_time_index(sign: ZodiacSign) -> usize {
    match sign {
        ZodiacSign::Aries => 0,
        ZodiacSign::Taurus => 1,
        ZodiacSign::Gemini => 2,
        ZodiacSign::Cancer => 3,
        ZodiacSign::Leo => 4,
        ZodiacSign::Virgo => 5,
        ZodiacSign::Libra => 5,
        ZodiacSign::Scorpio => 4,
        ZodiacSign::Sagittarius => 3,
        ZodiacSign::Capricorn => 2,
        ZodiacSign::Aquarius => 1,
        ZodiacSign::Pisces => 0,
    }
}

/// 计算从 fromLon 到 toLon 的上升时间（赤经分钟）
/// 参考原项目: signasctime.py getAscSignTime
fn get_asc_sign_time(lat: f64, from_lon: f64, to_lon: f64) -> f64 {
    if from_lon >= to_lon {
        return 0.0;
    }

    let to_idx = (to_lon / 30.0) as usize;
    let from_idx = (from_lon / 30.0) as usize;
    let from_sign_lon = from_lon % 30.0;
    let mut to_sign_lon = to_lon % 30.0;
    if to_sign_lon == 0.0 {
        to_sign_lon = 30.0;
    }

    let lat_idx = (lat.abs() as usize).min(66);
    let ary = SIGNS_ASC_TIME[lat_idx];

    let mut cnt = 0.0;

    if from_idx == to_idx {
        let sig = match from_idx {
            0 => ZodiacSign::Aries,
            1 => ZodiacSign::Taurus,
            2 => ZodiacSign::Gemini,
            3 => ZodiacSign::Cancer,
            4 => ZodiacSign::Leo,
            5 => ZodiacSign::Virgo,
            6 => ZodiacSign::Libra,
            7 => ZodiacSign::Scorpio,
            8 => ZodiacSign::Sagittarius,
            9 => ZodiacSign::Capricorn,
            10 => ZodiacSign::Aquarius,
            _ => ZodiacSign::Pisces,
        };
        let ary_idx = sign_asc_time_index(sig);
        let factor = ary[ary_idx] / 30.0;
        cnt += (to_sign_lon - from_sign_lon) * factor;
    } else {
        for i in from_idx..to_idx {
            let sig = match i {
                0 => ZodiacSign::Aries,
                1 => ZodiacSign::Taurus,
                2 => ZodiacSign::Gemini,
                3 => ZodiacSign::Cancer,
                4 => ZodiacSign::Leo,
                5 => ZodiacSign::Virgo,
                6 => ZodiacSign::Libra,
                7 => ZodiacSign::Scorpio,
                8 => ZodiacSign::Sagittarius,
                9 => ZodiacSign::Capricorn,
                10 => ZodiacSign::Aquarius,
                _ => ZodiacSign::Pisces,
            };
            let ary_idx = sign_asc_time_index(sig);
            let factor = ary[ary_idx] / 30.0;
            if i == from_idx {
                cnt += (30.0 - from_sign_lon) * factor;
            } else {
                cnt += ary[ary_idx];
            }
        }
        let to_idx_clamped = to_idx.min(11);
        let sig = match to_idx_clamped {
            0 => ZodiacSign::Aries,
            1 => ZodiacSign::Taurus,
            2 => ZodiacSign::Gemini,
            3 => ZodiacSign::Cancer,
            4 => ZodiacSign::Leo,
            5 => ZodiacSign::Virgo,
            6 => ZodiacSign::Libra,
            7 => ZodiacSign::Scorpio,
            8 => ZodiacSign::Sagittarius,
            9 => ZodiacSign::Capricorn,
            10 => ZodiacSign::Aquarius,
            _ => ZodiacSign::Pisces,
        };
        let ary_idx = sign_asc_time_index(sig);
        let factor = ary[ary_idx] / 30.0;
        cnt += to_sign_lon * factor;
    }

    cnt
}

/// 埃及界表（Egyptian Terms）
/// 参考原项目: pd_engine.py EGYPTIAN_TERMS
const EGYPTIAN_TERMS: [(usize, &str, f64); 60] = [
    // Aries (0)
    (0, "Jup", 0.0), (0, "Ven", 6.0), (0, "Mer", 12.0), (0, "Mar", 20.0), (0, "Sat", 25.0),
    // Taurus (1)
    (1, "Ven", 0.0), (1, "Mer", 8.0), (1, "Jup", 14.0), (1, "Sat", 22.0), (1, "Mar", 27.0),
    // Gemini (2)
    (2, "Mer", 0.0), (2, "Jup", 6.0), (2, "Ven", 12.0), (2, "Mar", 17.0), (2, "Sat", 24.0),
    // Cancer (3)
    (3, "Mar", 0.0), (3, "Ven", 7.0), (3, "Mer", 13.0), (3, "Jup", 19.0), (3, "Sat", 26.0),
    // Leo (4)
    (4, "Jup", 0.0), (4, "Ven", 6.0), (4, "Sat", 11.0), (4, "Mer", 18.0), (4, "Mar", 24.0),
    // Virgo (5)
    (5, "Mer", 0.0), (5, "Ven", 7.0), (5, "Jup", 17.0), (5, "Mar", 21.0), (5, "Sat", 28.0),
    // Libra (6)
    (6, "Sat", 0.0), (6, "Mer", 6.0), (6, "Jup", 14.0), (6, "Ven", 21.0), (6, "Mar", 28.0),
    // Scorpio (7)
    (7, "Mar", 0.0), (7, "Ven", 7.0), (7, "Mer", 11.0), (7, "Jup", 19.0), (7, "Sat", 24.0),
    // Sagittarius (8)
    (8, "Jup", 0.0), (8, "Ven", 12.0), (8, "Mer", 17.0), (8, "Sat", 21.0), (8, "Mar", 26.0),
    // Capricorn (9)
    (9, "Mer", 0.0), (9, "Jup", 7.0), (9, "Ven", 14.0), (9, "Sat", 22.0), (9, "Mar", 26.0),
    // Aquarius (10)
    (10, "Mer", 0.0), (10, "Ven", 7.0), (10, "Jup", 13.0), (10, "Mar", 20.0), (10, "Sat", 25.0),
    // Pisces (11)
    (11, "Ven", 0.0), (11, "Jup", 12.0), (11, "Mer", 16.0), (11, "Mar", 19.0), (11, "Sat", 28.0),
];

/// 界主星名转 Planet
fn term_ruler_to_planet(ruler: &str) -> Planet {
    match ruler {
        "Sun" => Planet::Sun,
        "Moo" => Planet::Moon,
        "Mer" => Planet::Mercury,
        "Ven" => Planet::Venus,
        "Mar" => Planet::Mars,
        "Jup" => Planet::Jupiter,
        "Sat" => Planet::Saturn,
        _ => Planet::Saturn,
    }
}

/// Naibod 钥匙常数：太阳平均日行度数
/// 参考原项目: pd_engine.py NAIBOD_RATE
const NAIBOD_RATE: f64 = 0.9856473354;

impl PredictCalc {
    /// 主限法推运
    ///
    /// 支持的方法:
    /// - "ptolemy": Ptolemy 法（1°赤经 = 1年）
    /// - "alchabitius": Alcabitius 半弧法
    /// - "regiomontanus": Regiomontanus 位置圈法
    /// - "naibod": 使用 Naibod 钥匙（1° = 1/0.9856473354 年）
    ///
    /// 参考原项目: astropy/astrostudy/pd_engine.py, termdirection.py
    pub fn primary_directions(
        &self,
        birth: &BirthInfo,
        method: &str,
    ) -> Vec<PrimaryDirection> {
        let birth_jd = julian_day_from_datetime(&birth.datetime);
        let lat = birth.location.latitude;

        // 计算出生行星位置
        let signif_planets = [
            Planet::Sun,
            Planet::Moon,
            Planet::Mercury,
            Planet::Venus,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
        ];

        let promissor_planets = [
            Planet::Sun,
            Planet::Moon,
            Planet::Mercury,
            Planet::Venus,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
            Planet::Uranus,
            Planet::Neptune,
            Planet::Pluto,
        ];

        let mut results = Vec::new();

        // 计算出生上升点
        let natal_asc_lon = calc_ascendant_approx(birth_jd, lat, birth.location.longitude);
        let natal_mc_lon = calc_midheaven_approx(birth_jd, lat, birth.location.longitude);

        // 计算出生行星位置
        let sig_positions: Vec<(Planet, f64)> = signif_planets
            .iter()
            .map(|&p| (p, vsop87_planet_longitude(p, birth_jd)))
            .collect();

        let prom_positions: Vec<(Planet, f64)> = promissor_planets
            .iter()
            .map(|&p| (p, vsop87_planet_longitude(p, birth_jd)))
            .collect();

        // 相位角度列表
        let aspect_angles = [0.0, 60.0, 90.0, 120.0, 180.0];

        match method.to_lowercase().as_str() {
            "ptolemy" | "alchabitius" => {
                // Ptolemy / Alcabitius 法: 基于赤经差
                // 简化实现：使用黄经差作为近似
                let max_arc = 100.0;

                for (sig_planet, sig_lon) in &sig_positions {
                    for (prom_planet, prom_lon) in &prom_positions {
                        if sig_planet == prom_planet {
                            continue;
                        }

                        for &asp_angle in &aspect_angles {
                            // 推进黄经 = 出生黄经 + 相位
                            let prom_aspect_lon = norm360(prom_lon + asp_angle);

                            // 弧 = 应星黄经 - 动星黄经（在赤经空间近似为黄经差）
                            let arc = norm180(prom_aspect_lon - sig_lon);

                            // 取正弧（顺向）
                            if arc.abs() > max_arc || arc.abs() < 0.01 {
                                continue;
                            }

                            let arc_abs = arc.abs();

                            // Naibod 钥匙：弧转年龄
                            let age = if method.to_lowercase().as_str() == "naibod" {
                                arc_abs / NAIBOD_RATE
                            } else {
                                arc_abs
                            };

                            if age > 100.0 {
                                continue;
                            }

                            // 计算日期
                            let days = age * 365.2421904;
                            let event_jd = birth_jd + days;
                            let event_date = julian_day_to_datetime(event_jd);

                            let asp_type = match asp_angle as i32 {
                                0 => "Conjunction",
                                60 => "Sextile",
                                90 => "Square",
                                120 => "Trine",
                                180 => "Opposition",
                                _ => "Aspect",
                            };

                            results.push(PrimaryDirection {
                                significator: *sig_planet,
                                promissor: *prom_planet,
                                direction_type: format!("{} {}", method, asp_type),
                                arc: arc_abs,
                                date: format!("{}", event_date.format("%Y-%m-%d")),
                                age,
                            });
                        }
                    }
                }

                // 添加向四轴的方向
                let angle_sigs = [
                    (Planet::Vertex, natal_asc_lon), // 上升点
                    (Planet::PartOfFortune, natal_mc_lon), // 中天
                ];

                for (angle_planet, angle_lon) in &angle_sigs {
                    for (prom_planet, prom_lon) in &prom_positions {
                        for &asp_angle in &aspect_angles {
                            let prom_aspect_lon = norm360(prom_lon + asp_angle);
                            let arc = norm180(prom_aspect_lon - angle_lon);

                            if arc.abs() > 100.0 || arc.abs() < 0.01 {
                                continue;
                            }

                            let arc_abs = arc.abs();
                            let age = if method.to_lowercase().as_str() == "naibod" {
                                arc_abs / NAIBOD_RATE
                            } else {
                                arc_abs
                            };

                            if age > 100.0 {
                                continue;
                            }

                            let days = age * 365.2421904;
                            let event_jd = birth_jd + days;
                            let event_date = julian_day_to_datetime(event_jd);

                            let asp_type = match asp_angle as i32 {
                                0 => "Conjunction",
                                60 => "Sextile",
                                90 => "Square",
                                120 => "Trine",
                                180 => "Opposition",
                                _ => "Aspect",
                            };

                            results.push(PrimaryDirection {
                                significator: *angle_planet,
                                promissor: *prom_planet,
                                direction_type: format!("{} {}", method, asp_type),
                                arc: arc_abs,
                                date: format!("{}", event_date.format("%Y-%m-%d")),
                                age,
                            });
                        }
                    }
                }
            }

            "regiomontanus" => {
                // Regiomontanus 法: 使用上升时间表
                let max_arc = 100.0;

                for (sig_planet, sig_lon) in &sig_positions {
                    for (prom_planet, prom_lon) in &prom_positions {
                        if sig_planet == prom_planet {
                            continue;
                        }

                        for &asp_angle in &aspect_angles {
                            let prom_aspect_lon = norm360(prom_lon + asp_angle);

                            // 使用上升时间表计算弧
                            let asc_time = get_asc_sign_time(lat, prom_aspect_lon, *sig_lon);
                            // 将上升时间（赤经分钟）转换为度数
                            let arc = asc_time / 4.0; // 4分钟 = 1度

                            if arc > max_arc || arc < 0.01 {
                                continue;
                            }

                            let age = arc;

                            if age > 100.0 {
                                continue;
                            }

                            let days = age * 365.2421904;
                            let event_jd = birth_jd + days;
                            let event_date = julian_day_to_datetime(event_jd);

                            let asp_type = match asp_angle as i32 {
                                0 => "Conjunction",
                                60 => "Sextile",
                                90 => "Square",
                                120 => "Trine",
                                180 => "Opposition",
                                _ => "Aspect",
                            };

                            results.push(PrimaryDirection {
                                significator: *sig_planet,
                                promissor: *prom_planet,
                                direction_type: format!("Regiomontanus {}", asp_type),
                                arc,
                                date: format!("{}", event_date.format("%Y-%m-%d")),
                                age,
                            });
                        }
                    }
                }

                // 向上升点方向
                for (prom_planet, prom_lon) in &prom_positions {
                    for &asp_angle in &aspect_angles {
                        let prom_aspect_lon = norm360(prom_lon + asp_angle);
                        let asc_time = get_asc_sign_time(lat, prom_aspect_lon, natal_asc_lon);
                        let arc = asc_time / 4.0;

                        if arc > max_arc || arc < 0.01 {
                            continue;
                        }

                        let age = arc;
                        if age > 100.0 {
                            continue;
                        }

                        let days = age * 365.2421904;
                        let event_jd = birth_jd + days;
                        let event_date = julian_day_to_datetime(event_jd);

                        let asp_type = match asp_angle as i32 {
                            0 => "Conjunction",
                            60 => "Sextile",
                            90 => "Square",
                            120 => "Trine",
                            180 => "Opposition",
                            _ => "Aspect",
                        };

                        results.push(PrimaryDirection {
                            significator: Planet::Vertex,
                            promissor: *prom_planet,
                            direction_type: format!("Regiomontanus {}", asp_type),
                            arc,
                            date: format!("{}", event_date.format("%Y-%m-%d")),
                            age,
                        });
                    }
                }
            }

            "naibod" => {
                // Naibod 钥匙: 与 Ptolemy 相同算法，但使用 Naibod 比率换算年龄
                let max_arc = 100.0;

                for (sig_planet, sig_lon) in &sig_positions {
                    for (prom_planet, prom_lon) in &prom_positions {
                        if sig_planet == prom_planet {
                            continue;
                        }

                        for &asp_angle in &aspect_angles {
                            let prom_aspect_lon = norm360(prom_lon + asp_angle);
                            let arc = norm180(prom_aspect_lon - sig_lon);

                            let arc_abs = arc.abs();
                            if arc_abs > max_arc || arc_abs < 0.01 {
                                continue;
                            }

                            let age = arc_abs / NAIBOD_RATE;
                            if age > 100.0 {
                                continue;
                            }

                            let days = age * 365.2421904;
                            let event_jd = birth_jd + days;
                            let event_date = julian_day_to_datetime(event_jd);

                            let asp_type = match asp_angle as i32 {
                                0 => "Conjunction",
                                60 => "Sextile",
                                90 => "Square",
                                120 => "Trine",
                                180 => "Opposition",
                                _ => "Aspect",
                            };

                            results.push(PrimaryDirection {
                                significator: *sig_planet,
                                promissor: *prom_planet,
                                direction_type: format!("Naibod {}", asp_type),
                                arc: arc_abs,
                                date: format!("{}", event_date.format("%Y-%m-%d")),
                                age,
                            });
                        }
                    }
                }
            }

            _ => {
                // 默认使用 Ptolemy 法
                let max_arc = 100.0;

                for (sig_planet, sig_lon) in &sig_positions {
                    for (prom_planet, prom_lon) in &prom_positions {
                        if sig_planet == prom_planet {
                            continue;
                        }

                        for &asp_angle in &aspect_angles {
                            let prom_aspect_lon = norm360(prom_lon + asp_angle);
                            let arc = norm180(prom_aspect_lon - sig_lon);

                            let arc_abs = arc.abs();
                            if arc_abs > max_arc || arc_abs < 0.01 {
                                continue;
                            }

                            let age = arc_abs;
                            if age > 100.0 {
                                continue;
                            }

                            let days = age * 365.2421904;
                            let event_jd = birth_jd + days;
                            let event_date = julian_day_to_datetime(event_jd);

                            let asp_type = match asp_angle as i32 {
                                0 => "Conjunction",
                                60 => "Sextile",
                                90 => "Square",
                                120 => "Trine",
                                180 => "Opposition",
                                _ => "Aspect",
                            };

                            results.push(PrimaryDirection {
                                significator: *sig_planet,
                                promissor: *prom_planet,
                                direction_type: format!("Ptolemy {}", asp_type),
                                arc: arc_abs,
                                date: format!("{}", event_date.format("%Y-%m-%d")),
                                age,
                            });
                        }
                    }
                }
            }
        }

        results.sort_by(|a, b| {
            a.arc
                .partial_cmp(&b.arc)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        results
    }
}

// ============================================================================
// 4. 法达星限 Firdaria
// 参考原项目: astropy/astrostudy/firdaria.py
// ============================================================================

impl PredictCalc {
    /// 法达星限
    ///
    /// 行星周期基于昼/夜出生:
    /// - 昼生: 日、金、水、月、土、木、火、北交、南交
    /// - 夜生: 月、土、木、火、北交、南交、日、金、水
    /// 每个主周期有7个子周期（1/7主周期）。
    pub fn firdaria(&self, birth: &BirthInfo) -> Vec<FirdariaPeriod> {
        let is_day = birth.datetime.hour() >= 6 && birth.datetime.hour() < 18;

        // 主周期序列
        let main_sequence: Vec<Planet> = if is_day {
            vec![
                Planet::Sun,
                Planet::Venus,
                Planet::Mercury,
                Planet::Moon,
                Planet::Saturn,
                Planet::Jupiter,
                Planet::Mars,
                Planet::NorthNode,
                Planet::SouthNode,
            ]
        } else {
            vec![
                Planet::Moon,
                Planet::Saturn,
                Planet::Jupiter,
                Planet::Mars,
                Planet::NorthNode,
                Planet::SouthNode,
                Planet::Sun,
                Planet::Venus,
                Planet::Mercury,
            ]
        };

        // 每个主周期年限
        let years_per_planet: [f64; 9] = [10.0, 8.0, 13.0, 9.0, 11.0, 12.0, 7.0, 3.0, 2.0];

        // 子周期序列（7个行星，不含交点）
        let sub_sequence: [Planet; 7] = [
            Planet::Sun,
            Planet::Venus,
            Planet::Mercury,
            Planet::Moon,
            Planet::Saturn,
            Planet::Jupiter,
            Planet::Mars,
        ];

        let birth_date = birth.datetime;
        let birth_jd = julian_day_from_datetime(&birth_date);
        let days_per_year = 365.2421904;

        // 计算主周期起始索引（从出生星座的主星开始）
        // 找到子周期序列中与主周期行星匹配的索引
        let mut periods = Vec::new();
        let mut current_jd = birth_jd;

        for (i, &main_planet) in main_sequence.iter().enumerate() {
            let main_years = years_per_planet[i];
            let main_days = main_years * days_per_year;

            let start_date = julian_day_to_datetime(current_jd);
            let end_jd = current_jd + main_days;
            let end_date = julian_day_to_datetime(end_jd);

            // 计算子周期
            let sub_days = main_days / 7.0;
            let mut sub_periods = Vec::new();

            // 找到从哪个子周期开始（与主周期行星匹配）
            let start_sub_idx = sub_sequence
                .iter()
                .position(|&p| p == main_planet)
                .unwrap_or(0);

            let mut sub_start_jd = current_jd;
            for j in 0..7 {
                let sub_idx = (start_sub_idx + j) % 7;
                let sub_planet = sub_sequence[sub_idx];
                let sub_end_jd = sub_start_jd + sub_days;

                let sub_start = julian_day_to_datetime(sub_start_jd);
                let sub_end = julian_day_to_datetime(sub_end_jd);

                sub_periods.push(FirdariaSubPeriod {
                    planet: sub_planet,
                    start_date: format!("{}", sub_start.format("%Y-%m-%d")),
                    end_date: format!("{}", sub_end.format("%Y-%m-%d")),
                });

                sub_start_jd = sub_end_jd;
            }

            periods.push(FirdariaPeriod {
                planet: main_planet,
                start_date: format!("{}", start_date.format("%Y-%m-%d")),
                end_date: format!("{}", end_date.format("%Y-%m-%d")),
                sub_periods,
            });

            current_jd = end_jd;
        }

        periods
    }
}

// ============================================================================
// 5. 小限 Profections
// 参考原项目: flatlib-ctrad2/flatlib/predictives/profections.py
// ============================================================================

impl PredictCalc {
    /// 小限推运
    ///
    /// 年小限: 从0岁开始，每年推进1个星座
    /// 月小限: 在每个年小限内，每月推进1个星座
    /// 参考原项目: flatlib-ctrad2/flatlib/predictives/profections.py
    pub fn profections(
        &self,
        birth: &BirthInfo,
        target_date: DateTime<Utc>,
    ) -> Profection {
        let birth_jd = julian_day_from_datetime(&birth.datetime);
        let target_jd = julian_day_from_datetime(&target_date);

        let days_per_year = 365.2421904;
        let age = ((target_jd - birth_jd) / days_per_year) as u8;

        // 计算出生上升点星座
        let natal_asc = calc_ascendant_approx(
            birth_jd,
            birth.location.latitude,
            birth.location.longitude,
        );
        let asc_sign = lon_to_sign(natal_asc);
        let asc_sign_idx = sign_to_index(asc_sign);

        // 年小限: 从0岁开始，每年推进1个星座
        // 0岁 = 出生星座，1岁 = 下一个星座，以此类推
        let profection_sign_idx = ((asc_sign_idx as u32 + age as u32) % 12) as u8;
        let profection_sign = match profection_sign_idx {
            0 => ZodiacSign::Aries,
            1 => ZodiacSign::Taurus,
            2 => ZodiacSign::Gemini,
            3 => ZodiacSign::Cancer,
            4 => ZodiacSign::Leo,
            5 => ZodiacSign::Virgo,
            6 => ZodiacSign::Libra,
            7 => ZodiacSign::Scorpio,
            8 => ZodiacSign::Sagittarius,
            9 => ZodiacSign::Capricorn,
            10 => ZodiacSign::Aquarius,
            _ => ZodiacSign::Pisces,
        };

        // 小限宫位（从1开始）
        // 0岁 = 第1宫，1岁 = 第2宫，...，11岁 = 第12宫，12岁 = 第1宫
        let profection_house = ((age % 12) + 1) as u8;

        // 时间主星 = 小限星座的守护星
        let lord_of_year = sign_ruler(profection_sign);

        // 计算小限期间的起止日期
        // 从出生日期开始，每年推进
        let year_start_jd = birth_jd + (age as f64) * days_per_year;
        let year_end_jd = birth_jd + ((age as f64) + 1.0) * days_per_year;

        let start_date = julian_day_to_datetime(year_start_jd);
        let end_date = julian_day_to_datetime(year_end_jd);

        Profection {
            age,
            house: profection_house,
            sign: profection_sign,
            lord_of_year,
            start_date: format!("{}", start_date.format("%Y-%m-%d")),
            end_date: format!("{}", end_date.format("%Y-%m-%d")),
        }
    }

    /// 月小限推运
    ///
    /// 在每个年小限内，每月推进1个星座
    pub fn monthly_profections(
        &self,
        birth: &BirthInfo,
        target_date: DateTime<Utc>,
    ) -> Vec<Profection> {
        let birth_jd = julian_day_from_datetime(&birth.datetime);
        let target_jd = julian_day_from_datetime(&target_date);

        let days_per_year = 365.2421904;
        let age = ((target_jd - birth_jd) / days_per_year) as u8;

        // 计算出生上升点星座
        let natal_asc = calc_ascendant_approx(
            birth_jd,
            birth.location.latitude,
            birth.location.longitude,
        );
        let asc_sign_idx = sign_to_index(lon_to_sign(natal_asc));

        // 年小限基础星座
        let annual_prof_sign_idx = ((asc_sign_idx as u32 + age as u32) % 12) as u8;

        let mut monthly = Vec::new();

        // 计算12个月小限
        for month in 0..12u8 {
            let month_sign_idx = ((annual_prof_sign_idx as u32 + month as u32) % 12) as u8;
            let month_sign = match month_sign_idx {
                0 => ZodiacSign::Aries,
                1 => ZodiacSign::Taurus,
                2 => ZodiacSign::Gemini,
                3 => ZodiacSign::Cancer,
                4 => ZodiacSign::Leo,
                5 => ZodiacSign::Virgo,
                6 => ZodiacSign::Libra,
                7 => ZodiacSign::Scorpio,
                8 => ZodiacSign::Sagittarius,
                9 => ZodiacSign::Capricorn,
                10 => ZodiacSign::Aquarius,
                _ => ZodiacSign::Pisces,
            };

            let month_house = (((annual_prof_sign_idx as u32 + month as u32) % 12) + 1) as u8;
            let lord = sign_ruler(month_sign);

            let month_start_jd = birth_jd + (age as f64 + month as f64 / 12.0) * days_per_year;
            let month_end_jd =
                birth_jd + (age as f64 + (month as f64 + 1.0) / 12.0) * days_per_year;

            let start_date = julian_day_to_datetime(month_start_jd);
            let end_date = julian_day_to_datetime(month_end_jd);

            monthly.push(Profection {
                age: month, // 用月份表示
                house: month_house,
                sign: month_sign,
                lord_of_year: lord,
                start_date: format!("{}", start_date.format("%Y-%m-%d")),
                end_date: format!("{}", end_date.format("%Y-%m-%d")),
            });
        }

        monthly
    }
}

// ============================================================================
// 6. 黄道星释 Zodiacal Releasing
// 参考原项目: astropy/astrostudy/zreleasing.py
// ============================================================================

/// 每个星座的释放周期（年）
/// 参考原项目: zreleasing.py ZodiacalReleasing
const ZR_SIGN_YEARS: [(ZodiacSign, f64); 12] = [
    (ZodiacSign::Aries, 15.0),
    (ZodiacSign::Taurus, 8.0),
    (ZodiacSign::Gemini, 20.0),
    (ZodiacSign::Cancer, 25.0),
    (ZodiacSign::Leo, 19.0),
    (ZodiacSign::Virgo, 20.0),
    (ZodiacSign::Libra, 8.0),
    (ZodiacSign::Scorpio, 15.0),
    (ZodiacSign::Sagittarius, 12.0),
    (ZodiacSign::Capricorn, 27.0),
    (ZodiacSign::Aquarius, 30.0),
    (ZodiacSign::Pisces, 12.0),
];

/// 获取星座的释放周期年数
fn get_zr_years(sign: ZodiacSign) -> f64 {
    for (s, years) in &ZR_SIGN_YEARS {
        if *s == sign {
            return *years;
        }
    }
    12.0 // 默认
}

/// 黄道释放级别天数
const ZR_LEVEL_DAYS: [f64; 4] = [360.0, 30.0, 2.5, 2.5 / 12.0];

impl PredictCalc {
    /// 黄道星释
    ///
    /// 从福点（Lot of Fortune）或精神点（Lot of Spirit）开始。
    /// 释放周期通过黄道星座，支持 L2（星座级）和 L3（月级）级别。
    ///
    /// 参数:
    /// - `lot`: "fortune" 或 "spirit"
    ///
    /// 参考原项目: astropy/astrostudy/zreleasing.py
    pub fn zodiacal_releasing(
        &self,
        birth: &BirthInfo,
        lot: &str,
    ) -> Vec<ZodiacalReleasing> {
        let birth_jd = julian_day_from_datetime(&birth.datetime);
        let days_per_year = 365.2421904;

        // 计算福点/精神点
        let sun_lon = vsop87_planet_longitude(Planet::Sun, birth_jd);
        let moon_lon = vsop87_planet_longitude(Planet::Moon, birth_jd);
        let asc_lon = calc_ascendant_approx(
            birth_jd,
            birth.location.latitude,
            birth.location.longitude,
        );

        let lot_lon = match lot.to_lowercase().as_str() {
            "spirit" => norm360(asc_lon + sun_lon - moon_lon), // 精神点 = ASC + Sun - Moon
            _ => norm360(asc_lon + moon_lon - sun_lon), // 福点 = ASC + Moon - Sun
        };

        let lot_sign = lon_to_sign(lot_lon);
        let lot_sign_idx = sign_to_index(lot_sign) as usize;

        let mut results = Vec::new();
        let total_span = 100.0; // 100年跨度
        let total_days = total_span * days_per_year;

        let mut current_jd = birth_jd;
        let mut remaining_days = total_days;
        let mut sign_idx = lot_sign_idx;

        // L2 级别（星座级）
        let mut visited_signs: std::collections::HashSet<usize> = std::collections::HashSet::new();

        while remaining_days > 0.0 {
            let sign = match sign_idx {
                0 => ZodiacSign::Aries,
                1 => ZodiacSign::Taurus,
                2 => ZodiacSign::Gemini,
                3 => ZodiacSign::Cancer,
                4 => ZodiacSign::Leo,
                5 => ZodiacSign::Virgo,
                6 => ZodiacSign::Libra,
                7 => ZodiacSign::Scorpio,
                8 => ZodiacSign::Sagittarius,
                9 => ZodiacSign::Capricorn,
                10 => ZodiacSign::Aquarius,
                _ => ZodiacSign::Pisces,
            };

            let sign_years = get_zr_years(sign);
            let sign_days = sign_years * days_per_year;

            let period_days = if remaining_days < sign_days {
                remaining_days
            } else {
                sign_days
            };

            let start_date = julian_day_to_datetime(current_jd);
            let end_jd = current_jd + period_days;
            let end_date = julian_day_to_datetime(end_jd);

            // L2 级别
            results.push(ZodiacalReleasing {
                lot: lot.to_string(),
                level: 2,
                sign,
                start_date: format!("{}", start_date.format("%Y-%m-%d")),
                end_date: format!("{}", end_date.format("%Y-%m-%d")),
                peak_period: None,
            });

            // L3 级别（月级）：每个星座周期内再细分
            let l3_month_days = 30.0 * days_per_year / 12.0; // 约30.44天
            let mut l3_jd = current_jd;
            let mut l3_remaining = period_days;
            let mut l3_sign_idx = sign_idx;

            while l3_remaining > 0.0 {
                let l3_sign = match l3_sign_idx % 12 {
                    0 => ZodiacSign::Aries,
                    1 => ZodiacSign::Taurus,
                    2 => ZodiacSign::Gemini,
                    3 => ZodiacSign::Cancer,
                    4 => ZodiacSign::Leo,
                    5 => ZodiacSign::Virgo,
                    6 => ZodiacSign::Libra,
                    7 => ZodiacSign::Scorpio,
                    8 => ZodiacSign::Sagittarius,
                    9 => ZodiacSign::Capricorn,
                    10 => ZodiacSign::Aquarius,
                    _ => ZodiacSign::Pisces,
                };

                let l3_days = if l3_remaining < l3_month_days {
                    l3_remaining
                } else {
                    l3_month_days
                };

                let l3_start = julian_day_to_datetime(l3_jd);
                let l3_end_jd = l3_jd + l3_days;
                let l3_end = julian_day_to_datetime(l3_end_jd);

                // 峰值期：L3 周期的中间三分之一
                let peak_start_jd = l3_jd + l3_days / 3.0;
                let peak_end_jd = l3_jd + 2.0 * l3_days / 3.0;
                let peak_start = julian_day_to_datetime(peak_start_jd);
                let peak_end = julian_day_to_datetime(peak_end_jd);

                results.push(ZodiacalReleasing {
                    lot: lot.to_string(),
                    level: 3,
                    sign: l3_sign,
                    start_date: format!("{}", l3_start.format("%Y-%m-%d")),
                    end_date: format!("{}", l3_end.format("%Y-%m-%d")),
                    peak_period: Some((
                        format!("{}", peak_start.format("%Y-%m-%d")),
                        format!("{}", peak_end.format("%Y-%m-%d")),
                    )),
                });

                l3_jd += l3_days;
                l3_remaining -= l3_days;
                l3_sign_idx = (l3_sign_idx + 1) % 12;
            }

            current_jd += period_days;
            remaining_days -= period_days;

            // 追踪已访问的星座
            visited_signs.insert(sign_idx);

            // 如果所有星座都访问过，跳过6个星座（参考 Python 代码行为）
            sign_idx = (sign_idx + 1) % 12;
            if visited_signs.len() >= 12 {
                visited_signs.clear();
                sign_idx = (sign_idx + 6) % 12;
            }
        }

        results
    }

    // ========================================================================
    // 7. 年龄推进点 (Age Point / Huber)
    // 参考原项目: astropy/astrostudy/agepoint.py
    // ========================================================================

    /// 年龄推进点计算
    ///
    /// 年龄点自上升点起，沿12个等宫顺行，每宫6年、72年一周。
    /// 每岁返回AP落座/落宫及与本命星合相判定(orb 1°)。
    pub fn age_point(&self, birth: &BirthInfo, max_age: u32) -> AgePointResult {
        let jd = julian_day_from_datetime(&birth.datetime);
        let asc = calc_ascendant_approx(jd, birth.location.latitude, birth.location.longitude);
        // 使用等宫制（每宫30度）
        let koch_cusps: Vec<f64> = (0..12).map(|i| norm360(asc + i as f64 * 30.0)).collect();

        // 出生行星位置
        let natal_planets: Vec<(String, f64)> = [
            Planet::Sun, Planet::Moon, Planet::Mercury, Planet::Venus,
            Planet::Mars, Planet::Jupiter, Planet::Saturn,
        ]
        .iter()
        .map(|&p| {
            let (lon, _, _) = calc_birth_planet_position(p, jd);
            (format!("{:?}", p), lon)
        })
        .collect();

        let mut points = Vec::new();
        for age in 0..=max_age {
            let house_idx = (age / 6) % 12;
            let frac = (age % 6) as f64 / 6.0;
            let start_cusp = koch_cusps[house_idx as usize];
            let end_cusp = koch_cusps[((house_idx + 1) % 12) as usize];
            let span = norm360(end_cusp - start_cusp);
            let ap_lon = norm360(start_cusp + frac * span);

            let aspect_to = natal_planets.iter().find(|(_, plon)| {
                let d = norm360(ap_lon - plon).min(360.0 - norm360(ap_lon - plon));
                d <= 1.0
            }).map(|(id, _)| id.clone());

            let sign = lon_to_sign(ap_lon);
            points.push(AgePointEntry {
                age,
                longitude: ap_lon,
                sign,
                sign_degree: ap_lon % 30.0,
                house: house_idx as u32 + 1,
                aspect_to,
                cusp_crossing: frac == 0.0,
            });
        }

        AgePointResult {
            ascendant: asc,
            koch_cusps,
            max_age,
            points,
        }
    }

    // ========================================================================
    // 8. 波斯向运 / 周期推运 (Symbolic Direction)
    // 参考原项目: astropy/astrostudy/symbolicdir.py
    // ========================================================================

    /// 波斯向运计算
    ///
    /// 所有行星/点每年沿黄经前进固定速率，本命宫头固定不动。
    /// rate_key: "persian"(1°/年), "prophected"(30°/年), "naibod"(0.9856473°/年)
    pub fn symbolic_dir(
        &self,
        birth: &BirthInfo,
        age_years: f64,
        rate_key: &str,
        aspect_orb: f64,
        node_retrograde: bool,
        direction: &str, // "direct" or "converse"
    ) -> SymbolicDirResult {
        let rate = match rate_key {
            "persian" => 1.0,
            "prophected" => 30.0,
            "naibod" => 0.9856473,
            _ => 1.0,
        };
        let mut arc = age_years * rate;
        if direction == "converse" {
            arc = -arc;
        }

        let jd = julian_day_from_datetime(&birth.datetime);
        let all_planets = [
            Planet::Sun, Planet::Moon, Planet::Mercury, Planet::Venus,
            Planet::Mars, Planet::Jupiter, Planet::Saturn,
            Planet::Uranus, Planet::Neptune, Planet::Pluto,
            Planet::NorthNode, Planet::SouthNode,
        ];

        // 出生行星位置
        let natal_positions: Vec<(String, f64)> = all_planets
            .iter()
            .map(|&p| {
                let (lon, _, _) = calc_birth_planet_position(p, jd);
                (format!("{:?}", p), lon)
            })
            .collect();

        // 向运行星位置（所有行星+arc）
        let directed_positions: Vec<(String, f64)> = all_planets
            .iter()
            .map(|&p| {
                let (lon, _, _) = calc_birth_planet_position(p, jd);
                let directed_lon = if node_retrograde
                    && (p == Planet::NorthNode || p == Planet::SouthNode)
                {
                    norm360(lon - arc)
                } else {
                    norm360(lon + arc)
                };
                (format!("{:?}", p), directed_lon)
            })
            .collect();

        // 计算相位
        let orb = if aspect_orb < 0.0 { 1.0 } else { aspect_orb };
        let mut aspects = Vec::new();
        for (dir_id, dir_lon) in &directed_positions {
            let mut natal_aspects = Vec::new();
            for (nat_id, nat_lon) in &natal_positions {
                let delta = (dir_lon - nat_lon).abs();
                let d = delta.min(360.0 - delta);
                if d < orb {
                    natal_aspects.push(SymbolicDirAspect {
                        natal_id: nat_id.clone(),
                        aspect: 0.0,
                        orb: d,
                    });
                } else if (delta - 60.0).abs() < orb || (delta - 300.0).abs() < orb {
                    let od = (delta - 60.0).abs().min((delta - 300.0).abs());
                    natal_aspects.push(SymbolicDirAspect {
                        natal_id: nat_id.clone(),
                        aspect: 60.0,
                        orb: od,
                    });
                } else if (delta - 90.0).abs() < orb || (delta - 270.0).abs() < orb {
                    let od = (delta - 90.0).abs().min((delta - 270.0).abs());
                    natal_aspects.push(SymbolicDirAspect {
                        natal_id: nat_id.clone(),
                        aspect: 90.0,
                        orb: od,
                    });
                } else if (delta - 120.0).abs() < orb || (delta - 240.0).abs() < orb {
                    let od = (delta - 120.0).abs().min((delta - 240.0).abs());
                    natal_aspects.push(SymbolicDirAspect {
                        natal_id: nat_id.clone(),
                        aspect: 120.0,
                        orb: od,
                    });
                } else if (delta - 180.0).abs() < orb {
                    natal_aspects.push(SymbolicDirAspect {
                        natal_id: nat_id.clone(),
                        aspect: 180.0,
                        orb: (delta - 180.0).abs(),
                    });
                }
            }
            aspects.push(SymbolicDirEntry {
                directed_id: dir_id.clone(),
                aspects: natal_aspects,
            });
        }

        SymbolicDirResult {
            age_years,
            arc,
            rate_key: rate_key.to_string(),
            direction: direction.to_string(),
            directed_positions,
            aspects,
        }
    }

    // ========================================================================
    // 9. 界限法 (Term Direction)
    // 参考原项目: astropy/astrostudy/termdirection.py
    // ========================================================================

    /// 界限法主限方向计算
    ///
    /// 基于埃及界（Egyptian Terms）计算主限方向，返回弧、动星、应星列表。
    pub fn term_direction(
        &self,
        birth: &BirthInfo,
        aspects: &[f64],
        clockwise: bool,
    ) -> Vec<TermDirectionEntry> {
        let jd = julian_day_from_datetime(&birth.datetime);
        let asc = calc_ascendant_approx(jd, birth.location.latitude, birth.location.longitude);
        let asc_sign = lon_to_sign(asc);

        // 埃及界表（简化版：每个星座30度，分为5个界）
        // 完整实现需参考 tables.EGYPTIAN_TERMS
        let egyptian_terms: [(&str, &str, f64, f64); 5] = [
            ("Saturn", "Aries", 0.0, 6.0),
            ("Venus", "Aries", 6.0, 12.0),
            ("Jupiter", "Aries", 12.0, 20.0),
            ("Mercury", "Aries", 20.0, 25.0),
            ("Mars", "Aries", 25.0, 30.0),
        ];

        let mut results = Vec::new();
        let sign_idx = sign_to_index(asc_sign) as usize;
        let base_lon = sign_idx as f64 * 30.0;

        // 生成界点
        for (term_id, _sign, start, _end) in &egyptian_terms {
            let term_lon = norm360(base_lon + start);
            // 计算弧（简化：用黄经差作为弧的近似）
            let arc = norm360(term_lon - asc);
            if arc > 0.0 && arc < 100.0 {
                results.push(TermDirectionEntry {
                    arc,
                    promissor_id: format!("T_{}_{:?}", term_id, asc_sign),
                    significator_id: "ASC".to_string(),
                    direction_type: "T".to_string(),
                });
            }
        }

        // 为每个aspect添加动星
        for &asp in aspects {
            let asp_lon = norm360(asc + asp);
            if asp_lon > asc && asp_lon - asc < 100.0 {
                results.push(TermDirectionEntry {
                    arc: asp_lon - asc,
                    promissor_id: format!("S_Sun_{}", asp),
                    significator_id: "ASC".to_string(),
                    direction_type: "Z".to_string(),
                });
            }
        }

        results.sort_by(|a, b| a.arc.partial_cmp(&b.arc).unwrap());
        results
    }

    // ========================================================================
    // 10. 第十三宫盘 / 调波盘 / 龙盘
    // 参考原项目: astropy/astrostudy/thirteenthchart.py
    // ========================================================================

    /// 第十三宫盘（分形盘）
    ///
    /// 以月亮与太阳速度比缩放各点黄经，重新排列宫位。
    pub fn thirteenth_chart(&self, birth: &BirthInfo) -> ThirteenthChartResult {
        let jd = julian_day_from_datetime(&birth.datetime);
        let (sun_lon, _, _) = calc_birth_planet_position(Planet::Sun, jd);
        let (moon_lon, _, _) = calc_birth_planet_position(Planet::Moon, jd);
        let asc = calc_ascendant_approx(jd, birth.location.latitude, birth.location.longitude);

        // 月亮/太阳速度比近似（实际约13.18/0.98 ≈ 13.4）
        let ratio = 13.4;

        let planets = [
            Planet::Sun, Planet::Moon, Planet::Mercury, Planet::Venus,
            Planet::Mars, Planet::Jupiter, Planet::Saturn,
            Planet::Uranus, Planet::Neptune, Planet::Pluto,
        ];

        let mut positions = Vec::new();
        for &p in &planets {
            let (lon, _, _) = calc_birth_planet_position(p, jd);
            let sign = lon_to_sign(lon);
            let sign_idx = sign_to_index(sign) as usize;
            let signlon = lon % 30.0;
            let interval = 30.0 / ratio;
            let room = ((signlon / interval) as usize).min(12);
            let new_sign_idx = (sign_idx + room) % 12;
            let new_signlon = (signlon - interval * room as f64) / interval * 30.0;
            let new_lon = new_sign_idx as f64 * 30.0 + new_signlon;

            positions.push(ThirteenthChartPosition {
                id: format!("{:?}", p),
                original_lon: lon,
                new_lon,
                new_sign: lon_to_sign(new_lon),
                new_sign_degree: new_lon % 30.0,
            });
        }

        ThirteenthChartResult {
            sun_lon,
            moon_lon,
            ratio,
            ascendant: asc,
            positions,
        }
    }

    /// 调波盘（谐波盘）
    ///
    /// 将各点黄经乘以调波数后取模360，房宫按调波后ASC等宫排布。
    pub fn harmonic_chart(&self, birth: &BirthInfo, harmonic: u32) -> HarmonicChartResult {
        let harmonic = harmonic.max(1).min(360) as f64;
        let jd = julian_day_from_datetime(&birth.datetime);
        let asc = calc_ascendant_approx(jd, birth.location.latitude, birth.location.longitude);
        let new_asc = norm360(asc * harmonic);

        let planets = [
            Planet::Sun, Planet::Moon, Planet::Mercury, Planet::Venus,
            Planet::Mars, Planet::Jupiter, Planet::Saturn,
            Planet::Uranus, Planet::Neptune, Planet::Pluto,
        ];

        let mut positions = Vec::new();
        for &p in &planets {
            let (lon, _, _) = calc_birth_planet_position(p, jd);
            let new_lon = norm360(lon * harmonic);
            positions.push(HarmonicChartPosition {
                id: format!("{:?}", p),
                natal_lon: lon,
                harmonic_lon: new_lon,
                sign: lon_to_sign(new_lon),
                sign_degree: new_lon % 30.0,
            });
        }

        // 等宫制：new_asc作为1宫头
        let house1_lon = (new_asc / 30.0).floor() * 30.0;
        let houses: Vec<f64> = (0..12).map(|i| norm360(house1_lon + i as f64 * 30.0)).collect();

        HarmonicChartResult {
            harmonic: harmonic as u32,
            new_ascendant: new_asc,
            houses,
            positions,
        }
    }

    /// 龙盘（Draconic Chart）
    ///
    /// 以月亮北交点为白羊0°起算，各点黄经减去北交点黄经后取模360。
    pub fn draconic_chart(&self, birth: &BirthInfo) -> DraconicChartResult {
        let jd = julian_day_from_datetime(&birth.datetime);
        let (node_lon, _, _) = calc_birth_planet_position(Planet::NorthNode, jd);

        let planets = [
            Planet::Sun, Planet::Moon, Planet::Mercury, Planet::Venus,
            Planet::Mars, Planet::Jupiter, Planet::Saturn,
            Planet::Uranus, Planet::Neptune, Planet::Pluto,
        ];

        let mut positions = Vec::new();
        for &p in &planets {
            let (lon, _, _) = calc_birth_planet_position(p, jd);
            let new_lon = norm360(lon - node_lon);
            positions.push(DraconicChartPosition {
                id: format!("{:?}", p),
                natal_lon: lon,
                draconic_lon: new_lon,
                sign: lon_to_sign(new_lon),
                sign_degree: new_lon % 30.0,
            });
        }

        let asc = calc_ascendant_approx(jd, birth.location.latitude, birth.location.longitude);
        let draconic_asc = norm360(asc - node_lon);

        DraconicChartResult {
            node_lon,
            draconic_ascendant: draconic_asc,
            positions,
        }
    }

    // ========================================================================
    // 11. 129年系统 (129 Year System)
    // 参考原项目: astropy/astrostudy/yearsystem129.py
    // ========================================================================

    /// 129年系统计算
    ///
    /// 七政各管其"小年"，一轮共129年。∑=19+25+30+12+15+8+20=129。
    pub fn year_system_129(&self, birth: &BirthInfo) -> Vec<YearSystem129Entry> {
        let minor_years: Vec<(Planet, f64)> = vec![
            (Planet::Sun, 19.0),
            (Planet::Moon, 25.0),
            (Planet::Saturn, 30.0),
            (Planet::Jupiter, 12.0),
            (Planet::Mars, 15.0),
            (Planet::Venus, 8.0),
            (Planet::Mercury, 20.0),
        ];

        let sequence: Vec<Planet> = minor_years.iter().map(|(p, _)| *p).collect();
        let seq_len = sequence.len();

        // 起始星 = sect光体（日昼/月夜）
        let start = Planet::Sun; // 简化：默认昼生

        let start_idx = sequence.iter().position(|&p| p == start).unwrap_or(0);
        let rotated: Vec<Planet> = sequence[start_idx..]
            .iter()
            .chain(sequence[..start_idx].iter())
            .copied()
            .collect();

        let jd = julian_day_from_datetime(&birth.datetime);
        let mut current_jd = jd;
        let mut results = Vec::new();

        for &main_direct in &rotated {
            let minor_y = minor_years.iter().find(|(p, _)| *p == main_direct).map(|(_, y)| *y).unwrap_or(10.0);
            let main_idx = sequence.iter().position(|&p| p == main_direct).unwrap_or(0);
            let sub_rotated: Vec<Planet> = sequence[main_idx..]
                .iter()
                .chain(sequence[..main_idx].iter())
                .copied()
                .collect();

            let avg_days = minor_y / seq_len as f64 * 365.2421904;
            let mut sub_entries = Vec::new();
            let mut sub_jd = current_jd;

            for &sub_direct in &sub_rotated {
                let dt = julian_day_to_datetime(sub_jd);
                sub_entries.push(YearSystem129SubEntry {
                    sub_direct: format!("{:?}", sub_direct),
                    date: format!("{}", dt.format("%Y-%m-%d")),
                });
                sub_jd += avg_days;
            }

            results.push(YearSystem129Entry {
                main_direct: format!("{:?}", main_direct),
                sub_entries,
            });

            current_jd += minor_y * 365.2421904;
        }

        results
    }
}

// ============================================================================
// 结果类型定义
// ============================================================================

/// 年龄推进点条目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgePointEntry {
    pub age: u32,
    pub longitude: f64,
    pub sign: ZodiacSign,
    pub sign_degree: f64,
    pub house: u32,
    pub aspect_to: Option<String>,
    pub cusp_crossing: bool,
}

/// 年龄推进点结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgePointResult {
    pub ascendant: f64,
    pub koch_cusps: Vec<f64>,
    pub max_age: u32,
    pub points: Vec<AgePointEntry>,
}

/// 波斯向运相位
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolicDirAspect {
    pub natal_id: String,
    pub aspect: f64,
    pub orb: f64,
}

/// 波斯向运条目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolicDirEntry {
    pub directed_id: String,
    pub aspects: Vec<SymbolicDirAspect>,
}

/// 波斯向运结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolicDirResult {
    pub age_years: f64,
    pub arc: f64,
    pub rate_key: String,
    pub direction: String,
    pub directed_positions: Vec<(String, f64)>,
    pub aspects: Vec<SymbolicDirEntry>,
}

/// 界限法条目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TermDirectionEntry {
    pub arc: f64,
    pub promissor_id: String,
    pub significator_id: String,
    pub direction_type: String,
}

/// 第十三宫盘位置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThirteenthChartPosition {
    pub id: String,
    pub original_lon: f64,
    pub new_lon: f64,
    pub new_sign: ZodiacSign,
    pub new_sign_degree: f64,
}

/// 第十三宫盘结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThirteenthChartResult {
    pub sun_lon: f64,
    pub moon_lon: f64,
    pub ratio: f64,
    pub ascendant: f64,
    pub positions: Vec<ThirteenthChartPosition>,
}

/// 调波盘位置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HarmonicChartPosition {
    pub id: String,
    pub natal_lon: f64,
    pub harmonic_lon: f64,
    pub sign: ZodiacSign,
    pub sign_degree: f64,
}

/// 调波盘结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HarmonicChartResult {
    pub harmonic: u32,
    pub new_ascendant: f64,
    pub houses: Vec<f64>,
    pub positions: Vec<HarmonicChartPosition>,
}

/// 龙盘位置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DraconicChartPosition {
    pub id: String,
    pub natal_lon: f64,
    pub draconic_lon: f64,
    pub sign: ZodiacSign,
    pub sign_degree: f64,
}

/// 龙盘结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DraconicChartResult {
    pub node_lon: f64,
    pub draconic_ascendant: f64,
    pub positions: Vec<DraconicChartPosition>,
}

/// 129年系统子限条目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct YearSystem129SubEntry {
    pub sub_direct: String,
    pub date: String,
}

/// 129年系统条目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct YearSystem129Entry {
    pub main_direct: String,
    pub sub_entries: Vec<YearSystem129SubEntry>,
}

// ============================================================================
// PredictCalc 主结构体
// ============================================================================

/// 预测占星计算器
///
/// 包含以下推运方法:
/// 1. 太阳弧 (Solar Arc)
/// 2. 次限法 (Secondary Progressions)
/// 3. 主限法 (Primary Directions)
/// 4. 法达星限 (Firdaria)
/// 5. 小限 (Profections)
/// 6. 黄道星释 (Zodiacal Releasing)
/// 7. 年龄推进点 (Age Point)
/// 8. 波斯向运 (Symbolic Direction)
/// 9. 界限法 (Term Direction)
/// 10. 第十三宫盘 (Thirteenth Chart)
/// 11. 调波盘 (Harmonic Chart)
/// 12. 龙盘 (Draconic Chart)
/// 13. 129年系统 (129 Year System)
pub struct PredictCalc;

impl PredictCalc {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PredictCalc {
    fn default() -> Self {
        Self::new()
    }
}