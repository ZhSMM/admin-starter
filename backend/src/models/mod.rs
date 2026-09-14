//! 数据模型
//!
//! 三类结构体放在一个文件里:
//! - `Xxx`: 数据库实体的直接映射(sqlx::FromRow)
//! - `XxxDto`: 对外响应结构(可隐藏敏感字段,如 password_hash)
//! - `CreateXxx / UpdateXxx`: 入参结构(带 validator 校验)
//!
//! 这样契约清晰:数据库结构、内部传输、API 输入输出互不污染。

pub mod permission;
pub mod role;
pub mod user;
