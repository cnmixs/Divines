// 星阙 Horosa - 寿星天文历（万年历）模块
// 参考原项目: sxwnl (sxwnl-cpp / SharpSxwnl)
//
// 寿星天文历是一款采用现代天文算法制作的天文、历算程序：
// - 公历、农历、回历三历转换
// - 精确的日月食计算
// - 行星、恒星星历表计算
// - 公元前4712年到公元9999年的公历查询
// - -3000年至+3000年的农历查询
// - 二十四节气精确计算
// - 干支纪年、纪月、纪日、纪时

pub mod julian;
pub mod vsop87;
pub mod syzygy;
pub mod lunar;
pub mod calendar;
pub mod eclipse;
pub mod data;

use horosa_core::*;

/// 寿星天文历计算引擎
///
/// 提供完整的万年历功能，所有排盘系统（八字、紫微等）均调用此引擎
pub struct Sxwnl {
    /// 儒略日计算器
    pub jd: julian::JulianDay,
    /// 朔望计算器
    pub syzygy: syzygy::SyzygyCalc,
    /// 农历转换器
    pub lunar: lunar::LunarCalc,
    /// 日历转换器
    pub calendar: calendar::CalendarCalc,
    /// 日月食计算器
    pub eclipse: eclipse::EclipseCalc,
}

impl Sxwnl {
    /// 创建新的万年历引擎
    pub fn new() -> Self {
        Self {
            jd: julian::JulianDay::new(),
            syzygy: syzygy::SyzygyCalc::new(),
            lunar: lunar::LunarCalc::new(),
            calendar: calendar::CalendarCalc::new(),
            eclipse: eclipse::EclipseCalc::new(),
        }
    }

    /// 计算儒略日
    pub fn to_jd(&self, year: i32, month: u32, day: f64) -> f64 {
        julian::JulianDay::to_jd(year, month, day)
    }

    /// 公历转农历
    pub fn solar_to_lunar(&self, year: i32, month: u32, day: u32) -> LunarDate {
        self.lunar.solar_to_lunar(year, month, day)
    }

    /// 农历转公历
    pub fn lunar_to_solar(&self, lunar_year: i32, lunar_month: u32, lunar_day: u32, is_leap: bool) -> (i32, u32, u32) {
        self.lunar.lunar_to_solar(lunar_year, lunar_month, lunar_day, is_leap)
    }

    /// 获取某年节气列表
    pub fn get_year_jieqi(&self, year: i32) -> Vec<JieQi> {
        self.syzygy.calc_year_jieqi(year)
    }

    /// 获取某年朔日列表
    pub fn get_year_shuo(&self, year: i32) -> Vec<f64> {
        self.syzygy.calc_year_shuo(year)
    }

    /// 公历转回历
    pub fn solar_to_islamic(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        self.calendar.solar_to_islamic(year, month, day)
    }

    /// 回历转公历
    pub fn islamic_to_solar(&self, year: i32, month: u32, day: u32) -> (i32, u32, u32) {
        self.calendar.islamic_to_solar(year, month, day)
    }

    /// 计算日月食
    pub fn calc_eclipses(&self, year: i32) -> Vec<eclipse::EclipseInfo> {
        self.eclipse.calc_year_eclipses(year)
    }

    /// 获取干支纪年
    pub fn get_year_ganzhi(&self, year: i32) -> String {
        self.calendar.get_year_ganzhi(year)
    }

    /// 获取生肖
    pub fn get_zodiac(&self, year: i32) -> String {
        self.calendar.get_zodiac(year)
    }

    /// 获取年号
    pub fn get_nianhao(&self, year: i32) -> Option<String> {
        self.calendar.get_nianhao(year)
    }

    /// 获取城市经纬度
    pub fn get_city_coords(&self, name: &str) -> Option<(f64, f64)> {
        self.calendar.get_city_coords(name)
    }

    /// 计算太阳位置
    pub fn calc_sun_position(&self, jd: f64) -> (f64, f64, f64) {
        vsop87::calc_sun_position(jd)
    }

    /// 计算月亮位置
    pub fn calc_moon_position(&self, jd: f64) -> (f64, f64, f64) {
        vsop87::calc_moon_position(jd)
    }

    /// 计算行星位置
    pub fn calc_planet_position(&self, planet: &str, jd: f64) -> Option<(f64, f64, f64)> {
        vsop87::calc_planet_position(planet, jd)
    }
}

impl Default for Sxwnl {
    fn default() -> Self {
        Self::new()
    }
}