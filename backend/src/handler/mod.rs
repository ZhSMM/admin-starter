//! HTTP 处理器
//!
//! 每个业务模块一个文件。handler 只做:参数 → service → JSON。
//! 鉴权通过 `CurrentUser` extractor;权限 code 在 handler 内 `check_permission`。

pub mod auth_handler;
pub mod permission_handler;
pub mod role_handler;
pub mod user_handler;

use axum::Json;
use serde::Serialize;

/// 统一响应壳
#[derive(Debug, Serialize)]
pub struct ApiResp<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResp<T> {
    pub fn ok(data: T) -> Json<Self> {
        Json(Self { code: 0, message: "ok".into(), data: Some(data) })
    }
    pub fn _raw(data: T) -> Self {
        Self { code: 0, message: "ok".into(), data: Some(data) }
    }
}
