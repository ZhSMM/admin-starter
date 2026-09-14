//! 业务层
//!
//! 业务层是 handler 和 repository 之间的中间层,负责:
//! - 组合多个 repository 调用
//! - 业务规则校验
//! - 事务边界
//!
//! handler 只做:拿参数 → 调 service → 包装响应,不做业务。

pub mod auth_service;
pub mod permission_service;
pub mod role_service;
pub mod user_service;
