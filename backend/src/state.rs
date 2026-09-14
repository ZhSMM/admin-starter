//! 全局应用状态
//!
//! 注入到 axum 的 State,在每个 handler 里通过 `State<AppState>` 拿。
//! 这里只放无状态 / 可 clone 的资源:db 池、jwt 服务、配置。

use std::sync::Arc;

use crate::auth::JwtService;
use crate::config::Config;
use crate::db::Db;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub jwt: JwtService,
    pub config: Arc<Config>,
}
