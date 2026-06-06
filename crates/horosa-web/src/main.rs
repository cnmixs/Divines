// 星阙 Horosa - Dioxus 前端入口
// 参考原项目: astrostudyui/src/app.js, dva.js

use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod components;
mod pages;
mod layouts;
mod services;
mod utils;
mod constants;

/// 路由定义
#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},

        // 命 · 命盘与推运
        #[route("/astro/natal")]
        AstroNatal {},
        #[route("/astro/timing")]
        AstroTiming {},
        #[route("/astro/relationship")]
        AstroRelationship {},
        #[route("/astro/specialty")]
        AstroSpecialty {},
        #[route("/astro/vedic")]
        AstroVedic {},
        #[route("/astro/qizheng")]
        AstroQizheng {},
        #[route("/bazi")]
        Bazi {},
        #[route("/ziwei")]
        Ziwei {},
        #[route("/shusuan")]
        ShuSuan {},
        #[route("/guolao")]
        GuoLao {},
        #[route("/guazhan")]
        GuaZhan {},

        // 卜 · 易与三式
        #[route("/sanshi")]
        Sanshi {},
        #[route("/qimen")]
        Qimen {},
        #[route("/taiyi")]
        Taiyi {},
        #[route("/liuren")]
        Liuren {},
        #[route("/liuyao")]
        Liuyao {},
        #[route("/dunjia")]
        DunJia {},
        #[route("/gua")]
        Gua {},
        #[route("/jieqi")]
        Jieqi {},
        #[route("/fengshui")]
        FengShui {},
        #[route("/divination-other")]
        DivinationOther {},

        // 传统术数 · 数算与神数
        #[route("/huangji")]
        HuangJi {},
        #[route("/jingjue")]
        JingJue {},
        #[route("/jinkou")]
        JinKou {},
        #[route("/shenyishu")]
        ShenYiShu {},
        #[route("/wuzhao")]
        WuZhao {},
        #[route("/taixuan")]
        TaiXuan {},
        #[route("/beiji")]
        BeiJi {},
        #[route("/cetian")]
        CeTian {},
        #[route("/chunzi")]
        ChunZi {},
        #[route("/fendjing")]
        FenJing {},
        #[route("/nanji")]
        NanJi {},
        #[route("/shaozi")]
        ShaoZi {},
        #[route("/tieban")]
        TieBan {},
        #[route("/xianqin")]
        XianQin {},

        // 西方占星 · 专项
        #[route("/astro/hellenistic")]
        AstroHellenistic {},
        #[route("/astro/horary")]
        AstroHorary {},
        #[route("/astro/electional")]
        AstroElectional {},
        #[route("/astro/mundane")]
        AstroMundane {},
        #[route("/astro/germany")]
        AstroGermany {},
        #[route("/astro/synastry")]
        AstroSynastry {},
        #[route("/astro/acg")]
        AstroAcg {},
        #[route("/astro/rectification")]
        AstroRectification {},

        // 工具 · 工具工作台
        #[route("/ai-analysis")]
        AiAnalysis {},
        #[route("/planetarium")]
        Planetarium {},
        #[route("/almanac")]
        Almanac {},
        #[route("/references")]
        References {},
        #[route("/dice")]
        Dice {},
        #[route("/su28")]
        Su28 {},

        // 邵子系列
        #[route("/sz/bagua")]
        SzBaGua {},
        #[route("/sz/dunjia")]
        SzDunJia {},
        #[route("/sz/taiyi")]
        SzTaiYi {},
        #[route("/sz/fangwei")]
        SzFangWei {},
        #[route("/sz/fengye")]
        SzFengYe {},
        #[route("/sz/nixiang")]
        SzNiXiang {},
        #[route("/sz/sign")]
        SzSign {},

        // 其他术数（命理其他、宿占、通设法、其他卜）
        #[route("/mingother")]
        MingOther {},
        #[route("/suzhan")]
        SuZhan {},
        #[route("/tongshefa")]
        TongSheFa {},
        #[route("/otherbu")]
        OtherBu {},

        // 其他
        #[route("/settings")]
        Settings {},
        #[route("/about")]
        About {},

    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

/// 应用入口
/// 
/// 支持多平台:
///   - Web:    dx serve --platform web
///   - Desktop: dx serve --platform desktop  
///   - iOS:    dx bundle --platform ios --release
///   - Android: dx bundle --platform android --release
fn main() {
    #[cfg(not(target_os = "ios"))]
    {
        dioxus_logger::init(dioxus_logger::tracing::Level::INFO)
            .expect("failed to init logger");
    }
    tracing::info!("星阙 Horosa 启动中...");

    dioxus::launch(App);
}

/// 根组件
#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}