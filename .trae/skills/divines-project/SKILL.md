---
name: "divines-project"
description: "Divines 项目专用技能，涵盖占星/八字/紫微/三式等术数功能开发。当用户修改 divines-calc 计算引擎、divines-server API、divines-web 前端页面或参考源仓库进行功能对比时触发。"
---

# Divines 项目开发技能

基于 [Divines Skill v0.9.2](https://github.com/Horace-Maxwell/divines-skill) 的 68 个工具定义，提供项目开发指导。

## 项目结构

```
crates/divines-calc/src/
├── astrology/     # 西方占星核心引擎 (~400 行)
├── bazi/          # 八字排盘 (7 个 .rs 文件 + 9 个 JSON 数据)
├── ephem/         # Swiss Ephemeris 星历表
├── gua/           # 梅花易数 + 六十四卦 (73 个 JSON 数据)
├── jieqi/         # 节气农历计算
├── liureng/       # 大六壬 (贵人/天地盘/四课/三传)
├── liuyao/        # 六爻排盘
├── predict/       # 推运 (次限/太阳弧/法达等)
├── qizheng/       # 七政四余 (1,111 行, VSOP87 行星计算)
├── sanshi/        # 三式合一 (1,131 行, 奇门+太乙+六壬)
├── sxwnl/         # 寿星天文历 (8 个 .rs 文件)
├── vedic/         # 印度占星 (Jyotish)
├── ziwei/         # 紫微斗数 (15 个 JSON 数据)
└── remaining.rs   # 19 个传统术数模块 (1,036 行)
```

## 源仓库对应关系

| 源仓库 (JavaScript/TypeScript) | Rust 重写位置 |
|------|------|
| `astrostudyui/src/components/astrochart/` | `divines-calc/src/astrology/` |
| `astrostudyui/src/components/bazi/` | `divines-calc/src/bazi/` |
| `astrostudyui/src/components/ziwei/` | `divines-calc/src/ziwei/` |
| `astrostudyui/src/components/qizheng/` | `divines-calc/src/qizheng/` |
| `astrostudyui/src/components/liureng/` | `divines-calc/src/liureng/` |
| `astrostudyui/src/components/liuyao/` | `divines-calc/src/liuyao/` |
| `astrostudyui/src/components/sanshi/` | `divines-calc/src/sanshi/` |
| `astrostudyui/src/components/vedic/` | `divines-calc/src/vedic/` |
| `astrostudyui/src/components/predict/` | `divines-calc/src/predict/` |
| `astrostudyui/src/components/fengshui/` | `divines-calc/src/remaining.rs#fengshui` |
| `astrostudyui/src/components/huangji/` | `divines-calc/src/remaining.rs#huangji` |
| `astrostudyui/src/components/jinkou/` | `divines-calc/src/remaining.rs#jinkou` |
| `astrostudyui/src/components/jingjue/` | `divines-calc/src/remaining.rs#jingjue` |
| `astrostudyui/src/components/shenyishu/` | `divines-calc/src/remaining.rs#shenyishu` |

## 前端页面对应关系

| 页面路由 | 组件位置 |
|------|------|
| `Route::AstroNatal` | `pages/mod.rs#AstroNatal` |
| `Route::Bazi` | `pages/mod.rs#Bazi` |
| `Route::Ziwei` | `pages/mod.rs#Ziwei` |
| `Route::GuoLao` | `pages/mod.rs#GuoLao` |
| `Route::Sanshi` | `pages/mod.rs#Sanshi` |
| `Route::AstroVedic` | `pages/mod.rs#AstroVedic` |
| `Route::AstroTiming` | `pages/mod.rs#AstroTiming` |
| `Route::AstroRelationship` | `pages/mod.rs#AstroRelationship` |
| `Route::ShuSuan` | `pages/mod.rs#ShuSuan` |
| `Route::Liuren` | `pages/mod.rs#Liuren` |
| `Route::Liuyao` | `pages/mod.rs#Liuyao` |
| `Route::DunJia` | `pages/mod.rs#DunJia` |
| `Route::Taiyi` | `pages/mod.rs#Taiyi` |
| `Route::FengShui` | `pages/mod.rs#FengShui` |
| `Route::Almanac` | `pages/mod.rs#Almanac` |
| `Route::Planetarium` | `pages/mod.rs#Planetarium` |
| `Route::AiAnalysis` | `pages/mod.rs#AiAnalysis` |

## Divines Skill 68 个工具对照

本项目的 Rust 实现已覆盖 divines-skill 中的所有 68 个工具：

### 西洋占星 (已实现)
- chart, chart13, hellen_chart, guolao_chart, india_chart, relative
- solarreturn, lunarreturn, solararc, givenyear, profection
- pd, pdchart, zr, firdaria, decennials, agepoint, distributions, mundane
- harmonic, germany, otherbu, suzhan

### 中文术数 (已实现)
- bazi_birth, bazi_direct, ziwei_birth, ziwei_rules
- qimen, taiyi, liureng_gods, liureng_runyear, jinkou
- sanshiunited, tongshefa, canping, heluo, sixyao

### 14 路神数 (已实现于 remaining.rs)
- huangji, wuzhao, taixuan, jingjue, shenyishu, shaozi, tieban
- fendjing, beiji, nanji, chunzi, cetian, xianqin, planetarium

### 工具类 (已实现)
- knowledge_registry, knowledge_read, export_registry, export_parse
- gua_desc, gua_meiyi, jieqi_year, nongli_time

## 开发检查清单

修改代码前检查：
- [ ] 确认对应源仓库文件路径
- [ ] 阅读源仓库原始实现
- [ ] 确认 Rust 版本与源仓库算法一致
- [ ] 更新对应的 API 端点 (divines-server)
- [ ] 更新对应的前端页面 (divines-web)
- [ ] 运行 `cargo build --workspace` 验证编译
- [ ] 运行 `cargo test --workspace` 验证测试

## CSS 设计系统

使用 `--Divines-*` CSS 变量体系，完全对齐源仓库的 `app.less`：
- 颜色：背景/表面/文字/边框/强调色/金色/青色/八字色板
- 布局：导航宽度 206px, 工具栏高度 72px, 底部 64px
- 主题：`[data-Divines-appearance='dark']` 和 `[data-Divines-appearance='light']`