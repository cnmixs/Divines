// Divines - 服务模块
// 参考原项目: astrostudyui/src/services/

use serde::{Deserialize, Serialize};

/// API 基础 URL
const API_BASE: &str = "http://127.0.0.1:3000/api";

/// 通用 API 请求
pub async fn api_request<T: for<'de> Deserialize<'de>>(
    method: &str,
    path: &str,
    body: Option<&impl Serialize>,
) -> Result<T, String> {
    let url = format!("{}{}", API_BASE, path);
    let client = reqwest::Client::new();

    let request = match method {
        "GET" => client.get(&url),
        "POST" => {
            let mut req = client.post(&url);
            if let Some(b) = body {
                req = req.json(b);
            }
            req
        }
        _ => return Err(format!("不支持的 HTTP 方法: {}", method)),
    };

    let response = request
        .send()
        .await
        .map_err(|e| format!("网络错误: {}", e))?;

    response
        .json::<T>()
        .await
        .map_err(|e| format!("解析错误: {}", e))
}

/// 占星服务
pub mod astro {
    use super::*;

    // 重新导出通用 API 请求函数，方便各页面调用
    pub use super::api_request;

    pub async fn get_natal_chart(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/astro/natal", Some(req)).await
    }

    pub async fn get_aspects(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/astro/aspects", Some(req)).await
    }

    pub async fn get_firdaria(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/astro/firdaria", Some(req)).await
    }

    pub async fn get_arabic_points(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/astro/arabic-points", Some(req)).await
    }

    pub async fn synastry_chart(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/astro/synastry", Some(req)).await
    }

    pub async fn composite_chart(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/astro/composite", Some(req)).await
    }

    pub async fn acg_lines(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/astro/acg", Some(req)).await
    }

    pub async fn solar_arc(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/predict/solar-arc", Some(req)).await
    }

    pub async fn progressions(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/predict/progressions", Some(req)).await
    }

    pub async fn primary_directions(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/predict/primary-directions", Some(req)).await
    }

    pub async fn profections(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/predict/profections", Some(req)).await
    }

    pub async fn age_point(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/predict/age-point", Some(req)).await
    }

    pub async fn symbolic_dir(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/predict/symbolic-dir", Some(req)).await
    }

    pub async fn term_direction(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/predict/term-direction", Some(req)).await
    }

    pub async fn thirteenth_chart(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/predict/thirteenth-chart", Some(req)).await
    }

    pub async fn harmonic_chart(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/predict/harmonic-chart", Some(req)).await
    }

    pub async fn draconic_chart(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/predict/draconic-chart", Some(req)).await
    }

    pub async fn year_system_129(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/predict/year-system-129", Some(req)).await
    }

    pub async fn huangji_yuan_hui(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/huangji/yuan-hui", Some(req)).await
    }

    pub async fn fengshui_ming_gua(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/fengshui/ming-gua", Some(req)).await
    }

    pub async fn fengshui_flying_stars(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/fengshui/flying-stars", Some(req)).await
    }

    pub async fn beiji_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/beiji/calculate", Some(req)).await
    }

    pub async fn cetian_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/cetian/calculate", Some(req)).await
    }

    pub async fn chunzi_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/chunzi/calculate", Some(req)).await
    }

    pub async fn fendjing_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/fendjing/calculate", Some(req)).await
    }

    pub async fn nanji_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/nanji/calculate", Some(req)).await
    }

    pub async fn shaozi_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/shaozi/calculate", Some(req)).await
    }

    pub async fn tieban_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/tieban/calculate", Some(req)).await
    }

    pub async fn xianqin_divination(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/xianqin/divination", Some(req)).await
    }

    pub async fn planetarium_current(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/planetarium/current", Some(req)).await
    }

    pub async fn germany_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/germany/calculate", Some(req)).await
    }

    pub async fn jingjue_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/jingjue/calculate", Some(req)).await
    }

    pub async fn jinkou_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/jinkou/calculate", Some(req)).await
    }

    pub async fn shenyishu_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/shenyishu/calculate", Some(req)).await
    }

    pub async fn taixuan_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/taixuan/calculate", Some(req)).await
    }

    pub async fn wuzhao_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/wuzhao/calculate", Some(req)).await
    }

    pub async fn wangji_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/wangji/calculate", Some(req)).await
    }

    pub async fn vedic_chart(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/vedic/chart", Some(req)).await
    }

    pub async fn vedic_dasha(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/vedic/dasha", Some(req)).await
    }

    pub async fn vedic_yogas(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/vedic/yogas", Some(req)).await
    }

    pub async fn vedic_nakshatra(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/vedic/nakshatra", Some(req)).await
    }
}

/// 八字服务
pub mod bazi {
    use super::*;

    pub async fn calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/bazi/calculate", Some(req)).await
    }

    pub async fn paipan(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/bazi/paipan", Some(req)).await
    }
}

/// 紫微服务
pub mod ziwei {
    use super::*;

    pub async fn calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/ziwei/calculate", Some(req)).await
    }
}

/// 七政四余服务
pub mod qizheng {
    use super::*;

    pub async fn get_chart(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/qizheng/chart", Some(req)).await
    }

    pub async fn get_pattern(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/qizheng/pattern", Some(req)).await
    }

    pub async fn get_moira(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/qizheng/moira", Some(req)).await
    }

    pub async fn get_dasha(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/qizheng/dasha", Some(req)).await
    }

    pub async fn get_dongwei(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/qizheng/dongwei", Some(req)).await
    }
}

/// 三式服务
pub mod sanshi {
    use super::*;

    pub async fn qimen(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/qimen/calculate", Some(req)).await
    }

    pub async fn taiyi(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/taiyi/calculate", Some(req)).await
    }

    pub async fn liuren(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/liuren/calculate", Some(req)).await
    }
}

/// AI 服务
pub mod ai {
    use super::*;

    pub async fn chat(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/ai/chat", Some(req)).await
    }

    pub async fn get_models() -> Result<serde_json::Value, String> {
        api_request::<serde_json::Value>("GET", "/ai/models", None::<&serde_json::Value>).await
    }
}

/// 日历服务（节气、黄历、万年历）
pub mod calendar {
    use super::*;

    pub async fn get_jieqi(year: i32) -> Result<serde_json::Value, String> {
        let url = format!("/calendar/jieqi?year={}", year);
        api_request::<serde_json::Value>("GET", &url, None::<&serde_json::Value>).await
    }

    pub async fn get_almanac(year: i32, month: u32, day: u32) -> Result<serde_json::Value, String> {
        let url = format!("/calendar/almanac?year={}&month={}&day={}", year, month, day);
        api_request::<serde_json::Value>("GET", &url, None::<&serde_json::Value>).await
    }

    /// 公历转农历（寿星万年历）
    pub async fn solar_to_lunar(year: i32, month: u32, day: u32) -> Result<serde_json::Value, String> {
        let url = format!("/calendar/solar-to-lunar?year={}&month={}&day={}", year, month, day);
        api_request::<serde_json::Value>("GET", &url, None::<&serde_json::Value>).await
    }

    /// 农历转公历
    pub async fn lunar_to_solar(
        lunar_year: i32, lunar_month: u32, lunar_day: u32, is_leap: bool,
    ) -> Result<serde_json::Value, String> {
        let url = format!(
            "/calendar/lunar-to-solar?lunar_year={}&lunar_month={}&lunar_day={}&is_leap={}",
            lunar_year, lunar_month, lunar_day, is_leap
        );
        api_request::<serde_json::Value>("GET", &url, None::<&serde_json::Value>).await
    }

    /// 公历转回历
    pub async fn solar_to_islamic(year: i32, month: u32, day: u32) -> Result<serde_json::Value, String> {
        let url = format!("/calendar/solar-to-islamic?year={}&month={}&day={}", year, month, day);
        api_request::<serde_json::Value>("GET", &url, None::<&serde_json::Value>).await
    }

    /// 干支查询
    pub async fn get_ganzhi(year: i32, month: u32, day: u32) -> Result<serde_json::Value, String> {
        let url = format!("/calendar/ganzhi?year={}&month={}&day={}", year, month, day);
        api_request::<serde_json::Value>("GET", &url, None::<&serde_json::Value>).await
    }

    /// 日月食查询
    pub async fn get_eclipses(year: i32) -> Result<serde_json::Value, String> {
        let url = format!("/calendar/eclipses?year={}", year);
        api_request::<serde_json::Value>("GET", &url, None::<&serde_json::Value>).await
    }

    /// 城市经纬度查询
    pub async fn get_city_coords(name: &str) -> Result<serde_json::Value, String> {
        let url = format!("/calendar/city?name={}", urlencoding(name));
        api_request::<serde_json::Value>("GET", &url, None::<&serde_json::Value>).await
    }

    fn urlencoding(s: &str) -> String {
        s.replace(' ', "%20")
    }
}

/// 用户服务
pub mod user {
    use super::*;

    pub async fn login(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/user/login", Some(req)).await
    }
}

/// 新增服务函数 - 宿占、通设法、命理其他、其他卜、邵子扩展
/// 这些函数对应补全的原项目功能模块
pub mod extended {
    use super::*;

    pub async fn suzhan_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/suzhan/calculate", Some(req)).await
    }

    pub async fn tongshefa_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/tongshefa/calculate", Some(req)).await
    }

    pub async fn mingother_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/mingother/calculate", Some(req)).await
    }

    pub async fn otherbu_calculate(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/otherbu/calculate", Some(req)).await
    }

    pub async fn sz_fangwei(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/sz/fangwei", Some(req)).await
    }

    pub async fn sz_fengye(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/sz/fengye", Some(req)).await
    }

    pub async fn sz_nixiang(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/sz/nixiang", Some(req)).await
    }

    pub async fn sz_sign(req: &serde_json::Value) -> Result<serde_json::Value, String> {
        api_request("POST", "/sz/sign", Some(req)).await
    }
}