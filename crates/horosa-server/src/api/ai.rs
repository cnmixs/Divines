// 星阙 Horosa - AI 分析 API
// 参考原项目: astrostudysrv/astrostudy/ AIAnalysisController.java

use axum::{extract::State, Json};
use std::sync::Arc;
use crate::AppState;
use horosa_core::*;

pub async fn chat(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<AiChatRequest>,
) -> Json<AiChatResponse> {
    // TODO: 实现与 OpenAI / Anthropic / Gemini / Ollama 等 API 的集成
    Json(AiChatResponse {
        message: AiMessage {
            role: "assistant".to_string(),
            content: "AI 分析功能开发中，请期待后续版本。".to_string(),
        },
        usage: None,
    })
}

pub async fn models(
    State(_state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "models": [
            { "provider": "openai", "models": ["gpt-4o", "gpt-4o-mini"] },
            { "provider": "anthropic", "models": ["claude-3-opus", "claude-3-sonnet"] },
            { "provider": "gemini", "models": ["gemini-2.5-pro", "gemini-2.5-flash"] },
            { "provider": "ollama", "models": ["llama3", "mistral", "qwen2"] },
            { "provider": "openrouter", "models": ["openai/gpt-4o", "anthropic/claude-3-opus"] }
        ]
    }))
}