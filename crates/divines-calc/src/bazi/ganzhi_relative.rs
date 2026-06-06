// Divines - 干支关系辅助模块
// 天干合/冲/禄，地支合/冲/刑/穿/破/会

use divines_core::*;

use super::data;

/// 干支关系辅助器
pub struct GanZhiRelative;

impl GanZhiRelative {
    /// 天干合：返回 (合干, 五行)
    pub fn gan_he(tg: TianGan) -> Option<(TianGan, WuXing)> {
        let gan_str = tg.name_zh();
        data::get_gan_he(gan_str).map(|(he_gan, wx)| {
            (parse_tian_gan(he_gan), parse_wuxing(wx))
        })
    }

    /// 天干冲：返回 (冲干, 描述)
    pub fn gan_chong(tg: TianGan) -> Option<(TianGan, &'static str)> {
        let gan_str = tg.name_zh();
        data::get_gan_chong(gan_str).map(|(chong_gan, desc)| {
            (parse_tian_gan(chong_gan), desc)
        })
    }

    /// 天干禄：返回禄位地支
    pub fn gan_lu(tg: TianGan) -> Option<DiZhi> {
        let gan_str = tg.name_zh();
        data::get_gan_lu(gan_str).map(parse_di_zhi)
    }

    /// 地支六合：返回 (合支, 五行)
    pub fn zi_he6(dz: DiZhi) -> Option<(DiZhi, WuXing)> {
        let zhi_str = dz.name_zh();
        data::get_zi_he6(zhi_str).map(|(he_zhi, wx)| {
            (parse_di_zhi(he_zhi), parse_wuxing(wx))
        })
    }

    /// 地支三合：返回 (合支列表, 五行)
    pub fn zi_he3(dz: DiZhi) -> (Vec<DiZhi>, WuXing) {
        let zhi_str = dz.name_zh();
        let (zhi_list, wx) = data::get_zi_he3(zhi_str);
        let zhis: Vec<DiZhi> = zhi_list.iter().map(|s| parse_di_zhi(s)).collect();
        (zhis, parse_wuxing(wx))
    }

    /// 地支三会：返回 (会支列表, 五行, 方位, 神兽)
    pub fn zi_hui(dz: DiZhi) -> (Vec<DiZhi>, WuXing, &'static str, &'static str) {
        let zhi_str = dz.name_zh();
        let (zhi_list, wx, direction, animal) = data::get_zi_hui(zhi_str);
        let zhis: Vec<DiZhi> = zhi_list.iter().map(|s| parse_di_zhi(s)).collect();
        (zhis, parse_wuxing(wx), direction, animal)
    }

    /// 地支六冲：返回 (冲支, 描述)
    pub fn zi_chong(dz: DiZhi) -> Option<(DiZhi, &'static str)> {
        let zhi_str = dz.name_zh();
        data::get_zi_chong(zhi_str).map(|(chong_zhi, desc)| {
            (parse_di_zhi(chong_zhi), desc)
        })
    }

    /// 地支相刑：返回刑支列表
    pub fn zi_xing(dz: DiZhi) -> Vec<DiZhi> {
        let zhi_str = dz.name_zh();
        data::get_zi_xing(zhi_str).iter().map(|s| parse_di_zhi(s)).collect()
    }

    /// 地支相穿（害）：返回穿支
    pub fn zi_chuan(dz: DiZhi) -> Option<DiZhi> {
        let zhi_str = dz.name_zh();
        data::get_zi_chuan(zhi_str).map(parse_di_zhi)
    }

    /// 地支相破：返回破支
    pub fn zi_po(dz: DiZhi) -> Option<DiZhi> {
        let zhi_str = dz.name_zh();
        data::get_zi_po(zhi_str).map(parse_di_zhi)
    }
}

// ============ 辅助解析函数 ============

fn parse_tian_gan(s: &str) -> TianGan {
    match s {
        "甲" => TianGan::Jia, "乙" => TianGan::Yi,
        "丙" => TianGan::Bing, "丁" => TianGan::Ding,
        "戊" => TianGan::Wu, "己" => TianGan::Ji,
        "庚" => TianGan::Geng, "辛" => TianGan::Xin,
        "壬" => TianGan::Ren, "癸" => TianGan::Gui,
        _ => TianGan::Jia,
    }
}

fn parse_di_zhi(s: &str) -> DiZhi {
    match s {
        "子" => DiZhi::Zi, "丑" => DiZhi::Chou,
        "寅" => DiZhi::Yin, "卯" => DiZhi::Mao,
        "辰" => DiZhi::Chen, "巳" => DiZhi::Si,
        "午" => DiZhi::Wu, "未" => DiZhi::Wei,
        "申" => DiZhi::Shen, "酉" => DiZhi::You,
        "戌" => DiZhi::Xu, "亥" => DiZhi::Hai,
        _ => DiZhi::Zi,
    }
}

fn parse_wuxing(s: &str) -> WuXing {
    match s {
        "木" => WuXing::Wood, "火" => WuXing::Fire,
        "土" => WuXing::Earth, "金" => WuXing::Metal,
        "水" => WuXing::Water,
        _ => WuXing::Earth,
    }
}