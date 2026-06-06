// Divines - 调候用神模块
// 参考原项目: astrostudysrv/astrostudy/helper

use divines_core::*;

use super::data;

/// 调候用神辅助器
pub struct TiaoHou;

impl TiaoHou {
    /// 获取调候用神
    /// day_gan: 日干
    /// month_zhi: 月支
    /// 返回调候用神的天干列表
    pub fn get_tiaohou(day_gan: TianGan, month_zhi: DiZhi) -> Vec<String> {
        let gan_str = day_gan.name_zh();
        let zhi_str = month_zhi.name_zh();
        data::get_tiaohou(gan_str, zhi_str).cloned().unwrap_or_default()
    }

    /// 获取调候用神（字符串版本）
    pub fn get_tiaohou_str(day_gan: &str, month_zhi: &str) -> Vec<String> {
        data::get_tiaohou(day_gan, month_zhi).cloned().unwrap_or_default()
    }
}