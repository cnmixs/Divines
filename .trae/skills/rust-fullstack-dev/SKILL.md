---
name: "rust-fullstack-dev"
description: "Rust 全栈开发专用技能，涵盖 Dioxus 0.7.9 前端、Axum 后端、Cargo Workspace 组织、构建优化。当用户编写 Rust 代码、调试 Dioxus 组件、配置 Axum 路由或操作 Cargo 工作区时自动触发。"
---

# Rust 全栈开发技能

基于 [anthropics/skills](https://github.com/anthropics/skills) 和 [vercel-labs/agent-skills](https://github.com/vercel-labs/agent-skills) 的最佳实践，为 Rust 全栈项目提供开发指导。

## 项目架构约定

本项目使用 Cargo Workspace 管理多 crate：

```
crates/
├── horosa-core/    # 核心类型定义 (serde, chrono)
├── horosa-calc/    # 计算引擎 (占星/八字/紫微/三式等)
├── horosa-server/  # Axum 后端 (API 路由, WebSocket)
└── horosa-web/     # Dioxus 前端 (路由, 组件, 样式)
```

## 前端开发规范 (Dioxus 0.7.9)

### 路由定义
- 使用 `#[derive(Routable, Clone)]` 定义路由枚举
- 路由路径使用 kebab-case
- 每个路由对应 `pages/mod.rs` 中的一个组件函数

### 组件开发
- 使用 `#[component]` 宏定义组件
- 使用 `use_signal` 管理局部状态
- 使用 `rsx! {}` 宏编写 JSX 风格模板
- 组件函数返回 `Element`

### 样式
- 全局样式在 `assets/styles.css` 中定义
- 使用 CSS 变量 `--horosa-*` 管理主题
- 支持亮色/暗色主题通过 `[data-horosa-appearance]` 切换

## 后端开发规范 (Axum)

### 路由组织
- 按功能模块拆分路由文件 (`api/astro.rs`, `api/bazi.rs` 等)
- 使用 `Router::new().merge()` 组合路由
- 所有路由前缀 `/api/`

### 请求处理
- 使用 `Json<T>` 提取 JSON 请求体
- 使用 `axum::extract::Query<T>` 提取查询参数
- 错误处理使用 `anyhow::Result`

## 计算引擎规范

- 所有计算函数返回 `Result<T, CalcError>`
- 使用 `horosa_core::chart::*` 中的共享类型
- 算法参考源仓库 (horosa-original) 中的 JavaScript 实现
- 关键算法添加单元测试

## Cargo 构建

```bash
# 构建所有 crate
cargo build --workspace

# 仅构建前端 (Dioxus 需要 Web 目标)
cargo build -p horosa-web

# 运行测试
cargo test --workspace

# 代码检查
cargo clippy --workspace
```