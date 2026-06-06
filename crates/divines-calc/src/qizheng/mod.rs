// Divines - 七政四余计算引擎
// 参考原项目: vendor/kinastro/astro/qizheng/calculator.py
// 参考原项目: vendor/kinastro/astro/qizheng/constants.py
// 参考原项目: vendor/kinastro/astro/qizheng/shensha.py
// 参考原项目: vendor/kinastro/astro/qizheng/qizheng_dasha.py
// 参考原项目: vendor/kinastro/astro/qizheng/zhangguo.py
// 参考原项目: astrostudysrv/astrostudycn/service/QizhengMoiraRuleService.java

use divines_core::qizheng::*;
use divines_core::chart::*;
use crate::sxwnl::vsop87;

// ============================================================
// 常量定义
// ============================================================

/// 七政（Seven Governors）
const SEVEN_GOVERNORS_NAMES: [(&str, &str, &str); 7] = [
    ("Sun", "太阳", "日"),
    ("Moon", "太阴", "月"),
    ("Mercury", "水星", "水"),
    ("Venus", "金星", "金"),
    ("Mars", "火星", "火"),
    ("Jupiter", "木星", "木"),
    ("Saturn", "土星", "土"),
];

/// 四余（Four Remainders）
const FOUR_REMAINDERS_NAMES: [(&str, &str, &str); 4] = [
    ("Rahu", "罗睺", "火"),
    ("Ketu", "计都", "土"),
    ("ZiQi", "紫气", "木"),
    ("YueBo", "月孛", "水"),
];

/// 十二地支
const EARTHLY_BRANCHES: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳",
    "午", "未", "申", "酉", "戌", "亥",
];

/// 十二宫名
const TWELVE_PALACES: [&str; 12] = [
    "命宫", "财帛宫", "兄弟宫", "田宅宫",
    "男女宫", "奴仆宫", "夫妻宫", "疾厄宫",
    "迁移宫", "官禄宫", "福德宫", "相貌宫",
];

/// 十二星次（中国黄道十二宫）
const TWELVE_SIGNS_CHINESE: [&str; 12] = [
    "戌宫(降娄)", "酉宫(大梁)", "申宫(实沈)", "未宫(鹑首)",
    "午宫(鹑火)", "巳宫(鹑尾)", "辰宫(寿星)", "卯宫(大火)",
    "寅宫(析木)", "丑宫(星纪)", "子宫(玄枵)", "亥宫(娵訾)",
];

/// 西方星座名
const TWELVE_SIGNS_WESTERN: [&str; 12] = [
    "白羊", "金牛", "双子", "巨蟹",
    "狮子", "处女", "天秤", "天蝎",
    "射手", "摩羯", "水瓶", "双鱼",
];

/// 星座五行属性（索引对应西方星座: 0=白羊, ..., 11=双鱼）
const ZODIAC_SIGN_ELEMENTS: [&str; 12] = [
    "火", "金", "水", "月", "日", "水",
    "金", "火", "木", "土", "土", "木",
];

/// 二十八宿（距星黄经基准）
const TWENTY_EIGHT_MANSIONS: [(&str, &str, &str, &str, f64); 28] = [
    // 东方青龙七宿
    ("角", "木", "蛟", "东方青龙", 203.8375),
    ("亢", "金", "龙", "东方青龙", 214.4899),
    ("氐", "土", "貉", "东方青龙", 225.0216),
    ("房", "日", "兔", "东方青龙", 242.9360),
    ("心", "月", "狐", "东方青龙", 249.7584),
    ("尾", "火", "虎", "东方青龙", 256.1517),
    ("箕", "水", "豹", "东方青龙", 271.2576),
    // 北方玄武七宿
    ("斗", "木", "獬", "北方玄武", 280.1775),
    ("牛", "金", "牛", "北方玄武", 304.0435),
    ("女", "土", "蝠", "北方玄武", 311.7193),
    ("虚", "日", "鼠", "北方玄武", 323.3912),
    ("危", "月", "燕", "北方玄武", 333.3486),
    ("室", "火", "猪", "北方玄武", 353.49),
    ("壁", "水", "貐", "北方玄武", 9.1522),
    // 西方白虎七宿
    ("奎", "木", "狼", "西方白虎", 22.3721),
    ("娄", "金", "狗", "西方白虎", 33.9661),
    ("胃", "土", "雉", "西方白虎", 46.9312),
    ("昴", "日", "鸡", "西方白虎", 59.4080),
    ("毕", "月", "乌", "西方白虎", 68.4612),
    ("觜", "火", "猴", "西方白虎", 83.7030),
    ("参", "水", "猿", "西方白虎", 84.6775),
    // 南方朱雀七宿
    ("井", "木", "犴", "南方朱雀", 95.2980),
    ("鬼", "金", "羊", "南方朱雀", 125.7246),
    ("柳", "土", "獐", "南方朱雀", 130.3005),
    ("星", "日", "马", "南方朱雀", 147.2753),
    ("张", "月", "鹿", "南方朱雀", 155.6874),
    ("翼", "火", "蛇", "南方朱雀", 173.6856),
    ("轸", "水", "蚓", "南方朱雀", 190.7218),
];

/// 星曜年限（大运行限）
const PLANET_PERIOD_YEARS: [(&str, u8); 7] = [
    ("太阳", 19),
    ("太阴", 25),
    ("火星", 7),
    ("水星", 20),
    ("木星", 12),
    ("金星", 15),
    ("土星", 22),
];

/// 十二宫主星（地支索引 → 主星名）
const BRANCH_LORD: [(&str, usize); 12] = [
    ("土星", 0),  // 子 → Aquarius
    ("土星", 1),  // 丑 → Capricorn
    ("木星", 2),  // 寅 → Sagittarius
    ("火星", 3),  // 卯 → Scorpio
    ("金星", 4),  // 辰 → Libra
    ("水星", 5),  // 巳 → Virgo
    ("太阳", 6),  // 午 → Leo
    ("太阴", 7),  // 未 → Cancer
    ("水星", 8),  // 申 → Gemini
    ("金星", 9),  // 酉 → Taurus
    ("火星", 10), // 戌 → Aries
    ("木星", 11), // 亥 → Pisces
];

// ============================================================
// 辅助函数
// ============================================================

/// 标准化角度到 0-360
fn normalize_degree(deg: f64) -> f64 {
    let mut d = deg % 360.0;
    if d < 0.0 {
        d += 360.0;
    }
    d
}

/// 黄经 → 星座索引 (0-11)
fn degree_to_sign_index(deg: f64) -> usize {
    (normalize_degree(deg) / 30.0) as usize % 12
}

/// 黄经 → 星座内度数
fn degree_to_sign_degree(deg: f64) -> f64 {
    normalize_degree(deg) % 30.0
}

/// 星座索引 → 地支索引
/// 戌(10)=0°, 酉(9)=30°, ..., 亥(11)=330°
fn sign_index_to_branch(sign_idx: usize) -> usize {
    (10 + 12 - sign_idx) % 12
}

/// 地支索引 → 星座索引
fn branch_to_sign_index(branch: usize) -> usize {
    (10 + 12 - branch) % 12
}

/// 地支索引 → 黄经起始度
/// 戌(10)=0°-30°, 酉(9)=30°-60°, ...
fn branch_to_cusp(branch: usize) -> f64 {
    let sign_index = (10 + 12 - branch) % 12;
    sign_index as f64 * 30.0
}

/// 根据出生时间取得时辰地支索引
fn get_hour_branch(hour: u8, minute: u8) -> usize {
    let total_minutes = hour as i32 * 60 + minute as i32;
    if total_minutes < 60 || total_minutes >= 23 * 60 {
        return 0; // 子时 (23:00–01:00)
    }
    ((total_minutes + 60) / 120) as usize % 12
}

/// 根据太阳黄经取得节气月 (1-12)
fn get_solar_month(sun_longitude: f64) -> i32 {
    ((sun_longitude - 315.0).rem_euclid(360.0) / 30.0) as i32 + 1
}

/// 计算命宫地支索引
fn get_ming_gong_branch(solar_month: i32, hour_branch: usize) -> usize {
    ((1 + solar_month as i32 - hour_branch as i32).rem_euclid(12)) as usize
}

/// 取得二十八宿信息
fn get_mansion_info(lon: f64) -> (String, f64, usize, f64) {
    let lon = normalize_degree(lon);
    let n = TWENTY_EIGHT_MANSIONS.len();
    for i in 0..n {
        let start = TWENTY_EIGHT_MANSIONS[i].4;
        let end = TWENTY_EIGHT_MANSIONS[(i + 1) % n].4;
        let width = normalize_degree(end - start);
        if start < end {
            if start <= lon && lon < end {
                return (TWENTY_EIGHT_MANSIONS[i].0.to_string(), lon - start, i, width);
            }
        } else {
            if lon >= start || lon < end {
                let deg_in = normalize_degree(lon - start);
                return (TWENTY_EIGHT_MANSIONS[i].0.to_string(), deg_in, i, width);
            }
        }
    }
    (TWENTY_EIGHT_MANSIONS[0].0.to_string(), 0.0, 0, 0.0)
}

/// 检查岐度（宫/宿交界 ±1.5°）
fn check_qidu(sign_degree: f64, mansion_degree: f64, mansion_width: f64) -> bool {
    let qidu_threshold = 1.5;
    if sign_degree <= qidu_threshold || sign_degree >= (30.0 - qidu_threshold) {
        return true;
    }
    if mansion_degree <= qidu_threshold || mansion_degree >= (mansion_width - qidu_threshold) {
        return true;
    }
    false
}

/// 计算儒略日
fn julian_day(year: i32, month: u8, day: u8, hour: f64) -> f64 {
    let a = (14 - month as i32) / 12;
    let y = year as i32 + 4800 - a;
    let m = month as i32 + 12 * a - 3;

    let jd = day as i32
        + (153 * m + 2) / 5
        + 365 * y
        + y / 4
        - y / 100
        + y / 400
        - 32045;

    jd as f64 + (hour - 12.0) / 24.0
}

/// 计算上升点（简化算法）
fn calc_ascendant(jd: f64, latitude: f64, longitude: f64) -> (f64, f64) {
    let st = vsop87::sidereal_time(jd);
    let lst = st + longitude / 15.0;
    // 简化上升点计算：使用本地恒星时和纬度
    let lst_deg = (lst * 15.0) % 360.0;
    let lat_rad = latitude.to_radians();
    let obl = vsop87::obliquity(jd).to_radians();

    let asc_rad = (lst_deg.to_radians().cos())
        .atan2(
            -(lst_deg.to_radians().sin()) * obl.cos()
                - lat_rad.tan() * obl.sin()
        );
    let asc = normalize_degree(asc_rad.to_degrees());

    let mc_rad = (lst_deg.to_radians().sin()).atan2(lst_deg.to_radians().cos() * obl.cos());
    let mc = normalize_degree(mc_rad.to_degrees());

    (asc, mc)
}

/// 计算行星速度（近似，通过两次计算差值）
fn calc_planet_speed(planet_name: &str, jd: f64) -> f64 {
    let dt = 0.1; // 0.1天
    let lon1 = match planet_name {
        "Sun" => vsop87::calc_sun_position(jd).0,
        "Moon" => vsop87::calc_moon_position(jd).0,
        "Mercury" => vsop87::calc_planet_position("mercury", jd).map(|p| p.0).unwrap_or(0.0),
        "Venus" => vsop87::calc_planet_position("venus", jd).map(|p| p.0).unwrap_or(0.0),
        "Mars" => vsop87::calc_planet_position("mars", jd).map(|p| p.0).unwrap_or(0.0),
        "Jupiter" => vsop87::calc_planet_position("jupiter", jd).map(|p| p.0).unwrap_or(0.0),
        "Saturn" => vsop87::calc_planet_position("saturn", jd).map(|p| p.0).unwrap_or(0.0),
        _ => 0.0,
    };
    let lon2 = match planet_name {
        "Sun" => vsop87::calc_sun_position(jd + dt).0,
        "Moon" => vsop87::calc_moon_position(jd + dt).0,
        "Mercury" => vsop87::calc_planet_position("mercury", jd + dt).map(|p| p.0).unwrap_or(0.0),
        "Venus" => vsop87::calc_planet_position("venus", jd + dt).map(|p| p.0).unwrap_or(0.0),
        "Mars" => vsop87::calc_planet_position("mars", jd + dt).map(|p| p.0).unwrap_or(0.0),
        "Jupiter" => vsop87::calc_planet_position("jupiter", jd + dt).map(|p| p.0).unwrap_or(0.0),
        "Saturn" => vsop87::calc_planet_position("saturn", jd + dt).map(|p| p.0).unwrap_or(0.0),
        _ => 0.0,
    };
    normalize_degree(lon2 - lon1) / dt
}

// ============================================================
// 七政四余计算器
// ============================================================

/// 七政四余计算器
#[derive(Default)]
pub struct QizhengCalc;

impl QizhengCalc {
    /// 计算完整七政四余星盘
    pub fn calculate_chart(
        &self,
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        timezone: f64,
        latitude: f64,
        longitude: f64,
        place_name: &str,
        gender: &str,
    ) -> QizhengChart {
        // 计算儒略日（UTC）
        let decimal_hour = hour as f64 + minute as f64 / 60.0 + second as f64 / 3600.0 - timezone;
        let jd = julian_day(year, month, day, decimal_hour);

        // 计算上升点和中天
        let (ascendant, midheaven) = calc_ascendant(jd, latitude, longitude);

        // 计算行星位置
        let mut planets = Vec::new();

        // ---- 七政 ----
        for (id, name, element) in &SEVEN_GOVERNORS_NAMES {
            let (lon, lat, _dist) = match *id {
                "Sun" => vsop87::calc_sun_position(jd),
                "Moon" => vsop87::calc_moon_position(jd),
                "Mercury" => vsop87::calc_planet_position("mercury", jd).unwrap_or((0.0, 0.0, 0.0)),
                "Venus" => vsop87::calc_planet_position("venus", jd).unwrap_or((0.0, 0.0, 0.0)),
                "Mars" => vsop87::calc_planet_position("mars", jd).unwrap_or((0.0, 0.0, 0.0)),
                "Jupiter" => vsop87::calc_planet_position("jupiter", jd).unwrap_or((0.0, 0.0, 0.0)),
                "Saturn" => vsop87::calc_planet_position("saturn", jd).unwrap_or((0.0, 0.0, 0.0)),
                _ => (0.0, 0.0, 0.0),
            };

            let lon = normalize_degree(lon);
            let speed = calc_planet_speed(id, jd);
            let is_retrograde = speed < 0.0;
            let (ra, decl) = vsop87::ecliptic_to_equatorial(lon, lat, jd);
            let sign_idx = degree_to_sign_index(lon);
            let sign_lon = degree_to_sign_degree(lon);
            let sign_element = ZODIAC_SIGN_ELEMENTS[sign_idx].to_string();
            let (mansion_name, mansion_deg, _m_idx, m_width) = get_mansion_info(lon);
            let is_qidu = check_qidu(sign_lon, mansion_deg, m_width);

            planets.push(QizhengPlanetPosition {
                id: id.to_string(),
                name: name.to_string(),
                planet_type: "qizheng".to_string(),
                lon,
                lat,
                ra,
                decl,
                sign: EARTHLY_BRANCHES[sign_index_to_branch(sign_idx)].to_string(),
                sign_lon,
                house: String::new(), // 稍后填充
                speed,
                is_retrograde,
                su28: Some(mansion_name),
                su28_degree: mansion_deg,
                element: element.to_string(),
                sign_element,
                is_qidu,
                altitude: 0.0,
            });
        }

        // ---- 四余 ----
        // 计都 = 升交点 (Mean Node)
        let moon_lon = vsop87::calc_moon_position(jd).0;
        let moon_lat = vsop87::calc_moon_position(jd).1;
        // 升交点计算：使用月球轨道简化的升交点黄经
        let t = (jd - 2451545.0) / 36525.0;
        let mean_node = normalize_degree(
            125.04452 - 1934.136261 * t + 0.0020708 * t * t + t * t * t / 450000.0
        );
        let ketu_lon = normalize_degree(mean_node);

        // 罗睺 = 降交点 = 计都 + 180°
        let rahu_lon = normalize_degree(ketu_lon + 180.0);

        // 四余行星星表
        let siyu_data: [(&str, &str, &str, f64, f64); 4] = [
            ("Rahu", "罗睺", "火", rahu_lon, 0.0),
            ("Ketu", "计都", "土", ketu_lon, 0.0),
            ("ZiQi", "紫气", "木", normalize_degree(vsop87::calc_sun_position(jd).0 + 30.0), 0.0),
            ("YueBo", "月孛", "水", normalize_degree(moon_lon + 180.0), 0.0),
        ];

        for (id, name, element, lon, lat) in &siyu_data {
            let lon = *lon;
            let lat = *lat;
            let (ra, decl) = vsop87::ecliptic_to_equatorial(lon, lat, jd);
            let sign_idx = degree_to_sign_index(lon);
            let sign_lon = degree_to_sign_degree(lon);
            let sign_element = ZODIAC_SIGN_ELEMENTS[sign_idx].to_string();
            let (mansion_name, mansion_deg, _m_idx, m_width) = get_mansion_info(lon);
            let is_qidu = check_qidu(sign_lon, mansion_deg, m_width);

            planets.push(QizhengPlanetPosition {
                id: id.to_string(),
                name: name.to_string(),
                planet_type: "siyu".to_string(),
                lon,
                lat,
                ra,
                decl,
                sign: EARTHLY_BRANCHES[sign_index_to_branch(sign_idx)].to_string(),
                sign_lon,
                house: String::new(),
                speed: 0.0,
                is_retrograde: false,
                su28: Some(mansion_name),
                su28_degree: mansion_deg,
                element: element.to_string(),
                sign_element,
                is_qidu,
                altitude: 0.0,
            });
        }

        // ---- 计算命宫 ----
        let sun_lon = planets[0].lon;
        let solar_month = get_solar_month(sun_lon);
        let hour_branch = get_hour_branch(hour, minute);
        // 命宫地支：依上升点所在星座确定
        let asc_sign_idx = degree_to_sign_index(ascendant);
        let ming_gong_branch = (10 + 12 - asc_sign_idx) % 12;

        // ---- 建立宫位 ----
        let direction: i32 = if gender == "female" { -1 } else { 1 };
        let mut branch_to_palace: [usize; 12] = [0; 12];
        for palace_idx in 0..12usize {
            let branch = ((ming_gong_branch as i32 + direction * palace_idx as i32).rem_euclid(12)) as usize;
            branch_to_palace[branch] = palace_idx;
        }

        let mut houses: Vec<QizhengHouse> = Vec::new();
        for palace_idx in 0..12usize {
            let branch = ((ming_gong_branch as i32 + direction * palace_idx as i32).rem_euclid(12)) as usize;
            let cusp = branch_to_cusp(branch);
            let (mansion_name, mansion_deg, _, _) = get_mansion_info(cusp);
            houses.push(QizhengHouse {
                number: palace_idx as u8,
                name: TWELVE_PALACES[palace_idx].to_string(),
                lon: cusp,
                sign: EARTHLY_BRANCHES[branch].to_string(),
                branch,
                su28: mansion_name,
                su28_degree: mansion_deg,
                planets: Vec::new(),
            });
        }

        // 将行星分配到宫位
        for planet in planets.iter_mut() {
            let planet_sign_idx = degree_to_sign_index(planet.lon);
            let planet_branch = (10 + 12 - planet_sign_idx) % 12;
            let palace_idx = branch_to_palace[planet_branch];
            planet.house = TWELVE_PALACES[palace_idx].to_string();
            houses[palace_idx].planets.push(planet.name.clone());
        }

        let houses_arr: [QizhengHouse; 12] = [
            houses[0].clone(), houses[1].clone(), houses[2].clone(), houses[3].clone(),
            houses[4].clone(), houses[5].clone(), houses[6].clone(), houses[7].clone(),
            houses[8].clone(), houses[9].clone(), houses[10].clone(), houses[11].clone(),
        ];

        // ---- 命度/身度 ----
        let (life_su, life_su_deg, _, _) = get_mansion_info(ascendant);
        let life_degree = ascendant;
        // 身度：月亮所在宿
        let moon_planet = &planets[1]; // Moon is second
        let (body_su, _, _, _) = get_mansion_info(moon_planet.lon);
        let body_degree = Some(moon_planet.lon);

        // ---- 身宫 ----
        // 身宫 = 月亮所在宫位
        let shen_gong = branch_to_palace[sign_index_to_branch(degree_to_sign_index(moon_planet.lon))];

        // ---- 计算大运 ----
        let birth_year = year;
        let da_yun = self.calc_da_yun(birth_year, ming_gong_branch, gender, &houses);

        // ---- 洞微大限 ----
        let dong_wei = self.calc_dong_wei(ascendant, &houses);

        // ---- 神煞 ----
        let shen_sha = self.calc_shen_sha(year, solar_month, jd, hour_branch, timezone, ming_gong_branch);

        // ---- 格局检测 ----
        let patterns = self.calc_patterns(&planets, &houses, ming_gong_branch);

        // ---- Moira规则 ----
        let moira_rules = self.calc_moira_rules(&planets, &houses, ming_gong_branch, &shen_sha);

        // 构建出生信息
        let birth = BirthInfo {
            datetime: chrono::DateTime::from_timestamp(
                (jd - 2440587.5) as i64 * 86400,
                0,
            ).unwrap_or_default(),
            local_datetime: format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                year, month, day, hour, minute, second
            ),
            location: GeoPosition {
                latitude,
                longitude,
                altitude: 0.0,
                timezone_offset: timezone,
                place_name: if place_name.is_empty() { None } else { Some(place_name.to_string()) },
                country: None,
            },
            gender: if gender == "female" { Gender::Female } else { Gender::Male },
            name: None,
        };

        QizhengChart {
            birth,
            planets,
            su28: self.build_su28_list(jd),
            houses: houses_arr,
            life_degree,
            life_su,
            body_degree,
            body_su: Some(body_su),
            ming_gong: ming_gong_branch,
            shen_gong,
            dong_wei,
            da_yun,
            patterns,
            shen_sha,
            moira_rules,
            ascendant,
            midheaven,
            solar_month,
            hour_branch,
            gender: gender.to_string(),
            julian_day: jd,
        }
    }

    /// 构建28宿列表
    fn build_su28_list(&self, jd: f64) -> Vec<Su28Info> {
        let n = TWENTY_EIGHT_MANSIONS.len();
        let mut result = Vec::with_capacity(n);
        for i in 0..n {
            let (name, element, animal, group, lon) = TWENTY_EIGHT_MANSIONS[i];
            let (ra, decl) = vsop87::ecliptic_to_equatorial(lon, 0.0, jd);
            let sign_idx = degree_to_sign_index(lon);
            let end = TWENTY_EIGHT_MANSIONS[(i + 1) % n].4;
            let width = normalize_degree(end - lon);
            result.push(Su28Info {
                name: name.to_string(),
                element: element.to_string(),
                animal: animal.to_string(),
                group: group.to_string(),
                ra,
                decl,
                lon,
                sign: TWELVE_SIGNS_CHINESE[sign_idx].to_string(),
                width,
            });
        }
        result
    }

    /// 计算大运（年限大运）
    pub fn calc_da_yun(
        &self,
        birth_year: i32,
        ming_gong_branch: usize,
        gender: &str,
        houses: &[QizhengHouse],
    ) -> Vec<QizhengDaYun> {
        let direction: i32 = if gender == "female" { 1 } else { -1 };
        let mut results = Vec::new();
        let mut age: u8 = 0;

        // 建立 branch → house 映射
        let mut branch_to_house: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (i, h) in houses.iter().enumerate() {
            branch_to_house.insert(h.branch, i);
        }

        for i in 0..12usize {
            let branch = ((ming_gong_branch as i32 + direction * i as i32).rem_euclid(12)) as usize;
            let lord = BRANCH_LORD[branch].0;
            let years = PLANET_PERIOD_YEARS.iter()
                .find(|(name, _)| *name == lord)
                .map(|(_, y)| *y)
                .unwrap_or(10);
            let palace_name = if let Some(&idx) = branch_to_house.get(&branch) {
                TWELVE_PALACES[idx].to_string()
            } else {
                String::new()
            };
            let start_year = birth_year + age as i32;
            let end_year = start_year + years as i32 - 1;

            results.push(QizhengDaYun {
                planet: lord.to_string(),
                age_start: age,
                age_end: age + years - 1,
                years,
                palace_name,
                branch_name: EARTHLY_BRANCHES[branch].to_string(),
                start_year,
                end_year,
            });
            age += years;
        }

        results
    }

    /// 计算洞微大限
    pub fn calc_dong_wei(
        &self,
        ascendant: f64,
        houses: &[QizhengHouse],
    ) -> Vec<DongWeiLimit> {
        // 洞微大限：以命度为起点，以各宿宽度为限
        let (_, _, mansion_idx, _) = get_mansion_info(ascendant);
        let mut results = Vec::new();
        let mut age: u8 = 0;

        // 从命度宿开始，依次经历28宿
        for i in 0..28usize {
            let idx = (mansion_idx + i) % 28;
            let (name, _, _, _, _) = TWENTY_EIGHT_MANSIONS[idx];
            let end = TWENTY_EIGHT_MANSIONS[(idx + 1) % 28].4;
            let start = TWENTY_EIGHT_MANSIONS[idx].4;
            let width = normalize_degree(end - start);
            let years = (width / 2.0).ceil() as u8; // 每宿约半度一年

            // 查找对应宫位
            let house_idx = degree_to_sign_index(start);
            let house_branch = sign_index_to_branch(house_idx);
            let house_id = houses.iter()
                .position(|h| h.branch == house_branch)
                .unwrap_or(0);

            results.push(DongWeiLimit {
                age_start: age,
                age_end: age + years - 1,
                house: house_id,
                planet: None,
                description: format!("{}宿限", name),
            });
            age += years;
        }

        results
    }

    /// 计算神煞
    pub fn calc_shen_sha(
        &self,
        year: i32,
        solar_month: i32,
        julian_day: f64,
        hour_branch: usize,
        timezone: f64,
        ming_gong_branch: usize,
    ) -> Vec<QizhengShenSha> {
        let year_stem = (year - 4).rem_euclid(10) as usize;
        let year_branch = (year - 4).rem_euclid(12) as usize;
        let month_branch = (solar_month as usize + 1) % 12;
        let gzi = (6 * year_stem as i32 - 5 * year_branch as i32).rem_euclid(60) as usize;

        let mut items: Vec<QizhengShenSha> = Vec::new();
        let mut seen: std::collections::HashSet<(String, usize)> = std::collections::HashSet::new();

        let mut add = |name: &str, branch: usize, category: &str, source: &str| {
            let key = (name.to_string(), branch);
            if !seen.contains(&key) {
                seen.insert(key);
                items.push(QizhengShenSha {
                    name: name.to_string(),
                    category: category.to_string(),
                    branch,
                    branch_name: EARTHLY_BRANCHES[branch].to_string(),
                    house: None,
                    planet: None,
                    source: source.to_string(),
                    description: String::new(),
                });
            }
        };

        // ---- 地支系神煞 (年支查) ----
        // 红鸾
        let hongluan_branch = (12 - year_branch) % 12;
        add("红鸾", hongluan_branch, "吉", "地支");
        // 天喜
        add("天喜", (hongluan_branch + 6) % 12, "吉", "地支");
        // 劫杀
        let jiesha_map = [5, 2, 11, 8, 5, 2, 11, 8, 5, 2, 11, 8];
        add("劫杀", jiesha_map[year_branch], "凶", "地支");
        // 亡神
        let wangshen_map = [11, 8, 5, 2, 11, 8, 5, 2, 11, 8, 5, 2];
        add("亡神", wangshen_map[year_branch], "凶", "地支");
        // 咸池(桃花)
        let xianchi_map = [9, 6, 3, 0, 9, 6, 3, 0, 9, 6, 3, 0];
        add("咸池", xianchi_map[year_branch], "中", "地支");
        // 孤辰
        let guchen_map = [2, 2, 5, 5, 5, 8, 8, 8, 11, 11, 11, 2];
        add("孤辰", guchen_map[year_branch], "凶", "地支");
        // 寡宿
        let guasu_map = [10, 10, 1, 1, 1, 4, 4, 4, 7, 7, 7, 10];
        add("寡宿", guasu_map[year_branch], "凶", "地支");
        // 驿马
        let yima_map = [2, 11, 8, 5, 2, 11, 8, 5, 2, 11, 8, 5];
        add("驿马", yima_map[year_branch], "中", "地支");
        // 华盖
        let huagai_map = [4, 1, 10, 7, 4, 1, 10, 7, 4, 1, 10, 7];
        add("华盖", huagai_map[year_branch], "吉", "地支");
        // 将星
        let jiangxing_map = [0, 9, 6, 3, 0, 9, 6, 3, 0, 9, 6, 3];
        add("将星", jiangxing_map[year_branch], "吉", "地支");
        // 三刑
        let sanxing_map = [3, 10, 5, 6, 4, 8, 6, 1, 2, 9, 7, 11];
        add("三刑", sanxing_map[year_branch], "凶", "地支");
        // 六害
        let liuhai_map = [7, 6, 5, 4, 3, 2, 1, 0, 11, 10, 9, 8];
        add("六害", liuhai_map[year_branch], "凶", "地支");
        // 血刃
        let xueren_map = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 11];
        add("血刃", xueren_map[year_branch], "凶", "地支");
        // 的杀
        let desha_map = [5, 1, 9, 5, 1, 9, 5, 1, 9, 5, 1, 9];
        add("的杀", desha_map[year_branch], "凶", "地支");
        // 天哭
        let tianku_map = [6, 5, 4, 3, 2, 1, 0, 11, 10, 9, 8, 7];
        add("天哭", tianku_map[year_branch], "凶", "地支");
        // 披头
        let pitou_map = [4, 3, 2, 1, 0, 11, 10, 9, 8, 7, 6, 5];
        add("披头", pitou_map[year_branch], "凶", "地支");

        // ---- 天干系神煞 (年干查) ----
        // 禄勋
        let luxun_map = [2, 3, 5, 6, 5, 6, 8, 9, 11, 0];
        add("禄勋", luxun_map[year_stem], "吉", "天干");
        // 文昌
        let wenchang_map = [5, 6, 8, 9, 8, 9, 11, 10, 2, 3];
        add("文昌", wenchang_map[year_stem], "吉", "天干");
        // 阳刃
        let yangren_map = [3, 99, 6, 99, 6, 99, 9, 99, 0, 99];
        if yangren_map[year_stem] != 99 {
            add("阳刃", yangren_map[year_stem], "凶", "天干");
        }
        // 飞刃
        let feiren_map = [9, 10, 0, 1, 0, 1, 3, 4, 6, 7];
        add("飞刃", feiren_map[year_stem], "凶", "天干");
        // 天厨
        let tianchu_map = [5, 6, 0, 5, 6, 8, 2, 6, 9, 11];
        add("天厨", tianchu_map[year_stem], "吉", "天干");
        // 学堂
        let xuetang_map = [11, 6, 2, 9, 2, 9, 5, 0, 8, 3];
        add("学堂", xuetang_map[year_stem], "吉", "天干");

        // 返回结果
        items
    }

    /// 检测星盘格局（果老星宗）
    pub fn calc_patterns(
        &self,
        planets: &[QizhengPlanetPosition],
        houses: &[QizhengHouse],
        ming_gong_branch: usize,
    ) -> Vec<String> {
        let mut patterns = Vec::new();

        // 获取关键行星位置
        let sun = planets.iter().find(|p| p.id == "Sun");
        let moon = planets.iter().find(|p| p.id == "Moon");
        let jupiter = planets.iter().find(|p| p.id == "Jupiter");
        let venus = planets.iter().find(|p| p.id == "Venus");
        let mars = planets.iter().find(|p| p.id == "Mars");
        let saturn = planets.iter().find(|p| p.id == "Saturn");
        let mercury = planets.iter().find(|p| p.id == "Mercury");
        let rahu = planets.iter().find(|p| p.id == "Rahu");
        let ketu = planets.iter().find(|p| p.id == "Ketu");

        // 日月夹拱
        if let (Some(sun), Some(moon)) = (sun, moon) {
            let sun_branch = sign_index_to_branch(degree_to_sign_index(sun.lon));
            let moon_branch = sign_index_to_branch(degree_to_sign_index(moon.lon));

            // 日月夹命宫
            let diff1 = (sun_branch as i32 - ming_gong_branch as i32).rem_euclid(12);
            let diff2 = (moon_branch as i32 - ming_gong_branch as i32).rem_euclid(12);
            if (diff1 == 1 && diff2 == 11) || (diff1 == 11 && diff2 == 1) {
                patterns.push("日月夹命".to_string());
            }

            // 日月拱命
            if (diff1 == 4 && diff2 == 8) || (diff1 == 8 && diff2 == 4) {
                patterns.push("日月拱命".to_string());
            }

            // 日月同宫
            if sun_branch == moon_branch {
                patterns.push("日月同宫".to_string());
            }
        }

        // 五星聚会
        if let (Some(jup), Some(ven), Some(mar), Some(mer), Some(sat)) =
            (jupiter, venus, mars, mercury, saturn)
        {
            let branches: Vec<usize> = [jup, ven, mar, mer, sat].iter()
                .map(|p| sign_index_to_branch(degree_to_sign_index(p.lon)))
                .collect();
            let mut sorted = branches.clone();
            sorted.sort();
            sorted.dedup();

            if sorted.len() <= 3 {
                patterns.push("五星聚会".to_string());
            }
        }

        // 罗计拦截
        if let (Some(rahu), Some(ketu)) = (rahu, ketu) {
            let rahu_branch = sign_index_to_branch(degree_to_sign_index(rahu.lon));
            let ketu_branch = sign_index_to_branch(degree_to_sign_index(ketu.lon));
            if (rahu_branch == 6 && ketu_branch == 0) || (rahu_branch == 0 && ketu_branch == 6) {
                patterns.push("罗计拦截".to_string());
            }
        }

        // 日月包五星
        if let (Some(sun), Some(moon)) = (sun, moon) {
            let sun_lon = sun.lon;
            let moon_lon = moon.lon;
            let left = sun_lon.min(moon_lon);
            let right = sun_lon.max(moon_lon);
            let span = if right - left > 180.0 {
                // 跨越0度的情况
                let mut count = 0;
                for p in planets.iter().filter(|p| p.planet_type == "qizheng" && p.id != "Sun" && p.id != "Moon") {
                    if p.lon >= right || p.lon <= left {
                        count += 1;
                    }
                }
                count >= 3
            } else {
                let mut count = 0;
                for p in planets.iter().filter(|p| p.planet_type == "qizheng" && p.id != "Sun" && p.id != "Moon") {
                    if p.lon >= left && p.lon <= right {
                        count += 1;
                    }
                }
                count >= 3
            };
            if span {
                patterns.push("日月包五星".to_string());
            }
        }

        // 命宫有星
        let ming_house = houses.iter().find(|h| h.branch == ming_gong_branch);
        if let Some(ming_h) = ming_house {
            if !ming_h.planets.is_empty() {
                patterns.push(format!("命宫聚星({})", ming_h.planets.join("、")));
            }
        }

        patterns
    }

    /// Moira规则引擎（果老星宗评断）
    pub fn calc_moira_rules(
        &self,
        planets: &[QizhengPlanetPosition],
        houses: &[QizhengHouse],
        ming_gong_branch: usize,
        shen_sha: &[QizhengShenSha],
    ) -> Vec<MoiraRuleResult> {
        let mut rules = Vec::new();

        // 星格检查
        for planet in planets {
            // 入垣检查
            let is_ru_yuan = self.check_ru_yuan(planet);
            if is_ru_yuan {
                rules.push(MoiraRuleResult {
                    rule_name: format!("{}入垣", planet.name),
                    signal: "吉".to_string(),
                    planets: vec![planet.name.clone()],
                    description: format!("{}在本垣，力量强旺", planet.name),
                });
            }

            // 升殿检查
            let is_sheng_dian = self.check_sheng_dian(planet);
            if is_sheng_dian {
                rules.push(MoiraRuleResult {
                    rule_name: format!("{}升殿", planet.name),
                    signal: "吉".to_string(),
                    planets: vec![planet.name.clone()],
                    description: format!("{}在升殿宿，地位尊崇", planet.name),
                });
            }

            // 逆行检查
            if planet.is_retrograde {
                rules.push(MoiraRuleResult {
                    rule_name: format!("{}逆行", planet.name),
                    signal: "凶".to_string(),
                    planets: vec![planet.name.clone()],
                    description: format!("{}逆行，力量减弱", planet.name),
                });
            }
        }

        // 命宫强/弱检查
        if let Some(ming_h) = houses.iter().find(|h| h.branch == ming_gong_branch) {
            let planet_count = ming_h.planets.len();
            if planet_count >= 3 {
                rules.push(MoiraRuleResult {
                    rule_name: "命宫强".to_string(),
                    signal: "吉".to_string(),
                    planets: ming_h.planets.clone(),
                    description: format!("命宫有{}颗星入驻，命主有力", planet_count),
                });
            } else if planet_count == 0 {
                rules.push(MoiraRuleResult {
                    rule_name: "命宫空".to_string(),
                    signal: "平".to_string(),
                    planets: vec![],
                    description: "命宫无星，需看命主星".to_string(),
                });
            }
        }

        // 财官两旺检查
        let cai_house = houses.iter().find(|h| h.name == "财帛宫");
        let guan_house = houses.iter().find(|h| h.name == "官禄宫");
        if let (Some(cai), Some(guan)) = (cai_house, guan_house) {
            if !cai.planets.is_empty() && !guan.planets.is_empty() {
                rules.push(MoiraRuleResult {
                    rule_name: "财官两旺".to_string(),
                    signal: "吉".to_string(),
                    planets: {
                        let mut all = cai.planets.clone();
                        all.extend(guan.planets.clone());
                        all
                    },
                    description: "财帛宫与官禄宫均有星，财官双美".to_string(),
                });
            }
        }

        // 神煞影响
        let ji_count = shen_sha.iter()
            .filter(|s| s.branch == ming_gong_branch && s.category == "吉")
            .count();
        let xiong_count = shen_sha.iter()
            .filter(|s| s.branch == ming_gong_branch && s.category == "凶")
            .count();

        if ji_count >= 3 {
            rules.push(MoiraRuleResult {
                rule_name: "命宫吉神汇聚".to_string(),
                signal: "吉".to_string(),
                planets: vec![],
                description: format!("命宫有{}个吉神", ji_count),
            });
        }
        if xiong_count >= 3 {
            rules.push(MoiraRuleResult {
                rule_name: "命宫凶煞汇聚".to_string(),
                signal: "凶".to_string(),
                planets: vec![],
                description: format!("命宫有{}个凶煞", xiong_count),
            });
        }

        rules
    }

    /// 检查是否入垣
    fn check_ru_yuan(&self, planet: &QizhengPlanetPosition) -> bool {
        let branch = sign_index_to_branch(degree_to_sign_index(planet.lon));
        match planet.id.as_str() {
            "Sun" => branch == 6,     // 午
            "Moon" => branch == 7,    // 未
            "Mercury" => branch == 5 || branch == 8, // 巳/申
            "Venus" => branch == 4 || branch == 9,   // 辰/酉
            "Mars" => branch == 3 || branch == 10,   // 卯/戌
            "Jupiter" => branch == 2 || branch == 11, // 寅/亥
            "Saturn" => branch == 0 || branch == 1,   // 子/丑
            _ => false,
        }
    }

    /// 检查是否升殿（在对应的二十八宿）
    fn check_sheng_dian(&self, planet: &QizhengPlanetPosition) -> bool {
        if let Some(ref su28) = planet.su28 {
            match planet.id.as_str() {
                "Sun" => ["房", "虚", "昴", "星"].contains(&su28.as_str()),
                "Moon" => ["心", "危", "毕", "张"].contains(&su28.as_str()),
                "Mercury" => ["箕", "壁", "参", "轸"].contains(&su28.as_str()),
                "Venus" => ["亢", "牛", "娄", "鬼"].contains(&su28.as_str()),
                "Mars" => ["尾", "室", "觜", "翼"].contains(&su28.as_str()),
                "Jupiter" => ["角", "斗", "奎", "井"].contains(&su28.as_str()),
                "Saturn" => ["氐", "女", "胃", "柳"].contains(&su28.as_str()),
                _ => false,
            }
        } else {
            false
        }
    }

    /// 计算星体状态（星格详情）
    pub fn calc_star_status(
        &self,
        planets: &[QizhengPlanetPosition],
        houses: &[QizhengHouse],
    ) -> Vec<StarStatus> {
        let mut results = Vec::new();

        for planet in planets {
            let is_ru_yuan = self.check_ru_yuan(planet);
            let is_sheng_dian = self.check_sheng_dian(planet);
            let branch = sign_index_to_branch(degree_to_sign_index(planet.lon));

            results.push(StarStatus {
                star: planet.name.clone(),
                is_ru_yuan,
                is_sheng_dian,
                is_miao_wang: is_ru_yuan || is_sheng_dian,
                is_xi_le: false,
                is_retrograde: planet.is_retrograde,
                is_fu: false,
                is_ji: false,
                is_chi: false,
                is_liu: false,
                is_bei_shang: false,
                is_bei_sheng: false,
                su28: planet.su28.clone().unwrap_or_default(),
                house: planet.house.clone(),
                sign: EARTHLY_BRANCHES[branch].to_string(),
            });
        }

        results
    }
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_degree() {
        assert_eq!(normalize_degree(370.0), 10.0);
        assert_eq!(normalize_degree(-10.0), 350.0);
        assert_eq!(normalize_degree(180.0), 180.0);
    }

    #[test]
    fn test_degree_to_sign_index() {
        assert_eq!(degree_to_sign_index(0.0), 0);
        assert_eq!(degree_to_sign_index(30.0), 1);
        assert_eq!(degree_to_sign_index(350.0), 11);
    }

    #[test]
    fn test_hour_branch() {
        assert_eq!(get_hour_branch(0, 0), 0);  // 子时
        assert_eq!(get_hour_branch(1, 0), 0);  // 子时 (23:00-01:00)
        assert_eq!(get_hour_branch(12, 0), 6); // 午时
        assert_eq!(get_hour_branch(23, 0), 0); // 子时 (23:00-01:00)
    }

    #[test]
    fn test_mansion_info() {
        let (name, deg, idx, width) = get_mansion_info(210.0);
        assert_eq!(name, "亢");
        assert!(deg >= 0.0);
        assert!(width > 0.0);
    }

    #[test]
    fn test_calculate_chart() {
        let calc = QizhengCalc::default();
        let chart = calc.calculate_chart(
            1985, 8, 26, 2, 55, 0,
            8.0, 22.3, 114.2,
            "香港", "male",
        );
        assert_eq!(chart.planets.len(), 11);
        assert_eq!(chart.houses.len(), 12);
        assert!(!chart.life_su.is_empty());
        assert!(!chart.da_yun.is_empty());
        assert!(!chart.shen_sha.is_empty());
    }
}