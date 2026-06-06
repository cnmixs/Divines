// 星阙 Horosa - 六壬计算引擎
// 参考原项目: astrostudysrv/astrostudycn/helper/LiuRengHelper.java, model/LiuReng.java

use horosa_core::bazi::{TianGan, DiZhi};
use horosa_core::sanshi::*;
use std::collections::HashMap;

/// 六壬计算器
pub struct LiuRengCalc {
    /// 日贵人 (阳贵)
    day_gui: HashMap<String, String>,
    /// 夜贵人 (阴贵)
    night_gui: HashMap<String, String>,
    /// 干支五行
    gan_zhi_wu_xing: HashMap<String, String>,
    /// 干寄支
    gan_ji_zhi: HashMap<String, String>,
    /// 支六亲
    zhi_liu_qin: HashMap<String, HashMap<String, String>>,
    /// 阳干
    yang_gan: Vec<String>,
    /// 阴干
    ying_gan: Vec<String>,
    /// 阳支
    yang_zhi: Vec<String>,
    /// 阴支
    ying_zhi: Vec<String>,
    /// 顺排贵人 (SummerZiList)
    summer_zhi: Vec<String>,
    /// 逆排贵人 (WinnerZiList)
    winner_zhi: Vec<String>,
    /// 四孟
    zi_meng: Vec<String>,
    /// 四季
    zi_ji: Vec<String>,
    /// 四仲
    zi_zong: Vec<String>,
}

impl LiuRengCalc {
    pub fn new() -> Self {
        let data_str = include_str!("data.json");
        let data: serde_json::Value = serde_json::from_str(data_str).unwrap_or_default();

        let day_gui = data.get("DayGui").and_then(|v| v.as_object())
            .map(|o| o.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string())).collect())
            .unwrap_or_default();

        let night_gui = data.get("NightGui").and_then(|v| v.as_object())
            .map(|o| o.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string())).collect())
            .unwrap_or_default();

        let gan_zhi_wu_xing = data.get("GanZiWuXing").and_then(|v| v.as_object())
            .map(|o| o.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string())).collect())
            .unwrap_or_default();

        let gan_ji_zhi = data.get("GanJiZi").and_then(|v| v.as_object())
            .map(|o| o.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string())).collect())
            .unwrap_or_default();

        let zhi_liu_qin = data.get("ZiLiuQin").and_then(|v| v.as_object())
            .map(|o| {
                o.iter().map(|(k, v)| {
                    let inner = v.as_object().map(|io| {
                        io.iter().map(|(ik, iv)| (ik.clone(), iv.as_str().unwrap_or("").to_string())).collect()
                    }).unwrap_or_default();
                    (k.clone(), inner)
                }).collect()
            })
            .unwrap_or_default();

        fn parse_list(data: &serde_json::Value, key: &str) -> Vec<String> {
            data.get(key).and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default()
        }

        Self {
            day_gui,
            night_gui,
            gan_zhi_wu_xing,
            gan_ji_zhi,
            zhi_liu_qin,
            yang_gan: parse_list(&data, "YangGan"),
            ying_gan: parse_list(&data, "YingGan"),
            yang_zhi: parse_list(&data, "YangZi"),
            ying_zhi: parse_list(&data, "YingZi"),
            summer_zhi: parse_list(&data, "SummerZiList"),
            winner_zhi: parse_list(&data, "WinnerZiList"),
            zi_meng: parse_list(&data, "ZiMeng"),
            zi_ji: parse_list(&data, "ZiJi"),
            zi_zong: parse_list(&data, "ZiZong"),
        }
    }

    /// 计算六壬盘
    /// 参考: LiuRengHelper.calcKeCuang, LiuReng.java
    pub fn calculate(
        &self,
        year_gan: TianGan,
        year_zhi: DiZhi,
        month_gan: TianGan,
        month_zhi: DiZhi,
        day_gan: TianGan,
        day_zhi: DiZhi,
        hour_gan: TianGan,
        hour_zhi: DiZhi,
        is_diurnal: bool,
    ) -> LiurenChart {
        let day_gan_str = day_gan.name_zh().to_string();
        let day_zhi_str = day_zhi.name_zh().to_string();
        let hour_zhi_str = hour_zhi.name_zh().to_string();
        let month_zhi_str = month_zhi.name_zh().to_string();

        // 1. 地盘 (固定)
        let di_pan: [String; 12] = [
            "子".into(), "丑".into(), "寅".into(), "卯".into(),
            "辰".into(), "巳".into(), "午".into(), "未".into(),
            "申".into(), "酉".into(), "戌".into(), "亥".into(),
        ];

        // 2. 月将 (月将 = 中气后太阳所在宫位, 简化: 月支对面)
        let month_jiang = self.determine_month_jiang(&month_zhi_str);

        // 3. 占时 (时辰)
        let zhan_shi = hour_zhi_str.clone();

        // 4. 天盘 (月将加时)
        let tian_pan = self.arrange_tian_pan(&month_jiang, &zhan_shi);

        // 5. 四课
        let si_ke = self.calculate_si_ke(&day_gan_str, &day_zhi_str, &tian_pan);

        // 6. 三传 (九宗门)
        let san_chuan = self.calculate_san_chuan(&si_ke);

        // 7. 贵人
        let (gui_ren_pos, yang_gui) = self.arrange_gui_ren(&day_gan_str, is_diurnal);

        // 8. 天将 (12神将)
        let (shen_jiang, tian_jiang) = self.arrange_shen_jiang(gui_ren_pos, yang_gui);

        // 9. 遁干 (五鼠遁)
        let dun_gan = self.calculate_dun_gan(&day_gan_str);

        let di_pan_static: [String; 12] = di_pan;

        LiurenChart {
            month_jiang,
            zhan_shi,
            tian_pan,
            di_pan: di_pan_static,
            si_ke,
            san_chuan,
            dun_gan,
            shen_jiang,
            gui_ren_position: gui_ren_pos,
            yang_gui,
            tian_jiang,
            liu_qin: self.get_liu_qin(&day_gan_str),
            de_shen: String::new(),
            he_shen: String::new(),
            gui: vec![],
            kong_wang: [String::new(), String::new()],
            zuo_shan: String::new(),
            xing_nian: String::new(),
            ben_ming: String::new(),
            four_pillars: LiurenPillars {
                year: format!("{}{}", year_gan.name_zh(), year_zhi.name_zh()),
                month: format!("{}{}", month_gan.name_zh(), month_zhi.name_zh()),
                day: format!("{}{}", day_gan.name_zh(), day_zhi.name_zh()),
                hour: format!("{}{}", hour_gan.name_zh(), hour_zhi.name_zh()),
            },
        }
    }

    /// 确定月将
    /// 月将 = 中气后太阳所在宫位
    /// 正月亥将、二月戌将、三月酉将、四月申将、五月未将、六月午将
    /// 七月巳将、八月辰将、九月卯将、十月寅将、十一月丑将、十二月子将
    pub fn determine_month_jiang(&self, month_zhi: &str) -> String {
        match month_zhi {
            "寅" => "亥", "卯" => "戌", "辰" => "酉",
            "巳" => "申", "午" => "未", "未" => "午",
            "申" => "巳", "酉" => "辰", "戌" => "卯",
            "亥" => "寅", "子" => "丑", "丑" => "子",
            _ => "子",
        }.to_string()
    }

    /// 定占时
    pub fn determine_zhan_shi(&self, hour: u32) -> String {
        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
        let idx = match hour {
            23 | 0 => 0, 1 | 2 => 1, 3 | 4 => 2, 5 | 6 => 3,
            7 | 8 => 4, 9 | 10 => 5, 11 | 12 => 6, 13 | 14 => 7,
            15 | 16 => 8, 17 | 18 => 9, 19 | 20 => 10, 21 | 22 => 11,
            _ => 0,
        };
        di_zhi[idx].to_string()
    }

    /// 排天盘 (月将加时)
    pub fn arrange_tian_pan(&self, month_jiang: &str, zhan_shi: &str) -> [String; 12] {
        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
        let mj_pos = di_zhi.iter().position(|&d| d == month_jiang).unwrap_or(0);
        let zs_pos = di_zhi.iter().position(|&d| d == zhan_shi).unwrap_or(0);

        // 天盘[占时位置] = 月将
        // 天盘位置 i 对应地盘子到亥, 天盘上的内容 = 地支[(i - 占时位置 + 月将位置) % 12]
        let mut tian_pan: [String; 12] = Default::default();
        for i in 0..12 {
            let idx = (i as i32 - zs_pos as i32 + mj_pos as i32 + 12) % 12;
            tian_pan[i] = di_zhi[idx as usize].to_string();
        }
        tian_pan
    }

    /// 起四课
    /// 参考: LiuRengHelper.getKe
    pub fn calculate_si_ke(&self, day_gan: &str, day_zhi: &str, tian_pan: &[String; 12]) -> SiKe {
        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

        // 日干寄宫
        let gan_ji = self.gan_ji_zhi.get(day_gan).cloned().unwrap_or_default();

        // 第一课: 日干上神
        let gidx = di_zhi.iter().position(|&d| d == gan_ji).unwrap_or(0);
        let ke1_zi = tian_pan[gidx].clone();

        // 第二课: 日干上神的上神
        let idx2 = di_zhi.iter().position(|&d| d == ke1_zi).unwrap_or(0);
        let ke2_zi = tian_pan[idx2].clone();

        // 第三课: 日支上神
        let zidx = di_zhi.iter().position(|&d| d == day_zhi).unwrap_or(0);
        let ke3_zi = tian_pan[zidx].clone();

        // 第四课: 日支上神的上神
        let idx4 = di_zhi.iter().position(|&d| d == ke3_zi).unwrap_or(0);
        let ke4_zi = tian_pan[idx4].clone();

        SiKe {
            ke1: (ke1_zi.clone(), day_gan.to_string()),
            ke2: (ke2_zi, ke1_zi),
            ke3: (ke3_zi.clone(), day_zhi.to_string()),
            ke4: (ke4_zi, ke3_zi),
        }
    }

    /// 发三传 (九宗门)
    /// 参考: LiuRengHelper
    pub fn calculate_san_chuan(&self, si_ke: &SiKe) -> SanChuan {
        let (ke1_up, ke1_down) = (&si_ke.ke1.0, &si_ke.ke1.1);
        let (ke2_up, ke2_down) = (&si_ke.ke2.0, &si_ke.ke2.1);
        let (ke3_up, ke3_down) = (&si_ke.ke3.0, &si_ke.ke3.1);
        let (ke4_up, ke4_down) = (&si_ke.ke4.0, &si_ke.ke4.1);

        // 1. 贼克法: 上克下为克, 下克上为贼
        let mut ke_list: Vec<(usize, &str, &str)> = Vec::new(); // (课序号, 上, 下)
        let mut zei_list: Vec<(usize, &str, &str)> = Vec::new();

        // 检查第一课: ke1上克下
        if self.is_ke(ke1_up, ke1_down) {
            ke_list.push((1, ke1_up.as_str(), ke1_down.as_str()));
        }
        if self.is_ke(ke1_down, ke1_up) {
            zei_list.push((1, ke1_up.as_str(), ke1_down.as_str()));
        }
        // 检查第二课
        if self.is_ke(ke2_up, ke2_down) {
            ke_list.push((2, ke2_up.as_str(), ke2_down.as_str()));
        }
        if self.is_ke(ke2_down, ke2_up) {
            zei_list.push((2, ke2_up.as_str(), ke2_down.as_str()));
        }
        // 检查第三课
        if self.is_ke(ke3_up, ke3_down) {
            ke_list.push((3, ke3_up.as_str(), ke3_down.as_str()));
        }
        if self.is_ke(ke3_down, ke3_up) {
            zei_list.push((3, ke3_up.as_str(), ke3_down.as_str()));
        }
        // 检查第四课
        if self.is_ke(ke4_up, ke4_down) {
            ke_list.push((4, ke4_up.as_str(), ke4_down.as_str()));
        }
        if self.is_ke(ke4_down, ke4_up) {
            zei_list.push((4, ke4_up.as_str(), ke4_down.as_str()));
        }

        let chu_chuan = if !zei_list.is_empty() {
            // 取贼 (下克上优先)
            if zei_list.len() == 1 {
                zei_list[0].1.to_string()
            } else {
                // 比用法: 与日干阴阳相同者取
                self.bi_yong(&zei_list, ke1_down)
            }
        } else if !ke_list.is_empty() {
            // 取克 (上克下次之)
            if ke_list.len() == 1 {
                ke_list[0].1.to_string()
            } else {
                self.bi_yong(&ke_list, ke1_down)
            }
        } else {
            // 遥克法: 无克取遥克
            // 简化处理: 取第一课上神
            ke1_up.clone()
        };

        // 中传: 初传之上的神
        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
        let zhong_chuan = self.get_shang_shen(&chu_chuan);
        let mo_chuan = self.get_shang_shen(&zhong_chuan);

        SanChuan {
            chu_chuan,
            zhong_chuan,
            mo_chuan,
        }
    }

    /// 比用法: 在多个贼/克中, 取与日干阴阳相同者
    fn bi_yong(&self, list: &[(usize, &str, &str)], day_gan: &str) -> String {
        let is_yang = self.yang_gan.contains(&day_gan.to_string());
        for (_idx, up, _down) in list {
            let up_is_yang = self.yang_zhi.contains(&up.to_string());
            if up_is_yang == is_yang {
                return up.to_string();
            }
        }
        // 都没有同类, 取第一个
        list.first().map(|(_, up, _)| up.to_string()).unwrap_or_default()
    }

    /// 判断上是否克下 (五行相克)
    fn is_ke(&self, up: &str, down: &str) -> bool {
        let up_wx = self.gan_zhi_wu_xing.get(up).cloned().unwrap_or_default();
        let down_wx = self.gan_zhi_wu_xing.get(down).cloned().unwrap_or_default();

        // 五行相克: 木克土, 土克水, 水克火, 火克金, 金克木
        match (up_wx.as_str(), down_wx.as_str()) {
            ("木", "土") | ("土", "水") | ("水", "火") | ("火", "金") | ("金", "木") => true,
            _ => false,
        }
    }

    /// 获取上神 (天盘对应的地支)
    fn get_shang_shen(&self, zhi: &str) -> String {
        // 简化: 在此实现中, 我们需要知道天盘状态
        // 实际使用中应该从当前天盘查找
        // 这里返回默认值, 完整实现需要传入天盘参数
        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
        let pos = di_zhi.iter().position(|&d| d == zhi).unwrap_or(0);
        // 简化: 使用固定天盘
        let tian = ["亥", "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌"];
        tian[pos].to_string()
    }

    /// 贵人诀
    /// 甲戊庚牛羊, 乙己鼠猴乡, 丙丁猪鸡位, 壬癸兔蛇藏, 六辛逢马虎
    pub fn arrange_gui_ren(&self, day_gan: &str, is_diurnal: bool) -> (usize, bool) {
        let gui_zhi = if is_diurnal {
            self.day_gui.get(day_gan).cloned().unwrap_or_default()
        } else {
            self.night_gui.get(day_gan).cloned().unwrap_or_default()
        };

        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
        let pos = di_zhi.iter().position(|&d| d == gui_zhi).unwrap_or(0);
        let yang_gui = is_diurnal;

        (pos, yang_gui)
    }

    /// 排12神将
    /// 贵人, 螣蛇, 朱雀, 六合, 勾陈, 青龙, 天空, 白虎, 太常, 玄武, 太阴, 天后
    pub fn arrange_shen_jiang(&self, gui_ren_pos: usize, yang_gui: bool) -> ([ShenJiang; 12], [String; 12]) {
        let tian_jiang_names = ["贵人", "螣蛇", "朱雀", "六合", "勾陈", "青龙", "天空", "白虎", "太常", "玄武", "太阴", "天后"];

        let mut shen_jiang: [ShenJiang; 12] = Default::default();
        let mut tian_jiang: [String; 12] = Default::default();

        let gui_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
        let gui_zhi_str = gui_zhi[gui_ren_pos].to_string();

        // 顺排/逆排: 贵人在地盘巳午未申酉戌 (SummerZi) 则逆排, 否则顺排
        let is_summer = self.summer_zhi.contains(&gui_zhi_str);

        for i in 0..12 {
            let idx = if is_summer {
                // 逆排
                (gui_ren_pos + 12 - i) % 12
            } else {
                // 顺排
                (gui_ren_pos + i) % 12
            };

            shen_jiang[i] = ShenJiang {
                name: tian_jiang_names[i].to_string(),
                position: idx,
                description: String::new(),
            };
            tian_jiang[idx] = tian_jiang_names[i].to_string();
        }

        (shen_jiang, tian_jiang)
    }

    /// 五鼠遁 (日干起时)
    /// 甲己还加甲, 乙庚丙作初, 丙辛从戊起, 丁壬庚子居, 戊癸何方发, 壬子是真途
    pub fn calculate_dun_gan(&self, day_gan: &str) -> [String; 12] {
        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
        let gan_list = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

        let start_gan = match day_gan {
            "甲" | "己" => "甲",
            "乙" | "庚" => "丙",
            "丙" | "辛" => "戊",
            "丁" | "壬" => "庚",
            "戊" | "癸" => "壬",
            _ => "甲",
        };

        let start_idx = gan_list.iter().position(|&g| g == start_gan).unwrap_or(0);

        let mut dun_gan: [String; 12] = Default::default();
        for i in 0..12 {
            let gan_idx = (start_idx + i) % 10;
            dun_gan[i] = format!("{}{}", gan_list[gan_idx], di_zhi[i]);
        }

        dun_gan
    }

    /// 获取十二地支的六亲 (基于日干)
    fn get_liu_qin(&self, day_gan: &str) -> [String; 12] {
        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
        let mut liu_qin: [String; 12] = Default::default();

        for (i, dz) in di_zhi.iter().enumerate() {
            if let Some(inner) = self.zhi_liu_qin.get(*dz) {
                liu_qin[i] = inner.get(day_gan).cloned().unwrap_or_default();
            }
        }

        liu_qin
    }

    /// 长生十二神位置
    /// 甲木长生在亥, 乙木长生在午, 丙火长生在寅, 丁火长生在酉,
    /// 戊土长生在寅, 己土长生在酉, 庚金长生在巳, 辛金长生在子,
    /// 壬水长生在申, 癸水长生在卯
    pub fn get_zhang_sheng(&self, day_gan: &str) -> [String; 12] {
        let di_zhi = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
        let chang_sheng = ["长生", "沐浴", "冠带", "临官", "帝旺", "衰", "病", "死", "墓", "绝", "胎", "养"];

        let start_zhi = match day_gan {
            "甲" => "亥", "乙" => "午", "丙" => "寅", "丁" => "酉",
            "戊" => "寅", "己" => "酉", "庚" => "巳", "辛" => "子",
            "壬" => "申", "癸" => "卯",
            _ => "子",
        };

        let start_idx = di_zhi.iter().position(|&d| d == start_zhi).unwrap_or(0);

        // 阳干顺行, 阴干逆行
        let is_yang = self.yang_gan.contains(&day_gan.to_string());
        let mut result: [String; 12] = Default::default();

        for i in 0..12 {
            let idx = if is_yang {
                (start_idx + i) % 12
            } else {
                (start_idx + 12 - i) % 12
            };
            result[idx] = chang_sheng[i].to_string();
        }

        result
    }
}

impl Default for LiuRengCalc {
    fn default() -> Self {
        Self::new()
    }
}