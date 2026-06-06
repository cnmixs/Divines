// Divines - 后端 API 服务器
// 参考原项目: astrostudysrv/ (Java Spring Boot) + astropy/websrv/ (Python HTTP)

use axum::{
    Router,
    routing::{get, post},
    Json,
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod middleware;

/// 应用状态
pub struct AppState {
    pub astrology: divines_calc::astrology::AstrologyCalc,
    pub bazi: divines_calc::bazi::BaziCalc,
    pub ziwei: divines_calc::ziwei::ZiWeiCalc,
    pub qimen: divines_calc::sanshi::QimenCalc,
    pub taiyi: divines_calc::sanshi::TaiyiCalc,
    pub liuren: divines_calc::sanshi::LiurenCalc,
    pub gua: divines_calc::gua::GuaCalc,
    pub liuren_full: divines_calc::liureng::LiuRengCalc,
    pub jieqi: divines_calc::jieqi::JieQiCalc,
    pub sxwnl: divines_calc::Sxwnl,
    pub qizheng: divines_calc::qizheng::QizhengCalc,
    pub liuyao: divines_calc::liuyao::LiuyaoCalc,
    pub vedic: divines_calc::vedic::VedicCalc,
    pub predict: divines_calc::predict::PredictCalc,
    pub fengshui: divines_calc::remaining::fengshui::FengShuiCalc,
    pub huangji: divines_calc::remaining::huangji::HuangJiCalc,
    pub modern: divines_calc::remaining::modern::ModernCalc,
    pub acg: divines_calc::remaining::acg::AcgCalc,
    pub jingjue: divines_calc::remaining::jingjue::JingJueCalc,
    pub shenyishu: divines_calc::remaining::shenyishu::ShenYiShuCalc,
    pub jinkou: divines_calc::remaining::jinkou::JinKouCalc,
    pub wuzhao: divines_calc::remaining::wuzhao::WuZhaoCalc,
    pub taixuan: divines_calc::remaining::taixuan::TaiXuanCalc,
    pub beiji: divines_calc::remaining::beiji::BeiJiCalc,
    pub cetian: divines_calc::remaining::cetian::CeTianCalc,
    pub chunzi: divines_calc::remaining::chunzi::ChunZiCalc,
    pub fendjing: divines_calc::remaining::fendjing::FenJingCalc,
    pub nanji: divines_calc::remaining::nanji::NanJiCalc,
    pub shaozi: divines_calc::remaining::shaozi::ShaoZiCalc,
    pub tieban: divines_calc::remaining::tieban::TieBanCalc,
    pub xianqin: divines_calc::remaining::xianqin::XianQinCalc,
    pub planetarium: divines_calc::remaining::planetarium::PlanetariumCalc,
    pub germany: divines_calc::remaining::germany::GermanyCalc,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            astrology: divines_calc::astrology::AstrologyCalc::default(),
            bazi: divines_calc::bazi::BaziCalc::default(),
            ziwei: divines_calc::ziwei::ZiWeiCalc::default(),
            qimen: divines_calc::sanshi::QimenCalc::default(),
            taiyi: divines_calc::sanshi::TaiyiCalc::default(),
            liuren: divines_calc::sanshi::LiurenCalc::default(),
            gua: divines_calc::gua::GuaCalc::default(),
            liuren_full: divines_calc::liureng::LiuRengCalc::default(),
            jieqi: divines_calc::jieqi::JieQiCalc::default(),
            sxwnl: divines_calc::Sxwnl::default(),
            qizheng: divines_calc::qizheng::QizhengCalc::default(),
            liuyao: divines_calc::liuyao::LiuyaoCalc::new(),
            vedic: divines_calc::vedic::VedicCalc::new(),
            predict: divines_calc::predict::PredictCalc::new(),
            fengshui: divines_calc::remaining::fengshui::FengShuiCalc::new(),
            huangji: divines_calc::remaining::huangji::HuangJiCalc::new(),
            modern: divines_calc::remaining::modern::ModernCalc::new(),
            acg: divines_calc::remaining::acg::AcgCalc::new(),
            jingjue: divines_calc::remaining::jingjue::JingJueCalc::new(),
            shenyishu: divines_calc::remaining::shenyishu::ShenYiShuCalc::new(),
            jinkou: divines_calc::remaining::jinkou::JinKouCalc::new(),
            wuzhao: divines_calc::remaining::wuzhao::WuZhaoCalc::new(),
            taixuan: divines_calc::remaining::taixuan::TaiXuanCalc::new(),
            beiji: divines_calc::remaining::beiji::BeiJiCalc::new(),
            cetian: divines_calc::remaining::cetian::CeTianCalc::new(),
            chunzi: divines_calc::remaining::chunzi::ChunZiCalc::new(),
            fendjing: divines_calc::remaining::fendjing::FenJingCalc::new(),
            nanji: divines_calc::remaining::nanji::NanJiCalc::new(),
            shaozi: divines_calc::remaining::shaozi::ShaoZiCalc::new(),
            tieban: divines_calc::remaining::tieban::TieBanCalc::new(),
            xianqin: divines_calc::remaining::xianqin::XianQinCalc::new(),
            planetarium: divines_calc::remaining::planetarium::PlanetariumCalc::new(),
            germany: divines_calc::remaining::germany::GermanyCalc::new(),
        }
    }
}

/// 启动服务器
///
/// 参考原项目: astrostudysrv/astrostudyboot/ 启动入口
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "divines_server=info,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Divines 服务器启动中...");

    let state = Arc::new(AppState::new());

    let app = Router::new()
        // 健康检查
        .route("/api/health", get(health_check))
        // 占星 API
        .route("/api/astro/natal", post(api::astro::natal_chart))
        .route("/api/astro/aspects", post(api::astro::aspects))
        .route("/api/astro/firdaria", post(api::astro::firdaria))
        .route("/api/astro/arabic-points", post(api::astro::arabic_points))
        // 七政四余 API
        .route("/api/qizheng/chart", post(api::astro::qizheng_chart))
        .route("/api/qizheng/pattern", post(api::astro::qizheng_pattern))
        .route("/api/qizheng/moira", post(api::astro::qizheng_moira))
        .route("/api/qizheng/dasha", post(api::astro::qizheng_dasha))
        .route("/api/qizheng/dongwei", post(api::astro::qizheng_dongwei))
        // 八字 API
        .route("/api/bazi/calculate", post(api::bazi::calculate))
        .route("/api/bazi/paipan", post(api::bazi::paipan))
        // 紫微斗数 API
        .route("/api/ziwei/calculate", post(api::ziwei::calculate))
        .route("/api/ziwei/luck", post(api::ziwei::luck))
        // 三式 API
        .route("/api/qimen/calculate", post(api::sanshi::qimen_calculate))
        .route("/api/qimen/ju", post(api::sanshi::qimen_ju))
        .route("/api/taiyi/calculate", post(api::sanshi::taiyi))
        .route("/api/liuren/calculate", post(api::sanshi::liuren))
        // 卦象 API (梅花易数 / 六爻)
        .route("/api/gua/desc", post(api::liuyao::gua_desc))
        .route("/api/gua/meiyi", post(api::liuyao::gua_meiyi))
        .route("/api/gua/meihua", post(api::liuyao::gua_meihua))
        .route("/api/gua/relation", post(api::liuyao::gua_relation))
        .route("/api/gua/yao", post(api::liuyao::gua_yao))
        .route("/api/gua/list", get(api::liuyao::gua_list_all))
        .route("/api/gua/sixiang", get(api::liuyao::gua_sixiang))
        // 六壬 API (完整)
        .route("/api/liureng/calculate", post(api::liureng::calculate))
        .route("/api/liureng/month-jiang", post(api::liureng::month_jiang))
        .route("/api/liureng/zhan-shi", post(api::liureng::zhan_shi))
        .route("/api/liureng/gui-ren", post(api::liureng::gui_ren))
        .route("/api/liureng/shen-jiang", post(api::liureng::shen_jiang))
        .route("/api/liureng/si-ke", post(api::liureng::si_ke))
        .route("/api/liureng/san-chuan", post(api::liureng::san_chuan))
        .route("/api/liureng/dun-gan", post(api::liureng::dun_gan))
        .route("/api/liureng/zhang-sheng", post(api::liureng::zhang_sheng))
        // 节气/黄历 API
        .route("/api/calendar/jieqi", get(api::calendar::jieqi))
        .route("/api/calendar/almanac", get(api::calendar::almanac))
        // 寿星万年历 API
        .route("/api/calendar/solar-to-lunar", get(api::calendar::solar_to_lunar))
        .route("/api/calendar/lunar-to-solar", get(api::calendar::lunar_to_solar))
        .route("/api/calendar/solar-to-islamic", get(api::calendar::solar_to_islamic))
        .route("/api/calendar/ganzhi", get(api::calendar::ganzhi))
        .route("/api/calendar/eclipses", get(api::calendar::eclipses))
        .route("/api/calendar/city", get(api::calendar::city_coords))
        // AI 分析 API
        .route("/api/ai/chat", post(api::ai::chat))
        .route("/api/ai/models", get(api::ai::models))
        // 用户 API
        .route("/api/user/login", post(api::user::login))
        // 六爻 API
        .route("/api/liuyao/cast", post(api::liuyao::cast))
        .route("/api/liuyao/divine", post(api::liuyao::divine))
        // 印度占星 API
        .route("/api/vedic/chart", post(api::astro::vedic_chart))
        .route("/api/vedic/dasha", post(api::astro::vedic_dasha))
        .route("/api/vedic/yogas", post(api::astro::vedic_yogas))
        .route("/api/vedic/nakshatra", post(api::astro::vedic_nakshatra))
        // 推运 API
        .route("/api/predict/solar-arc", post(api::astro::solar_arc))
        .route("/api/predict/progressions", post(api::astro::progressions))
        .route("/api/predict/primary-directions", post(api::astro::primary_directions))
        .route("/api/predict/profections", post(api::astro::profections))
        .route("/api/predict/age-point", post(api::astro::age_point))
        .route("/api/predict/symbolic-dir", post(api::astro::symbolic_dir))
        .route("/api/predict/term-direction", post(api::astro::term_direction))
        .route("/api/predict/thirteenth-chart", post(api::astro::thirteenth_chart))
        .route("/api/predict/harmonic-chart", post(api::astro::harmonic_chart))
        .route("/api/predict/draconic-chart", post(api::astro::draconic_chart))
        .route("/api/predict/year-system-129", post(api::astro::year_system_129))
        // 风水 API
        .route("/api/fengshui/ming-gua", post(api::astro::fengshui_ming_gua))
        .route("/api/fengshui/flying-stars", post(api::astro::fengshui_flying_stars))
        // 皇极经世 API
        .route("/api/huangji/yuan-hui", post(api::astro::huangji_yuan_hui))
        // 合盘 API
        .route("/api/astro/synastry", post(api::astro::synastry))
        .route("/api/astro/composite", post(api::astro::composite))
        // ACG API
        .route("/api/astro/acg", post(api::astro::acg_lines))
        // 北极神数 API
        .route("/api/beiji/calculate", post(api::astro::beiji_calculate))
        // 策天 API
        .route("/api/cetian/calculate", post(api::astro::cetian_calculate))
        // 春子 API
        .route("/api/chunzi/calculate", post(api::astro::chunzi_calculate))
        // 分经 API
        .route("/api/fendjing/calculate", post(api::astro::fendjing_calculate))
        // 南极神数 API
        .route("/api/nanji/calculate", post(api::astro::nanji_calculate))
        // 邵子神数 API
        .route("/api/shaozi/calculate", post(api::astro::shaozi_calculate))
        // 铁板神数 API
        .route("/api/tieban/calculate", post(api::astro::tieban_calculate))
        // 先秦占卜 API
        .route("/api/xianqin/divination", post(api::astro::xianqin_divination))
        // 天象馆 API
        .route("/api/planetarium/current", post(api::astro::planetarium_current))
        // 德国占星 API
        .route("/api/germany/calculate", post(api::astro::germany_calculate))
        // 荆诀 API
        .route("/api/jingjue/calculate", post(api::astro::jingjue_calculate))
        // 金口诀 API
        .route("/api/jinkou/calculate", post(api::astro::jinkou_calculate))
        // 神易数 API
        .route("/api/shenyishu/calculate", post(api::astro::shenyishu_calculate))
        // 太玄 API
        .route("/api/taixuan/calculate", post(api::astro::taixuan_calculate))
        // 五兆 API
        .route("/api/wuzhao/calculate", post(api::astro::wuzhao_calculate))
        // 皇极(王极) API
        .route("/api/wangji/calculate", post(api::astro::wangji_calculate))
        // 希腊星术 API
        .route("/api/astro/hellenistic", post(api::astro::hellenistic))
        // 卜卦占星 API
        .route("/api/astro/horary", post(api::astro::horary))
        // 择时占星 API
        .route("/api/astro/electional", post(api::astro::electional))
        // 世俗占星 API
        .route("/api/astro/mundane", post(api::astro::mundane))
        .route("/api/astro/aries-ingress", post(api::astro::aries_ingress))
        // 生时校正 API
        .route("/api/astro/rectification", post(api::astro::rectification))
        // 骰子占卜 API
        .route("/api/dice/roll", post(api::astro::dice_roll))
        // 二十八宿 API
        .route("/api/su28/calculate", post(api::astro::su28_calculate))
        // 邵子系列 API
        .route("/api/sz/bagua", post(api::astro::sz_bagua))
        .route("/api/sz/dunjia", post(api::astro::sz_dunjia))
        .route("/api/sz/taiyi", post(api::astro::sz_taiyi))
        // 邵子扩展 API
        .route("/api/sz/fangwei", post(api::astro::sz_fangwei))
        .route("/api/sz/fengye", post(api::astro::sz_fengye))
        .route("/api/sz/nixiang", post(api::astro::sz_nixiang))
        .route("/api/sz/sign", post(api::astro::sz_sign))
        // 宿占 API
        .route("/api/suzhan/calculate", post(api::astro::suzhan_calculate))
        // 通设法 API
        .route("/api/tongshefa/calculate", post(api::astro::tongshefa_calculate))
        // 命理其他 API
        .route("/api/mingother/calculate", post(api::astro::mingother_calculate))
        // 其他卜 API
        .route("/api/otherbu/calculate", post(api::astro::otherbu_calculate))
        // WebSocket
        .route("/api/ws", get(ws_handler))
        // 静态文件服务（前端）
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("服务器监听: http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

/// 健康检查
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "name": "Divines",
        "version": "0.1.0",
        "rust_rewrite": true
    }))
}

/// WebSocket 处理器
/// 参考原项目: boundless/ WebSocket 支持
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(
    mut socket: axum::extract::ws::WebSocket,
    _state: Arc<AppState>,
) {
    use axum::extract::ws::Message;
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                if socket.send(Message::Text(format!("Echo: {}", text).into())).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}