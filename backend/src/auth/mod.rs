//! 认证模块
//!
//! - `password`: 密码 hash/verify
//! - `jwt`: 签发与校验 token
//! - `middleware`: 从 `Authorization: Bearer <token>` 中解析用户身份
//! - `extractor`: 在 handler 里直接拿当前用户

pub mod jwt;
pub mod middleware;
pub mod password;

pub use jwt::{Claims, JwtService};
pub use middleware::{check_permission, CurrentUser};
