// 星阙 Horosa - AI 分析相关类型
// 参考原项目: astrostudyui/src/components/aianalysis/, astrostudysrv/.../AIAnalysisController.java

use serde::{Deserialize, Serialize};

/// AI 提供商
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiProvider {
    OpenAI,
    Anthropic,
    Gemini,
    Ollama,
    OpenRouter,
    Custom,
}

/// AI 模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiModelConfig {
    pub provider: AiProvider,
    pub model_name: String,
    pub api_key: String,
    pub api_base: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f64>,
}

/// AI 消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessage {
    pub role: String,  // "user" | "assistant" | "system"
    pub content: String,
}

/// AI 对话请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatRequest {
    pub messages: Vec<AiMessage>,
    pub context: Option<String>,
    pub chart_data: Option<serde_json::Value>,
}

/// AI 对话响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatResponse {
    pub message: AiMessage,
    pub usage: Option<AiUsage>,
}

/// AI 用量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// AI 对话历史
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConversation {
    pub id: String,
    pub title: String,
    pub messages: Vec<AiMessage>,
    pub model: String,
    pub created_at: String,
    pub updated_at: String,
}

/// AI 导出模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiExportTemplate {
    pub name: String,
    pub content: String,
    pub variables: Vec<String>,
}