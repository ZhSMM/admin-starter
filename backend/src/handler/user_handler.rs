//! 用户管理 HTTP 接口

use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::auth::middleware::{check_permission, CurrentUser};
use crate::error::AppResult;
use crate::handler::ApiResp;
use crate::models::user::{CreateUser, PageQuery, PageResp, UpdateUser, UserDto};
use crate::service::user_service::UserService;
use crate::state::AppState;

pub async fn page(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(q): Query<PageQuery>,
) -> AppResult<Json<ApiResp<PageResp<UserDto>>>> {
    check_permission(&user, "system:user:list")?;
    let data = UserService::page(&state, &q).await?;
    Ok(ApiResp::ok(data))
}

pub async fn detail(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResp<UserDto>>> {
    check_permission(&user, "system:user:list")?;
    let data = UserService::detail(&state, id).await?;
    Ok(ApiResp::ok(data))
}

pub async fn create(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<CreateUser>,
) -> AppResult<Json<ApiResp<UserDto>>> {
    check_permission(&user, "system:user:create")?;
    let data = UserService::create(&state, req).await?;
    Ok(ApiResp::ok(data))
}

pub async fn update(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i64>,
    Json(req): Json<UpdateUser>,
) -> AppResult<Json<ApiResp<UserDto>>> {
    check_permission(&user, "system:user:update")?;
    let data = UserService::update(&state, id, req).await?;
    Ok(ApiResp::ok(data))
}

pub async fn delete(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i64>,
) -> AppResult<Json<ApiResp<()>>> {
    check_permission(&user, "system:user:delete")?;
    UserService::delete(&state, id).await?;
    Ok(ApiResp::ok(()))
}
