//! 角色业务

use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::models::role::{CreateRole, RoleDto, UpdateRole};
use crate::repository::role::RoleRepo;
use crate::state::AppState;

pub struct RoleService;

impl RoleService {
    pub async fn list(state: &AppState) -> AppResult<Vec<RoleDto>> {
        let roles = RoleRepo::list(&state.db).await?;
        let mut out = Vec::with_capacity(roles.len());
        for r in roles {
            let pids = RoleRepo::get_permission_ids(&state.db, r.id).await?;
            out.push(RoleDto::from(r, pids));
        }
        Ok(out)
    }

    pub async fn detail(state: &AppState, id: i64) -> AppResult<RoleDto> {
        let role = RoleRepo::find_by_id(&state.db, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("角色 {id} 不存在")))?;
        let pids = RoleRepo::get_permission_ids(&state.db, id).await?;
        Ok(RoleDto::from(role, pids))
    }

    pub async fn create(state: &AppState, req: CreateRole) -> AppResult<RoleDto> {
        req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
        crate::models::role::validate_role_code(&req.code)
            .map_err(AppError::BadRequest)?;
        let role = RoleRepo::create(
            &state.db,
            &req.name,
            &req.code,
            req.description.as_deref().unwrap_or(""),
            req.permission_ids.as_deref().unwrap_or(&[]),
        )
        .await?;
        let pids = RoleRepo::get_permission_ids(&state.db, role.id).await?;
        Ok(RoleDto::from(role, pids))
    }

    pub async fn update(state: &AppState, id: i64, req: UpdateRole) -> AppResult<RoleDto> {
        req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
        let role = RoleRepo::update(
            &state.db,
            id,
            &req,
            req.permission_ids.as_deref(),
        )
        .await?;
        let pids = RoleRepo::get_permission_ids(&state.db, id).await?;
        Ok(RoleDto::from(role, pids))
    }

    pub async fn delete(state: &AppState, id: i64) -> AppResult<()> {
        RoleRepo::delete(&state.db, id).await
    }
}
