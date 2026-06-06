// 星阙 Horosa - 印度占星（Jyotish）计算引擎
// 参考原项目: astropy/astrostudy/india/jyotish_engine.py
// 参考原项目: astropy/astrostudy/india/yoga_engine.py

use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use horosa_core::chart::*;
use horosa_core::astrology::*;
use std::collections::HashMap;

// ============================================================================
// 常量数据
// ============================================================================

/// 维摩沙里大运序列（120年周期）
/// 顺序: Ketu, Venus, Sun, Moon, Mars, Rahu, Jupiter, Saturn, Mercury
const DASHA_SEQUENCE: &[(Planet, f64)] = &[
    (Planet::SouthNode, 7.0),   // Ketu
    (Planet::Venus, 20.0),
    (Planet::Sun, 6.0),
    (Planet::Moon, 10.0),
    (Planet::Mars, 7.0),
    (Planet::NorthNode, 18.0),  // Rahu
    (Planet::Jupiter, 16.0),
    (Planet::Saturn, 19.0),
    (Planet::Mercury, 17.0),
];

/// 27 星宿（Nakshatra）数据: (英文名, 中文名, 主星Planet)
const NAKSHATRAS: &[(&str, &str, Planet)] = &[
    ("Ashwini", "马", Planet::SouthNode),          // Ketu
    ("Bharani", "胃", Planet::Venus),
    ("Krittika", "昴", Planet::Sun),
    ("Rohini", "毕", Planet::Moon),
    ("Mrigashira", "参", Planet::Mars),
    ("Ardra", "井", Planet::NorthNode),            // Rahu
    ("Punarvasu", "鬼", Planet::Jupiter),
    ("Pushya", "柳", Planet::Saturn),
    ("Ashlesha", "星", Planet::Mercury),
    ("Magha", "张", Planet::SouthNode),            // Ketu
    ("Purva Phalguni", "翼", Planet::Venus),
    ("Uttara Phalguni", "轸", Planet::Sun),
    ("Hasta", "角", Planet::Moon),
    ("Chitra", "亢", Planet::Mars),
    ("Swati", "氐", Planet::NorthNode),            // Rahu
    ("Vishakha", "房", Planet::Jupiter),
    ("Anuradha", "心", Planet::Saturn),
    ("Jyeshtha", "尾", Planet::Mercury),
    ("Mula", "箕", Planet::SouthNode),             // Ketu
    ("Purva Ashadha", "斗", Planet::Venus),
    ("Uttara Ashadha", "牛", Planet::Sun),
    ("Shravana", "女", Planet::Moon),
    ("Dhanishta", "虚", Planet::Mars),
    ("Shatabhisha", "危", Planet::NorthNode),      // Rahu
    ("Purva Bhadrapada", "室", Planet::Jupiter),
    ("Uttara Bhadrapada", "壁", Planet::Saturn),
    ("Revati", "奎", Planet::Mercury),
];

/// 星宿宽度（每宿13°20'）
const NAKSHATRA_SPAN: f64 = 360.0 / 27.0;

/// 太阴日名称
const TITHI_NAMES: &[&str] = &[
    "Pratipada", "Dvitiya", "Tritiya", "Chaturthi", "Panchami", "Shashthi",
    "Saptami", "Ashtami", "Navami", "Dashami", "Ekadashi", "Dwadashi",
    "Trayodashi", "Chaturdashi", "Purnima/Amavasya",
];

/// 27 Nitya Yoga 名称（Panchanga 瑜伽）
const NITYA_YOGA_NAMES: &[&str] = &[
    "Vishkambha", "Priti", "Ayushman", "Saubhagya", "Shobhana", "Atiganda",
    "Sukarman", "Dhriti", "Shula", "Ganda", "Vriddhi", "Dhruva", "Vyaghata",
    "Harshana", "Vajra", "Siddhi", "Vyatipata", "Variyan", "Parigha", "Shiva",
    "Siddha", "Sadhya", "Shubha", "Shukla", "Brahma", "Indra", "Vaidhriti",
];

/// 半太阴日（Karana）序列（7个循环）
const KARANA_SEQUENCE: &[&str] = &[
    "Bava", "Balava", "Kaulava", "Taitila", "Gara", "Vanija", "Vishti",
];

/// 星期名称
const VARA_NAMES: &[&str] = &[
    "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday",
];

/// 星期中文名
const VARA_CN: &[&str] = &[
    "星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六",
];

/// 星期主星
const VARA_LORDS: &[Planet] = &[
    Planet::Sun, Planet::Moon, Planet::Mars, Planet::Mercury,
    Planet::Jupiter, Planet::Venus, Planet::Saturn,
];

/// 本宫星座（Own Signs）
fn own_signs(planet: Planet) -> &'static [ZodiacSign] {
    match planet {
        Planet::Sun => &[ZodiacSign::Leo],
        Planet::Moon => &[ZodiacSign::Cancer],
        Planet::Mars => &[ZodiacSign::Aries, ZodiacSign::Scorpio],
        Planet::Mercury => &[ZodiacSign::Gemini, ZodiacSign::Virgo],
        Planet::Jupiter => &[ZodiacSign::Sagittarius, ZodiacSign::Pisces],
        Planet::Venus => &[ZodiacSign::Taurus, ZodiacSign::Libra],
        Planet::Saturn => &[ZodiacSign::Capricorn, ZodiacSign::Aquarius],
        _ => &[],
    }
}

/// 旺相星座（Exaltation）: (星座, 精确旺相度数)
fn exaltation(planet: Planet) -> Option<(ZodiacSign, f64)> {
    match planet {
        Planet::Sun => Some((ZodiacSign::Aries, 10.0)),
        Planet::Moon => Some((ZodiacSign::Taurus, 3.0)),
        Planet::Mars => Some((ZodiacSign::Capricorn, 28.0)),
        Planet::Mercury => Some((ZodiacSign::Virgo, 15.0)),
        Planet::Jupiter => Some((ZodiacSign::Cancer, 5.0)),
        Planet::Venus => Some((ZodiacSign::Pisces, 27.0)),
        Planet::Saturn => Some((ZodiacSign::Libra, 20.0)),
        Planet::NorthNode => Some((ZodiacSign::Taurus, 20.0)),
        Planet::SouthNode => Some((ZodiacSign::Scorpio, 20.0)),
        _ => None,
    }
}

/// 本三角星座（Moolatrikona）: (星座, 起始度数, 结束度数)
fn moolatrikona(planet: Planet) -> Option<(ZodiacSign, f64, f64)> {
    match planet {
        Planet::Sun => Some((ZodiacSign::Leo, 0.0, 20.0)),
        Planet::Moon => Some((ZodiacSign::Taurus, 4.0, 30.0)),
        Planet::Mars => Some((ZodiacSign::Aries, 0.0, 12.0)),
        Planet::Mercury => Some((ZodiacSign::Virgo, 16.0, 20.0)),
        Planet::Jupiter => Some((ZodiacSign::Sagittarius, 0.0, 10.0)),
        Planet::Venus => Some((ZodiacSign::Libra, 0.0, 15.0)),
        Planet::Saturn => Some((ZodiacSign::Aquarius, 0.0, 20.0)),
        _ => None,
    }
}

/// 星座主星
fn sign_lord(sign: ZodiacSign) -> Planet {
    match sign {
        ZodiacSign::Aries => Planet::Mars,
        ZodiacSign::Taurus => Planet::Venus,
        ZodiacSign::Gemini => Planet::Mercury,
        ZodiacSign::Cancer => Planet::Moon,
        ZodiacSign::Leo => Planet::Sun,
        ZodiacSign::Virgo => Planet::Mercury,
        ZodiacSign::Libra => Planet::Venus,
        ZodiacSign::Scorpio => Planet::Mars,
        ZodiacSign::Sagittarius => Planet::Jupiter,
        ZodiacSign::Capricorn => Planet::Saturn,
        ZodiacSign::Aquarius => Planet::Saturn,
        ZodiacSign::Pisces => Planet::Jupiter,
    }
}

/// 自然吉星
const NATURAL_BENEFICS: &[Planet] = &[Planet::Jupiter, Planet::Venus, Planet::Mercury];

/// 自然凶星
const NATURAL_MALEFICS: &[Planet] = &[
    Planet::Sun, Planet::Mars, Planet::Saturn, Planet::NorthNode, Planet::SouthNode,
];

/// 角宫（Kendra）
const KENDRA: &[u8] = &[1, 4, 7, 10];

/// 三分宫（Trikona）
const TRIKONA: &[u8] = &[1, 5, 9];

/// 困难宫（Dusthana）
const DUSTHANA: &[u8] = &[6, 8, 12];

/// 上升宫（Upachaya）
const UPACHAYA: &[u8] = &[3, 6, 10, 11];

/// 经典七曜 + 计都罗睺
const JYOTISH_PLANETS: &[Planet] = &[
    Planet::Sun, Planet::Moon, Planet::Mars, Planet::Mercury, Planet::Jupiter,
    Planet::Venus, Planet::Saturn, Planet::NorthNode, Planet::SouthNode,
];

/// 经典七曜
const CLASSICAL_PLANETS: &[Planet] = &[
    Planet::Sun, Planet::Moon, Planet::Mars, Planet::Mercury,
    Planet::Jupiter, Planet::Venus, Planet::Saturn,
];

/// 五大尊严瑜伽（Pancha Mahapurusha）
const MAHAPURUSHA_YOGAS: &[(Planet, &str, &str, &str)] = &[
    (Planet::Mars, "Ruchaka Yoga", "鲁查卡瑜伽", "勇气、行动力、统御与竞争能力增强。"),
    (Planet::Mercury, "Bhadra Yoga", "跋陀罗瑜伽", "智性、表达、商业和学习能力增强。"),
    (Planet::Jupiter, "Hamsa Yoga", "汉萨瑜伽", "德性、智慧、教学、宗教与保护性增强。"),
    (Planet::Venus, "Malavya Yoga", "摩罗毗耶瑜伽", "审美、舒适、关系、艺术与享受能力增强。"),
    (Planet::Saturn, "Shasha Yoga", "沙沙瑜伽", "纪律、组织、长期权力、耐力与制度能力增强。"),
];

/// 星体相位规则: 行星 -> 照射的宫位（从该星算起）
fn aspect_houses(planet: Planet) -> &'static [u8] {
    match planet {
        Planet::Sun => &[7],
        Planet::Moon => &[7],
        Planet::Mars => &[4, 7, 8],
        Planet::Mercury => &[7],
        Planet::Jupiter => &[5, 7, 9],
        Planet::Venus => &[7],
        Planet::Saturn => &[3, 7, 10],
        Planet::NorthNode => &[5, 7, 9],
        Planet::SouthNode => &[5, 7, 9],
        _ => &[],
    }
}

// ============================================================================
// 八层占（Ashtakavarga）受益宫位数据
// 参考原项目: jyotish_engine.py BENEFIC_HOUSES
// ============================================================================

type BeneficTable = HashMap<&'static str, &'static [u8]>;

fn get_benefic_houses(target: &str) -> BeneficTable {
    let mut m = HashMap::new();
    match target {
        "Sun" => {
            m.insert("Sun", &[1u8, 2, 4, 7, 8, 9, 10, 11][..]);
            m.insert("Moon", &[3u8, 6, 10, 11][..]);
            m.insert("Mars", &[1u8, 2, 4, 7, 8, 9, 10, 11][..]);
            m.insert("Mercury", &[3u8, 5, 6, 9, 10, 11, 12][..]);
            m.insert("Jupiter", &[5u8, 6, 9, 11][..]);
            m.insert("Venus", &[6u8, 7, 12][..]);
            m.insert("Saturn", &[1u8, 2, 4, 7, 8, 9, 10, 11][..]);
            m.insert("Lagna", &[3u8, 4, 6, 10, 11, 12][..]);
        }
        "Moon" => {
            m.insert("Sun", &[3u8, 6, 7, 8, 10, 11][..]);
            m.insert("Moon", &[1u8, 3, 6, 7, 10, 11][..]);
            m.insert("Mars", &[2u8, 3, 5, 6, 9, 10, 11][..]);
            m.insert("Mercury", &[1u8, 3, 4, 5, 7, 8, 10, 11][..]);
            m.insert("Jupiter", &[1u8, 4, 7, 8, 10, 11, 12][..]);
            m.insert("Venus", &[3u8, 4, 5, 7, 9, 10, 11][..]);
            m.insert("Saturn", &[3u8, 5, 6, 11][..]);
            m.insert("Lagna", &[3u8, 6, 10, 11][..]);
        }
        "Mars" => {
            m.insert("Sun", &[3u8, 5, 6, 10, 11][..]);
            m.insert("Moon", &[3u8, 6, 11][..]);
            m.insert("Mars", &[1u8, 2, 4, 7, 8, 10, 11][..]);
            m.insert("Mercury", &[3u8, 5, 6, 11][..]);
            m.insert("Jupiter", &[6u8, 10, 11, 12][..]);
            m.insert("Venus", &[6u8, 8, 11, 12][..]);
            m.insert("Saturn", &[1u8, 4, 7, 8, 9, 10, 11][..]);
            m.insert("Lagna", &[1u8, 3, 6, 10, 11][..]);
        }
        "Mercury" => {
            m.insert("Sun", &[5u8, 6, 9, 11, 12][..]);
            m.insert("Moon", &[2u8, 4, 6, 8, 10, 11][..]);
            m.insert("Mars", &[1u8, 2, 4, 7, 8, 9, 10, 11][..]);
            m.insert("Mercury", &[1u8, 3, 5, 6, 9, 10, 11, 12][..]);
            m.insert("Jupiter", &[6u8, 8, 11, 12][..]);
            m.insert("Venus", &[1u8, 2, 3, 4, 5, 8, 9, 11][..]);
            m.insert("Saturn", &[1u8, 2, 4, 7, 8, 9, 10, 11][..]);
            m.insert("Lagna", &[1u8, 2, 4, 6, 8, 10, 11][..]);
        }
        "Jupiter" => {
            m.insert("Sun", &[1u8, 2, 3, 4, 7, 8, 9, 10, 11][..]);
            m.insert("Moon", &[2u8, 5, 7, 9, 11][..]);
            m.insert("Mars", &[1u8, 2, 4, 7, 8, 10, 11][..]);
            m.insert("Mercury", &[1u8, 2, 4, 5, 6, 9, 10, 11][..]);
            m.insert("Jupiter", &[1u8, 2, 3, 4, 7, 8, 10, 11][..]);
            m.insert("Venus", &[2u8, 5, 6, 9, 10, 11][..]);
            m.insert("Saturn", &[3u8, 5, 6, 12][..]);
            m.insert("Lagna", &[1u8, 2, 4, 5, 6, 7, 9, 10, 11][..]);
        }
        "Venus" => {
            m.insert("Sun", &[8u8, 11, 12][..]);
            m.insert("Moon", &[1u8, 2, 3, 4, 5, 8, 9, 11, 12][..]);
            m.insert("Mars", &[3u8, 5, 6, 9, 11, 12][..]);
            m.insert("Mercury", &[3u8, 5, 6, 9, 11][..]);
            m.insert("Jupiter", &[5u8, 8, 9, 10, 11][..]);
            m.insert("Venus", &[1u8, 2, 3, 4, 5, 8, 9, 10, 11][..]);
            m.insert("Saturn", &[3u8, 4, 5, 8, 9, 10, 11][..]);
            m.insert("Lagna", &[1u8, 2, 3, 4, 5, 8, 9, 11][..]);
        }
        "Saturn" => {
            m.insert("Sun", &[1u8, 2, 4, 7, 8, 10, 11][..]);
            m.insert("Moon", &[3u8, 6, 11][..]);
            m.insert("Mars", &[3u8, 5, 6, 10, 11, 12][..]);
            m.insert("Mercury", &[6u8, 8, 9, 10, 11, 12][..]);
            m.insert("Jupiter", &[5u8, 6, 11, 12][..]);
            m.insert("Venus", &[6u8, 11, 12][..]);
            m.insert("Saturn", &[3u8, 5, 6, 11][..]);
            m.insert("Lagna", &[1u8, 3, 4, 6, 10, 11][..]);
        }
        _ => {}
    }
    m
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 标准化角度到 [0, 360)
fn norm(deg: f64) -> f64 {
    let d = deg % 360.0;
    if d < 0.0 { d + 360.0 } else { d }
}

/// 从黄经获取星座索引（0-11）
fn sign_index_from_lon(lon: f64) -> u8 {
    (norm(lon) / 30.0) as u8 % 12
}

/// 从索引获取星座
fn sign_from_index(index: u8) -> ZodiacSign {
    match index % 12 {
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

/// 星座名称（中文）
fn sign_name_zh(sign: ZodiacSign) -> &'static str {
    match sign {
        ZodiacSign::Aries => "白羊",
        ZodiacSign::Taurus => "金牛",
        ZodiacSign::Gemini => "双子",
        ZodiacSign::Cancer => "巨蟹",
        ZodiacSign::Leo => "狮子",
        ZodiacSign::Virgo => "处女",
        ZodiacSign::Libra => "天秤",
        ZodiacSign::Scorpio => "天蝎",
        ZodiacSign::Sagittarius => "射手",
        ZodiacSign::Capricorn => "摩羯",
        ZodiacSign::Aquarius => "水瓶",
        ZodiacSign::Pisces => "双鱼",
    }
}

/// 星座英文名
fn sign_name_en(sign: ZodiacSign) -> &'static str {
    match sign {
        ZodiacSign::Aries => "Aries",
        ZodiacSign::Taurus => "Taurus",
        ZodiacSign::Gemini => "Gemini",
        ZodiacSign::Cancer => "Cancer",
        ZodiacSign::Leo => "Leo",
        ZodiacSign::Virgo => "Virgo",
        ZodiacSign::Libra => "Libra",
        ZodiacSign::Scorpio => "Scorpio",
        ZodiacSign::Sagittarius => "Sagittarius",
        ZodiacSign::Capricorn => "Capricorn",
        ZodiacSign::Aquarius => "Aquarius",
        ZodiacSign::Pisces => "Pisces",
    }
}

/// 行星名称（中文）
fn planet_name_zh(planet: Planet) -> &'static str {
    match planet {
        Planet::Sun => "太阳",
        Planet::Moon => "月亮",
        Planet::Mars => "火星",
        Planet::Mercury => "水星",
        Planet::Jupiter => "木星",
        Planet::Venus => "金星",
        Planet::Saturn => "土星",
        Planet::NorthNode => "罗睺",
        Planet::SouthNode => "计都",
        _ => "未知",
    }
}

/// 行星名称（英文）
fn planet_name_en(planet: Planet) -> &'static str {
    match planet {
        Planet::Sun => "Sun",
        Planet::Moon => "Moon",
        Planet::Mars => "Mars",
        Planet::Mercury => "Mercury",
        Planet::Jupiter => "Jupiter",
        Planet::Venus => "Venus",
        Planet::Saturn => "Saturn",
        Planet::NorthNode => "Rahu",
        Planet::SouthNode => "Ketu",
        _ => "Unknown",
    }
}

/// 行星字符串键（用于 HashMap）
fn planet_key(planet: Planet) -> String {
    planet_name_en(planet).to_string()
}

/// 计算两个角度之间的角距离（取最短弧）
fn angular_distance(a: f64, b: f64) -> f64 {
    let dist = (norm(a) - norm(b)).abs() % 360.0;
    dist.min(360.0 - dist)
}

/// 计算相对宫位：from_sign 中看 to_sign 在哪一宫（1-12）
fn rel_house(from_sign_index: u8, to_sign_index: u8) -> u8 {
    ((to_sign_index as i8 - from_sign_index as i8 + 12) % 12) as u8 + 1
}

/// 计算儒略日（JD）—— 使用 Meeus 算法
fn julian_day(year: i32, month: u8, day: u8, hour: f64) -> f64 {
    let a = (14 - month as i32) / 12;
    let y = year + 4800 - a;
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

/// 从 DateTime<Utc> 计算儒略日
fn datetime_to_jd(dt: &DateTime<Utc>) -> f64 {
    julian_day(
        dt.year(),
        dt.month() as u8,
        dt.day() as u8,
        dt.hour() as f64
            + dt.minute() as f64 / 60.0
            + dt.second() as f64 / 3600.0,
    )
}

/// 从儒略日计算 DateTime<Utc>（近似）
fn jd_to_datetime(jd: f64) -> DateTime<Utc> {
    let jd_int = jd + 0.5;
    let z = jd_int as i32;
    let f = jd_int - z as f64;

    let a = if z < 2299161 {
        z
    } else {
        let alpha = ((z as f64 - 1867216.25) / 36524.25) as i32;
        z + 1 + alpha - alpha / 4
    };

    let b = a + 1524;
    let c = ((b as f64 - 122.1) / 365.25) as i32;
    let d = (365.25 * c as f64) as i32;
    let e = ((b - d) as f64 / 30.6001) as i32;

    let day = b - d - (30.6001 * e as f64) as i32;
    let month = if e < 14 { e - 1 } else { e - 13 };
    let year = if month > 2 { c - 4716 } else { c - 4715 };

    let day_frac = f + 0.5 - (jd_int - jd);
    if day_frac < 0.0 {
        let day_frac = day_frac + 1.0;
        let total_hours = day_frac * 24.0;
        let hour = total_hours as u32;
        let minute = ((total_hours - hour as f64) * 60.0) as u32;
        let second = ((total_hours - hour as f64 - minute as f64 / 60.0) * 3600.0) as u32;

        DateTime::<Utc>::from_timestamp(0, 0)
            .unwrap()
            .with_year(year)
            .unwrap()
            .with_month(month as u32)
            .unwrap()
            .with_day(day as u32)
            .unwrap()
            .with_hour(hour)
            .unwrap()
            .with_minute(minute)
            .unwrap()
            .with_second(second)
            .unwrap()
    } else {
        let total_hours = day_frac * 24.0;
        let hour = total_hours as u32;
        let minute = ((total_hours - hour as f64) * 60.0) as u32;
        let second = ((total_hours - hour as f64 - minute as f64 / 60.0) * 3600.0) as u32;

        DateTime::<Utc>::from_timestamp(0, 0)
            .unwrap()
            .with_year(year)
            .unwrap()
            .with_month(month as u32)
            .unwrap()
            .with_day(day as u32)
            .unwrap()
            .with_hour(hour)
            .unwrap()
            .with_minute(minute)
            .unwrap()
            .with_second(second)
            .unwrap()
    }
}

/// 格式化日期
fn format_date(dt: &DateTime<Utc>) -> String {
    format!("{:04}-{:02}-{:02}", dt.year(), dt.month(), dt.day())
}

// ============================================================================
// 行星近似位置计算（使用平均轨道要素）
// 当 Swiss Ephemeris 不可用时使用
// ============================================================================

/// 计算行星近似黄经（赤道坐标，未修正岁差）
fn mean_longitude(planet: Planet, jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0; // 儒略世纪数（从 J2000.0 起算）
    match planet {
        Planet::Sun => norm(280.46646 + 36000.76983 * t + 0.0003032 * t * t),
        Planet::Moon => norm(218.31665 + 481267.8813 * t - 0.001326 * t * t),
        Planet::Mercury => norm(252.2509 + 149472.6746 * t),
        Planet::Venus => norm(181.9798 + 58517.8157 * t),
        Planet::Mars => norm(355.4530 + 19139.8585 * t),
        Planet::Jupiter => norm(34.3515 + 3034.9057 * t),
        Planet::Saturn => norm(50.0774 + 1222.1138 * t),
        Planet::NorthNode => {
            // 罗睺 = 平均升交点
            norm(125.0445 - 1934.1363 * t)
        }
        Planet::SouthNode => {
            // 计都 = 升交点 + 180°
            norm(125.0445 - 1934.1363 * t + 180.0)
        }
        _ => 0.0,
    }
}

// ============================================================================
// VedicCalc 结构体
// ============================================================================

/// 印度占星（Jyotish）计算器
pub struct VedicCalc;

impl VedicCalc {
    pub fn new() -> Self {
        Self
    }

    // ========================================================================
    // 主计算入口
    // ========================================================================

    /// 计算完整的印度占星盘
    pub fn calculate(&self, birth: &BirthInfo) -> VedicChart {
        let jd = datetime_to_jd(&birth.datetime);
        let ayanamsa = self.get_ayanamsa(jd);

        // 计算行星位置（近似）
        let mut planets: Vec<PlanetPosition> = Vec::new();
        for &planet in JYOTISH_PLANETS {
            let trop_lon = mean_longitude(planet, jd);
            let sid_lon = norm(trop_lon - ayanamsa);
            let sign_idx = sign_index_from_lon(sid_lon);
            let sign = sign_from_index(sign_idx);
            let degree_in_sign = sid_lon - sign_idx as f64 * 30.0;

            planets.push(PlanetPosition {
                planet,
                longitude: sid_lon,
                latitude: 0.0,
                right_ascension: 0.0,
                declination: 0.0,
                sign,
                degree_in_sign,
                house: 1, // 将在后续设置
                is_retrograde: false,
                distance: 0.0,
                speed: 0.0,
            });
        }

        // 上升点（简化：使用中午的上升近似）
        let asc_lon = self.calc_ascendant_approx(jd, ayanamsa, birth.location.latitude);
        let asc_sign_idx = sign_index_from_lon(asc_lon);
        let ascendant = Ascendant {
            longitude: asc_lon,
            sign: sign_from_index(asc_sign_idx),
            degree: asc_lon - asc_sign_idx as f64 * 30.0,
        };

        // 计算宫位（等宫制 Whole Sign）
        let houses = self.calc_whole_sign_houses(&ascendant);

        // 为每个行星设置宫位
        for p in planets.iter_mut() {
            let sign_idx = sign_index_from_lon(p.longitude);
            p.house = rel_house(asc_sign_idx, sign_idx);
        }

        // 计算星宿
        let mut nakshatras: HashMap<String, NakshatraInfo> = HashMap::new();
        for p in &planets {
            let nak = self.get_nakshatra(p.longitude);
            nakshatras.insert(planet_key(p.planet), nak);
        }
        nakshatras.insert("Ascendant".to_string(), self.get_nakshatra(asc_lon));

        // 计算星体尊严
        let dignities = self.calc_dignities(&planets);

        // 计算大运
        let dashas = self.get_dasha(birth);

        // 计算瑜伽
        let yogas = self.calc_yogas(&planets, asc_sign_idx);

        // 计算分盘
        let vargas = self.calc_all_vargas(&planets, asc_sign_idx);

        // 计算八层占
        let ashtakavarga = self.calc_ashtakavarga(&planets, asc_sign_idx);

        // 计算五支
        let panchanga = self.calc_panchanga_from_planets(&planets, &birth.datetime);

        VedicChart {
            ayanamsa,
            birth: birth.clone(),
            planets,
            houses,
            ascendant,
            nakshatras,
            dashas,
            yogas,
            vargas,
            dignities,
            ashtakavarga,
            panchanga,
        }
    }

    // ========================================================================
    // 1. Ayanamsa（恒星黄道偏移）
    // ========================================================================

    /// 计算 Lahiri Ayanamsa（恒星黄道偏移角度）
    /// 公式基于 J2000.0 基准值 23.853056° + 岁差率
    pub fn get_ayanamsa(&self, jd: f64) -> f64 {
        // Lahiri ayanamsa at J2000.0 = 23°51'11" ≈ 23.853056°
        // 岁差率 ≈ 50.29" / 年 ≈ 0.0139694°/年
        let years_since_j2000 = (jd - 2451545.0) / 365.25;
        23.853056 + 0.0139694 * years_since_j2000
    }

    // ========================================================================
    // 2. Nakshatras（27宿）
    // ========================================================================

    /// 获取指定黄经对应的星宿信息
    pub fn get_nakshatra(&self, lon: f64) -> NakshatraInfo {
        let value = norm(lon);
        let idx = {
            let i = (value / NAKSHATRA_SPAN) as usize;
            if i >= 27 { 26 } else { i }
        };
        let progress = (value - idx as f64 * NAKSHATRA_SPAN) / NAKSHATRA_SPAN;
        let pada = {
            let p = (progress * 4.0) as u8 + 1;
            if p > 4 { 4 } else { p }
        };

        let (name, name_zh, lord) = NAKSHATRAS[idx];
        let degree_start = idx as f64 * NAKSHATRA_SPAN;
        let degree_end = degree_start + NAKSHATRA_SPAN;

        NakshatraInfo {
            index: idx as u8 + 1,
            name: name.to_string(),
            name_zh: name_zh.to_string(),
            lord,
            pada,
            degree_start,
            degree_end,
            progress,
            remaining_ratio: 1.0 - progress,
        }
    }

    // ========================================================================
    // 3. Vimshottari Dasha（大运）
    // ========================================================================

    /// 计算维摩沙里大运序列
    pub fn get_dasha(&self, birth: &BirthInfo) -> Vec<VimshottariDasha> {
        // 获取月亮星宿
        let moon_lon = mean_longitude(Planet::Moon, datetime_to_jd(&birth.datetime));
        let ayanamsa = self.get_ayanamsa(datetime_to_jd(&birth.datetime));
        let moon_sid_lon = norm(moon_lon - ayanamsa);
        let moon_nak = self.get_nakshatra(moon_sid_lon);

        let moon_lord = moon_nak.lord;
        let moon_lord_years = dasha_years(moon_lord);

        // 出生时已过的大运年数
        let first_elapsed_years = moon_lord_years * (1.0 - moon_nak.remaining_ratio);
        let first_balance_years = moon_lord_years * moon_nak.remaining_ratio;

        // 循环开始日期
        let cycle_start_days = first_elapsed_years * 365.25;
        let cycle_start = birth.datetime - Duration::seconds((cycle_start_days * 86400.0) as i64);

        // 找到月亮主星在序列中的位置
        let lord_index = DASHA_SEQUENCE
            .iter()
            .position(|(p, _)| *p == moon_lord)
            .unwrap_or(0);

        let mut current_start = cycle_start;
        let now = Utc::now();

        let mut dashas = Vec::with_capacity(10);

        for i in 0..10 {
            let (lord, years) = DASHA_SEQUENCE[(lord_index + i) % DASHA_SEQUENCE.len()];
            let duration_days = years * 365.25;
            let current_end = current_start + Duration::seconds((duration_days * 86400.0) as i64);

            let bhuktis = self.calc_bhuktis(lord, current_start, duration_days);

            let start_age = (current_start - birth.datetime).num_seconds() as f64
                / (365.25 * 86400.0);
            let end_age = (current_end - birth.datetime).num_seconds() as f64
                / (365.25 * 86400.0);

            dashas.push(VimshottariDasha {
                lord,
                lord_name: planet_name_en(lord).to_string(),
                years,
                start_date: format_date(&current_start),
                end_date: format_date(&current_end),
                start_age,
                end_age,
                is_birth_balance: i == 0,
                is_active: current_start <= now && now < current_end,
                bhuktis,
            });

            current_start = current_end;
        }

        dashas
    }

    /// 计算小运（Bhukti/Antardasha）
    fn calc_bhuktis(
        &self,
        parent_lord: Planet,
        start: DateTime<Utc>,
        parent_days: f64,
    ) -> Vec<BhuktiPeriod> {
        let lord_index = DASHA_SEQUENCE
            .iter()
            .position(|(p, _)| *p == parent_lord)
            .unwrap_or(0);

        let mut bhuktis = Vec::with_capacity(9);
        let mut current_start = start;

        for i in 0..DASHA_SEQUENCE.len() {
            let (sub_lord, sub_years) = DASHA_SEQUENCE[(lord_index + i) % DASHA_SEQUENCE.len()];
            let sub_days = parent_days * sub_years / 120.0;
            let end = current_start + Duration::seconds((sub_days * 86400.0) as i64);

            bhuktis.push(BhuktiPeriod {
                lord: sub_lord,
                lord_name: planet_name_en(sub_lord).to_string(),
                start_date: format_date(&current_start),
                end_date: format_date(&end),
                years: sub_days / 365.25,
            });

            current_start = end;
        }

        bhuktis
    }

    // ========================================================================
    // 4. Yogas（格局）
    // ========================================================================

    /// 获取命盘中的所有瑜伽
    pub fn get_yogas(&self, chart: &VedicChart) -> Vec<String> {
        let sign_map: HashMap<Planet, u8> = chart
            .planets
            .iter()
            .map(|p| (p.planet, sign_index_from_lon(p.longitude)))
            .collect();

        let asc_sign = sign_index_from_lon(chart.ascendant.longitude);
        let mut yogas = Vec::new();

        // Pancha Mahapurusha Yogas
        yogas.extend(self.check_pancha_mahapurusha(&sign_map, asc_sign));

        // Gaja Kesari Yoga (Jupiter-Moon in Kendra)
        yogas.extend(self.check_gaja_kesari(&sign_map));

        // Chandra Mangala Yoga (Moon-Mars conjunction/opposition)
        yogas.extend(self.check_chandra_mangala(&sign_map));

        // Budha Aditya Yoga (Mercury-Sun conjunction)
        yogas.extend(self.check_budha_aditya(&sign_map));

        // Raj Yogas (Kendra-Trikona lord associations)
        yogas.extend(self.check_raj_yogas(&sign_map, asc_sign));

        // Dhana Yogas
        yogas.extend(self.check_dhana_yogas(&sign_map, asc_sign));

        // Viparita Raja Yogas
        yogas.extend(self.check_viparita_raja(&sign_map, asc_sign));

        // Parivartana Yogas
        yogas.extend(self.check_parivartana(&sign_map, asc_sign));

        // Neecha Bhanga Yogas
        yogas.extend(self.check_neecha_bhanga(&sign_map, asc_sign));

        yogas
    }

    fn calc_yogas(&self, planets: &[PlanetPosition], asc_sign: u8) -> Vec<String> {
        let sign_map: HashMap<Planet, u8> = planets
            .iter()
            .map(|p| (p.planet, sign_index_from_lon(p.longitude)))
            .collect();

        let mut yogas = Vec::new();
        yogas.extend(self.check_pancha_mahapurusha(&sign_map, asc_sign));
        yogas.extend(self.check_gaja_kesari(&sign_map));
        yogas.extend(self.check_chandra_mangala(&sign_map));
        yogas.extend(self.check_budha_aditya(&sign_map));
        yogas.extend(self.check_raj_yogas(&sign_map, asc_sign));
        yogas.extend(self.check_dhana_yogas(&sign_map, asc_sign));
        yogas.extend(self.check_viparita_raja(&sign_map, asc_sign));
        yogas.extend(self.check_parivartana(&sign_map, asc_sign));
        yogas.extend(self.check_neecha_bhanga(&sign_map, asc_sign));
        yogas
    }

    /// Pancha Mahapurusha Yogas: 五大尊严瑜伽
    fn check_pancha_mahapurusha(
        &self,
        sign_map: &HashMap<Planet, u8>,
        asc_sign: u8,
    ) -> Vec<String> {
        let mut yogas = Vec::new();
        let bases: [(u8, &str); 2] = [(asc_sign, "Lagna"), (sign_map.get(&Planet::Moon).copied().unwrap_or(0), "Moon")];

        for &(planet, name, zh_name, desc) in MAHAPURUSHA_YOGAS {
            let planet_sign = match sign_map.get(&planet) {
                Some(s) => *s,
                None => continue,
            };

            // 检查是否在本宫/旺相/本三角
            let strong = self.is_planet_strong(planet, planet_sign, 0.0);
            if !strong {
                continue;
            }

            for &(base_sign, base_label) in &bases {
                if base_sign == 0 && base_label == "Moon" {
                    continue;
                }
                let house = rel_house(base_sign, planet_sign);
                if KENDRA.contains(&house) {
                    yogas.push(format!(
                        "{} ({}) - {}位于{}第{}宫，{}",
                        name, zh_name, planet_name_zh(planet), base_label, house, desc
                    ));
                }
            }
        }
        yogas
    }

    /// Gaja Kesari Yoga: 木星-月亮在角宫
    fn check_gaja_kesari(&self, sign_map: &HashMap<Planet, u8>) -> Vec<String> {
        let moon_sign = match sign_map.get(&Planet::Moon) {
            Some(s) => *s,
            None => return vec![],
        };
        let jupiter_sign = match sign_map.get(&Planet::Jupiter) {
            Some(s) => *s,
            None => return vec![],
        };

        let house = rel_house(moon_sign, jupiter_sign);
        if KENDRA.contains(&house) {
            vec![format!(
                "Gaja Kesari Yoga (象狮瑜伽) - 木星在月亮第{}宫（角宫），增强名望、智慧、保护力",
                house
            )]
        } else {
            vec![]
        }
    }

    /// Chandra Mangala Yoga: 月亮-火星合相/对冲
    fn check_chandra_mangala(&self, sign_map: &HashMap<Planet, u8>) -> Vec<String> {
        let moon_sign = match sign_map.get(&Planet::Moon) {
            Some(s) => *s,
            None => return vec![],
        };
        let mars_sign = match sign_map.get(&Planet::Mars) {
            Some(s) => *s,
            None => return vec![],
        };

        let house = rel_house(moon_sign, mars_sign);
        if house == 1 || house == 7 {
            let relation = if house == 1 { "同宫" } else { "对冲" };
            vec![format!(
                "Chandra Mangala Yoga (月火瑜伽) - 火星与月亮{}，增强交易、行动力、财务动机",
                relation
            )]
        } else {
            vec![]
        }
    }

    /// Budha Aditya Yoga: 水星-太阳合相
    fn check_budha_aditya(&self, sign_map: &HashMap<Planet, u8>) -> Vec<String> {
        let sun_sign = match sign_map.get(&Planet::Sun) {
            Some(s) => *s,
            None => return vec![],
        };
        let mercury_sign = match sign_map.get(&Planet::Mercury) {
            Some(s) => *s,
            None => return vec![],
        };

        if sun_sign == mercury_sign {
            vec![format!(
                "Budha Aditya Yoga (日水瑜伽) - 太阳与水星同在{}，增强学习、表达、分析、行政能力",
                sign_name_zh(sign_from_index(sun_sign))
            )]
        } else {
            vec![]
        }
    }

    /// Raj Yogas: 角宫主与三分宫主关联
    fn check_raj_yogas(
        &self,
        sign_map: &HashMap<Planet, u8>,
        asc_sign: u8,
    ) -> Vec<String> {
        let mut yogas = Vec::new();

        // 计算各宫主星
        let house_lords: HashMap<u8, Planet> = (1..=12)
            .map(|h| (h, sign_lord(sign_from_index((asc_sign as usize + h as usize - 1) as u8 % 12))))
            .collect();

        // 角宫主与三分宫主关联
        for &k_house in KENDRA {
            for &t_house in TRIKONA {
                let k_lord = house_lords[&k_house];
                let t_lord = house_lords[&t_house];
                if k_lord == t_lord {
                    continue;
                }
                if self.planets_associated(sign_map, k_lord, t_lord) {
                    yogas.push(format!(
                        "Kendra-Trikona Raja Yoga (角宫三分主星王瑜伽) - 第{}宫主{}与第{}宫主{}关联",
                        k_house, planet_name_zh(k_lord), t_house, planet_name_zh(t_lord)
                    ));
                }
            }
        }

        // Yogakaraka 行星
        if let Some(yogakaraka) = self.find_yogakaraka(&house_lords) {
            if let Some(&ys) = sign_map.get(&yogakaraka) {
                let house = rel_house(asc_sign, ys);
                if KENDRA.contains(&house) || TRIKONA.contains(&house) {
                    yogas.push(format!(
                        "Yogakaraka Planet (瑜伽成就星) - {}同时掌管角宫/三分宫，位于第{}宫",
                        planet_name_zh(yogakaraka), house
                    ));
                }
            }
        }

        yogas
    }

    /// Dhana Yogas: 财富宫主星关联
    fn check_dhana_yogas(
        &self,
        sign_map: &HashMap<Planet, u8>,
        asc_sign: u8,
    ) -> Vec<String> {
        let mut yogas = Vec::new();
        let house_lords: HashMap<u8, Planet> = (1..=12)
            .map(|h| (h, sign_lord(sign_from_index((asc_sign as usize + h as usize - 1) as u8 % 12))))
            .collect();

        let wealth_houses = [2, 5, 9, 11];
        for i in 0..wealth_houses.len() {
            for j in (i + 1)..wealth_houses.len() {
                let h1 = wealth_houses[i];
                let h2 = wealth_houses[j];
                let p1 = house_lords[&h1];
                let p2 = house_lords[&h2];
                if p1 != p2 && self.planets_associated(sign_map, p1, p2) {
                    yogas.push(format!(
                        "Dhana Yoga (财富瑜伽) - 第{}宫主{}与第{}宫主{}关联，增强积累与财富",
                        h1, planet_name_zh(p1), h2, planet_name_zh(p2)
                    ));
                }
            }
        }
        yogas
    }

    /// Viparita Raja Yogas: 困难宫主入困难宫
    fn check_viparita_raja(
        &self,
        sign_map: &HashMap<Planet, u8>,
        asc_sign: u8,
    ) -> Vec<String> {
        let mut yogas = Vec::new();
        let house_lords: HashMap<u8, Planet> = (1..=12)
            .map(|h| (h, sign_lord(sign_from_index((asc_sign as usize + h as usize - 1) as u8 % 12))))
            .collect();

        let specs = [
            (6u8, "Harsha Viparita Raja Yoga (哈沙逆转王瑜伽)", "第6宫主落入困难宫，转化竞争为胜利"),
            (8u8, "Sarala Viparita Raja Yoga (萨罗逆转王瑜伽)", "第8宫主落入困难宫，转化危机为突破"),
            (12u8, "Vimala Viparita Raja Yoga (毗摩罗逆转王瑜伽)", "第12宫主落入困难宫，转化损耗为净化"),
        ];

        for &(house, name, desc) in &specs {
            let lord = house_lords[&house];
            if let Some(&lord_sign) = sign_map.get(&lord) {
                let lord_house = rel_house(asc_sign, lord_sign);
                if DUSTHANA.contains(&lord_house) {
                    yogas.push(format!(
                        "{} - {}（第{}宫）", name, desc, lord_house
                    ));
                }
            }
        }
        yogas
    }

    /// Parivartana Yogas: 星座交换
    fn check_parivartana(
        &self,
        sign_map: &HashMap<Planet, u8>,
        asc_sign: u8,
    ) -> Vec<String> {
        let mut yogas = Vec::new();
        for &p1 in CLASSICAL_PLANETS {
            for &p2 in CLASSICAL_PLANETS {
                if p1 as u8 >= p2 as u8 {
                    continue;
                }
                let s1 = match sign_map.get(&p1) { Some(s) => *s, None => continue };
                let s2 = match sign_map.get(&p2) { Some(s) => *s, None => continue };
                if sign_lord(sign_from_index(s1)) == p2 && sign_lord(sign_from_index(s2)) == p1 {
                    let house_lords: HashMap<u8, Planet> = (1..=12)
                        .map(|h| (h, sign_lord(sign_from_index((asc_sign as usize + h as usize - 1) as u8 % 12))))
                        .collect();
                    let owned_houses: Vec<u8> = house_lords
                        .iter()
                        .filter(|(_, &p)| p == p1 || p == p2)
                        .map(|(&h, _)| h)
                        .collect();
                    yogas.push(format!(
                        "Parivartana Yoga (交换瑜伽) - {}在{}、{}在{}，互居对方星座",
                        planet_name_zh(p1), sign_name_zh(sign_from_index(s1)),
                        planet_name_zh(p2), sign_name_zh(sign_from_index(s2))
                    ));
                }
            }
        }
        yogas
    }

    /// Neecha Bhanga Yogas: 落陷取消
    fn check_neecha_bhanga(
        &self,
        sign_map: &HashMap<Planet, u8>,
        asc_sign: u8,
    ) -> Vec<String> {
        let mut yogas = Vec::new();
        for &planet in CLASSICAL_PLANETS {
            let sign_idx = match sign_map.get(&planet) { Some(s) => *s, None => continue };
            let sign = sign_from_index(sign_idx);
            let dignity = self.get_dignity(planet, sign, 15.0); // 假设在星座中段
            if dignity != "debilitation" {
                continue;
            }

            let dispositor = sign_lord(sign);
            let exalt = exaltation(planet);
            let mut conditions = Vec::new();

            // 落陷星座主在角宫
            if let Some(&disp_sign) = sign_map.get(&dispositor) {
                if KENDRA.contains(&rel_house(asc_sign, disp_sign)) {
                    conditions.push(format!("落陷星座主{}在角宫", planet_name_zh(dispositor)));
                }
                if sign_map.get(&dispositor) == Some(&sign_idx) {
                    conditions.push("落陷星与其星座主同宫".to_string());
                }
            }

            // 旺相星座主在角宫
            if let Some((exalt_sign, _)) = exalt {
                let exalt_lord = sign_lord(exalt_sign);
                if let Some(&el_sign) = sign_map.get(&exalt_lord) {
                    if KENDRA.contains(&rel_house(asc_sign, el_sign)) {
                        conditions.push(format!("旺相星座主{}在角宫", planet_name_zh(exalt_lord)));
                    }
                }
            }

            if !conditions.is_empty() {
                yogas.push(format!(
                    "Neecha Bhanga Raja Yoga (落陷取消王瑜伽) - {}在{}落陷，{}",
                    planet_name_zh(planet),
                    sign_name_zh(sign),
                    conditions.join("；")
                ));
            }
        }
        yogas
    }

    // ========================================================================
    // 5. Vargas（分盘）
    // ========================================================================

    /// 获取指定分盘
    pub fn get_varga(&self, chart: &VedicChart, division: u8) -> VargaChart {
        let planet_positions: Vec<(Planet, f64)> = chart
            .planets
            .iter()
            .map(|p| (p.planet, p.longitude))
            .collect();

        let asc_lon = chart.ascendant.longitude;
        self.calc_varga(division, &planet_positions, asc_lon)
    }

    fn calc_all_vargas(
        &self,
        planets: &[PlanetPosition],
        _asc_sign: u8,
    ) -> HashMap<u8, VargaChart> {
        let planet_positions: Vec<(Planet, f64)> = planets
            .iter()
            .map(|p| (p.planet, p.longitude))
            .collect();

        // 使用一个固定的上升点（简化：0°）
        let asc_lon = 0.0;

        let divisions = [1u8, 2, 3, 9, 12];
        let mut vargas = HashMap::new();
        for &div in &divisions {
            vargas.insert(div, self.calc_varga(div, &planet_positions, asc_lon));
        }
        vargas
    }

    fn calc_varga(
        &self,
        division: u8,
        planet_positions: &[(Planet, f64)],
        asc_lon: f64,
    ) -> VargaChart {
        let (name, name_zh) = match division {
            1 => ("D-1 Rasi", "本命盘"),
            2 => ("D-2 Hora", "财帛分盘"),
            3 => ("D-3 Drekkana", "兄弟分盘"),
            9 => ("D-9 Navamsa", "婚姻分盘"),
            12 => ("D-12 Dwadasamsa", "父母分盘"),
            _ => ("Unknown", "未知"),
        };

        let mut varga_planets = Vec::new();
        for &(planet, lon) in planet_positions {
            let varga_lon = self.varga_longitude(lon, division);
            let sign_idx = sign_index_from_lon(varga_lon);
            varga_planets.push(VargaPlanetPosition {
                planet,
                sign: sign_from_index(sign_idx),
                degree: varga_lon - sign_idx as f64 * 30.0,
            });
        }

        let asc_varga_lon = self.varga_longitude(asc_lon, division);
        let asc_sign = sign_from_index(sign_index_from_lon(asc_varga_lon));

        VargaChart {
            division,
            name: name.to_string(),
            name_zh: name_zh.to_string(),
            planets: varga_planets,
            ascendant_sign: asc_sign,
        }
    }

    /// 计算分盘中的经度
    fn varga_longitude(&self, lon: f64, division: u8) -> f64 {
        let normalized = norm(lon);
        let sign_idx = (normalized / 30.0) as u8;
        let degree_in_sign = normalized - sign_idx as f64 * 30.0;

        match division {
            1 => {
                // D-1 Rasi: 每个星座映射到自身
                normalized
            }
            2 => {
                // D-2 Hora: 每个星座分两半，前半归太阳，后半归月亮
                // 太阳主宰的 Hora → 狮子座，月亮主宰的 Hora → 巨蟹座
                if degree_in_sign < 15.0 {
                    // Sun Hora → Leo
                    4.0 * 30.0 + degree_in_sign * 2.0
                } else {
                    // Moon Hora → Cancer
                    3.0 * 30.0 + (degree_in_sign - 15.0) * 2.0
                }
            }
            3 => {
                // D-3 Drekkana: 每个星座分3等份，每份10°
                let part = (degree_in_sign / 10.0) as u8;
                let part_deg = degree_in_sign - part as f64 * 10.0;
                let varga_sign = match part {
                    0 => sign_idx,                          // 自身
                    1 => (sign_idx + 4) % 12,              // 第5宫
                    2 => (sign_idx + 8) % 12,              // 第9宫
                    _ => sign_idx,
                };
                varga_sign as f64 * 30.0 + part_deg * 3.0
            }
            9 => {
                // D-9 Navamsa: 每个星座分9等份，每份3°20'
                let part = (degree_in_sign / (30.0 / 9.0)) as u8;
                let remainder = degree_in_sign - part as f64 * (30.0 / 9.0);
                let varga_sign = match sign_idx {
                    0 | 4 | 8 => part,                              // 火象：从白羊开始
                    1 | 5 | 9 => (9 + part) % 12,                   // 土象：从摩羯开始
                    2 | 6 | 10 => (6 + part) % 12,                  // 风象：从天秤开始
                    _ => (3 + part) % 12,                            // 水象：从巨蟹开始
                };
                varga_sign as f64 * 30.0 + remainder * 9.0
            }
            12 => {
                // D-12 Dwadasamsa: 每个星座分12等份，每份2°30'
                let part = (degree_in_sign / 2.5) as u8;
                let remainder = degree_in_sign - part as f64 * 2.5;
                let varga_sign = (sign_idx + part) % 12;
                varga_sign as f64 * 30.0 + remainder * 12.0
            }
            _ => {
                // 通用公式：按等分映射
                let span = 30.0 / division as f64;
                let part = (degree_in_sign / span) as u8;
                let remainder = degree_in_sign - part as f64 * span;
                let varga_sign = (sign_idx + part) % 12;
                varga_sign as f64 * 30.0 + remainder * division as f64
            }
        }
    }

    // ========================================================================
    // 6. Planetary Dignity（星体尊严）
    // ========================================================================

    fn calc_dignities(
        &self,
        planets: &[PlanetPosition],
    ) -> HashMap<String, PlanetaryDignity> {
        let mut dignities = HashMap::new();
        for p in planets {
            let dignity = self.get_dignity(p.planet, p.sign, p.degree_in_sign);
            let status_zh = match dignity.as_str() {
                "deep_exaltation" => "精确旺相",
                "exaltation" => "旺相",
                "moolatrikona" => "本三角",
                "own_sign" => "本宫",
                "debilitation" => "落陷",
                _ => "平常",
            };
            dignities.insert(
                planet_key(p.planet),
                PlanetaryDignity {
                    planet: p.planet,
                    status: dignity,
                    status_zh: status_zh.to_string(),
                    sign: p.sign,
                    degree_in_sign: p.degree_in_sign,
                },
            );
        }
        dignities
    }

    /// 获取星体尊严状态
    fn get_dignity(&self, planet: Planet, sign: ZodiacSign, degree_in_sign: f64) -> String {
        // 检查旺相
        if let Some((exalt_sign, exalt_deg)) = exaltation(planet) {
            if sign == exalt_sign {
                return if (degree_in_sign - exalt_deg).abs() < 1.0 {
                    "deep_exaltation".to_string()
                } else {
                    "exaltation".to_string()
                };
            }
            // 检查落陷（旺相对宫）
            let exalt_idx = sign_index_from_lon(exalt_sign as u8 as f64 * 30.0);
            let deb_sign = sign_from_index((exalt_idx + 6) % 12);
            if sign == deb_sign {
                return "debilitation".to_string();
            }
        }

        // 检查本三角
        if let Some((mt_sign, mt_start, mt_end)) = moolatrikona(planet) {
            if sign == mt_sign && degree_in_sign >= mt_start && degree_in_sign <= mt_end {
                return "moolatrikona".to_string();
            }
        }

        // 检查本宫
        if own_signs(planet).contains(&sign) {
            return "own_sign".to_string();
        }

        "neutral".to_string()
    }

    // ========================================================================
    // 7. Ashtakavarga（八层占）
    // ========================================================================

    fn calc_ashtakavarga(
        &self,
        planets: &[PlanetPosition],
        asc_sign: u8,
    ) -> AshtakavargaData {
        let sign_map: HashMap<Planet, u8> = planets
            .iter()
            .map(|p| (p.planet, sign_index_from_lon(p.longitude)))
            .collect();

        let mut natal: HashMap<String, u8> = HashMap::new();
        for &planet in &[Planet::Sun, Planet::Moon, Planet::Mars, Planet::Mercury, Planet::Jupiter, Planet::Venus, Planet::Saturn] {
            if let Some(&s) = sign_map.get(&planet) {
                natal.insert(planet_name_en(planet).to_string(), s);
            }
        }
        natal.insert("Lagna".to_string(), asc_sign);

        if natal.len() < 8 {
            return AshtakavargaData {
                available: false,
                bhinna: HashMap::new(),
                sarva: HashMap::new(),
            };
        }

        let targets = ["Sun", "Moon", "Mars", "Mercury", "Jupiter", "Venus", "Saturn"];
        let mut bhinna: HashMap<String, HashMap<String, u8>> = HashMap::new();
        let mut sarva: HashMap<String, u8> = HashMap::new();

        for &target in &targets {
            let table = get_benefic_houses(target);
            let mut values: HashMap<String, u8> = HashMap::new();

            for &contributor_name in &["Sun", "Moon", "Mars", "Mercury", "Jupiter", "Venus", "Saturn", "Lagna"] {
                let source_key = if contributor_name == "Lagna" { "Lagna" } else { contributor_name };
                let source_sign = match natal.get(source_key) {
                    Some(s) => *s,
                    None => continue,
                };

                let houses = match table.get(contributor_name) {
                    Some(h) => *h,
                    None => continue,
                };

                for &house in houses {
                    let sign = sign_from_index((source_sign + house - 1) % 12);
                    let sign_name = sign_name_en(sign).to_string();
                    *values.entry(sign_name).or_insert(0) += 1;
                }
            }

            // 汇总到 sarva
            for (sign, count) in &values {
                *sarva.entry(sign.clone()).or_insert(0) += count;
            }

            bhinna.insert(target.to_string(), values);
        }

        AshtakavargaData {
            available: true,
            bhinna,
            sarva,
        }
    }

    // ========================================================================
    // 8. Panchanga（五支）
    // ========================================================================

    /// 计算任意日期的五支
    pub fn get_panchanga(&self, dt: DateTime<Utc>) -> Panchanga {
        let jd = datetime_to_jd(&dt);
        let ayanamsa = self.get_ayanamsa(jd);

        let sun_lon = norm(mean_longitude(Planet::Sun, jd) - ayanamsa);
        let moon_lon = norm(mean_longitude(Planet::Moon, jd) - ayanamsa);

        self.calc_panchanga(sun_lon, moon_lon, &dt)
    }

    fn calc_panchanga_from_planets(
        &self,
        planets: &[PlanetPosition],
        dt: &DateTime<Utc>,
    ) -> Panchanga {
        let sun_lon = planets
            .iter()
            .find(|p| p.planet == Planet::Sun)
            .map(|p| p.longitude)
            .unwrap_or(0.0);
        let moon_lon = planets
            .iter()
            .find(|p| p.planet == Planet::Moon)
            .map(|p| p.longitude)
            .unwrap_or(0.0);

        self.calc_panchanga(sun_lon, moon_lon, dt)
    }

    fn calc_panchanga(&self, sun_lon: f64, moon_lon: f64, dt: &DateTime<Utc>) -> Panchanga {
        let lunar_angle = norm(moon_lon - sun_lon);

        // Tithi（太阴日）
        let tithi_index = {
            let i = (lunar_angle / 12.0) as usize;
            if i >= 30 { 29 } else { i }
        };
        let tithi_day = tithi_index + 1;
        let paksha = if tithi_day <= 15 { "Shukla" } else { "Krishna" };
        let tithi_name_idx = (tithi_day - 1) % 15;
        let tithi = TithiInfo {
            index: tithi_day as u8,
            name: TITHI_NAMES[tithi_name_idx].to_string(),
            paksha: paksha.to_string(),
            angle: lunar_angle,
            progress: (lunar_angle % 12.0) / 12.0,
        };

        // Vara（星期）
        let weekday = dt.weekday().num_days_from_sunday() as usize;
        let vara = VaraInfo {
            index: weekday as u8,
            name: VARA_NAMES[weekday].to_string(),
            name_zh: VARA_CN[weekday].to_string(),
            lord: VARA_LORDS[weekday],
        };

        // Nakshatra（月亮星宿）
        let nakshatra = self.get_nakshatra(moon_lon);

        // Nitya Yoga（Panchanga 瑜伽）
        let yoga_sum = norm(sun_lon + moon_lon);
        let yoga_index = {
            let i = (yoga_sum / (360.0 / 27.0)) as usize;
            if i >= 27 { 26 } else { i }
        };
        let yoga = PanchangaYogaInfo {
            index: yoga_index as u8 + 1,
            name: NITYA_YOGA_NAMES[yoga_index].to_string(),
            progress: (yoga_sum % (360.0 / 27.0)) / (360.0 / 27.0),
        };

        // Karana（半太阴日）
        let karana_index = {
            let i = (lunar_angle / 6.0) as usize;
            if i >= 60 { 59 } else { i }
        };
        let karana_name = match karana_index {
            0 => "Kimstughna".to_string(),
            57 => "Shakuni".to_string(),
            58 => "Chatushpada".to_string(),
            59 => "Naga".to_string(),
            _ => KARANA_SEQUENCE[(karana_index - 1) % KARANA_SEQUENCE.len()].to_string(),
        };
        let karana = KaranaInfo {
            index: karana_index as u8 + 1,
            name: karana_name,
        };

        Panchanga {
            tithi,
            vara,
            nakshatra,
            yoga,
            karana,
        }
    }

    // ========================================================================
    // 辅助方法
    // ========================================================================

    /// 计算近似上升点（简化算法）
    fn calc_ascendant_approx(&self, jd: f64, ayanamsa: f64, latitude: f64) -> f64 {
        // 使用太阳的黄经 + 地方恒星时估算
        let sun_lon = mean_longitude(Planet::Sun, jd);

        // 格林尼治恒星时（近似）
        let t = (jd - 2451545.0) / 36525.0;
        let gmst = norm(
            280.46061837
                + 360.98564736629 * (jd - 2451545.0)
                + 0.000387933 * t * t
                - t * t * t / 38710000.0,
        );

        // 地方恒星时（简化，不考虑经度）
        let lst = norm(gmst + 0.0); // 假定经度0

        // 上升点近似（简化公式）
        let obliquity: f64 = 23.439291;
        let lat_rad = latitude.to_radians();
        let obl_rad = obliquity.to_radians();

        let asc_trop = norm(
            lst.to_radians().atan2(
                -(lst.to_radians().sin() * obl_rad.cos() + lat_rad.tan() * obl_rad.sin())
            ).to_degrees()
        );

        // 转换为恒星黄道
        norm(asc_trop - ayanamsa)
    }

    /// 等宫制宫位计算
    fn calc_whole_sign_houses(&self, ascendant: &Ascendant) -> Vec<House> {
        let asc_sign_idx = sign_index_from_lon(ascendant.longitude);
        let mut houses = Vec::with_capacity(12);

        for i in 0..12 {
            let sign_idx = (asc_sign_idx + i as u8) % 12;
            let sign = sign_from_index(sign_idx);
            let cusp = sign_idx as f64 * 30.0;
            houses.push(House {
                number: i + 1,
                cusp,
                sign,
                degree: 0.0,
                size: 30.0,
            });
        }

        houses
    }

    /// 检查星体是否强壮（在本宫/旺相/本三角）
    fn is_planet_strong(&self, planet: Planet, sign_idx: u8, _degree: f64) -> bool {
        let sign = sign_from_index(sign_idx);
        let dignity = self.get_dignity(planet, sign, 15.0);
        matches!(
            dignity.as_str(),
            "deep_exaltation" | "exaltation" | "moolatrikona" | "own_sign"
        )
    }

    /// 检查两个星体是否关联（同宫/相位/交换）
    fn planets_associated(
        &self,
        sign_map: &HashMap<Planet, u8>,
        a: Planet,
        b: Planet,
    ) -> bool {
        let sa = match sign_map.get(&a) { Some(s) => *s, None => return false };
        let sb = match sign_map.get(&b) { Some(s) => *s, None => return false };

        // 同宫
        if sa == sb {
            return true;
        }

        // 相位
        if self.has_aspect(a, sa, sb) || self.has_aspect(b, sb, sa) {
            return true;
        }

        // 交换
        if sign_lord(sign_from_index(sa)) == b && sign_lord(sign_from_index(sb)) == a {
            return true;
        }

        false
    }

    /// 检查星体 giver 是否照射到 target_sign
    fn has_aspect(&self, giver: Planet, giver_sign: u8, target_sign: u8) -> bool {
        let target_house = rel_house(giver_sign, target_sign);
        aspect_houses(giver).contains(&target_house)
    }

    /// 查找 Yogakaraka 行星（同时掌握角宫和三分宫）
    fn find_yogakaraka(&self, house_lords: &HashMap<u8, Planet>) -> Option<Planet> {
        for &planet in CLASSICAL_PLANETS {
            let owned: Vec<u8> = house_lords
                .iter()
                .filter(|(_, &p)| p == planet)
                .map(|(&h, _)| h)
                .collect();

            let has_kendra = owned.iter().any(|h| KENDRA.contains(h));
            let has_trikona = owned.iter().any(|h| *h == 5 || *h == 9);

            if has_kendra && has_trikona {
                return Some(planet);
            }
        }
        None
    }
}

// ============================================================================
// 辅助函数
// ============================================================================

/// 获取大运年数
fn dasha_years(planet: Planet) -> f64 {
    for &(p, years) in DASHA_SEQUENCE {
        if p == planet {
            return years;
        }
    }
    7.0 // 默认
}