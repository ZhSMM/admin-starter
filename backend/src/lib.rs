//! 后端 lib 入口
//!
//! 把模块声明放到 lib.rs,这样 `main.rs` 只需要 `use admin_starter_backend::*;`
//! 后续如果想加集成测试或子二进制,可以直接复用 lib。

pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod handler;
pub mod models;
pub mod repository;
pub mod routes;
pub mod service;
pub mod state;
