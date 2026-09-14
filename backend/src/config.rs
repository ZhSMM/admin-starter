//! 全局配置
//!
//! 从 `.env` 加载,通过 `AppState` 注入到每个 handler。
//! 所有配置集中在一处,新增配置项只需:
/*!
1. 在 `Config` 里加字段
2. 在 `.env.example` 同步
*/

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_addr: String,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expire_hours: i64,
    pub bcrypt_cost: u32,
}

impl Config {
    /// 从环境变量加载配置,启动期 fail-fast。
    pub fn from_env() -> anyhow::Result<Self> {
        // 加载 .env(若不存在也不报错)
        let _ = dotenvy::dotenv();

        Ok(Self {
            server_addr: env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into()),
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://data/admin.db".into()),
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| anyhow::anyhow!("JWT_SECRET 未设置"))?,
            jwt_expire_hours: env::var("JWT_EXPIRE_HOURS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(24),
            bcrypt_cost: env::var("BCRYPT_COST")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
        })
    }
}
