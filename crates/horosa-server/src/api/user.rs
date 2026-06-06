// 星阙 Horosa - 用户 API
// 参考原项目: astrostudysrv/astrostudy/ UserController.java

use axum::{extract::State, Json};
use std::sync::Arc;
use crate::AppState;
use horosa_core::*;

pub async fn login(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Json<serde_json::Value> {
    // TODO: 实现完整的用户认证
    Json(serde_json::json!({
        "token": "mock-token",
        "user": {
            "id": uuid::Uuid::new_v4(),
            "username": req.username,
            "role": "user"
        }
    }))
}