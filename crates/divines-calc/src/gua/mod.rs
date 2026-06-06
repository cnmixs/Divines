// Divines - 卦象系统计算引擎 (梅花易数 / 六爻)
// 参考原项目: astrostudysrv/astrostudycn/helper/GuaHelper.java

use divines_core::liuyao::*;
use divines_core::bazi::{TianGan, DiZhi};
use std::collections::HashMap;

pub mod data;

/// 卦象计算器
pub struct GuaCalc {
    /// 六十四卦 (按名称、简称、二进制名索引)
    gua_64: HashMap<String, LiuShiSiGua>,
    /// 梅花易数八卦 (按名称、简称、二进制名索引)
    mei_yi_gua: HashMap<String, MeiYiGua>,
    /// 干支卦映射
    gan_zhi_gua: HashMap<String, String>,
    /// 四象
    si_xiang: Vec<SiXiang>,
}

impl GuaCalc {
    pub fn new() -> Self {
        Self {
            gua_64: data::load_64_gua(),
            mei_yi_gua: data::load_meiyi_gua(),
            gan_zhi_gua: data::load_ganzhi_gua(),
            si_xiang: data::load_sixiang(),
        }
    }

    /// 按名称获取六十四卦
    /// 支持: 卦名("乾"), 简称("天天"), 全名("乾乾"), 二进制("111111")
    pub fn get_gua_by_name(&self, name: &str) -> Option<&LiuShiSiGua> {
        self.gua_64.get(name)
    }

    /// 按序号获取六十四卦 (0-63)
    pub fn get_gua_by_index(&self, idx: usize) -> Option<&LiuShiSiGua> {
        if idx < 64 {
            let binary = format!("{:06b}", idx);
            // 小端序: 最低位=初爻, 需要反转
            let fname: String = binary.chars().rev().collect();
            self.gua_64.get(&fname)
        } else {
            None
        }
    }

    /// 由上下卦获取六十四卦
    pub fn get_gua_by_trigram(&self, upper: BaGua, lower: BaGua) -> Option<&LiuShiSiGua> {
        let lower_yao = lower.yao();
        let upper_yao = upper.yao();
        let fname = format!(
            "{}{}{}{}{}{}",
            lower_yao[0], lower_yao[1], lower_yao[2],
            upper_yao[0], upper_yao[1], upper_yao[2]
        );
        self.gua_64.get(&fname)
    }

    /// 计算梅花易数卦 (从数字)
    /// 参考: GuaHelper.java
    pub fn get_meihua_gua(&self, numbers: &[u32]) -> Option<MeiHuaYiChart> {
        if numbers.is_empty() {
            return None;
        }

        // 上卦: 取第一个数除以8的余数 (1=乾, 2=兑, ..., 8=坤, 0=坤)
        let upper_num = numbers[0] % 8;
        let upper_gua = BaGua::from_xian_tian_number(if upper_num == 0 { 8 } else { upper_num as u8 })?;

        // 下卦: 取第二个数除以8的余数 (如果没有第二个数则用第一个数)
        let lower_num = if numbers.len() >= 2 { numbers[1] % 8 } else { numbers[0] % 8 };
        let lower_gua = BaGua::from_xian_tian_number(if lower_num == 0 { 8 } else { lower_num as u8 })?;

        // 动爻: 取第三个数除以6的余数 (1-6, 0=6)
        let changing_yao = if numbers.len() >= 3 {
            let yao = numbers[2] % 6;
            Some(if yao == 0 { 6 } else { yao as u8 })
        } else {
            None
        };

        // 本卦
        let ben_gua = self.get_gua_by_trigram(upper_gua, lower_gua)?.clone();

        // 互卦
        let (hu_gua, hu_upper, hu_lower) = self.calc_hu_gua(upper_gua, lower_gua);

        // 变卦
        let bian_gua = changing_yao.map(|cy| {
            let mut yao = ben_gua.yao;
            let idx = (cy - 1) as usize;
            yao[idx] = if yao[idx] == 0 { 1 } else { 0 };
            self.yao_to_gua(&yao)
        }).flatten();

        // 体卦/用卦: 动爻在上卦则为用卦(上), 体卦(下); 动爻在下卦则为用卦(下), 体卦(上)
        let (ti_gua, yong_gua) = if let Some(cy) = changing_yao {
            if cy <= 3 {
                (upper_gua, lower_gua) // 动爻在下卦, 下卦为用, 上卦为体
            } else {
                (lower_gua, upper_gua) // 动爻在上卦, 上卦为用, 下卦为体
            }
        } else {
            (lower_gua, upper_gua)
        };

        Some(MeiHuaYiChart {
            upper_gua,
            lower_gua,
            ben_gua,
            changing_yao,
            hu_gua,
            hu_upper,
            hu_lower,
            bian_gua: bian_gua.cloned(),
            ti_gua,
            yong_gua,
        })
    }

    /// 计算互卦
    /// 互卦上卦: 下卦2爻 + 上卦0爻 + 上卦1爻
    /// 互卦下卦: 下卦1爻 + 下卦2爻 + 上卦0爻
    fn calc_hu_gua(&self, upper: BaGua, lower: BaGua) -> (Option<LiuShiSiGua>, Option<BaGua>, Option<BaGua>) {
        let l = lower.yao();
        let u = upper.yao();

        let hu_upper_yao = [l[2], u[0], u[1]];
        let hu_lower_yao = [l[1], l[2], u[0]];

        let hu_upper = BaGua::from_yao_array(&hu_upper_yao);
        let hu_lower = BaGua::from_yao_array(&hu_lower_yao);

        let hu_gua = match (hu_upper, hu_lower) {
            (Some(up), Some(down)) => self.get_gua_by_trigram(up, down).cloned(),
            _ => None,
        };

        (hu_gua, hu_upper, hu_lower)
    }

    /// 获取互卦 (从64卦)
    pub fn get_mutual_gua(&self, gua: &LiuShiSiGua) -> Option<LiuShiSiGua> {
        let yao = gua.yao;
        // 上卦: yao[3], yao[4], yao[5]
        let upper = BaGua::from_yao_array(&[yao[3], yao[4], yao[5]]);
        // 下卦: yao[0], yao[1], yao[2]
        let lower = BaGua::from_yao_array(&[yao[0], yao[1], yao[2]]);

        if let (Some(up), Some(down)) = (upper, lower) {
            let (hu_gua, _, _) = self.calc_hu_gua(up, down);
            hu_gua
        } else {
            None
        }
    }

    /// 获取错卦 (阴阳爻全部相反)
    pub fn get_cuo_gua(&self, gua: &LiuShiSiGua) -> Option<&LiuShiSiGua> {
        let inverted: Vec<u8> = gua.yao.iter().map(|&y| if y == 0 { 1 } else { 0 }).collect();
        let mut arr = [0u8; 6];
        arr.copy_from_slice(&inverted);
        self.yao_to_gua(&arr)
    }

    /// 获取综卦 (上下颠倒)
    pub fn get_zong_gua(&self, gua: &LiuShiSiGua) -> Option<&LiuShiSiGua> {
        let mut reversed = gua.yao;
        reversed.reverse();
        self.yao_to_gua(&reversed)
    }

    /// 由6爻构造卦
    pub fn yao_to_gua(&self, six_yao: &[u8; 6]) -> Option<&LiuShiSiGua> {
        let fname = format!(
            "{}{}{}{}{}{}",
            six_yao[0], six_yao[1], six_yao[2],
            six_yao[3], six_yao[4], six_yao[5]
        );
        self.gua_64.get(&fname)
    }

    /// 获取干支卦 (用于八字)
    /// 参考: GuaHelper.getMeiyiGanZiGua
    pub fn get_ganzhi_gua(&self, gan: TianGan, zhi: DiZhi) -> Option<&MeiYiGua> {
        // 先查完整的干支组合, 如 "戊寅"
        let gan_str = gan.name_zh();
        let zhi_str = zhi.name_zh();
        let gan_zhi = format!("{}{}", gan_str, zhi_str);

        if let Some(gua_name) = self.gan_zhi_gua.get(&gan_zhi) {
            return self.mei_yi_gua.get(gua_name);
        }

        // 再查单独的天干
        if let Some(gua_name) = self.gan_zhi_gua.get(gan_str) {
            return self.mei_yi_gua.get(gua_name);
        }

        // 最后查单独的地支
        if let Some(gua_name) = self.gan_zhi_gua.get(zhi_str) {
            return self.mei_yi_gua.get(gua_name);
        }

        None
    }

    /// 获取梅花易数八卦
    pub fn get_meiyi_gua_by_name(&self, name: &str) -> Option<&MeiYiGua> {
        self.mei_yi_gua.get(name)
    }

    /// 获取四象
    pub fn get_sixiang(&self) -> &[SiXiang] {
        &self.si_xiang
    }

    /// 列出所有六十四卦
    pub fn list_all_gua(&self) -> Vec<&LiuShiSiGua> {
        let mut gua_list: Vec<&LiuShiSiGua> = Vec::new();
        for i in 0..64 {
            let binary = format!("{:06b}", i);
            let fname: String = binary.chars().rev().collect();
            if let Some(gua) = self.gua_64.get(&fname) {
                gua_list.push(gua);
            }
        }
        gua_list
    }
}

impl Default for GuaCalc {
    fn default() -> Self {
        Self::new()
    }
}