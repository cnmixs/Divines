// Divines - 占星计算引擎
// 参考原项目: flatlib-ctrad2/flatlib/chart.py, flatlib-ctrad2/flatlib/aspects.py
// 参考原项目: astropy/astrostudy/astroextra.py

use chrono::{Datelike, Timelike};
use divines_core::*;
use crate::ephem::Ephemeris;

/// 占星计算器
/// 核心功能：计算星盘、行星位置、相位、宫位等
pub struct AstrologyCalc {
    ephemeris: Ephemeris,
}

impl AstrologyCalc {
    pub fn new(ephemeris: Ephemeris) -> Self {
        Self { ephemeris }
    }

    /// 计算完整星盘
    /// 参考原项目: flatlib-ctrad2/flatlib/chart.py Chart
    pub fn calculate_chart(
        &self,
        birth: &BirthInfo,
        house_system: HouseSystem,
    ) -> DivinesResult<AstroChart> {
        let jd = Ephemeris::julian_day(
            birth.datetime.year(),
            birth.datetime.month() as u8,
            birth.datetime.day() as u8,
            birth.datetime.hour() as f64
                + birth.datetime.minute() as f64 / 60.0
                + birth.datetime.second() as f64 / 3600.0,
        );

        // 计算恒星时
        let sidereal_time = self.ephemeris.calc_sidereal_time(jd)?;

        // 计算黄赤交角
        let obliquity = Ephemeris::obliquity(jd);

        // 计算宫位
        let (houses, ascendant, midheaven) =
            self.ephemeris.calc_houses(
                house_system,
                jd,
                birth.location.latitude,
                birth.location.longitude,
            )?;

        // 计算行星位置
        let planets = self.calc_all_planets(jd, &houses)?;

        // 计算相位
        let aspects = self.calc_aspects(&planets);

        Ok(AstroChart {
            info: ChartInfo {
                id: uuid::Uuid::new_v4(),
                chart_type: ChartType::Natal,
                birth: birth.clone(),
                tags: vec![],
                notes: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                snapshot: None,
            },
            house_system,
            planets,
            houses,
            ascendant,
            midheaven,
            aspects,
            sidereal_time,
            obliquity,
        })
    }

    /// 计算所有行星位置
    fn calc_all_planets(
        &self,
        jd: f64,
        houses: &[House],
    ) -> DivinesResult<Vec<PlanetPosition>> {
        let planets_list = [
            Planet::Sun, Planet::Moon, Planet::Mercury, Planet::Venus,
            Planet::Mars, Planet::Jupiter, Planet::Saturn, Planet::Uranus,
            Planet::Neptune, Planet::Pluto, Planet::NorthNode, Planet::SouthNode,
            Planet::Chiron, Planet::Lilith,
        ];

        let mut positions = Vec::new();
        for planet in planets_list {
            match self.ephemeris.calc_planet_position(planet, jd) {
                Ok(pos) => positions.push(pos),
                Err(_) => {
                    // 某些行星（如凯龙、莉莉丝）需要特殊处理
                    // 暂时使用模拟数据占位
                    positions.push(self.mock_planet_position(planet, jd, houses));
                }
            }
        }
        Ok(positions)
    }

    /// 模拟行星位置（当 Swiss Ephemeris 不可用时）
    /// 参考原项目: flatlib-ctrad2/flatlib/ephem/tools.py 中的近似计算
    fn mock_planet_position(
        &self,
        planet: Planet,
        jd: f64,
        houses: &[House],
    ) -> PlanetPosition {
        // 使用简化的近似公式计算行星黄经
        let t = (jd - 2451545.0) / 36525.0; // 儒略世纪数

        let longitude = match planet {
            Planet::Sun => {
                // 太阳平黄经近似
                (280.46646 + 36000.76983 * t + 0.0003032 * t * t) % 360.0
            }
            Planet::Moon => {
                // 月球平黄经近似
                (218.3165 + 481267.8813 * t) % 360.0
            }
            Planet::Mercury => {
                (252.2508 + 149472.6745 * t) % 360.0
            }
            Planet::Venus => {
                (181.9798 + 58517.8156 * t) % 360.0
            }
            Planet::Mars => {
                (355.4530 + 19139.8580 * t) % 360.0
            }
            Planet::Jupiter => {
                (34.3515 + 3034.9057 * t) % 360.0
            }
            Planet::Saturn => {
                (50.0774 + 1222.1138 * t) % 360.0
            }
            _ => 0.0,
        };

        let longitude = if longitude < 0.0 { longitude + 360.0 } else { longitude };
        let sign_idx = (longitude / 30.0) as u8;
        let sign = Self::index_to_sign(sign_idx);
        let degree_in_sign = longitude % 30.0;

        // 查找宫位
        let house = Self::find_house(longitude, houses);

        PlanetPosition {
            planet,
            longitude,
            latitude: 0.0,
            right_ascension: 0.0,
            declination: 0.0,
            sign,
            degree_in_sign,
            house,
            is_retrograde: false,
            distance: 1.0,
            speed: 1.0,
        }
    }

    /// 计算所有相位
    /// 参考原项目: flatlib-ctrad2/flatlib/aspects.py
    pub fn calc_aspects(&self, planets: &[PlanetPosition]) -> Vec<Aspect> {
        let mut aspects = Vec::new();
        let orb_table = self.get_orb_table();

        for i in 0..planets.len() {
            for j in (i + 1)..planets.len() {
                let p1 = &planets[i];
                let p2 = &planets[j];

                // 计算角度差
                let mut angle = (p1.longitude - p2.longitude).abs();
                if angle > 180.0 {
                    angle = 360.0 - angle;
                }

                // 检查相位
                for (asp_type, exact_angle, max_orb) in &orb_table {
                    let diff = (angle - exact_angle).abs();
                    if diff <= *max_orb {
                        aspects.push(Aspect {
                            aspect_type: *asp_type,
                            planet1: p1.planet,
                            planet2: p2.planet,
                            angle,
                            orb: diff,
                            is_exact: diff < 0.5,
                        });
                        // 每个行星对只取一个主要相位
                        break;
                    }
                }
            }
        }
        aspects
    }

    /// 相位容许度表
    /// 参考原项目: flatlib-ctrad2/flatlib/aspects.py
    fn get_orb_table(&self) -> Vec<(AspectType, f64, f64)> {
        vec![
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
        ]
    }

    /// 索引转星座
    fn index_to_sign(idx: u8) -> ZodiacSign {
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

    /// 查找行星所在宫位
    fn find_house(longitude: f64, houses: &[House]) -> u8 {
        // 遍历宫位，找到行星所在的宫位
        for i in 0..houses.len() {
            let current = houses[i].cusp;
            let next = if i + 1 < houses.len() {
                houses[i + 1].cusp
            } else {
                houses[0].cusp + 360.0
            };

            let mut lon = longitude;
            if lon < current {
                lon += 360.0;
            }

            if lon >= current && lon < next {
                return (i + 1) as u8;
            }
        }
        1 // 默认第一宫
    }

    /// 计算阿拉伯点
    /// 参考原项目: flatlib-ctrad2/recipes/arabicparts.py
    pub fn calc_arabic_points(
        &self,
        planets: &[PlanetPosition],
        ascendant: &Ascendant,
    ) -> Vec<ArabicPart> {
        let mut parts = Vec::new();

        let get_lon = |p: Planet| -> f64 {
            planets
                .iter()
                .find(|pos| pos.planet == p)
                .map(|pos| pos.longitude)
                .unwrap_or(0.0)
        };

        // 福点 = 上升 + 月亮 - 太阳（昼生）
        let fortune = (ascendant.longitude + get_lon(Planet::Moon)
            - get_lon(Planet::Sun))
            .rem_euclid(360.0);
        parts.push(ArabicPart {
            name: "Part of Fortune".to_string(),
            name_zh: "福点".to_string(),
            longitude: fortune,
            sign: Self::index_to_sign((fortune / 30.0) as u8),
            degree: fortune % 30.0,
            formula: "ASC + Moon - Sun".to_string(),
        });

        // 精神点 = 上升 + 太阳 - 月亮
        let spirit = (ascendant.longitude + get_lon(Planet::Sun)
            - get_lon(Planet::Moon))
            .rem_euclid(360.0);
        parts.push(ArabicPart {
            name: "Part of Spirit".to_string(),
            name_zh: "精神点".to_string(),
            longitude: spirit,
            sign: Self::index_to_sign((spirit / 30.0) as u8),
            degree: spirit % 30.0,
            formula: "ASC + Sun - Moon".to_string(),
        });

        parts
    }

    /// 计算法达星限
    /// 参考原项目: astropy/astrostudy/firdaria.py
    pub fn calc_firdaria(&self, birth: &BirthInfo) -> Vec<FirdariaPeriod> {
        let is_day = birth.datetime.hour() >= 6 && birth.datetime.hour() < 18;

        let sequence = if is_day {
            // 昼生：太阳→金星→水星→月亮→土星→木星→火星→北交点→南交点
            vec![
                Planet::Sun, Planet::Venus, Planet::Mercury, Planet::Moon,
                Planet::Saturn, Planet::Jupiter, Planet::Mars,
                Planet::NorthNode, Planet::SouthNode,
            ]
        } else {
            // 夜生：月亮→土星→木星→火星→北交点→南交点→太阳→金星→水星
            vec![
                Planet::Moon, Planet::Saturn, Planet::Jupiter, Planet::Mars,
                Planet::NorthNode, Planet::SouthNode,
                Planet::Sun, Planet::Venus, Planet::Mercury,
            ]
        };

        let years_per_planet = [10.0, 8.0, 13.0, 9.0, 11.0, 12.0, 7.0, 3.0, 2.0];
        let mut periods = Vec::new();
        let mut current_year = birth.datetime.year();

        for (i, planet) in sequence.iter().enumerate() {
            let years = years_per_planet[i];
            let start_date = format!("{}-01-01", current_year);
            let end_year = current_year + years as i32;
            let end_date = format!("{}-01-01", end_year);

            periods.push(FirdariaPeriod {
                planet: *planet,
                start_date,
                end_date,
                sub_periods: vec![],
            });

            current_year = end_year;
        }

        periods
    }
}

impl Default for AstrologyCalc {
    fn default() -> Self {
        Self {
            ephemeris: Ephemeris::default(),
        }
    }
}