//! 路由聚合
//!
//! 规则:
//! - 公开接口(登录)在 `/api/auth/login`
//! - 受保护接口挂在 `/api/...` 下,统一经过 `auth_middleware`

use axum::{
    middleware::from_fn_with_state,
    routing::{get, post, put},
    Router,
};

use crate::auth::middleware::auth_middleware;
use crate::handler::{
    auth_handler, permission_handler, role_handler, user_handler,
};
use crate::state::AppState;

pub fn build(state: AppState) -> Router {
    // 公开路由
    let public = Router::new()
        .route("/api/auth/login", post(auth_handler::login));

    // 需要登录的路由
    let protected = Router::new()
        // auth
        .route("/api/auth/me", get(auth_handler::me))
        .route("/api/auth/profile", get(auth_handler::profile))
        .route("/api/auth/change-password", post(auth_handler::change_password))
        .route("/api/auth/secret-ping", get(auth_handler::secret_ping))
        // user
        .route("/api/users", get(user_handler::page).post(user_handler::create))
        .route(
            "/api/users/:id",
            get(user_handler::detail).put(user_handler::update).delete(user_handler::delete),
        )
        // role
        .route("/api/roles", get(role_handler::list).post(role_handler::create))
        .route(
            "/api/roles/:id",
            get(role_handler::detail)
                .put(role_handler::update)
                .delete(role_handler::delete),
        )
        // permission
        .route(
            "/api/permissions",
            get(permission_handler::tree).post(permission_handler::create),
        )
        .route(
            "/api/permissions/:id",
            put(permission_handler::update).delete(permission_handler::delete),
        )
        // 全局中间件:所有 /api/* 受保护接口都要先过 JWT
        .route_layer(from_fn_with_state(state.clone(), auth_middleware));

    public.merge(protected).with_state(state)
}
