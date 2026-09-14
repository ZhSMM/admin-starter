//! 后端启动入口

use std::sync::Arc;
use std::time::Duration;

use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use admin_starter_backend::{
    auth::JwtService,
    config::Config,
    db,
    routes,
    state::AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // 2. 配置
    let cfg = Config::from_env()?;
    tracing::info!("配置加载完成,server_addr = {}", cfg.server_addr);

    // 3. 数据库
    let pool = db::init_pool(&cfg).await?;
    tracing::info!("数据库初始化完成");

    // 4. 状态
    let state = AppState {
        db: pool,
        jwt: JwtService::new(&cfg),
        config: Arc::new(cfg.clone()),
    };

    // 5. CORS:开发期放开,生产请改白名单
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(Duration::from_secs(3600));

    // 6. 路由
    let app = routes::build(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // 7. 启动
    let listener = tokio::net::TcpListener::bind(&cfg.server_addr).await?;
    tracing::info!("🚀 listening on http://{}", cfg.server_addr);
    axum::serve(listener, app).await?;
    Ok(())
}
