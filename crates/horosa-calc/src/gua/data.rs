// 星阙 Horosa - 卦象数据加载
// 参考原项目: astrostudysrv/astrostudycn/helper/GuaHelper.java

use horosa_core::liuyao::*;
use std::collections::HashMap;

/// 加载所有六十四卦数据
pub fn load_64_gua() -> HashMap<String, LiuShiSiGua> {
    let mut map = HashMap::new();

    let gua_files: &[(&str, &str)] = &[
        ("000000", include_str!("data/000000.json")),
        ("100000", include_str!("data/100000.json")),
        ("010000", include_str!("data/010000.json")),
        ("110000", include_str!("data/110000.json")),
        ("001000", include_str!("data/001000.json")),
        ("101000", include_str!("data/101000.json")),
        ("011000", include_str!("data/011000.json")),
        ("111000", include_str!("data/111000.json")),
        ("000100", include_str!("data/000100.json")),
        ("100100", include_str!("data/100100.json")),
        ("010100", include_str!("data/010100.json")),
        ("110100", include_str!("data/110100.json")),
        ("001100", include_str!("data/001100.json")),
        ("101100", include_str!("data/101100.json")),
        ("011100", include_str!("data/011100.json")),
        ("111100", include_str!("data/111100.json")),
        ("000010", include_str!("data/000010.json")),
        ("100010", include_str!("data/100010.json")),
        ("010010", include_str!("data/010010.json")),
        ("110010", include_str!("data/110010.json")),
        ("001010", include_str!("data/001010.json")),
        ("101010", include_str!("data/101010.json")),
        ("011010", include_str!("data/011010.json")),
        ("111010", include_str!("data/111010.json")),
        ("000110", include_str!("data/000110.json")),
        ("100110", include_str!("data/100110.json")),
        ("010110", include_str!("data/010110.json")),
        ("110110", include_str!("data/110110.json")),
        ("001110", include_str!("data/001110.json")),
        ("101110", include_str!("data/101110.json")),
        ("011110", include_str!("data/011110.json")),
        ("111110", include_str!("data/111110.json")),
        ("000001", include_str!("data/000001.json")),
        ("100001", include_str!("data/100001.json")),
        ("010001", include_str!("data/010001.json")),
        ("110001", include_str!("data/110001.json")),
        ("001001", include_str!("data/001001.json")),
        ("101001", include_str!("data/101001.json")),
        ("011001", include_str!("data/011001.json")),
        ("111001", include_str!("data/111001.json")),
        ("000101", include_str!("data/000101.json")),
        ("100101", include_str!("data/100101.json")),
        ("010101", include_str!("data/010101.json")),
        ("110101", include_str!("data/110101.json")),
        ("001101", include_str!("data/001101.json")),
        ("101101", include_str!("data/101101.json")),
        ("011101", include_str!("data/011101.json")),
        ("111101", include_str!("data/111101.json")),
        ("000011", include_str!("data/000011.json")),
        ("100011", include_str!("data/100011.json")),
        ("010011", include_str!("data/010011.json")),
        ("110011", include_str!("data/110011.json")),
        ("001011", include_str!("data/001011.json")),
        ("101011", include_str!("data/101011.json")),
        ("011011", include_str!("data/011011.json")),
        ("111011", include_str!("data/111011.json")),
        ("000111", include_str!("data/000111.json")),
        ("100111", include_str!("data/100111.json")),
        ("010111", include_str!("data/010111.json")),
        ("110111", include_str!("data/110111.json")),
        ("001111", include_str!("data/001111.json")),
        ("101111", include_str!("data/101111.json")),
        ("011111", include_str!("data/011111.json")),
        ("111111", include_str!("data/111111.json")),
    ];

    for (fname, json_str) in gua_files {
        if let Ok(gua) = parse_gua_json(json_str) {
            map.insert(fname.to_string(), gua.clone());
            map.insert(gua.name.clone(), gua.clone());
            map.insert(gua.abr_name.clone(), gua.clone());
            map.insert(gua.gua_name.clone(), gua);
        }
    }

    map
}

/// 加载梅花易数八卦数据
pub fn load_meiyi_gua() -> HashMap<String, MeiYiGua> {
    let mut map = HashMap::new();

    let mei_yi_files: &[(&str, &str)] = &[
        ("111", include_str!("data/111.json")),
        ("011", include_str!("data/011.json")),
        ("101", include_str!("data/101.json")),
        ("001", include_str!("data/001.json")),
        ("110", include_str!("data/110.json")),
        ("010", include_str!("data/010.json")),
        ("100", include_str!("data/100.json")),
        ("000", include_str!("data/000.json")),
    ];

    for (fname, json_str) in mei_yi_files {
        if let Ok(gua) = parse_meiyi_json(json_str) {
            map.insert(fname.to_string(), gua.clone());
            map.insert(gua.name.clone(), gua.clone());
            map.insert(gua.abr_name.clone(), gua);
        }
    }

    map
}

/// 加载干支卦映射表
pub fn load_ganzhi_gua() -> HashMap<String, String> {
    let json_str = include_str!("data/ganzhi_gua.json");
    let data: serde_json::Value = serde_json::from_str(json_str).unwrap_or_default();
    let mut map = HashMap::new();

    if let Some(ganzhi) = data.get("干支") {
        if let Some(obj) = ganzhi.as_object() {
            for (k, v) in obj {
                if let Some(name) = v.as_str() {
                    map.insert(k.clone(), name.to_string());
                }
            }
        }
    }

    map
}

/// 加载四象数据
pub fn load_sixiang() -> Vec<SiXiang> {
    let json_str = include_str!("data/ganzhi_gua.json");
    let data: serde_json::Value = serde_json::from_str(json_str).unwrap_or_default();
    let mut result = Vec::new();

    if let Some(sixiang) = data.get("四象") {
        if let Some(arr) = sixiang.as_array() {
            for item in arr {
                let yao = item.get("yao").and_then(|v| v.as_array()).map(|a| {
                    [a.get(0).and_then(|v| v.as_u64()).unwrap_or(0) as u8,
                     a.get(1).and_then(|v| v.as_u64()).unwrap_or(0) as u8]
                }).unwrap_or([0, 0]);

                result.push(SiXiang {
                    yao,
                    name: item.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    season: item.get("season").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    xiang: item.get("xiang").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                });
            }
        }
    }

    result
}

fn parse_gua_json(json_str: &str) -> Result<LiuShiSiGua, serde_json::Error> {
    let data: serde_json::Value = serde_json::from_str(json_str)?;

    let yao_arr = data.get("yao").and_then(|v| v.as_array()).map(|a| {
        let mut yao = [0u8; 6];
        for (i, v) in a.iter().take(6).enumerate() {
            yao[i] = v.as_u64().unwrap_or(0) as u8;
        }
        yao
    }).unwrap_or([0u8; 6]);

    let yao_ci_texts: Vec<String> = data.get("爻辞").and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();

    let yao_xiang: Vec<String> = data.get("爻象").and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();

    let yao_ci: Vec<YaoCi> = yao_ci_texts.iter().enumerate().map(|(i, text)| {
        let position = if i < 6 { (i + 1) as u8 } else { 0 };
        let xiang = yao_xiang.get(i).cloned().unwrap_or_default();
        YaoCi {
            position,
            text: text.clone(),
            xiang,
        }
    }).collect();

    Ok(LiuShiSiGua {
        ord: data.get("ord").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
        name: data.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        gua_name: data.get("guaname").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        abr_name: data.get("abrname").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        desc: data.get("desc").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        yao: yao_arr,
        gua_ci: data.get("卦辞").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        yao_ci,
        tuan_ci: data.get("彖").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        xiang_ci: data.get("象").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        wen_yan: data.get("文言").and_then(|v| v.as_str()).map(|s| s.to_string()),
        url: data.get("url").and_then(|v| v.as_str()).map(|s| s.to_string()),
        symbolize: data.get("symbolize").and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default(),
    })
}

fn parse_meiyi_json(json_str: &str) -> Result<MeiYiGua, serde_json::Error> {
    let data: serde_json::Value = serde_json::from_str(json_str)?;

    let yao_arr = data.get("yao").and_then(|v| v.as_array()).map(|a| {
        let mut yao = [0u8; 3];
        for (i, v) in a.iter().take(3).enumerate() {
            yao[i] = v.as_u64().unwrap_or(0) as u8;
        }
        yao
    }).unwrap_or([0u8; 3]);

    let mei_yi = data.get("梅易").map(|mei| {
        MeiYiCategory {
            ren_wu: mei.get("人物").and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            shen_ti: mei.get("身体").and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            dong_wu: mei.get("动物").and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            wu_pin: mei.get("物品").and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            chang_suo: mei.get("场所").and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            tian_xiang: mei.get("天象").and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            shi_jian: mei.get("时间").and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            shu_zi: mei.get("数字").and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            fang_wei: mei.get("方位").and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            gan_zhi: mei.get("干支").and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            wei: mei.get("味").and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            se: mei.get("色").and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
        }
    });

    Ok(MeiYiGua {
        ord: data.get("ord").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
        name: data.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        abr_name: data.get("abrname").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        desc: data.get("desc").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        yao: yao_arr,
        mei_yi,
        symbolize: data.get("symbolize").and_then(|v| v.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default(),
    })
}