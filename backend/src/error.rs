//! 统一错误类型
//!
//! 设计原则:
//! - `AppError` 是业务错误,所有 handler 都返回 `Result<T, AppError>`
//! - 自动实现 `IntoResponse`,把错误转成统一格式的 JSON
//! - 业务错误(404、400、401、403)返回对应 HTTP 状态码
//! - 内部错误(数据库、IO)统一 500,不暴露细节

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // ===== 业务错误 =====
    #[error("资源未找到: {0}")]
    NotFound(String),

    #[error("参数错误: {0}")]
    BadRequest(String),

    #[error("未授权")]
    Unauthorized,

    #[error("禁止访问: {0}")]
    Forbidden(String),

    #[error("冲突: {0}")]
    Conflict(String),

    // ===== 基础设施错误 =====
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("密码哈希错误: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("JWT 错误: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("内部错误: {0}")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    fn status_and_code(&self) -> (StatusCode, &'static str) {
        match self {
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "bad_request"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            AppError::Forbidden(_) => (StatusCode::FORBIDDEN, "forbidden"),
            AppError::Conflict(_) => (StatusCode::CONFLICT, "conflict"),
            AppError::Database(_)
            | AppError::Bcrypt(_)
            | AppError::Jwt(_)
            | AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = self.status_and_code();

        // 业务错误把消息原样返回;基础设施错误只记日志、不暴露给前端
        let message = match &self {
            AppError::Database(e) => {
                tracing::error!("DB error: {e}");
                "数据库错误".to_string()
            }
            AppError::Bcrypt(e) => {
                tracing::error!("bcrypt error: {e}");
                "密码处理错误".to_string()
            }
            AppError::Jwt(e) => {
                tracing::warn!("jwt error: {e}");
                "令牌无效".to_string()
            }
            AppError::Internal(e) => {
                tracing::error!("internal error: {e:?}");
                "服务器内部错误".to_string()
            }
            other => other.to_string(),
        };

        // 开发环境可以把 internal 错误的详细链路打出来
        if status == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::debug!("stack: {self:?}");
        }

        let body = Json(json!({
            "code": code,
            "message": message,
        }));
        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
