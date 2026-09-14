//! 数据访问层
//!
//! 每个实体一个文件,所有 SQL 都集中在这里,service / handler 不直接写 SQL。
//! 这样换数据库 / 优化 SQL 时影响面可控。

pub mod permission;
pub mod role;
pub mod user;
