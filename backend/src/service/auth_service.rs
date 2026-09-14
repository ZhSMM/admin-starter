//! 认证业务

use validator::Validate;

use crate::auth::password::ensure_password;
use crate::error::{AppError, AppResult};
use crate::models::user::{LoginReq, LoginResp, UserDto};
use crate::repository::user::UserRepo;
use crate::state::AppState;

pub struct AuthService;

impl AuthService {
    pub async fn login(state: &AppState, req: LoginReq) -> AppResult<LoginResp> {
        req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;

        let user = UserRepo::find_by_username(&state.db, &req.username)
            .await?
            .ok_or(AppError::Unauthorized)?;

        if user.status == 0 {
            return Err(AppError::Forbidden("账号已禁用".into()));
        }

        ensure_password(&req.password, &user.password_hash)?;

        let token = state.jwt.issue(user.id, &user.username)?;
        let role_ids = UserRepo::get_role_ids(&state.db, user.id).await?;
        Ok(LoginResp {
            token,
            user: UserDto::from(user, role_ids),
        })
    }

    /// 改自己密码
    pub async fn change_password(
        state: &AppState,
        user_id: i64,
        old: &str,
        new: &str,
    ) -> AppResult<()> {
        let user = UserRepo::find_by_id(&state.db, user_id)
            .await?
            .ok_or(AppError::Unauthorized)?;
        ensure_password(old, &user.password_hash)?;

        let new_hash = crate::auth::password::hash_password(new, state.config.bcrypt_cost)?;
        sqlx::query("UPDATE users SET password_hash = ?, updated_at = datetime('now') WHERE id = ?")
            .bind(&new_hash)
            .bind(user_id)
            .execute(&state.db)
            .await?;
        Ok(())
    }
}
