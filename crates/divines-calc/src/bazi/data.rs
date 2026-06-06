// Divines - 八字数据加载模块
// 使用 include_str! 嵌入 JSON 数据文件，通过 LazyLock 提供懒加载静态数据

use std::collections::HashMap;
use std::sync::LazyLock;
use serde::{Deserialize, Serialize};

// ============ 嵌入 JSON 数据文件 ============

const NAYIN_JSON: &str = include_str!("data/nayin.json");
const SEASON_JSON: &str = include_str!("data/season.json");
const TIAOHOU_JSON: &str = include_str!("data/tiaohou.json");
const GANZHI_RELATIVE_JSON: &str = include_str!("data/ganzhi_relative.json");
const WUXING_PHASE_JSON: &str = include_str!("data/wuxing_phase.json");
const GODS_JSON: &str = include_str!("data/gods.json");
const GONG12_JSON: &str = include_str!("data/gong12.json");
const BAZIPITHY_JSON: &str = include_str!("data/bazipithy.json");
const BAZICOMPOSE_JSON: &str = include_str!("data/bazicompose.json");

// ============ 懒加载静态数据 ============

static NAYIN_DATA: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_str(NAYIN_JSON).unwrap_or_default()
});

static SEASON_DATA: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    serde_json::from_str(SEASON_JSON).unwrap_or_default()
});

static TIAOHOU_DATA: LazyLock<HashMap<String, HashMap<String, Vec<String>>>> = LazyLock::new(|| {
    serde_json::from_str(TIAOHOU_JSON).unwrap_or_default()
});

static WUXING_PHASE_DATA: LazyLock<serde_json::Value> = LazyLock::new(|| {
    serde_json::from_str(WUXING_PHASE_JSON).unwrap_or_default()
});

static GONG12_DATA: LazyLock<serde_json::Value> = LazyLock::new(|| {
    serde_json::from_str(GONG12_JSON).unwrap_or_default()
});

static BAZIPITHY_DATA: LazyLock<serde_json::Value> = LazyLock::new(|| {
    serde_json::from_str(BAZIPITHY_JSON).unwrap_or_default()
});

static BAZICOMPOSE_DATA: LazyLock<serde_json::Value> = LazyLock::new(|| {
    serde_json::from_str(BAZICOMPOSE_JSON).unwrap_or_default()
});

// ============ 干支关系数据 ============

#[derive(Debug, Clone, Deserialize)]
struct GanZhiRelativeRaw {
    ganhe: HashMap<String, Vec<String>>,
    zihe6: HashMap<String, Vec<String>>,
    zihe3: HashMap<String, Vec<String>>,
    zihui: HashMap<String, Vec<String>>,
    ganchong: HashMap<String, Vec<String>>,
    zichong: HashMap<String, Vec<String>>,
    zixin: HashMap<String, Vec<String>>,
    zicuan: HashMap<String, Vec<String>>,
    zipo: HashMap<String, Vec<String>>,
    lu: HashMap<String, String>,
}

static GANZHI_RELATIVE_DATA: LazyLock<GanZhiRelativeRaw> = LazyLock::new(|| {
    serde_json::from_str(GANZHI_RELATIVE_JSON).unwrap_or_else(|_| GanZhiRelativeRaw {
        ganhe: HashMap::new(),
        zihe6: HashMap::new(),
        zihe3: HashMap::new(),
        zihui: HashMap::new(),
        ganchong: HashMap::new(),
        zichong: HashMap::new(),
        zixin: HashMap::new(),
        zicuan: HashMap::new(),
        zipo: HashMap::new(),
        lu: HashMap::new(),
    })
});

// ============ 神煞规则 ============

#[derive(Debug, Clone, Deserialize)]
pub struct GodRule {
    pub name: String,
    pub jixiong: String,
    #[serde(default)]
    pub keyZhu: Vec<String>,
    pub keyType: String,
    #[serde(default)]
    pub valueZhu: Vec<String>,
    pub valueType: String,
    pub rule: serde_json::Value,
}

static GODS_RULES: LazyLock<Vec<GodRule>> = LazyLock::new(|| {
    serde_json::from_str(GODS_JSON).unwrap_or_default()
});

// ============ 公共访问器 ============

/// 获取纳音
pub fn get_nayin(ganzhi: &str) -> Option<&'static str> {
    NAYIN_DATA.get(ganzhi).map(|s| s.as_str())
}

/// 获取季节旺衰状态
/// key: "{wuxing}_{zhi}", e.g., "木_寅"
pub fn get_season_state(wuxing: &str, zhi: &str) -> Option<&'static str> {
    let key = format!("{}_{}", wuxing, zhi);
    SEASON_DATA.get(&key).map(|s| s.as_str())
}

/// 获取调候用神
/// day_gan: 日干 (甲/乙/丙/丁/戊/己/庚/辛/壬/癸)
/// month_zhi: 月支 (子/丑/寅/卯/辰/巳/午/未/申/酉/戌/亥)
pub fn get_tiaohou(day_gan: &str, month_zhi: &str) -> Option<&'static Vec<String>> {
    TIAOHOU_DATA
        .get(day_gan)
        .and_then(|inner| inner.get(month_zhi))
        .map(|v| {
            // Leak the Vec to get a &'static reference
            // This is safe because the data is static and never deallocated
            let boxed: &'static Vec<String> = Box::leak(Box::new(v.clone()));
            boxed
        })
}

/// 获取天干合
/// 返回 (合干, 五行)
pub fn get_gan_he(gan: &str) -> Option<(&'static str, &'static str)> {
    GANZHI_RELATIVE_DATA.ganhe.get(gan).map(|v| {
        let he_gan: &'static str = leak_str(&v[0]);
        let wuxing: &'static str = leak_str(&v[1]);
        (he_gan, wuxing)
    })
}

/// 获取地支六合
/// 返回 (合支, 五行)
pub fn get_zi_he6(zhi: &str) -> Option<(&'static str, &'static str)> {
    GANZHI_RELATIVE_DATA.zihe6.get(zhi).map(|v| {
        let he_zhi: &'static str = leak_str(&v[0]);
        let wuxing: &'static str = leak_str(&v[1]);
        (he_zhi, wuxing)
    })
}

/// 获取地支三合
/// 返回 (合支列表, 五行)
pub fn get_zi_he3(zhi: &str) -> (Vec<&'static str>, &'static str) {
    GANZHI_RELATIVE_DATA.zihe3.get(zhi).map(|v| {
        let zhi1: &'static str = leak_str(&v[0]);
        let zhi2: &'static str = leak_str(&v[1]);
        let wuxing: &'static str = leak_str(&v[2]);
        (vec![zhi1, zhi2], wuxing)
    }).unwrap_or_else(|| (vec![], ""))
}

/// 获取地支三会
/// 返回 (会支列表, 五行, 方位, 神兽)
pub fn get_zi_hui(zhi: &str) -> (Vec<&'static str>, &'static str, &'static str, &'static str) {
    GANZHI_RELATIVE_DATA.zihui.get(zhi).map(|v| {
        let zhi1: &'static str = leak_str(&v[0]);
        let zhi2: &'static str = leak_str(&v[1]);
        let wuxing: &'static str = leak_str(&v[2]);
        let direction: &'static str = leak_str(&v[3]);
        let animal: &'static str = leak_str(&v[4]);
        (vec![zhi1, zhi2], wuxing, direction, animal)
    }).unwrap_or_else(|| (vec![], "", "", ""))
}

/// 获取天干冲
/// 返回 (冲干, 描述)
pub fn get_gan_chong(gan: &str) -> Option<(&'static str, &'static str)> {
    GANZHI_RELATIVE_DATA.ganchong.get(gan).map(|v| {
        let chong_gan: &'static str = leak_str(&v[0]);
        let desc: &'static str = leak_str(&v[1]);
        (chong_gan, desc)
    })
}

/// 获取地支冲
/// 返回 (冲支, 描述)
pub fn get_zi_chong(zhi: &str) -> Option<(&'static str, &'static str)> {
    GANZHI_RELATIVE_DATA.zichong.get(zhi).map(|v| {
        let chong_zhi: &'static str = leak_str(&v[0]);
        let desc: &'static str = leak_str(&v[1]);
        (chong_zhi, desc)
    })
}

/// 获取地支刑
/// 返回刑支列表（不含描述）
pub fn get_zi_xing(zhi: &str) -> Vec<&'static str> {
    GANZHI_RELATIVE_DATA.zixin.get(zhi).map(|v| {
        // 最后一项是描述，前面的都是刑支
        let len = v.len();
        if len > 1 {
            v[..len - 1].iter().map(|s| leak_str(s)).collect()
        } else {
            vec![]
        }
    }).unwrap_or_default()
}

/// 获取地支穿（害）
/// 返回穿支
pub fn get_zi_chuan(zhi: &str) -> Option<&'static str> {
    GANZHI_RELATIVE_DATA.zicuan.get(zhi).map(|v| leak_str(&v[0]))
}

/// 获取地支破
/// 返回破支
pub fn get_zi_po(zhi: &str) -> Option<&'static str> {
    GANZHI_RELATIVE_DATA.zipo.get(zhi).map(|v| leak_str(&v[0]))
}

/// 获取天干禄
pub fn get_gan_lu(gan: &str) -> Option<&'static str> {
    GANZHI_RELATIVE_DATA.lu.get(gan).map(|s| s.as_str())
}

/// 获取五行长生状态（五行在地支上的十二长生阶段）
/// wuxing: 木/火/土/金/水
/// zhi: 子/丑/寅/卯/辰/巳/午/未/申/酉/戌/亥
pub fn get_wuxing_phase(wuxing: &str, zhi: &str) -> Option<&'static str> {
    let key = format!("{}_{}", wuxing, zhi);
    WUXING_PHASE_DATA
        .get("huotutong")
        .and_then(|h| h.get("wxzhi"))
        .and_then(|wxzhi| wxzhi.get(&key))
        .and_then(|v| v.as_str())
        .map(leak_str)
}

/// 获取干支长生状态（天干在地支上的十二长生阶段）
/// gan: 甲/乙/丙/丁/戊/己/庚/辛/壬/癸
/// zhi: 子/丑/寅/卯/辰/巳/午/未/申/酉/戌/亥
pub fn get_ganzhi_phase(gan: &str, zhi: &str) -> Option<&'static str> {
    let key = format!("{}_{}", gan, zhi);
    WUXING_PHASE_DATA
        .get("huotutong")
        .and_then(|h| h.get("ganzi"))
        .and_then(|ganzi| ganzi.get(&key))
        .and_then(|v| v.as_str())
        .map(leak_str)
}

/// 获取所有神煞规则
pub fn get_gods() -> Vec<GodRule> {
    GODS_RULES.clone()
}

/// 获取十二宫原始数据
pub fn get_gong12_data() -> &'static serde_json::Value {
    &GONG12_DATA
}

/// 获取八字口诀原始数据
pub fn get_bazipithy_data() -> &'static serde_json::Value {
    &BAZIPITHY_DATA
}

/// 获取八字组合原始数据
pub fn get_bazicompose() -> &'static serde_json::Value {
    &BAZICOMPOSE_DATA
}

/// 获取季节数据（原始访问）
pub fn get_season_data() -> &'static HashMap<String, String> {
    &SEASON_DATA
}

/// 获取调候数据（原始访问）
pub fn get_tiaohou_data() -> &'static HashMap<String, HashMap<String, Vec<String>>> {
    &TIAOHOU_DATA
}

/// 获取干支关系原始数据
pub fn get_ganzhi_relative_raw() -> &'static GanZhiRelativeRaw {
    &GANZHI_RELATIVE_DATA
}

/// 获取五行长生阶段原始数据
pub fn get_wuxing_phase_data() -> &'static serde_json::Value {
    &WUXING_PHASE_DATA
}

// ============ 内部辅助函数 ============

/// 将 String 泄漏为 &'static str
/// 用于从静态数据中返回引用
fn leak_str(s: &str) -> &'static str {
    unsafe { std::str::from_utf8_unchecked(std::mem::transmute::<&[u8], &'static [u8]>(s.as_bytes())) }
}