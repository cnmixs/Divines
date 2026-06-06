// 星阙 Horosa - 神煞查找模块
// 参考原项目: astrostudysrv/astrostudy/helper

use std::collections::HashMap;
use horosa_core::*;

use super::data;

/// 神煞查找结果：(神煞名称, 所在柱位, 吉凶)
pub type GodResult = (String, String, String);

/// 神煞查找器
pub struct GodFinder;

impl GodFinder {
    /// 根据四柱查找所有匹配的神煞
    pub fn find_gods(four_pillars: &[Pillar]) -> Vec<GodResult> {
        let mut results = Vec::new();
        let rules = data::get_gods();

        // 确保有四个柱：年、月、日、时
        if four_pillars.len() < 4 {
            return results;
        }

        let year = &four_pillars[0];
        let month = &four_pillars[1];
        let day = &four_pillars[2];
        let hour = &four_pillars[3];

        // 纳音五行
        let year_nayin = data::get_nayin(&format!("{}{}", year.tian_gan.name_zh(), year.di_zhi.name_zh())).unwrap_or("");
        let month_nayin = data::get_nayin(&format!("{}{}", month.tian_gan.name_zh(), month.di_zhi.name_zh())).unwrap_or("");
        let day_nayin = data::get_nayin(&format!("{}{}", day.tian_gan.name_zh(), day.di_zhi.name_zh())).unwrap_or("");
        let hour_nayin = data::get_nayin(&format!("{}{}", hour.tian_gan.name_zh(), hour.di_zhi.name_zh())).unwrap_or("");

        let nayins = [year_nayin, month_nayin, day_nayin, hour_nayin];

        for rule in &rules {
            let matched = Self::match_rule(
                rule,
                year, month, day, hour,
                &nayins,
            );
            results.extend(matched);
        }

        results
    }

    fn match_rule(
        rule: &data::GodRule,
        year: &Pillar, month: &Pillar, day: &Pillar, hour: &Pillar,
        nayins: &[&str; 4],
    ) -> Vec<GodResult> {
        let mut results = Vec::new();

        // 确定 key pillar(s)
        let key_pillars = Self::get_key_pillars(rule, year, month, day, hour);

        // 确定 value pillar(s)
        let value_pillars = Self::get_value_pillars(rule, year, month, day, hour);

        match (rule.keyType.as_str(), rule.valueType.as_str()) {
            // 干 -> 支
            ("干", "支") => {
                for (key_pillar_name, key_pillar) in &key_pillars {
                    let key_gan = key_pillar.tian_gan.name_zh();
                    if let Some(values) = rule.rule.get(key_gan) {
                        if let Some(arr) = values.as_array() {
                            for v in arr {
                                if let Some(v_str) = v.as_str() {
                                    for (vp_name, vp) in &value_pillars {
                                        if vp.di_zhi.name_zh() == v_str {
                                            results.push((
                                                rule.name.clone(),
                                                vp_name.to_string(),
                                                rule.jixiong.clone(),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 支 -> 支
            ("支", "支") => {
                for (key_pillar_name, key_pillar) in &key_pillars {
                    let key_zhi = key_pillar.di_zhi.name_zh();
                    if let Some(values) = rule.rule.get(key_zhi) {
                        if let Some(arr) = values.as_array() {
                            for v in arr {
                                if let Some(v_str) = v.as_str() {
                                    for (vp_name, vp) in &value_pillars {
                                        if vp.di_zhi.name_zh() == v_str {
                                            results.push((
                                                rule.name.clone(),
                                                vp_name.to_string(),
                                                rule.jixiong.clone(),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 纳音 -> 长生
            ("纳音", "长生") => {
                let nayin_wuxing = Self::extract_nayin_wuxing;
                for (key_pillar_name, _key_pillar) in &key_pillars {
                    let nayin_idx = Self::pillar_name_to_idx(key_pillar_name);
                    let nayin_str = nayins[nayin_idx];
                    let wx = nayin_wuxing(nayin_str);
                    if let Some(values) = rule.rule.get(wx) {
                        if let Some(arr) = values.as_array() {
                            for v in arr {
                                if let Some(phase) = v.as_str() {
                                    for (vp_name, vp) in &value_pillars {
                                        let vp_gan = vp.tian_gan.name_zh();
                                        let vp_zhi = vp.di_zhi.name_zh();
                                        if let Some(actual_phase) = data::get_ganzhi_phase(vp_gan, vp_zhi) {
                                            if actual_phase == phase {
                                                results.push((
                                                    rule.name.clone(),
                                                    vp_name.to_string(),
                                                    rule.jixiong.clone(),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 干 -> 长生
            ("干", "长生") => {
                for (key_pillar_name, key_pillar) in &key_pillars {
                    let key_gan = key_pillar.tian_gan.name_zh();
                    if let Some(values) = rule.rule.get(key_gan) {
                        if let Some(arr) = values.as_array() {
                            for v in arr {
                                if let Some(phase) = v.as_str() {
                                    for (vp_name, vp) in &value_pillars {
                                        let vp_gan = vp.tian_gan.name_zh();
                                        let vp_zhi = vp.di_zhi.name_zh();
                                        if let Some(actual_phase) = data::get_ganzhi_phase(vp_gan, vp_zhi) {
                                            if actual_phase == phase {
                                                results.push((
                                                    rule.name.clone(),
                                                    vp_name.to_string(),
                                                    rule.jixiong.clone(),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 甲子 -> 甲子
            ("甲子", "甲子") => {
                for (key_pillar_name, key_pillar) in &key_pillars {
                    let key_ganzhi = format!("{}{}", key_pillar.tian_gan.name_zh(), key_pillar.di_zhi.name_zh());
                    if let Some(values) = rule.rule.get(key_pillar_name.as_str()) {
                        if let Some(arr) = values.as_array() {
                            for v in arr {
                                if let Some(v_str) = v.as_str() {
                                    if v_str == key_ganzhi {
                                        for (vp_name, vp) in &value_pillars {
                                            let vp_ganzhi = format!("{}{}", vp.tian_gan.name_zh(), vp.di_zhi.name_zh());
                                            if let Some(v_values) = rule.rule.get(vp_name.as_str()) {
                                                if let Some(v_arr) = v_values.as_array() {
                                                    if v_arr.iter().any(|v| v.as_str() == Some(&vp_ganzhi[..])) {
                                                        results.push((
                                                            rule.name.clone(),
                                                            vp_name.to_string(),
                                                            rule.jixiong.clone(),
                                                        ));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 支 -> 干支 (天德等)
            ("支", "干支") => {
                for (key_pillar_name, key_pillar) in &key_pillars {
                    let key_zhi = key_pillar.di_zhi.name_zh();
                    if let Some(values) = rule.rule.get(key_zhi) {
                        if let Some(arr) = values.as_array() {
                            for v in arr {
                                if let Some(v_str) = v.as_str() {
                                    for (vp_name, vp) in &value_pillars {
                                        // 天德的值可能是天干或地支
                                        if vp.tian_gan.name_zh() == v_str || vp.di_zhi.name_zh() == v_str {
                                            results.push((
                                                rule.name.clone(),
                                                vp_name.to_string(),
                                                rule.jixiong.clone(),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 支 -> 干 (月德等)
            ("支", "干") => {
                for (key_pillar_name, key_pillar) in &key_pillars {
                    let key_zhi = key_pillar.di_zhi.name_zh();
                    if let Some(values) = rule.rule.get(key_zhi) {
                        if let Some(arr) = values.as_array() {
                            for v in arr {
                                if let Some(v_str) = v.as_str() {
                                    for (vp_name, vp) in &value_pillars {
                                        if vp.tian_gan.name_zh() == v_str {
                                            results.push((
                                                rule.name.clone(),
                                                vp_name.to_string(),
                                                rule.jixiong.clone(),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            _ => {}
        }

        results
    }

    /// 获取 key pillar(s)
    fn get_key_pillars<'a>(
        rule: &data::GodRule,
        year: &'a Pillar, month: &'a Pillar, day: &'a Pillar, hour: &'a Pillar,
    ) -> Vec<(String, &'a Pillar)> {
        if rule.keyZhu.is_empty() {
            // 空表示所有四柱
            vec![
                ("年柱".into(), year),
                ("月柱".into(), month),
                ("日柱".into(), day),
                ("时柱".into(), hour),
            ]
        } else {
            rule.keyZhu.iter().map(|zhu| {
                match zhu.as_str() {
                    "年" => ("年柱".into(), year),
                    "月" => ("月柱".into(), month),
                    "日" => ("日柱".into(), day),
                    "时" => ("时柱".into(), hour),
                    _ => ("日柱".into(), day),
                }
            }).collect()
        }
    }

    /// 获取 value pillar(s)
    fn get_value_pillars<'a>(
        rule: &data::GodRule,
        year: &'a Pillar, month: &'a Pillar, day: &'a Pillar, hour: &'a Pillar,
    ) -> Vec<(String, &'a Pillar)> {
        if rule.valueZhu.is_empty() {
            // 空表示所有四柱
            vec![
                ("年柱".into(), year),
                ("月柱".into(), month),
                ("日柱".into(), day),
                ("时柱".into(), hour),
            ]
        } else {
            rule.valueZhu.iter().map(|zhu| {
                match zhu.as_str() {
                    "年" => ("年柱".into(), year),
                    "月" => ("月柱".into(), month),
                    "日" => ("日柱".into(), day),
                    "时" => ("时柱".into(), hour),
                    _ => ("日柱".into(), day),
                }
            }).collect()
        }
    }

    fn pillar_name_to_idx(name: &str) -> usize {
        match name {
            "年柱" => 0,
            "月柱" => 1,
            "日柱" => 2,
            "时柱" => 3,
            "年" => 0,
            "月" => 1,
            "日" => 2,
            "时" => 3,
            _ => 0,
        }
    }

    fn extract_nayin_wuxing(nayin: &str) -> &str {
        // 纳音字符串中提取五行
        if nayin.contains("金") { "金" }
        else if nayin.contains("木") { "木" }
        else if nayin.contains("水") { "水" }
        else if nayin.contains("火") { "火" }
        else if nayin.contains("土") { "土" }
        else { "土" }
    }
}

// 重新导出简化函数
pub fn find_gods(four_pillars: &[Pillar]) -> Vec<GodResult> {
    GodFinder::find_gods(four_pillars)
}