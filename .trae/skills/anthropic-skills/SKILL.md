---
name: "anthropic-skills"
description: "Anthropic 官方技能集，来自 anthropics/skills。包含算法艺术、品牌指南、画布设计、文档生成、前端设计、内部沟通、MCP 服务器开发、PDF 处理、Skill 创建、Slack GIF 等技能。当进行设计、文档、MCP 开发或创意工作时自动触发。"
---

# Anthropic 官方技能集

基于 [anthropics/skills](https://github.com/anthropics/skills)，提供 15+ 个官方技能的最佳实践。

## 技能目录

### 设计与创意
| 技能 | 用途 | 触发关键词 |
|------|------|------|
| algorithmic-art | 生成算法艺术 (p5.js) | 生成艺术、算法艺术、p5.js |
| brand-guidelines | 品牌一致性和颜色管理 | 品牌、配色、logo |
| canvas-design | 创建视觉设计 (海报/PNG/PDF) | 设计、海报、视觉设计 |
| frontend-design | 构建高质量前端界面 | 前端、UI、界面 |
| slack-gif | 创建 Slack 优化的 GIF | GIF、Slack、动图 |

### 文档与开发
| 技能 | 用途 | 触发关键词 |
|------|------|------|
| doc-coauthoring | 协作编写文档/提案 | 写文档、提案、技术文档 |
| internal-comms | 内部沟通（状态报告、更新） | 状态报告、周报、更新 |
| mcp-builder | 构建 MCP 服务器 | MCP、服务器、工具 |
| pdf | PDF 处理（提取、创建、合并） | PDF、合并、提取 |
| skill-creator | 创建新技能 | 创建技能、写技能 |
| web-artifacts-builder | 构建复杂 Web 工件 | Web 工件、React |

### 开发工具
| 技能 | 用途 | 触发关键词 |
|------|------|------|
| xlsx | 电子表格处理 | Excel、表格、xlsx |
| pptx | 演示文稿处理 | PPT、演示、幻灯片 |
| docx | Word 文档处理 | Word、文档、docx |

## 本项目最相关的技能

### MCP Builder
构建 MCP (Model Context Protocol) 服务器的完整指南：
- 工具定义 (JSON Schema)
- 资源暴露
- 提示模板
- 传输层 (stdio/SSE)

### Frontend Design
高质量前端设计原则：
- 避免通用 AI 美学（过度使用的字体、图标、配色）
- 使用独特的字体和配色方案
- 关注视觉层次和信息密度
- 响应式设计优先

### Skill Creator
创建高质量技能的标准：
- 清晰的名称和描述
- 结构化的指令
- 明确的触发条件
- 包含示例和反模式

## 使用方式

这些技能在 Claude 中以 `@skill-name` 方式调用。在 Trae 中，对应的技能通过 `.trae/skills/` 目录下的 SKILL.md 文件实现。