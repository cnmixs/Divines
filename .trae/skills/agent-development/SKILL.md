---
name: "agent-development"
description: "Agent 开发最佳实践，参考 vercel-labs/agent-skills 和 ComposioHQ/awesome-claude-skills。当用户需要设计 AI Agent、构建 MCP 工具、编写 Agent 指令或优化 Agent 工作流时触发。"
---

# Agent 开发技能

基于 [vercel-labs/agent-skills](https://github.com/vercel-labs/agent-skills) 和 [ComposioHQ/awesome-claude-skills](https://github.com/ComposioHQ/awesome-claude-skills) 的最佳实践。

## Agent 设计原则

### 1. 单一职责
每个 Agent 或工具只做一件事，做好一件事。

### 2. 明确输入输出
- 输入：定义清晰的 JSON Schema
- 输出：返回结构化数据（JSON），附带可读的文本描述

### 3. 错误处理
- 所有错误返回结构化错误信息
- 包含 `error_code` 和 `error_message`
- 提供用户可操作的修复建议

## MCP (Model Context Protocol) 工具设计

### 工具命名规范
- 使用 `namespace_verb_noun` 格式
- 例如：`horosa_astro_chart`, `horosa_cn_qimen`

### 输入契约
- 必填字段使用 `required` 标记
- 可选字段提供合理的 `default` 值
- 时间字段统一使用 ISO 8601 格式

### 输出契约
- 统一 envelope 格式：
```json
{
  "ok": true,
  "tool": "tool_name",
  "run_id": "uuid",
  "export_snapshot": {},
  "export_format": {},
  "summary": ""
}
```

## Horosa Skill 集成参考

参考 [horosa-skill](https://github.com/Horace-Maxwell/horosa-skill) 的 68 个工具设计：

- **西洋占星**：chart, solarreturn, lunarreturn, profection, pd, zr, firdaria 等
- **中文术数**：bazi_birth, ziwei_birth, qimen, liureng_gods, jinkou 等
- **14 路神数**：huangji, wuzhao, taixuan, shaozi, tieban 等

### 参数确认规则
- 当工具会受时间、地点、性别、宫制等设置影响时，必须先确认参数再调用
- 使用 `must_ask_user=true` 标记需要确认的参数
- 提供结构化追问文本

## 本项目的 MCP 集成方案

本项目 (Horosa Rust 重写) 的 MCP 工具可以通过以下方式暴露：

1. **Axum 端点** → 标准 HTTP API → 可被 MCP 网关调用
2. **WebSocket** → 实时通信 → 适用于流式输出
3. **CLI** → 命令行工具 → 适用于脚本集成

## 开发检查清单

- [ ] 工具定义完整的 JSON Schema
- [ ] 输入校验覆盖所有必填字段
- [ ] 输出遵循统一 envelope 格式
- [ ] 错误信息包含修复建议
- [ ] 添加 agent 使用文档