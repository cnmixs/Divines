// 星阙 Horosa - 季节旺衰辅助模块
// 参考原项目: astrostudysrv/astrostudy/helper

use std::collections::HashMap;
use horosa_core::*;

use super::data;

/// 季节旺衰辅助器
pub struct SeasonHelper;

impl SeasonHelper {
    /// 获取五行在某一地支的旺衰状态
    /// 返回: 王/相/休/囚/死
    pub fn get_state(wuxing: &str, zhi: &str) -> &'static str {
        data::get_season_state(wuxing, zhi).unwrap_or("休")
    }

    /// 获取某地支上所有五行的旺衰状态
    pub fn get_full_state(zhi: &str) -> HashMap<String, String> {
        let wuxing_list = ["木", "火", "土", "金", "水"];
        let mut result = HashMap::new();
        for wx in &wuxing_list {
            let state = Self::get_state(wx, zhi);
            result.insert(wx.to_string(), state.to_string());
        }
        result
    }

    /// 使用 WuXing 枚举获取状态
    pub fn get_state_by_wuxing(wx: WuXing, zhi: DiZhi) -> &'static str {
        let wx_str = match wx {
            WuXing::Wood => "木",
            WuXing::Fire => "火",
            WuXing::Earth => "土",
            WuXing::Metal => "金",
            WuXing::Water => "水",
        };
        Self::get_state(wx_str, zhi.name_zh())
    }

    /// 使用 DiZhi 枚举获取完整状态
    pub fn get_full_state_by_dizhi(zhi: DiZhi) -> HashMap<String, String> {
        Self::get_full_state(zhi.name_zh())
    }
}