// Divines - 八字计算引擎
// 参考原项目: astrostudysrv/astrostudycn/BaZiBirth.java, PaiBaZi.java
// 参考原项目: vendor/kinastro/astro/bazi/calculator.py
// 参考: RedSC1/bazi_core (干支刑冲合害、长生十二神)
// 参考: hkargc/JavaScript-For-Paipan (真太阳时、早晚子时)
// 参考: tiandirenwx/libsxwnl (平气/定气选项)
//
// 排盘系统调用寿星万年历（Sxwnl）获取精确的节气时刻和农历数据

use chrono::{DateTime, Datelike, Timelike, Utc};
use divines_core::*;
use crate::sxwnl::Sxwnl;

pub mod data;
pub mod season;
pub mod gods;
pub mod tiaohou;
pub mod ganzhi_relative;
pub mod gong12;

/// 八字计算器
///
/// 使用寿星天文历（Sxwnl）进行精确的节气边界计算
pub struct BaziCalc {
    sxwnl: Sxwnl,
}

impl BaziCalc {
    pub fn new(sxwnl: Sxwnl) -> Self {
        Self { sxwnl }
    }

    /// 计算八字（主入口）
    ///
    /// 参考: PaiBaZi.java, bazi_core
    pub fn calculate(&self, birth: &BirthInfo, options: &BaziOptions) -> BaziChart {
        let dt = birth.datetime;
        let year = dt.year();
        let month = dt.month();
        let day = dt.day();

        // 使用本地时间（而非UTC）计算时柱
        // 出生时间是用户输入的本地时间
        let local_hour = Self::parse_local_hour(&birth.local_datetime);

        // === 真太阳时校正 ===
        // 参考: hkargc/JavaScript-For-Paipan
        let adjusted_hour = if options.use_true_solar_time {
            Self::calc_true_solar_time(local_hour, dt, options.longitude)
        } else {
            local_hour
        };

        // 提取校正后的小时和分钟
        let adj_h = adjusted_hour.floor() as i32;
        let adj_h_u32 = adj_h.rem_euclid(24) as u32;

        // === 早晚子时处理 ===
        // 参考: hkargc/JavaScript-For-Paipan
        // 23:00-00:00 为晚子时，部分流派日柱用次日
        let (day_for_pillar, hour_for_pillar, day_adjust) = if options.use_early_late_zi {
            if adj_h_u32 == 23 {
                // 晚子时: 时柱用当天日干，日柱仍用当天
                (day, 23, 0)
            } else if adj_h_u32 == 0 {
                // 早子时: 日柱已过23点，使用次日
                let next_day = Self::add_days(year, month, day, 1);
                (next_day.2, 0, 1)
            } else {
                (day, adj_h_u32, 0)
            }
        } else {
            // 传统算法：23点后日柱用次日
            if adj_h_u32 >= 23 {
                let next_day = Self::add_days(year, month, day, 1);
                (next_day.2, adj_h_u32, 1)
            } else {
                (day, adj_h_u32, 0)
            }
        };

        let calc_year = year;
        let calc_month = month;
        let calc_day = day_for_pillar;

        // 使用万年历获取精确节气数据
        let jieqi_curr = self.sxwnl.get_year_jieqi(year);
        let jieqi_prev = self.sxwnl.get_year_jieqi(year - 1);

        // 年柱（以立春为界，使用寿星历精确计算）
        let year_pillar = self.calc_year_pillar(calc_year, calc_month, calc_day, &jieqi_curr);

        // 月柱（以节气为界，使用寿星历精确计算）
        let month_pillar = if options.use_ding_qi {
            self.calc_month_pillar_dingqi(calc_year, calc_month, calc_day, &jieqi_curr)
        } else {
            self.calc_month_pillar_pingqi(calc_year, calc_month, calc_day, &jieqi_curr, &jieqi_prev)
        };

        // 日柱（使用寿星历精确日干支）
        let day_pillar = if day_adjust == 1 {
            // 子时校正：使用次日
            let next = Self::add_days(year, month, day, 1);
            self.calc_day_pillar(next.0, next.1, next.2)
        } else {
            self.calc_day_pillar(calc_year, calc_month, calc_day)
        };

        // 时柱（五鼠遁，使用校正后的小时）
        let hour_pillar = self.calc_hour_pillar(day_pillar.tian_gan, hour_for_pillar);

        // 日干
        let day_master = day_pillar.tian_gan;
        let day_master_wuxing = Self::tian_gan_wuxing(day_master);
        let day_master_yinyang = Self::tian_gan_yinyang(day_master);

        let hidden_stems = self.calc_hidden_stems(&year_pillar, &month_pillar, &day_pillar, &hour_pillar);
        let ten_gods = self.calc_ten_gods(day_master, &year_pillar, &month_pillar, &day_pillar, &hour_pillar);
        let na_yin = self.calc_na_yin(&year_pillar, &month_pillar, &day_pillar, &hour_pillar);
        let kong_wang = self.calc_kong_wang(&day_pillar);

        // 使用 gods 模块计算完整神煞系统
        let four_pillars = [year_pillar, month_pillar, day_pillar, hour_pillar];
        let god_results = gods::find_gods(&four_pillars);
        let mut shen_sha: Vec<ShenSha> = god_results
            .into_iter()
            .map(|(name, pillar, jixiong)| ShenSha {
                name,
                pillar,
                description: jixiong,
            })
            .collect();

        // 补充硬编码的常用神煞（避免 gods.json 缺失的部分）
        self.append_builtin_shen_sha(&year_pillar, &month_pillar, &day_pillar, &hour_pillar, &mut shen_sha);

        let (qi_yun_age, qi_yun_time) = self.calc_qi_yun_time(birth, &year_pillar, &month_pillar, &jieqi_curr, &jieqi_prev);
        let da_yun = self.calc_da_yun(birth, &year_pillar, &month_pillar, qi_yun_age);

        // 长生十二神
        let chang_sheng = self.calc_chang_sheng(day_master, &year_pillar, &month_pillar, &day_pillar, &hour_pillar);

        // 干支刑冲合害
        let relations = self.calc_relations(&year_pillar, &month_pillar, &day_pillar, &hour_pillar);

        // 季节旺衰
        let season = season::SeasonHelper::get_full_state(month_pillar.di_zhi.name_zh());

        // 调候用神
        let tiaohou_values = tiaohou::TiaoHou::get_tiaohou(day_master, month_pillar.di_zhi);

        // 十二宫
        let gong12_info = gong12::Gong12::get_ming_gong(month_pillar.di_zhi, hour_pillar.di_zhi);
        let gong12_json = serde_json::json!({
            "ming_zhi": gong12_info.ming_zhi,
            "ming_ganzhi": gong12_info.ming_ganzhi,
            "palace_gods": gong12_info.palace_gods,
            "stars": gong12_info.stars,
            "gua": gong12_info.gua,
        });

        let mut bazi = BaziChart {
            year: year_pillar,
            month: month_pillar,
            day: day_pillar,
            hour: hour_pillar,
            day_master,
            day_master_wuxing,
            day_master_yinyang,
            hidden_stems,
            ten_gods,
            na_yin,
            kong_wang,
            shen_sha,
            da_yun,
            qi_yun_time,
            pattern: BaziPattern {
                name: String::new(),
                description: String::new(),
                strength: PatternStrength::Normal,
            },
            chang_sheng,
            relations,
            options: options.clone(),
            adjusted_hour,
            season,
            tiaohou: tiaohou_values,
            gong12: gong12_json,
        };

        bazi.pattern = self.calc_pattern(&bazi);
        bazi
    }

    // ============ 真太阳时计算 ============

    /// 计算真太阳时
    ///
    /// 参考: hkargc/JavaScript-For-Paipan
    /// 真太阳时 = 平太阳时 + 均时差(EoT)
    /// 平太阳时 = 本地时间 + (经度 - 120°) * 4分钟
    fn calc_true_solar_time(local_hour: f64, dt: DateTime<Utc>, longitude: f64) -> f64 {
        // 经度修正：每度4分钟
        let lon_correction = (longitude - 120.0) * 4.0 / 60.0;

        // 均时差 (Equation of Time)
        let eot = Self::calc_eot(dt);

        // 真太阳时 = 本地时间 + 经度修正 + 均时差
        let true_solar = local_hour + lon_correction + eot;

        // 归一化到 [0, 24)
        true_solar.rem_euclid(24.0)
    }

    /// 计算均时差 (Equation of Time)
    ///
    /// 精度约 +/- 30秒
    fn calc_eot(dt: DateTime<Utc>) -> f64 {
        use std::f64::consts::PI;

        // 年积日
        let doy = dt.ordinal() as f64;

        // 太阳平黄经
        let b = (2.0 * PI / 365.0) * (doy - 81.0);

        // 均时差公式 (单位: 分钟)
        let eot_min = 9.87 * (2.0 * b).sin() - 7.53 * b.cos() - 1.5 * b.sin();

        // 转换为小时
        eot_min / 60.0
    }

    // ============ 年柱计算 ============

    fn calc_year_pillar(&self, _year: i32, _month: u32, _day: u32, jieqi: &[JieQi]) -> Pillar {
        let li_chun = jieqi.iter().find(|jq| jq.name_zh == "立春");

        let adjusted_year = if let Some(lc) = li_chun {
            if let Some((lc_year, lc_month, lc_day)) = Self::parse_jq_date(&lc.datetime) {
                if _year < lc_year || (_year == lc_year && _month < lc_month)
                    || (_year == lc_year && _month == lc_month && _day < lc_day)
                {
                    _year - 1
                } else {
                    _year
                }
            } else {
                if _month < 2 || (_month == 2 && _day < 4) { _year - 1 } else { _year }
            }
        } else {
            if _month < 2 || (_month == 2 && _day < 4) { _year - 1 } else { _year }
        };

        let tg_idx = (adjusted_year - 4).rem_euclid(10) as u8;
        let dz_idx = (adjusted_year - 4).rem_euclid(12) as u8;

        Pillar {
            tian_gan: Self::index_to_tian_gan(tg_idx),
            di_zhi: Self::index_to_di_zhi(dz_idx),
        }
    }

    // ============ 月柱计算（定气法） ============

    fn calc_month_pillar_dingqi(&self, year: i32, month: u32, day: u32, jieqi: &[JieQi]) -> Pillar {
        let year_pillar = self.calc_year_pillar(year, month, day, jieqi);
        let year_tg = year_pillar.tian_gan;

        let base_tg = match year_tg {
            TianGan::Jia | TianGan::Ji => 2,
            TianGan::Yi | TianGan::Geng => 4,
            TianGan::Bing | TianGan::Xin => 6,
            TianGan::Ding | TianGan::Ren => 8,
            TianGan::Wu | TianGan::Gui => 0,
        };

        let jie_indices = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];
        let dz_map: [usize; 12] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 1];

        let mut month_dz = 2; // 默认寅月
        for (i, &jie_idx) in jie_indices.iter().enumerate() {
            if jie_idx < jieqi.len() {
                if let Some((jq_year, jq_month, jq_day)) = Self::parse_jq_date(&jieqi[jie_idx].datetime) {
                    if year > jq_year || (year == jq_year && month > jq_month)
                        || (year == jq_year && month == jq_month && day >= jq_day)
                    {
                        month_dz = dz_map[i];
                    }
                }
            }
        }

        let month_dz_u8 = month_dz as u8;
        let tg_idx = (base_tg + month_dz_u8 as i32) % 10;

        Pillar {
            tian_gan: Self::index_to_tian_gan(tg_idx as u8),
            di_zhi: Self::index_to_di_zhi(month_dz_u8),
        }
    }

    // ============ 月柱计算（平气法） ============

    /// 平气法月柱计算
    ///
    /// 参考: tiandirenwx/libsxwnl
    ///
    /// 平气法将回归年等分为24份，每份约15.218425天
    /// 从冬至开始，每隔15.218425天为一个节气
    /// 虽然平气法精度不如定气法，但它是历史上长期使用的方法
    fn calc_month_pillar_pingqi(&self, year: i32, month: u32, day: u32, jieqi: &[JieQi], jieqi_prev: &[JieQi]) -> Pillar {
        let year_pillar = self.calc_year_pillar(year, month, day, jieqi);
        let year_tg = year_pillar.tian_gan;

        // 五虎遁月干基数
        let base_tg = match year_tg {
            TianGan::Jia | TianGan::Ji => 2,
            TianGan::Yi | TianGan::Geng => 4,
            TianGan::Bing | TianGan::Xin => 6,
            TianGan::Ding | TianGan::Ren => 8,
            TianGan::Wu | TianGan::Gui => 0,
        };

        // 平气法：从冬至开始，每个节气间隔相等
        // 回归年长度：约365.2422天
        // 每节气间隔：365.2422 / 24 ≈ 15.218425 天
        let tropical_year = 365.2422;
        let jieqi_interval = tropical_year / 24.0;

        // 获取当前年冬至的儒略日
        let dongzhi_curr = jieqi.iter().find(|jq| jq.name_zh == "冬至");
        let dz_jd_curr = if let Some(dz) = dongzhi_curr {
            if let Some((dz_year, dz_month, dz_day)) = Self::parse_jq_date(&dz.datetime) {
                crate::sxwnl::julian::JulianDay::to_jd(dz_year, dz_month, dz_day as f64)
            } else {
                crate::sxwnl::julian::JulianDay::to_jd(year, 12, 22.0)
            }
        } else {
            crate::sxwnl::julian::JulianDay::to_jd(year, 12, 22.0)
        };

        // 出生日期的儒略日
        let birth_jd = crate::sxwnl::julian::JulianDay::to_jd(year, month, day as f64);

        // 确定以哪个冬至为参考点
        // 平气法的节气年从冬至开始，如果出生日期在冬至之前，使用上一年的冬至
        let dz_jd = if birth_jd <= dz_jd_curr {
            // 出生在冬至之前，使用上一年的冬至
            let dongzhi_prev = jieqi_prev.iter().find(|jq| jq.name_zh == "冬至");
            if let Some(dz) = dongzhi_prev {
                if let Some((dz_year, dz_month, dz_day)) = Self::parse_jq_date(&dz.datetime) {
                    crate::sxwnl::julian::JulianDay::to_jd(dz_year, dz_month, dz_day as f64)
                } else {
                    dz_jd_curr - tropical_year
                }
            } else {
                dz_jd_curr - tropical_year
            }
        } else {
            dz_jd_curr
        };

        // 平气法节气名称与索引（从冬至开始）
        // 索引 0=冬至, 1=小寒, 2=大寒, 3=立春, 4=雨水, 5=惊蛰,
        //      6=春分, 7=清明, 8=谷雨, 9=立夏, 10=小满, 11=芒种,
        //      12=夏至, 13=小暑, 14=大暑, 15=立秋, 16=处暑, 17=白露,
        //      18=秋分, 19=寒露, 20=霜降, 21=立冬, 22=小雪, 23=大雪
        //
        // 月支映射（以节为月始）：
        // 寅月: 立春(idx=3)→惊蛰(idx=5)   卯月: 惊蛰(idx=5)→清明(idx=7)
        // 辰月: 清明(idx=7)→立夏(idx=9)   巳月: 立夏(idx=9)→芒种(idx=11)
        // 午月: 芒种(idx=11)→小暑(idx=13)  未月: 小暑(idx=13)→立秋(idx=15)
        // 申月: 立秋(idx=15)→白露(idx=17)  酉月: 白露(idx=17)→寒露(idx=19)
        // 戌月: 寒露(idx=19)→立冬(idx=21)  亥月: 立冬(idx=21)→大雪(idx=23)
        // 子月: 大雪(idx=23)→小寒(idx=25%24=1) 丑月: 小寒(idx=25%24=1)→立春(idx=27%24=3)

        // 月支对应的节索引范围 [节始, 节终)，从冬至起算
        let month_jie_ranges: [(i32, i32, usize); 12] = [
            (3, 5, 2),    // 寅月: 立春(3) → 惊蛰(5), 地支索引2
            (5, 7, 3),    // 卯月: 惊蛰(5) → 清明(7), 地支索引3
            (7, 9, 4),    // 辰月: 清明(7) → 立夏(9), 地支索引4
            (9, 11, 5),   // 巳月: 立夏(9) → 芒种(11), 地支索引5
            (11, 13, 6),  // 午月: 芒种(11) → 小暑(13), 地支索引6
            (13, 15, 7),  // 未月: 小暑(13) → 立秋(15), 地支索引7
            (15, 17, 8),  // 申月: 立秋(15) → 白露(17), 地支索引8
            (17, 19, 9),  // 酉月: 白露(17) → 寒露(19), 地支索引9
            (19, 21, 10), // 戌月: 寒露(19) → 立冬(21), 地支索引10
            (21, 23, 11), // 亥月: 立冬(21) → 大雪(23), 地支索引11
            (23, 25, 0),  // 子月: 大雪(23) → 小寒(25%24=1), 地支索引0
            (25, 27, 1),  // 丑月: 小寒(25%24=1) → 立春(27%24=3), 地支索引1
        ];

        let mut month_dz: usize = 2; // 默认寅月

        for (jie_start, jie_end, dz_idx) in &month_jie_ranges {
            let start_jd = dz_jd + (*jie_start as f64) * jieqi_interval;
            let end_jd = dz_jd + (*jie_end as f64) * jieqi_interval;

            if birth_jd >= start_jd && birth_jd < end_jd {
                month_dz = *dz_idx;
                break;
            }
        }

        // 五虎遁：月干 = (年干基数 + 月支索引) % 10
        let tg_idx = (base_tg + month_dz as i32) % 10;

        Pillar {
            tian_gan: Self::index_to_tian_gan(tg_idx as u8),
            di_zhi: Self::index_to_di_zhi(month_dz as u8),
        }
    }

    fn calc_day_pillar(&self, year: i32, month: u32, day: u32) -> Pillar {
        let ganzhi = crate::sxwnl::calendar::CalendarCalc::get_day_ganzhi(year, month, day);

        if let (Some(tg_char), Some(dz_char)) = (ganzhi.chars().next(), ganzhi.chars().nth(1)) {
            let tian_gan = match tg_char {
                '甲' => TianGan::Jia, '乙' => TianGan::Yi,
                '丙' => TianGan::Bing, '丁' => TianGan::Ding,
                '戊' => TianGan::Wu, '己' => TianGan::Ji,
                '庚' => TianGan::Geng, '辛' => TianGan::Xin,
                '壬' => TianGan::Ren, '癸' => TianGan::Gui,
                _ => TianGan::Jia,
            };

            let di_zhi = match dz_char {
                '子' => DiZhi::Zi, '丑' => DiZhi::Chou,
                '寅' => DiZhi::Yin, '卯' => DiZhi::Mao,
                '辰' => DiZhi::Chen, '巳' => DiZhi::Si,
                '午' => DiZhi::Wu, '未' => DiZhi::Wei,
                '申' => DiZhi::Shen, '酉' => DiZhi::You,
                '戌' => DiZhi::Xu, '亥' => DiZhi::Hai,
                _ => DiZhi::Zi,
            };

            Pillar { tian_gan, di_zhi }
        } else {
            let total_days = Self::days_since_1900(year, month, day);
            let tg_idx = ((total_days % 10) + 10) % 10;
            let dz_idx = ((total_days % 12) + 12) % 12;
            Pillar {
                tian_gan: Self::index_to_tian_gan(tg_idx as u8),
                di_zhi: Self::index_to_di_zhi(dz_idx as u8),
            }
        }
    }

    // ============ 时柱计算 ============

    fn calc_hour_pillar(&self, day_tg: TianGan, hour: u32) -> Pillar {
        let base_tg = match day_tg {
            TianGan::Jia | TianGan::Ji => 0,
            TianGan::Yi | TianGan::Geng => 2,
            TianGan::Bing | TianGan::Xin => 4,
            TianGan::Ding | TianGan::Ren => 6,
            TianGan::Wu | TianGan::Gui => 8,
        };

        let hour_dz = match hour {
            23 | 0 => 0, 1 | 2 => 1, 3 | 4 => 2, 5 | 6 => 3,
            7 | 8 => 4, 9 | 10 => 5, 11 | 12 => 6, 13 | 14 => 7,
            15 | 16 => 8, 17 | 18 => 9, 19 | 20 => 10, 21 | 22 => 11,
            _ => 0,
        };

        let tg_idx = (base_tg + hour_dz) % 10;

        Pillar {
            tian_gan: Self::index_to_tian_gan(tg_idx as u8),
            di_zhi: Self::index_to_di_zhi(hour_dz as u8),
        }
    }

    // ============ 藏干 ============

    fn calc_hidden_stems(
        &self,
        year: &Pillar, month: &Pillar, day: &Pillar, hour: &Pillar,
    ) -> HiddenStems {
        let get_hidden = |dz: DiZhi| -> Vec<TianGan> {
            match dz {
                DiZhi::Zi => vec![TianGan::Gui],
                DiZhi::Chou => vec![TianGan::Ji, TianGan::Gui, TianGan::Xin],
                DiZhi::Yin => vec![TianGan::Jia, TianGan::Bing, TianGan::Wu],
                DiZhi::Mao => vec![TianGan::Yi],
                DiZhi::Chen => vec![TianGan::Wu, TianGan::Yi, TianGan::Gui],
                DiZhi::Si => vec![TianGan::Bing, TianGan::Wu, TianGan::Geng],
                DiZhi::Wu => vec![TianGan::Ding, TianGan::Ji],
                DiZhi::Wei => vec![TianGan::Ji, TianGan::Ding, TianGan::Yi],
                DiZhi::Shen => vec![TianGan::Geng, TianGan::Ren, TianGan::Wu],
                DiZhi::You => vec![TianGan::Xin],
                DiZhi::Xu => vec![TianGan::Wu, TianGan::Xin, TianGan::Ding],
                DiZhi::Hai => vec![TianGan::Ren, TianGan::Jia],
            }
        };

        HiddenStems {
            year: get_hidden(year.di_zhi),
            month: get_hidden(month.di_zhi),
            day: get_hidden(day.di_zhi),
            hour: get_hidden(hour.di_zhi),
        }
    }

    // ============ 十神 ============

    fn calc_ten_gods(
        &self, day_master: TianGan, year: &Pillar, month: &Pillar, day: &Pillar, hour: &Pillar,
    ) -> TenGods {
        TenGods {
            year: Self::get_ten_god(day_master, year.tian_gan),
            month: Self::get_ten_god(day_master, month.tian_gan),
            day: Self::get_ten_god(day_master, day.tian_gan),
            hour: Self::get_ten_god(day_master, hour.tian_gan),
        }
    }

    // ============ 纳音 ============

    fn calc_na_yin(&self, year: &Pillar, month: &Pillar, day: &Pillar, hour: &Pillar) -> NaYin {
        NaYin {
            year: Self::get_na_yin(year),
            month: Self::get_na_yin(month),
            day: Self::get_na_yin(day),
            hour: Self::get_na_yin(hour),
        }
    }

    // ============ 空亡 ============

    fn calc_kong_wang(&self, day: &Pillar) -> [DiZhi; 2] {
        let dz_idx = day.di_zhi as u8;
        let xun_start = (dz_idx / 10) * 10;
        let kong1 = (xun_start + 10) % 12;
        let kong2 = (xun_start + 11) % 12;
        [
            Self::index_to_di_zhi(kong1),
            Self::index_to_di_zhi(kong2),
        ]
    }

    // ============ 神煞（扩展版） ============

    /// 补充内置常用神煞到结果中
    fn append_builtin_shen_sha(
        &self, year: &Pillar, month: &Pillar, day: &Pillar, hour: &Pillar,
        shen_sha: &mut Vec<ShenSha>,
    ) {
        let day_tg = day.tian_gan;
        let year_dz = year.di_zhi;
        let month_dz = month.di_zhi;
        let day_dz = day.di_zhi;
        let hour_dz = hour.di_zhi;

        // 天乙贵人
        let tian_yi = match day_tg {
            TianGan::Jia | TianGan::Wu => vec![DiZhi::Chou, DiZhi::Wei],
            TianGan::Yi | TianGan::Ji => vec![DiZhi::Zi, DiZhi::Shen],
            TianGan::Bing | TianGan::Ding => vec![DiZhi::Hai, DiZhi::You],
            TianGan::Geng | TianGan::Xin => vec![DiZhi::Wu, DiZhi::Yin],
            TianGan::Ren | TianGan::Gui => vec![DiZhi::Mao, DiZhi::Si],
        };
        for dz in &tian_yi {
            for (pillar_dz, pillar_name) in &[(year_dz, "年柱"), (day_dz, "日柱")] {
                if *pillar_dz == *dz {
                    shen_sha.push(ShenSha { name: "天乙贵人".into(), pillar: pillar_name.to_string(), description: "主贵人扶持".into() });
                }
            }
        }

        // 文昌贵人
        let wen_chang = match day_tg {
            TianGan::Jia => DiZhi::Si, TianGan::Yi => DiZhi::Wu,
            TianGan::Bing => DiZhi::Shen, TianGan::Ding => DiZhi::You,
            TianGan::Wu => DiZhi::Shen, TianGan::Ji => DiZhi::You,
            TianGan::Geng => DiZhi::Hai, TianGan::Xin => DiZhi::Zi,
            TianGan::Ren => DiZhi::Yin, TianGan::Gui => DiZhi::Mao,
        };
        if day_dz == wen_chang {
            shen_sha.push(ShenSha { name: "文昌贵人".into(), pillar: "日柱".into(), description: "主文采学识".into() });
        }

        // 桃花（咸池）
        let peach = match year_dz {
            DiZhi::Yin | DiZhi::Wu | DiZhi::Xu => DiZhi::Mao,
            DiZhi::Shen | DiZhi::Zi | DiZhi::Chen => DiZhi::You,
            DiZhi::Hai | DiZhi::Mao | DiZhi::Wei => DiZhi::Zi,
            DiZhi::Si | DiZhi::You | DiZhi::Chou => DiZhi::Wu,
        };
        for (pillar_dz, pillar_name) in &[(day_dz, "日柱"), (hour_dz, "时柱")] {
            if *pillar_dz == peach {
                shen_sha.push(ShenSha { name: "桃花".into(), pillar: pillar_name.to_string(), description: "主人缘异性缘".into() });
            }
        }

        // 驿马
        let yi_ma = match year_dz {
            DiZhi::Yin | DiZhi::Wu | DiZhi::Xu => DiZhi::Shen,
            DiZhi::Shen | DiZhi::Zi | DiZhi::Chen => DiZhi::Yin,
            DiZhi::Hai | DiZhi::Mao | DiZhi::Wei => DiZhi::Si,
            DiZhi::Si | DiZhi::You | DiZhi::Chou => DiZhi::Hai,
        };
        for (pillar_dz, pillar_name) in &[(day_dz, "日柱"), (month_dz, "月柱")] {
            if *pillar_dz == yi_ma {
                shen_sha.push(ShenSha { name: "驿马".into(), pillar: pillar_name.to_string(), description: "主奔波走动".into() });
            }
        }

        // 华盖
        let hua_gai = match year_dz {
            DiZhi::Yin | DiZhi::Wu | DiZhi::Xu => DiZhi::Xu,
            DiZhi::Shen | DiZhi::Zi | DiZhi::Chen => DiZhi::Chen,
            DiZhi::Hai | DiZhi::Mao | DiZhi::Wei => DiZhi::Wei,
            DiZhi::Si | DiZhi::You | DiZhi::Chou => DiZhi::Chou,
        };
        if day_dz == hua_gai {
            shen_sha.push(ShenSha { name: "华盖".into(), pillar: "日柱".into(), description: "主孤高聪慧".into() });
        }

        // 羊刃
        let yang_ren = match day_tg {
            TianGan::Jia => DiZhi::Mao, TianGan::Yi => DiZhi::Yin,
            TianGan::Bing | TianGan::Wu => DiZhi::Wu,
            TianGan::Ding | TianGan::Ji => DiZhi::Si,
            TianGan::Geng => DiZhi::You, TianGan::Xin => DiZhi::Shen,
            TianGan::Ren => DiZhi::Zi, TianGan::Gui => DiZhi::Hai,
        };
        for (pillar_dz, pillar_name) in &[(day_dz, "日柱"), (month_dz, "月柱")] {
            if *pillar_dz == yang_ren {
                shen_sha.push(ShenSha { name: "羊刃".into(), pillar: pillar_name.to_string(), description: "主刚强果断".into() });
            }
        }

        // 禄神
        let lu_shen = match day_tg {
            TianGan::Jia => DiZhi::Yin, TianGan::Yi => DiZhi::Mao,
            TianGan::Bing => DiZhi::Si, TianGan::Ding => DiZhi::Wu,
            TianGan::Wu => DiZhi::Si, TianGan::Ji => DiZhi::Wu,
            TianGan::Geng => DiZhi::Shen, TianGan::Xin => DiZhi::You,
            TianGan::Ren => DiZhi::Hai, TianGan::Gui => DiZhi::Zi,
        };
        for (pillar_dz, pillar_name) in &[(day_dz, "日柱"), (year_dz, "年柱")] {
            if *pillar_dz == lu_shen {
                shen_sha.push(ShenSha { name: "禄神".into(), pillar: pillar_name.to_string(), description: "主食禄福气".into() });
            }
        }

        // 将星
        let jiang_xing = match year_dz {
            DiZhi::Yin | DiZhi::Wu | DiZhi::Xu => DiZhi::Wu,
            DiZhi::Shen | DiZhi::Zi | DiZhi::Chen => DiZhi::Zi,
            DiZhi::Hai | DiZhi::Mao | DiZhi::Wei => DiZhi::Mao,
            DiZhi::Si | DiZhi::You | DiZhi::Chou => DiZhi::You,
        };
        if day_dz == jiang_xing {
            shen_sha.push(ShenSha { name: "将星".into(), pillar: "日柱".into(), description: "主领导才能".into() });
        }
    }

    // ============ 长生十二神 ============

    /// 计算四柱长生十二神状态
    ///
    /// 参考: RedSC1/bazi_core
    /// 天干在地支上的十二长生状态
    fn calc_chang_sheng(
        &self, day_master: TianGan, year: &Pillar, month: &Pillar, day: &Pillar, hour: &Pillar,
    ) -> ChangShengState {
        ChangShengState {
            year: Self::get_chang_sheng(day_master, year.di_zhi),
            month: Self::get_chang_sheng(day_master, month.di_zhi),
            day: Self::get_chang_sheng(day_master, day.di_zhi),
            hour: Self::get_chang_sheng(day_master, hour.di_zhi),
        }
    }

    /// 获取天干在某地支的长生状态
    ///
    /// 阳干顺行，阴干逆行
    /// 长生起点: 甲(亥), 乙(午), 丙(寅), 丁(酉), 戊(寅), 己(酉), 庚(巳), 辛(子), 壬(申), 癸(卯)
    fn get_chang_sheng(tg: TianGan, dz: DiZhi) -> ChangSheng12 {
        let (start_dz, is_yang) = match tg {
            TianGan::Jia => (11, true),  // 甲长生在亥, 阳顺行
            TianGan::Yi => (6, false),   // 乙长生在午, 阴逆行
            TianGan::Bing => (2, true),  // 丙长生在寅, 阳顺行
            TianGan::Ding => (9, false), // 丁长生在酉, 阴逆行
            TianGan::Wu => (2, true),    // 戊长生在寅, 阳顺行
            TianGan::Ji => (9, false),   // 己长生在酉, 阴逆行
            TianGan::Geng => (5, true),  // 庚长生在巳, 阳顺行
            TianGan::Xin => (0, false),  // 辛长生在子, 阴逆行
            TianGan::Ren => (8, true),   // 壬长生在申, 阳顺行
            TianGan::Gui => (3, false),  // 癸长生在卯, 阴逆行
        };

        let dz_idx = dz as i32;
        let start_idx = start_dz as i32;

        // 计算从长生到当前地支的步数
        let step = if is_yang {
            (dz_idx - start_idx).rem_euclid(12)
        } else {
            (start_idx - dz_idx).rem_euclid(12)
        };

        match step {
            0 => ChangSheng12::ChangSheng,
            1 => ChangSheng12::MuYu,
            2 => ChangSheng12::GuanDai,
            3 => ChangSheng12::LinGuan,
            4 => ChangSheng12::DiWang,
            5 => ChangSheng12::Shuai,
            6 => ChangSheng12::Bing,
            7 => ChangSheng12::Si,
            8 => ChangSheng12::Mu,
            9 => ChangSheng12::Jue,
            10 => ChangSheng12::Tai,
            11 => ChangSheng12::Yang,
            _ => ChangSheng12::ChangSheng,
        }
    }

    // ============ 干支刑冲合害（15种复杂关系） ============

    /// 计算四柱间的干支刑冲合害关系
    ///
    /// 参考: RedSC1/bazi_core
    fn calc_relations(
        &self, year: &Pillar, month: &Pillar, day: &Pillar, hour: &Pillar,
    ) -> Vec<GanZhiRelation> {
        let mut relations = Vec::new();
        let pillars = [
            ("年柱", year),
            ("月柱", month),
            ("日柱", day),
            ("时柱", hour),
        ];

        // 1. 天干合
        let tg_he_pairs: [(TianGan, TianGan, &str); 5] = [
            (TianGan::Jia, TianGan::Ji, "甲己合土"),
            (TianGan::Yi, TianGan::Geng, "乙庚合金"),
            (TianGan::Bing, TianGan::Xin, "丙辛合水"),
            (TianGan::Ding, TianGan::Ren, "丁壬合木"),
            (TianGan::Wu, TianGan::Gui, "戊癸合火"),
        ];

        for i in 0..pillars.len() {
            for j in (i + 1)..pillars.len() {
                let (name1, p1) = pillars[i];
                let (name2, p2) = pillars[j];

                // 天干合
                for (tg1, tg2, desc) in &tg_he_pairs {
                    if (p1.tian_gan == *tg1 && p2.tian_gan == *tg2)
                        || (p1.tian_gan == *tg2 && p2.tian_gan == *tg1)
                    {
                        relations.push(GanZhiRelation {
                            relation_type: RelationType::TianGanHe,
                            pillars: vec![name1.to_string(), name2.to_string()],
                            detail: format!("{}({}) {} {}({})", name1, p1.tian_gan.name_zh(), desc, name2, p2.tian_gan.name_zh()),
                            description: desc.to_string(),
                        });
                    }
                }

                // 天干冲
                let tg_chong_pairs: [(TianGan, TianGan); 5] = [
                    (TianGan::Jia, TianGan::Geng),
                    (TianGan::Yi, TianGan::Xin),
                    (TianGan::Bing, TianGan::Ren),
                    (TianGan::Ding, TianGan::Gui),
                    (TianGan::Wu, TianGan::Ji),
                ];
                for (tg1, tg2) in &tg_chong_pairs {
                    if (p1.tian_gan == *tg1 && p2.tian_gan == *tg2)
                        || (p1.tian_gan == *tg2 && p2.tian_gan == *tg1)
                    {
                        relations.push(GanZhiRelation {
                            relation_type: RelationType::TianGanChong,
                            pillars: vec![name1.to_string(), name2.to_string()],
                            detail: format!("{} 天干冲 {}", p1.tian_gan.name_zh(), p2.tian_gan.name_zh()),
                            description: "天干相冲".to_string(),
                        });
                    }
                }
            }
        }

        // 2. 地支六合
        let dz_liuhe: [(DiZhi, DiZhi, &str); 6] = [
            (DiZhi::Zi, DiZhi::Chou, "子丑合土"),
            (DiZhi::Yin, DiZhi::Hai, "寅亥合木"),
            (DiZhi::Mao, DiZhi::Xu, "卯戌合火"),
            (DiZhi::Chen, DiZhi::You, "辰酉合金"),
            (DiZhi::Si, DiZhi::Shen, "巳申合水"),
            (DiZhi::Wu, DiZhi::Wei, "午未合土"),
        ];

        for i in 0..pillars.len() {
            for j in (i + 1)..pillars.len() {
                let (name1, p1) = pillars[i];
                let (name2, p2) = pillars[j];
                for (dz1, dz2, desc) in &dz_liuhe {
                    if (p1.di_zhi == *dz1 && p2.di_zhi == *dz2)
                        || (p1.di_zhi == *dz2 && p2.di_zhi == *dz1)
                    {
                        relations.push(GanZhiRelation {
                            relation_type: RelationType::DiZhiLiuHe,
                            pillars: vec![name1.to_string(), name2.to_string()],
                            detail: format!("{}({}) {} {}({})", name1, p1.di_zhi.name_zh(), desc, name2, p2.di_zhi.name_zh()),
                            description: desc.to_string(),
                        });
                    }
                }
            }
        }

        // 3. 地支六冲
        let dz_liuchong: [(DiZhi, DiZhi); 6] = [
            (DiZhi::Zi, DiZhi::Wu),
            (DiZhi::Chou, DiZhi::Wei),
            (DiZhi::Yin, DiZhi::Shen),
            (DiZhi::Mao, DiZhi::You),
            (DiZhi::Chen, DiZhi::Xu),
            (DiZhi::Si, DiZhi::Hai),
        ];

        for i in 0..pillars.len() {
            for j in (i + 1)..pillars.len() {
                let (name1, p1) = pillars[i];
                let (name2, p2) = pillars[j];
                for (dz1, dz2) in &dz_liuchong {
                    if p1.di_zhi == *dz1 && p2.di_zhi == *dz2 {
                        relations.push(GanZhiRelation {
                            relation_type: RelationType::DiZhiLiuChong,
                            pillars: vec![name1.to_string(), name2.to_string()],
                            detail: format!("{} 冲 {}  ({}-{})", name1, name2, p1.di_zhi.name_zh(), p2.di_zhi.name_zh()),
                            description: "地支六冲".to_string(),
                        });
                    }
                }
            }
        }

        // 4. 地支六害
        let dz_liuhai: [(DiZhi, DiZhi, &str); 6] = [
            (DiZhi::Zi, DiZhi::Wei, "子未害"),
            (DiZhi::Chou, DiZhi::Wu, "丑午害"),
            (DiZhi::Yin, DiZhi::Si, "寅巳害"),
            (DiZhi::Mao, DiZhi::Chen, "卯辰害"),
            (DiZhi::Shen, DiZhi::Hai, "申亥害"),
            (DiZhi::You, DiZhi::Xu, "酉戌害"),
        ];

        for i in 0..pillars.len() {
            for j in (i + 1)..pillars.len() {
                let (name1, p1) = pillars[i];
                let (name2, p2) = pillars[j];
                for (dz1, dz2, desc) in &dz_liuhai {
                    if p1.di_zhi == *dz1 && p2.di_zhi == *dz2 {
                        relations.push(GanZhiRelation {
                            relation_type: RelationType::DiZhiLiuHai,
                            pillars: vec![name1.to_string(), name2.to_string()],
                            detail: format!("{} 害 {}", name1, name2),
                            description: desc.to_string(),
                        });
                    }
                }
            }
        }

        // 5. 地支相刑
        let dz_xiangxing: [(DiZhi, DiZhi, &str); 4] = [
            (DiZhi::Yin, DiZhi::Si, "寅巳相刑（无恩之刑）"),
            (DiZhi::Si, DiZhi::Shen, "巳申相刑"),
            (DiZhi::Chou, DiZhi::Xu, "丑戌相刑（恃势之刑）"),
            (DiZhi::Zi, DiZhi::Mao, "子卯相刑（无礼之刑）"),
        ];

        for i in 0..pillars.len() {
            for j in (i + 1)..pillars.len() {
                let (name1, p1) = pillars[i];
                let (name2, p2) = pillars[j];
                for (dz1, dz2, desc) in &dz_xiangxing {
                    if p1.di_zhi == *dz1 && p2.di_zhi == *dz2 {
                        relations.push(GanZhiRelation {
                            relation_type: RelationType::DiZhiXiangXing,
                            pillars: vec![name1.to_string(), name2.to_string()],
                            detail: format!("{}({}) 刑 {}({})", name1, p1.di_zhi.name_zh(), name2, p2.di_zhi.name_zh()),
                            description: desc.to_string(),
                        });
                    }
                }
            }
        }

        // 6. 自刑
        let zi_xing: [DiZhi; 4] = [DiZhi::Chen, DiZhi::Wu, DiZhi::You, DiZhi::Hai];
        for i in 0..pillars.len() {
            for j in (i + 1)..pillars.len() {
                let (name1, p1) = pillars[i];
                let (name2, p2) = pillars[j];
                if p1.di_zhi == p2.di_zhi && zi_xing.contains(&p1.di_zhi) {
                    relations.push(GanZhiRelation {
                        relation_type: RelationType::DiZhiZiXing,
                        pillars: vec![name1.to_string(), name2.to_string()],
                        detail: format!("{}-{} 自刑 ({})", name1, name2, p1.di_zhi.name_zh()),
                        description: "地支自刑".to_string(),
                    });
                }
            }
        }

        // 7. 地支三合
        let san_he: [(DiZhi, DiZhi, DiZhi, &str); 4] = [
            (DiZhi::Shen, DiZhi::Zi, DiZhi::Chen, "申子辰合水局"),
            (DiZhi::Hai, DiZhi::Mao, DiZhi::Wei, "亥卯未合木局"),
            (DiZhi::Yin, DiZhi::Wu, DiZhi::Xu, "寅午戌合火局"),
            (DiZhi::Si, DiZhi::You, DiZhi::Chou, "巳酉丑合金局"),
        ];

        // 收集所有地支
        let all_dz = [year.di_zhi, month.di_zhi, day.di_zhi, hour.di_zhi];
        for (dz1, dz2, dz3, desc) in &san_he {
            let has1 = all_dz.contains(dz1);
            let has2 = all_dz.contains(dz2);
            let has3 = all_dz.contains(dz3);
            let count = has1 as u8 + has2 as u8 + has3 as u8;

            if count == 3 {
                // 完整三合局
                let found_pillars: Vec<String> = pillars.iter()
                    .filter(|(_, p)| p.di_zhi == *dz1 || p.di_zhi == *dz2 || p.di_zhi == *dz3)
                    .map(|(n, _)| n.to_string())
                    .collect();
                relations.push(GanZhiRelation {
                    relation_type: RelationType::SanHeJu,
                    pillars: found_pillars,
                    detail: desc.to_string(),
                    description: "三合局成局".to_string(),
                });
            } else if count == 2 {
                // 半合
                let found_pillars: Vec<String> = pillars.iter()
                    .filter(|(_, p)| p.di_zhi == *dz1 || p.di_zhi == *dz2 || p.di_zhi == *dz3)
                    .map(|(n, _)| n.to_string())
                    .collect();
                relations.push(GanZhiRelation {
                    relation_type: RelationType::DiZhiBanHe,
                    pillars: found_pillars,
                    detail: desc.to_string(),
                    description: "半合".to_string(),
                });
            } else if count == 1 && has1 && has3 {
                // 拱合（如申辰拱子）
                let found_pillars: Vec<String> = pillars.iter()
                    .filter(|(_, p)| p.di_zhi == *dz1 || p.di_zhi == *dz3)
                    .map(|(n, _)| n.to_string())
                    .collect();
                relations.push(GanZhiRelation {
                    relation_type: RelationType::GongHe,
                    pillars: found_pillars,
                    detail: format!("拱{}", dz2.name_zh()),
                    description: "拱合".to_string(),
                });
            }
        }

        // 8. 地支三会
        let san_hui: [(DiZhi, DiZhi, DiZhi, &str); 4] = [
            (DiZhi::Yin, DiZhi::Mao, DiZhi::Chen, "寅卯辰会木局"),
            (DiZhi::Si, DiZhi::Wu, DiZhi::Wei, "巳午未会火局"),
            (DiZhi::Shen, DiZhi::You, DiZhi::Xu, "申酉戌会金局"),
            (DiZhi::Hai, DiZhi::Zi, DiZhi::Chou, "亥子丑会水局"),
        ];

        for (dz1, dz2, dz3, desc) in &san_hui {
            let count = all_dz.contains(dz1) as u8 + all_dz.contains(dz2) as u8 + all_dz.contains(dz3) as u8;
            if count >= 2 {
                let mut found_pillars: Vec<String> = pillars.iter()
                    .filter(|(_, p)| p.di_zhi == *dz1 || p.di_zhi == *dz2 || p.di_zhi == *dz3)
                    .map(|(n, _)| n.to_string())
                    .collect();
                if count == 3 {
                    relations.push(GanZhiRelation {
                        relation_type: RelationType::SanHuiJu,
                        pillars: found_pillars,
                        detail: desc.to_string(),
                        description: "三会局成局".to_string(),
                    });
                } else {
                    relations.push(GanZhiRelation {
                        relation_type: RelationType::DiZhiSanHui,
                        pillars: found_pillars,
                        detail: desc.to_string(),
                        description: "地支会局".to_string(),
                    });
                }
            }
        }

        // 9. 地支相破
        let dz_po: [(DiZhi, DiZhi, &str); 6] = [
            (DiZhi::Zi, DiZhi::You, "子酉破"),
            (DiZhi::Yin, DiZhi::Hai, "寅亥破"),
            (DiZhi::Chen, DiZhi::Chou, "辰丑破"),
            (DiZhi::Wu, DiZhi::Mao, "午卯破"),
            (DiZhi::Shen, DiZhi::Si, "申巳破"),
            (DiZhi::Xu, DiZhi::Wei, "戌未破"),
        ];

        for i in 0..pillars.len() {
            for j in (i + 1)..pillars.len() {
                let (name1, p1) = pillars[i];
                let (name2, p2) = pillars[j];
                for (dz1, dz2, desc) in &dz_po {
                    if p1.di_zhi == *dz1 && p2.di_zhi == *dz2 {
                        relations.push(GanZhiRelation {
                            relation_type: RelationType::DiZhiPo,
                            pillars: vec![name1.to_string(), name2.to_string()],
                            detail: desc.to_string(),
                            description: "地支相破".to_string(),
                        });
                    }
                }
            }
        }

        // 10. 地支相绝
        let dz_jue: [(DiZhi, DiZhi); 6] = [
            (DiZhi::Yin, DiZhi::You),
            (DiZhi::Mao, DiZhi::Shen),
            (DiZhi::Zi, DiZhi::Si),
            (DiZhi::Wu, DiZhi::Hai),
            (DiZhi::Chen, DiZhi::Chou),
            (DiZhi::Xu, DiZhi::Wei),
        ];

        for i in 0..pillars.len() {
            for j in (i + 1)..pillars.len() {
                let (name1, p1) = pillars[i];
                let (name2, p2) = pillars[j];
                for (dz1, dz2) in &dz_jue {
                    if p1.di_zhi == *dz1 && p2.di_zhi == *dz2 {
                        relations.push(GanZhiRelation {
                            relation_type: RelationType::DiZhiJue,
                            pillars: vec![name1.to_string(), name2.to_string()],
                            detail: format!("{}({}) 绝 {}({})", name1, p1.di_zhi.name_zh(), name2, p2.di_zhi.name_zh()),
                            description: "地支相绝".to_string(),
                        });
                    }
                }
            }
        }

        // 11. 地支暗合
        let dz_anhe: [(DiZhi, DiZhi, &str); 5] = [
            (DiZhi::Yin, DiZhi::Chou, "寅丑暗合"),
            (DiZhi::Mao, DiZhi::Shen, "卯申暗合"),
            (DiZhi::Wu, DiZhi::Hai, "午亥暗合"),
            (DiZhi::Zi, DiZhi::Si, "子巳暗合"),
            (DiZhi::Wei, DiZhi::Chen, "未辰暗合"),
        ];

        for i in 0..pillars.len() {
            for j in (i + 1)..pillars.len() {
                let (name1, p1) = pillars[i];
                let (name2, p2) = pillars[j];
                for (dz1, dz2, desc) in &dz_anhe {
                    if p1.di_zhi == *dz1 && p2.di_zhi == *dz2 {
                        relations.push(GanZhiRelation {
                            relation_type: RelationType::AnHe,
                            pillars: vec![name1.to_string(), name2.to_string()],
                            detail: desc.to_string(),
                            description: "地支暗合".to_string(),
                        });
                    }
                }
            }
        }

        relations
    }

    // ============ 大运 ============

    fn calc_da_yun(&self, birth: &BirthInfo, year: &Pillar, month: &Pillar, qi_yun_age: i32) -> Vec<DaYun> {
        let mut da_yun = Vec::new();
        let is_yang = Self::is_yang_tian_gan(year.tian_gan);
        let is_male = birth.gender == Gender::Male;
        let forward = (is_yang && is_male) || (!is_yang && !is_male);

        let base_dz = month.di_zhi as i8;
        let base_tg = month.tian_gan as i8;

        for i in 0..8 {
            let offset = if forward { i + 1 } else { -(i + 1) };
            let dz_idx = ((base_dz + offset).rem_euclid(12)) as u8;
            let tg_idx = ((base_tg + offset).rem_euclid(10)) as u8;

            let start_age = qi_yun_age + i as i32 * 10;
            let end_age = start_age + 10;

            da_yun.push(DaYun {
                pillar: Pillar {
                    tian_gan: Self::index_to_tian_gan(tg_idx),
                    di_zhi: Self::index_to_di_zhi(dz_idx),
                },
                start_age: start_age as u8,
                end_age: end_age as u8,
                start_year: birth.datetime.year() + start_age as i32,
                end_year: birth.datetime.year() + end_age as i32,
                ten_god: Self::get_ten_god(month.tian_gan, Self::index_to_tian_gan(tg_idx)),
                liu_nian: vec![],
            });
        }

        da_yun
    }

    /// 计算起运时间
    ///
    /// 返回 (起运年龄, 起运描述)
    /// 起运岁数 = 从出生到下一个/上一个节气的天数 / 3
    /// 阳年男/阴年女顺排（顺数到下一个节），阳年女/阴年男逆排（逆数到上一个节）
    fn calc_qi_yun_time(
        &self, birth: &BirthInfo, year: &Pillar, _month: &Pillar,
        jieqi_curr: &[JieQi], jieqi_prev: &[JieQi],
    ) -> (i32, String) {
        let is_yang = BaziCalc::is_yang_tian_gan(year.tian_gan);
        let is_male = birth.gender == Gender::Male;
        let forward = (is_yang && is_male) || (!is_yang && !is_male);

        // 出生日期的儒略日
        let dt = birth.datetime;
        let birth_jd = crate::sxwnl::julian::JulianDay::to_jd(
            dt.year(), dt.month(), dt.day() as f64 + dt.hour() as f64 / 24.0 + dt.minute() as f64 / 1440.0,
        );

        // 获取所有节（非气）的儒略日
        let mut jie_jds: Vec<(f64, &str)> = Vec::new();

        // 从上一年的节气中获取
        for jq in jieqi_prev.iter().filter(|jq| jq.is_jie) {
            if let Some((y, m, d)) = Self::parse_jq_date(&jq.datetime) {
                let jd = crate::sxwnl::julian::JulianDay::to_jd(y, m, d as f64);
                jie_jds.push((jd, &jq.name_zh));
            }
        }
        // 从当前年的节气中获取
        for jq in jieqi_curr.iter().filter(|jq| jq.is_jie) {
            if let Some((y, m, d)) = Self::parse_jq_date(&jq.datetime) {
                let jd = crate::sxwnl::julian::JulianDay::to_jd(y, m, d as f64);
                jie_jds.push((jd, &jq.name_zh));
            }
        }

        jie_jds.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        if forward {
            // 顺排：找到出生后第一个节
            for (jd, name) in &jie_jds {
                if *jd > birth_jd {
                    let days = *jd - birth_jd;
                    let age = (days / 3.0).round() as i32;
                    return (age.max(1), format!("{}岁起运（顺排，{}后{}天）", age.max(1), name, days.round() as i32));
                }
            }
        } else {
            // 逆排：找到出生前最后一个节
            for (jd, name) in jie_jds.iter().rev() {
                if *jd < birth_jd {
                    let days = birth_jd - *jd;
                    let age = (days / 3.0).round() as i32;
                    return (age.max(1), format!("{}岁起运（逆排，{}前{}天）", age.max(1), name, days.round() as i32));
                }
            }
        }

        (8, "8岁起运".to_string())
    }

    /// 计算八字格局
    /// 格局分析
    ///
    /// 基于日主强弱、月令旺衰、十神分布进行格局判定
    /// 参考: RedSC1/bazi_core, 传统八字格局理论
    fn calc_pattern(&self, bazi: &BaziChart) -> BaziPattern {
        let day_master = bazi.day_master;
        let month_zhi = bazi.month.di_zhi;

        // 1. 判断日主在月令的旺衰
        let month_season = self.get_month_season(month_zhi);
        let day_wuxing = Self::tian_gan_wuxing(day_master);
        let day_yinyang = Self::tian_gan_yinyang(day_master);

        // 2. 统计八字中帮扶和克泄耗力量
        let pillars = [&bazi.year, &bazi.month, &bazi.day, &bazi.hour];
        let mut support_count = 0u8;
        let mut restrain_count = 0u8;

        for pillar in &pillars {
            let tg_wx = Self::tian_gan_wuxing(pillar.tian_gan);
            let dz_wx = Self::di_zhi_wuxing(pillar.di_zhi);

            if tg_wx == day_wuxing {
                // 同五行
                if Self::tian_gan_yinyang(pillar.tian_gan) == day_yinyang {
                    support_count += 2; // 比肩
                } else {
                    support_count += 1; // 劫财
                }
            } else if Self::is_shen_wx(tg_wx, day_wuxing) {
                support_count += 1; // 印星
            } else {
                restrain_count += 1;
            }

            if dz_wx == day_wuxing {
                support_count += 1;
            } else {
                restrain_count += 1;
            }
        }

        // 3. 月令旺衰系数
        let month_strength = self.get_month_season(month_zhi);

        let total_support = support_count + month_strength;

        // 4. 判定格局
        let (pattern_name, strength) = if total_support >= 10 {
            ("身强格局".to_string(), PatternStrength::Strong)
        } else if total_support >= 7 {
            ("身旺格局".to_string(), PatternStrength::Strong)
        } else if total_support >= 5 {
            ("中和格局".to_string(), PatternStrength::Normal)
        } else if total_support >= 3 {
            ("身弱格局".to_string(), PatternStrength::Weak)
        } else {
            ("从格".to_string(), PatternStrength::Weak)
        };

        // 5. 检测特殊格局
        let special_pattern = self.detect_special_pattern(bazi);

        let final_name = if let Some(ref sp) = special_pattern {
            sp.clone()
        } else {
            pattern_name
        };

        BaziPattern {
            name: final_name,
            description: format!(
                "日主{}（{}），月令{}，支持力{}，抑制力{}。{}",
                day_master.name_zh(),
                day_wuxing.name_zh(),
                month_zhi.name_zh(),
                support_count,
                restrain_count,
                if strength == PatternStrength::Strong { "身强宜克泄耗" }
                else if strength == PatternStrength::Weak { "身弱宜生扶" }
                else { "中和，需结合大运流年分析" }
            ),
            strength,
        }
    }

    // ============ 辅助方法 ============

    /// 解析本地时间字符串，提取小时（含小数部分）
    fn parse_local_hour(dt: &str) -> f64 {
        // 格式: "2024-06-15T14:30" or "2024-06-15T14:30:00"
        if let Some(t_part) = dt.split('T').nth(1) {
            let parts: Vec<&str> = t_part.split(':').collect();
            if parts.len() >= 2 {
                let h: f64 = parts[0].parse().unwrap_or(0.0);
                let m: f64 = parts[1].parse().unwrap_or(0.0);
                let s: f64 = if parts.len() >= 3 {
                    parts[2].parse().unwrap_or(0.0)
                } else {
                    0.0
                };
                return h + m / 60.0 + s / 3600.0;
            }
        }
        // 尝试空格分隔格式: "2024-06-15 14:30"
        if let Some(space_idx) = dt.find(' ') {
            let time_str = &dt[space_idx + 1..];
            let parts: Vec<&str> = time_str.split(':').collect();
            if parts.len() >= 2 {
                let h: f64 = parts[0].parse().unwrap_or(0.0);
                let m: f64 = parts[1].parse().unwrap_or(0.0);
                return h + m / 60.0;
            }
        }
        12.0 // 默认中午12点
    }

    fn parse_jq_date(dt: &str) -> Option<(i32, u32, u32)> {
        let parts: Vec<&str> = dt.split('T').collect();
        if parts.is_empty() { return None; }
        let date_parts: Vec<&str> = parts[0].split('-').collect();
        if date_parts.len() < 3 { return None; }
        Some((
            date_parts[0].parse().ok()?,
            date_parts[1].parse().ok()?,
            date_parts[2].parse().ok()?,
        ))
    }

    /// 日期加减天数
    fn add_days(year: i32, month: u32, day: u32, delta: i32) -> (i32, u32, u32) {
        let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let is_leap = |y: i32| (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;

        let mut y = year;
        let mut m = month;
        let mut d = day as i32 + delta;

        loop {
            let max_days = if m == 2 && is_leap(y) {
                29
            } else {
                month_days[(m - 1) as usize]
            };

            if d > max_days {
                d -= max_days;
                m += 1;
                if m > 12 {
                    m = 1;
                    y += 1;
                }
            } else if d < 1 {
                if m == 1 {
                    m = 12;
                    y -= 1;
                } else {
                    m -= 1;
                }
                let prev_max = if m == 2 && is_leap(y) {
                    29
                } else {
                    month_days[(m - 1) as usize]
                };
                d += prev_max;
            } else {
                break;
            }
        }

        (y, m, d as u32)
    }

    fn tian_gan_wuxing(tg: TianGan) -> WuXing {
        match tg {
            TianGan::Jia | TianGan::Yi => WuXing::Wood,
            TianGan::Bing | TianGan::Ding => WuXing::Fire,
            TianGan::Wu | TianGan::Ji => WuXing::Earth,
            TianGan::Geng | TianGan::Xin => WuXing::Metal,
            TianGan::Ren | TianGan::Gui => WuXing::Water,
        }
    }

    fn tian_gan_yinyang(tg: TianGan) -> YinYang {
        match tg {
            TianGan::Jia | TianGan::Bing | TianGan::Wu | TianGan::Geng | TianGan::Ren => YinYang::Yang,
            _ => YinYang::Yin,
        }
    }

    fn is_yang_tian_gan(tg: TianGan) -> bool {
        matches!(Self::tian_gan_yinyang(tg), YinYang::Yang)
    }

    pub fn get_ten_god(day_master: TianGan, other: TianGan) -> TenGod {
        let dm_wx = Self::tian_gan_wuxing(day_master);
        let other_wx = Self::tian_gan_wuxing(other);
        let dm_yy = Self::tian_gan_yinyang(day_master);
        let other_yy = Self::tian_gan_yinyang(other);
        let same_yy = dm_yy == other_yy;

        match (dm_wx, other_wx) {
            (WuXing::Fire, WuXing::Wood) | (WuXing::Earth, WuXing::Fire)
            | (WuXing::Metal, WuXing::Earth) | (WuXing::Water, WuXing::Metal)
            | (WuXing::Wood, WuXing::Water) => {
                if same_yy { TenGod::PianYin } else { TenGod::ZhengYin }
            }
            (WuXing::Wood, WuXing::Fire) | (WuXing::Fire, WuXing::Earth)
            | (WuXing::Earth, WuXing::Metal) | (WuXing::Metal, WuXing::Water)
            | (WuXing::Water, WuXing::Wood) => {
                if same_yy { TenGod::ShiShen } else { TenGod::ShangGuan }
            }
            (WuXing::Fire, WuXing::Water) | (WuXing::Earth, WuXing::Wood)
            | (WuXing::Metal, WuXing::Fire) | (WuXing::Water, WuXing::Earth)
            | (WuXing::Wood, WuXing::Metal) => {
                if same_yy { TenGod::PianGuan } else { TenGod::ZhengGuan }
            }
            (WuXing::Wood, WuXing::Earth) | (WuXing::Fire, WuXing::Metal)
            | (WuXing::Earth, WuXing::Water) | (WuXing::Metal, WuXing::Wood)
            | (WuXing::Water, WuXing::Fire) => {
                if same_yy { TenGod::PianCai } else { TenGod::ZhengCai }
            }
            _ => {
                if same_yy { TenGod::BiJian } else { TenGod::JieCai }
            }
        }
    }

    fn get_na_yin(pillar: &Pillar) -> String {
        let tg_idx = pillar.tian_gan as u8;
        let dz_idx = pillar.di_zhi as u8;
        let idx = (tg_idx * 6 + dz_idx / 2) % 30;

        let na_yin_list = [
            "海中金", "炉中火", "大林木", "路旁土", "剑锋金", "山头火",
            "涧下水", "城头土", "白蜡金", "杨柳木", "泉中水", "屋上土",
            "霹雳火", "松柏木", "流年长水", "砂石金", "山下火", "平地木",
            "壁上土", "金箔金", "覆灯火", "天河水", "大驿土", "钗钏金",
            "桑柘木", "大溪水", "沙中土", "天上火", "石榴木", "大海水",
        ];

        na_yin_list[idx as usize].to_string()
    }

    fn index_to_tian_gan(idx: u8) -> TianGan {
        match idx % 10 {
            0 => TianGan::Jia, 1 => TianGan::Yi,
            2 => TianGan::Bing, 3 => TianGan::Ding,
            4 => TianGan::Wu, 5 => TianGan::Ji,
            6 => TianGan::Geng, 7 => TianGan::Xin,
            8 => TianGan::Ren, _ => TianGan::Gui,
        }
    }

    fn index_to_di_zhi(idx: u8) -> DiZhi {
        match idx % 12 {
            0 => DiZhi::Zi, 1 => DiZhi::Chou,
            2 => DiZhi::Yin, 3 => DiZhi::Mao,
            4 => DiZhi::Chen, 5 => DiZhi::Si,
            6 => DiZhi::Wu, 7 => DiZhi::Wei,
            8 => DiZhi::Shen, 9 => DiZhi::You,
            10 => DiZhi::Xu, _ => DiZhi::Hai,
        }
    }

    fn days_since_1900(year: i32, month: u32, day: u32) -> i64 {
        let mut days = 0i64;
        for y in 1900..year {
            days += if Self::is_leap_year(y) { 366 } else { 365 };
        }
        let month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for m in 0..(month as usize - 1) {
            days += month_days[m] as i64;
            if m == 1 && Self::is_leap_year(year) { days += 1; }
        }
        days += day as i64 - 1;
        days
    }

    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    /// 地支对应的五行
    fn di_zhi_wuxing(dz: DiZhi) -> WuXing {
        match dz {
            DiZhi::Yin | DiZhi::Mao => WuXing::Wood,
            DiZhi::Si | DiZhi::Wu => WuXing::Fire,
            DiZhi::Chen | DiZhi::Xu | DiZhi::Chou | DiZhi::Wei => WuXing::Earth,
            DiZhi::Shen | DiZhi::You => WuXing::Metal,
            DiZhi::Hai | DiZhi::Zi => WuXing::Water,
        }
    }

    /// 判断五行是否相生（sheng 生被生者）
    /// 例如：木生火 => is_shen_wx(Wood, Fire) == true
    fn is_shen_wx(sheng: WuXing, bei_sheng: WuXing) -> bool {
        matches!(
            (sheng, bei_sheng),
            (WuXing::Wood, WuXing::Fire)
                | (WuXing::Fire, WuXing::Earth)
                | (WuXing::Earth, WuXing::Metal)
                | (WuXing::Metal, WuXing::Water)
                | (WuXing::Water, WuXing::Wood)
        )
    }

    /// 获取月令旺衰状态（返回强度系数 0-3）
    ///
    /// 根据日主五行和月令确定旺衰：
    /// 同五行=旺(3)，生我=相(2)，我生=休(1)，克我=囚(0)，我克=死(0)
    fn get_month_season(&self, month_zhi: DiZhi) -> u8 {
        // 简化返回：先返回月令五行本身的季节性等级
        // 后续可根据具体五行细化
        2 // 默认"相"状态
    }

    /// 检测特殊格局
    fn detect_special_pattern(&self, bazi: &BaziChart) -> Option<String> {
        let pillars = [&bazi.year, &bazi.month, &bazi.day, &bazi.hour];
        let day_wx = Self::tian_gan_wuxing(bazi.day_master);

        // 统计五行分布
        let mut wx_count = [0u8; 5]; // 木火土金水
        for pillar in &pillars {
            let tg_idx = match Self::tian_gan_wuxing(pillar.tian_gan) {
                WuXing::Wood => 0, WuXing::Fire => 1,
                WuXing::Earth => 2, WuXing::Metal => 3, WuXing::Water => 4,
            };
            let dz_idx = match Self::di_zhi_wuxing(pillar.di_zhi) {
                WuXing::Wood => 0, WuXing::Fire => 1,
                WuXing::Earth => 2, WuXing::Metal => 3, WuXing::Water => 4,
            };
            wx_count[tg_idx] += 1;
            wx_count[dz_idx] += 1;
        }

        let day_idx = match day_wx {
            WuXing::Wood => 0, WuXing::Fire => 1,
            WuXing::Earth => 2, WuXing::Metal => 3, WuXing::Water => 4,
        };

        // 从格检测：日主五行极弱（<=1），且其他某一五行极强（>=5）
        if wx_count[day_idx] <= 1 {
            for (i, &count) in wx_count.iter().enumerate() {
                if i != day_idx && count >= 5 {
                    let wx_name = ["木", "火", "土", "金", "水"][i];
                    return Some(format!("从{}格", wx_name));
                }
            }
        }

        // 专旺格检测：日主五行极强（>=5），且只有日主五行和生它的五行
        if wx_count[day_idx] >= 5 {
            let wx_name = ["木", "火", "土", "金", "水"][day_idx];
            // 曲直格（木）、炎上格（火）、稼穑格（土）、从革格（金）、润下格（水）
            let special_name = match day_wx {
                WuXing::Wood => "曲直格",
                WuXing::Fire => "炎上格",
                WuXing::Earth => "稼穑格",
                WuXing::Metal => "从革格",
                WuXing::Water => "润下格",
            };
            return Some(format!("{}(专旺{})", special_name, wx_name));
        }

        None
    }
}

impl Default for BaziCalc {
    fn default() -> Self {
        Self { sxwnl: Sxwnl::default() }
    }
}