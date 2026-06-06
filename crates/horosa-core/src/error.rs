// 星阙 Horosa - 错误类型定义
// 参考原项目: Horosa-Web/astrostudyui/src/msg/errmsg.js

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HorosaError {
    #[error("计算错误: {0}")]
    CalcError(String),

    #[error("参数错误: {0}")]
    ParamError(String),

    #[error("数据未找到: {0}")]
    NotFound(String),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("序列化错误: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("日期时间错误: {0}")]
    DateTimeError(String),

    #[error("历法错误: {0}")]
    CalendarError(String),

    #[error("星历表错误: {0}")]
    EphemerisError(String),

    #[error("认证错误: {0}")]
    AuthError(String),

    #[error("网络错误: {0}")]
    NetworkError(String),

    #[error("内部错误: {0}")]
    InternalError(String),
}

pub type HorosaResult<T> = Result<T, HorosaError>;