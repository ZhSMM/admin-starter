//! 认证 & 权限
//!
//! - `auth_middleware`: 解析 `Authorization: Bearer <token>`,把 `CurrentUser` 注入到 request extensions
//! - `CurrentUser`: 在 handler 里通过 extractor 直接拿当前用户
//! - `check_permission`: 在 handler 里显式校验权限 code

use std::collections::HashSet;

use axum::{
    async_trait,
    body::Body,
    extract::{FromRequestParts, State},
    http::{header, request::Parts, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::Serialize;

use crate::error::AppError;
use crate::state::AppState;

/// 解析后的当前用户信息。
#[derive(Debug, Clone, Serialize)]
pub struct CurrentUser {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    /// 扁平化的权限 code 集合,前端按需组装树
    pub permissions: HashSet<String>,
    /// 超级管理员(拥有 code='admin' 的角色)直接放行
    pub is_super: bool,
}

/// 中间件:解析 JWT,把 CurrentUser 注入到 extensions。
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.trim().to_string());

    let Some(token) = token else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let claims = state.jwt.verify(&token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user = load_user_with_permissions(&state, claims.sub)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(user) = user else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

async fn load_user_with_permissions(
    state: &AppState,
    user_id: i64,
) -> Result<Option<CurrentUser>, AppError> {
    use sqlx::Row;

    let user_row = sqlx::query("SELECT id, username, nickname, status FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(&state.db)
        .await?;

    let Some(user_row) = user_row else { return Ok(None) };
    let id: i64 = user_row.try_get("id").unwrap_or(0);
    let username: String = user_row.try_get("username").unwrap_or_default();
    let nickname: String = user_row.try_get("nickname").unwrap_or_default();
    let status: i64 = user_row.try_get("status").unwrap_or(0);
    if status == 0 {
        return Ok(None);
    }

    let perm_codes: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT DISTINCT p.code
        FROM permissions p
        INNER JOIN role_permissions rp ON rp.permission_id = p.id
        INNER JOIN user_roles ur ON ur.role_id = rp.role_id
        WHERE ur.user_id = ? AND p.status = 1
        "#,
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    // 是否超级管理员(SQLite 的 EXISTS 返回 0/1)
    let is_super_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM user_roles ur
        INNER JOIN roles r ON r.id = ur.role_id
        WHERE ur.user_id = ? AND r.code = 'admin' AND r.status = 1
        "#,
    )
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    Ok(Some(CurrentUser {
        id,
        username,
        nickname,
        permissions: perm_codes.into_iter().collect(),
        is_super: is_super_count > 0,
    }))
}

/// 在 handler 里直接拿当前用户。
/// axum 0.7 使用 `#[async_trait]`,所以这里也加宏对齐。
#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<CurrentUser>()
            .cloned()
            .ok_or(AppError::Unauthorized)
    }
}

/// 显式权限校验,放在需要保护的 handler 内部。
pub fn check_permission(user: &CurrentUser, code: &str) -> Result<(), AppError> {
    if user.is_super {
        return Ok(());
    }
    if user.permissions.contains(code) {
        Ok(())
    } else {
        Err(AppError::Forbidden(format!("缺少权限: {code}")))
    }
}

/// 同时校验多个 code,任一通过即可(or 语义)。
pub fn check_any_permission(user: &CurrentUser, codes: &[&str]) -> Result<(), AppError> {
    if user.is_super {
        return Ok(());
    }
    if codes.iter().any(|c| user.permissions.contains(*c)) {
        Ok(())
    } else {
        Err(AppError::Forbidden("权限不足".into()))
    }
}
