//! 角色管理 HTTP 接口

use axum::{
    extract::{Path, State},
    Json,
};

use crate::auth::middleware::{check_permission, CurrentUser};
use crate::error::AppResult;
use crate::handler::ApiResp;
use crate::models::role::{CreateRole, RoleDto, UpdateRole};
use crate::service::role_service::RoleService;
use crate::state::AppState;

pub async fn list(
    State(state): State<AppState>,
    user: CurrentUser,
) -> AppResult<Json<ApiResp<Vec<RoleDto>>>> {
    check_permission(&user, "system:role:list")?;
    let data = RoleService::list(&state).await?;
    Ok(ApiResp::ok(data))
}

pub async fn detail(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResp<RoleDto>>> {
    check_permission(&user, "system:role:list")?;
    let data = RoleService::detail(&state, id).await?;
    Ok(ApiResp::ok(data))
}

pub async fn create(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<CreateRole>,
) -> AppResult<Json<ApiResp<RoleDto>>> {
    check_permission(&user, "system:role:create")?;
    let data = RoleService::create(&state, req).await?;
    Ok(ApiResp::ok(data))
}

pub async fn update(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i64>,
    Json(req): Json<UpdateRole>,
) -> AppResult<Json<ApiResp<RoleDto>>> {
    check_permission(&user, "system:role:update")?;
    let data = RoleService::update(&state, id, req).await?;
    Ok(ApiResp::ok(data))
}

pub async fn delete(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResp<()>>> {
    check_permission(&user, "system:role:delete")?;
    RoleService::delete(&state, id).await?;
    Ok(ApiResp::ok(()))
}
