//! 权限管理 HTTP 接口

use axum::{
    extract::{Path, State},
    Json,
};

use crate::auth::middleware::{check_permission, CurrentUser};
use crate::error::AppResult;
use crate::handler::ApiResp;
use crate::models::permission::{CreatePermission, PermissionNode, UpdatePermission};
use crate::service::permission_service::PermissionService;
use crate::state::AppState;

pub async fn tree(
    State(state): State<AppState>,
    user: CurrentUser,
) -> AppResult<Json<ApiResp<Vec<PermissionNode>>>> {
    check_permission(&user, "system:permission:list")?;
    let data = PermissionService::tree(&state).await?;
    Ok(ApiResp::ok(data))
}

pub async fn create(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<CreatePermission>,
) -> AppResult<Json<ApiResp<PermissionNode>>> {
    check_permission(&user, "system:permission:create")?;
    let data = PermissionService::create(&state, req).await?;
    Ok(ApiResp::ok(data))
}

pub async fn update(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePermission>,
) -> AppResult<Json<ApiResp<PermissionNode>>> {
    check_permission(&user, "system:permission:update")?;
    let data = PermissionService::update(&state, id, req).await?;
    Ok(ApiResp::ok(data))
}

pub async fn delete(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResp<()>>> {
    check_permission(&user, "system:permission:delete")?;
    PermissionService::delete(&state, id).await?;
    Ok(ApiResp::ok(()))
}
