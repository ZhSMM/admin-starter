//! 认证相关 HTTP 接口

use axum::{extract::State, Json};

use crate::auth::middleware::{check_permission, CurrentUser};
use crate::error::AppResult;
use crate::handler::ApiResp;
use crate::models::user::{ChangePassword, LoginReq};
use crate::service::auth_service::AuthService;
use crate::state::AppState;

/// POST /api/auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> AppResult<Json<ApiResp<crate::models::user::LoginResp>>> {
    let resp = AuthService::login(&state, req).await?;
    Ok(ApiResp::ok(resp))
}

/// GET /api/auth/me
pub async fn me(
    State(state): State<AppState>,
    user: CurrentUser,
) -> AppResult<Json<ApiResp<serde_json::Value>>> {
    let menus = crate::service::permission_service::PermissionService::my_menus(&state, user.id).await?;
    let permissions = crate::service::permission_service::PermissionService::my_permissions(&state, user.id).await?;
    let payload = serde_json::json!({
        "user": {
            "id": user.id,
            "username": user.username,
            "nickname": user.nickname,
            "is_super": user.is_super,
        },
        "permissions": permissions,
        "menus": menus,
    });
    Ok(ApiResp::ok(payload))
}

/// POST /api/auth/change-password
pub async fn change_password(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<ChangePassword>,
) -> AppResult<Json<ApiResp<()>>> {
    AuthService::change_password(&state, user.id, &req.old_password, &req.new_password).await?;
    Ok(ApiResp::ok(()))
}

/// GET /api/auth/profile
pub async fn profile(
    State(_state): State<AppState>,
    user: CurrentUser,
) -> AppResult<Json<ApiResp<serde_json::Value>>> {
    Ok(ApiResp::ok(serde_json::json!({
        "id": user.id,
        "username": user.username,
        "nickname": user.nickname,
        "is_super": user.is_super,
    })))
}

// 一个示例:需要权限的 handler
pub async fn secret_ping(user: CurrentUser) -> AppResult<Json<ApiResp<&'static str>>> {
    check_permission(&user, "system:user:create")?;
    Ok(ApiResp::ok("pong"))
}
