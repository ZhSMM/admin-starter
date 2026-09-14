//! 用户业务

use validator::Validate;

use crate::auth::password::hash_password;
use crate::error::{AppError, AppResult};
use crate::models::user::{CreateUser, PageQuery, PageResp, UpdateUser, UserDto};
use crate::repository::user::UserRepo;
use crate::state::AppState;

pub struct UserService;

impl UserService {
    pub async fn page(
        state: &AppState,
        query: &PageQuery,
    ) -> AppResult<PageResp<UserDto>> {
        let (page, page_size) = query.normalize();
        let total = UserRepo::count(&state.db, query.keyword.as_deref()).await?;
        let users = UserRepo::list(
            &state.db,
            query.keyword.as_deref(),
            query.offset(),
            page_size,
        )
        .await?;
        let mut list = Vec::with_capacity(users.len());
        for u in users {
            let role_ids = UserRepo::get_role_ids(&state.db, u.id).await?;
            list.push(UserDto::from(u, role_ids));
        }
        Ok(PageResp { list, total, page, page_size })
    }

    pub async fn create(state: &AppState, req: CreateUser) -> AppResult<UserDto> {
        req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
        let password_hash = hash_password(&req.password, state.config.bcrypt_cost)?;
        let user = UserRepo::create(
            &state.db,
            &req.username,
            &password_hash,
            req.nickname.as_deref().unwrap_or(""),
            req.email.as_deref().unwrap_or(""),
        )
        .await?;
        if let Some(roles) = req.role_ids {
            UserRepo::set_role_ids(&state.db, user.id, &roles).await?;
        }
        let role_ids = UserRepo::get_role_ids(&state.db, user.id).await?;
        Ok(UserDto::from(user, role_ids))
    }

    pub async fn update(state: &AppState, id: i64, req: UpdateUser) -> AppResult<UserDto> {
        req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
        let new_hash = match req.password.as_deref() {
            Some(p) if !p.is_empty() => Some(hash_password(p, state.config.bcrypt_cost)?),
            _ => None,
        };
        let user = UserRepo::update(&state.db, id, &req, new_hash.as_deref()).await?;
        if let Some(roles) = req.role_ids.as_ref() {
            UserRepo::set_role_ids(&state.db, id, roles).await?;
        }
        let role_ids = UserRepo::get_role_ids(&state.db, id).await?;
        Ok(UserDto::from(user, role_ids))
    }

    pub async fn delete(state: &AppState, id: i64) -> AppResult<()> {
        if id == 1 {
            return Err(AppError::BadRequest("不能删除超级管理员".into()));
        }
        UserRepo::delete(&state.db, id).await
    }

    pub async fn detail(state: &AppState, id: i64) -> AppResult<UserDto> {
        let user = UserRepo::find_by_id(&state.db, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("用户 {id} 不存在")))?;
        let role_ids = UserRepo::get_role_ids(&state.db, id).await?;
        Ok(UserDto::from(user, role_ids))
    }
}
