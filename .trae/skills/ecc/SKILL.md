---
name: "ecc"
description: "ECC (Efficient Coding Companion) 编码规范，来自 affaan-m/ECC。涵盖跨平台 Agent 配置、代码规范、安全规则、Monorepo 管理等。当编写代码、配置项目规则或管理多模块项目时自动触发。"
---

# ECC (Efficient Coding Companion)

基于 [affaan-m/ECC](https://github.com/affaan-m/ECC)（182K+ stars，12+ 语言支持），提供跨平台的 Agent 编码规范。

## 核心原则

### 1. 代码规范
- **命名**：使用描述性名称，避免缩写（除非是通用约定）
- **函数**：单一职责，每个函数只做一件事
- **注释**：解释"为什么"而非"是什么"，代码本身应自解释
- **错误处理**：永远不要吞掉异常，使用结构化错误类型

### 2. 安全规则
- 永远不要在代码中硬编码密钥、令牌或密码
- 使用环境变量或安全的配置管理
- 输入验证：所有外部输入必须验证
- SQL 注入防护：使用参数化查询

### 3. Monorepo 管理
- 使用工作区管理多包项目
- 共享配置放在根目录
- 每个包有独立的版本和依赖
- 使用 changesets 管理版本发布

### 4. 跨平台支持
- 优先使用跨平台 API
- 路径使用 `std::path::Path` 而非字符串拼接
- 避免平台特定的 shell 命令
- 使用条件编译 `#[cfg(target_os = "...")]` 处理平台差异

### 5. Agent 配置
- `.agents/` 目录存放 Agent 专用配置
- 每个 Agent 有独立的规则文件
- 使用 YAML/Markdown 格式定义 Agent 行为

## 项目结构建议

```
project/
├── .agents/           # Agent 配置
├── .cursor/           # Cursor 规则
├── .github/           # GitHub Actions
├── src/               # 源代码
├── docs/              # 文档
├── tests/             # 测试
└── scripts/           # 构建脚本
```

## 开发检查清单

- [ ] 代码通过 lint 检查
- [ ] 所有测试通过
- [ ] 无硬编码密钥
- [ ] 输入已验证
- [ ] 错误处理完善
- [ ] 文档已更新
- [ ] 跨平台兼容