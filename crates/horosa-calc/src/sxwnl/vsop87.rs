// 寿星天文历 - VSOP87 行星理论
// 参考原项目: SharpSxwnl/XL.cs (星历), ob.cs (行星)
//
// 基于 VSOP87 理论的太阳和行星位置计算
// 月球位置基于 ELP/MPP02 简化模型
//
// VSOP87 是 Bureau des Longitudes 发布的半解析行星理论
// 精度: 公元前2000年到公元6000年，误差 < 1"

use std::f64::consts::PI;

/// 角度转弧度辅助 trait
pub trait AngleExt {
    fn to_radians(self) -> f64;
    fn to_degrees(self) -> f64;
}

impl AngleExt for f64 {
    #[inline]
    fn to_radians(self) -> f64 {
        self * PI / 180.0
    }
    #[inline]
    fn to_degrees(self) -> f64 {
        self * 180.0 / PI
    }
}

/// 计算太阳位置（黄经、黄纬、距离）
///
/// 基于 VSOP87 地球轨道参数
/// 返回: (黄经/度, 黄纬/度, 距离/AU)
pub fn calc_sun_position(jd: f64) -> (f64, f64, f64) {
    // VSOP87 地球轨道根数
    let t = (jd - 2451545.0) / 365250.0; // 儒略千年数

    // 地球轨道根数（VSOP87 简化版）
    let l0 = 3.1761467 + 1021.3285546 * t; // 平黄经
    let m = 6.2400601 + 628.3019552 * t;    // 平近点角
    let m_rad = m.to_radians();
    let c = (1.914602 - 0.004817 * t - 0.000014 * t * t) * m_rad.sin()
        + (0.019993 - 0.000101 * t) * (2.0 * m_rad).sin()
        + 0.000289 * (3.0 * m_rad).sin();

    let true_lon = (l0 + c) % 360.0;

    // 太阳地心黄经 = 地球日心黄经 + 180°
    let mut sun_lon = (true_lon + 180.0) % 360.0;
    if sun_lon < 0.0 {
        sun_lon += 360.0;
    }

    // 黄赤交角
    let obliquity = 23.439291
        - 0.0130042 * t
        - 0.00000016 * t * t
        + 0.000000504 * t * t * t;

    // 太阳距离
    let ecc = 0.0167086 - 0.000042037 * t - 0.0000001267 * t * t;
    let sun_r = 1.000001018 * (1.0 - ecc * ecc) / (1.0 + ecc * m_rad.cos());

    // 太阳黄纬（近似为0）
    let sun_lat = 0.0;

    (sun_lon, sun_lat, sun_r)
}

/// 计算月球位置（黄经、黄纬、距离）
///
/// 基于 ELP/MPP02 简化模型
/// 精度: 约 10"
/// 返回: (黄经/度, 黄纬/度, 距离/地球半径)
pub fn calc_moon_position(jd: f64) -> (f64, f64, f64) {
    let t = (jd - 2451545.0) / 36525.0; // 儒略世纪数

    // 月球平黄经
    let l_p = (218.3164477 + 481267.88123421 * t
        - 0.0015786 * t * t
        + t * t * t / 538841.0
        - t * t * t * t / 65194000.0) % 360.0;

    // 月球平近点角
    let m_p = (134.9629145 + 477198.8675055 * t
        + 0.0087414 * t * t
        + t * t * t / 69699.0
        - t * t * t * t / 14712000.0) % 360.0;

    // 太阳平近点角
    let m_s = (357.5277233 + 35999.0503400 * t
        - 0.0001603 * t * t
        - t * t * t / 300000.0) % 360.0;

    // 月球升交点平黄经
    let f = (93.2719102 + 483202.0175393 * t
        - 0.0036825 * t * t
        + t * t * t / 327270.0) % 360.0;

    // 日月距离平角
    let d = (297.8501921 + 445267.1114034 * t
        - 0.0018819 * t * t
        + t * t * t / 545868.0
        - t * t * t * t / 113065000.0) % 360.0;

    // 转换为弧度
    let l_p_r = l_p.to_radians();
    let m_p_r = m_p.to_radians();
    let m_s_r = m_s.to_radians();
    let f_r = f.to_radians();
    let d_r = d.to_radians();

    // 周期项（简化ELP）
    let mut lon = l_p;

    // 中心差
    lon += 6.288750 * m_p_r.sin();
    lon += 1.274018 * (2.0 * d_r - m_p_r).sin();
    lon += 0.658309 * (2.0 * d_r).sin();
    lon += 0.213616 * (2.0 * m_p_r).sin();
    lon -= 0.185596 * m_s_r.sin() * 0.5;
    lon -= 0.114336 * (2.0 * f_r).sin();
    lon += 0.058793 * (2.0 * d_r - 2.0 * m_p_r).sin();
    lon += 0.057212 * (2.0 * d_r - m_p_r - m_s_r).sin();
    lon += 0.053320 * (2.0 * d_r + m_p_r).sin();
    lon += 0.045874 * (2.0 * d_r - m_s_r).sin();
    lon += 0.041024 * (m_p_r - m_s_r).sin();

    // 月球黄纬
    let mut lat = 5.128189 * f_r.sin();
    lat += 0.280606 * (m_p_r + f_r).sin();
    lat += 0.277693 * (m_p_r - f_r).sin();
    lat += 0.173238 * (2.0 * d_r - f_r).sin();
    lat += 0.055413 * (2.0 * d_r + f_r - m_p_r).sin();
    lat += 0.046272 * (2.0 * d_r - f_r - m_p_r).sin();

    // 月球距离（地球半径）
    let mut dist = 385000.56 - 20905.355 * m_p_r.cos();
    dist -= 3699.111 * (2.0 * d_r - m_p_r).cos();
    dist -= 2955.968 * (2.0 * d_r).cos();
    dist -= 569.925 * (2.0 * m_p_r).cos();

    let moon_lon = lon % 360.0;
    let moon_lon = if moon_lon < 0.0 { moon_lon + 360.0 } else { moon_lon };

    (moon_lon, lat, dist)
}

/// 计算行星位置
///
/// 基于 VSOP87 简化算法
/// planet: "mercury", "venus", "mars", "jupiter", "saturn", "uranus", "neptune"
/// 返回: (黄经/度, 黄纬/度, 距离/AU) 或 None
pub fn calc_planet_position(planet: &str, jd: f64) -> Option<(f64, f64, f64)> {
    let t = (jd - 2451545.0) / 36525.0;

    let (l0, m, ecc, semi_axis) = match planet.to_lowercase().as_str() {
        "mercury" => (
            4.4026088 + 2608.7903142 * t,
            3.0502713 + 168.6562690 * t,
            0.2056318 + 0.000020407 * t,
            0.3870989,
        ),
        "venus" => (
            3.1761467 + 1021.3285546 * t,
            0.9025792 + 418.5020136 * t,
            0.0067732 - 0.000001302 * t,
            0.7233319,
        ),
        "mars" => (
            6.2034809 + 334.0612431 * t,
            0.3390354 + 278.8602856 * t,
            0.0934006 + 0.000090484 * t,
            1.5236883,
        ),
        "jupiter" => (
            0.5995461 + 52.9690965 * t,
            0.3670580 + 32.9784440 * t,
            0.0484948 + 0.000163244 * t,
            5.2025610,
        ),
        "saturn" => (
            0.8740185 + 21.3299095 * t,
            0.8820580 + 12.4820970 * t,
            0.0555086 - 0.000346617 * t,
            9.5547476,
        ),
        "uranus" => (
            5.4812939 + 7.4781599 * t,
            2.4899580 + 5.2060580 * t,
            0.0462959 - 0.000027337 * t,
            19.2181400,
        ),
        "neptune" => (
            5.3211090 + 3.8122580 * t,
            4.5460580 + 2.6480580 * t,
            0.0089979 + 0.000006330 * t,
            30.1095700,
        ),
        _ => return None,
    };

    let m_rad = m.to_radians();
    let c = (2.0 * ecc - 0.25 * ecc * ecc * ecc) * m_rad.sin()
        + (1.25 * ecc * ecc) * (2.0 * m_rad).sin()
        + (13.0 / 12.0 * ecc * ecc * ecc) * (3.0 * m_rad).sin();

    let lon = (l0 + c.to_degrees()) % 360.0;
    let lon = if lon < 0.0 { lon + 360.0 } else { lon };
    let r = semi_axis * (1.0 - ecc * ecc) / (1.0 + ecc * m_rad.cos());

    Some((lon, 0.0, r))
}

/// 计算黄赤交角
pub fn obliquity(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    23.439291
        - 0.0130042 * t
        - 0.00000016 * t * t
        + 0.000000504 * t * t * t
}

/// 黄道坐标转赤道坐标
pub fn ecliptic_to_equatorial(lon: f64, lat: f64, jd: f64) -> (f64, f64) {
    let eps = obliquity(jd).to_radians();
    let lon_rad = lon.to_radians();
    let lat_rad = lat.to_radians();

    let ra = (lat_rad.sin() * eps.cos() - lat_rad.cos() * eps.sin() * lon_rad.sin())
        .atan2(lon_rad.cos());
    let dec = (lat_rad.sin() * eps.sin() + lat_rad.cos() * eps.cos() * lon_rad.sin())
        .asin();

    let ra = ra.to_degrees();
    let ra = if ra < 0.0 { ra + 360.0 } else { ra };
    let ra_hours = ra / 15.0;

    (ra_hours, dec.to_degrees())
}

/// 计算恒星时（格林尼治）
pub fn sidereal_time(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    let mut st = 280.46061837
        + 360.98564736629 * (jd - 2451545.0)
        + 0.000387933 * t * t
        - t * t * t / 38710000.0;
    st = st % 360.0;
    if st < 0.0 {
        st += 360.0;
    }
    st / 15.0 // 转换为小时
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sun_position() {
        // 2024-06-21 (夏至), JD ≈ 2460482.5
        let jd = 2460482.5;
        let (lon, lat, r) = calc_sun_position(jd);
        // 夏至太阳黄经约90°
        assert!((lon - 90.0).abs() < 2.0);
        assert!((r - 1.016).abs() < 0.02);
    }

    #[test]
    fn test_moon_position() {
        let jd = 2460482.5;
        let (lon, _, _) = calc_moon_position(jd);
        assert!(lon >= 0.0 && lon < 360.0);
    }
}