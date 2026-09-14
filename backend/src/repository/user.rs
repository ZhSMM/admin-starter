//! 用户仓库
//!
//! 所有 SQL 都集中在这里,service 不直接写 SQL。

use sqlx::SqlitePool;

use crate::error::{AppError, AppResult};
use crate::models::user::{UpdateUser, User};

pub struct UserRepo;

impl UserRepo {
    pub async fn find_by_id(pool: &SqlitePool, id: i64) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, nickname, email, status, created_at, updated_at
             FROM users WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(user)
    }

    pub async fn find_by_username(pool: &SqlitePool, username: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, nickname, email, status, created_at, updated_at
             FROM users WHERE username = ?",
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;
        Ok(user)
    }

    pub async fn list(
        pool: &SqlitePool,
        keyword: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> AppResult<Vec<User>> {
        let kw = keyword.map(|s| format!("%{s}%")).unwrap_or_else(|| "%".to_string());
        let users = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, nickname, email, status, created_at, updated_at
             FROM users
             WHERE username LIKE ? OR nickname LIKE ? OR email LIKE ?
             ORDER BY id DESC
             LIMIT ? OFFSET ?",
        )
        .bind(&kw)
        .bind(&kw)
        .bind(&kw)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        Ok(users)
    }

    pub async fn count(pool: &SqlitePool, keyword: Option<&str>) -> AppResult<i64> {
        let kw = keyword.map(|s| format!("%{s}%")).unwrap_or_else(|| "%".to_string());
        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM users
             WHERE username LIKE ? OR nickname LIKE ? OR email LIKE ?",
        )
        .bind(&kw)
        .bind(&kw)
        .bind(&kw)
        .fetch_one(pool)
        .await?;
        Ok(total)
    }

    pub async fn create(
        pool: &SqlitePool,
        username: &str,
        password_hash: &str,
        nickname: &str,
        email: &str,
    ) -> AppResult<User> {
        let result = sqlx::query(
            "INSERT INTO users (username, password_hash, nickname, email, status)
             VALUES (?, ?, ?, ?, 1)",
        )
        .bind(username)
        .bind(password_hash)
        .bind(nickname)
        .bind(email)
        .execute(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db) if db.message().contains("UNIQUE") => {
                AppError::Conflict(format!("用户名 {username} 已存在"))
            }
            _ => AppError::Database(e),
        })?;

        let id = result.last_insert_rowid();
        Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("新建用户后未找到")))
    }

    pub async fn update(
        pool: &SqlitePool,
        id: i64,
        patch: &UpdateUser,
        new_password_hash: Option<&str>,
    ) -> AppResult<User> {
        // 简化:取出再写回。SQL 字段多时这个方式比动态 SQL 容易理解
        let user = Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("用户 {id} 不存在")))?;

        let username = patch.username.clone().unwrap_or(user.username);
        let nickname = patch.nickname.clone().unwrap_or(user.nickname);
        let email = patch.email.clone().unwrap_or(user.email);
        let status = patch.status.unwrap_or(user.status);
        let password_hash = new_password_hash
            .map(|s| s.to_string())
            .unwrap_or(user.password_hash);

        sqlx::query(
            "UPDATE users
             SET username = ?, password_hash = ?, nickname = ?, email = ?, status = ?, updated_at = datetime('now')
             WHERE id = ?",
        )
        .bind(&username)
        .bind(&password_hash)
        .bind(&nickname)
        .bind(&email)
        .bind(status)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db) if db.message().contains("UNIQUE") => {
                AppError::Conflict(format!("用户名 {username} 已存在"))
            }
            _ => AppError::Database(e),
        })?;

        Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("用户 {id} 不存在")))
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> AppResult<()> {
        let res = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        if res.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("用户 {id} 不存在")));
        }
        // 顺手清掉用户-角色关联
        sqlx::query("DELETE FROM user_roles WHERE user_id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_role_ids(pool: &SqlitePool, user_id: i64) -> AppResult<Vec<i64>> {
        let ids: Vec<i64> = sqlx::query_scalar("SELECT role_id FROM user_roles WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(ids)
    }

    pub async fn set_role_ids(
        pool: &SqlitePool,
        user_id: i64,
        role_ids: &[i64],
    ) -> AppResult<()> {
        let mut tx = pool.begin().await?;
        sqlx::query("DELETE FROM user_roles WHERE user_id = ?")
            .bind(user_id)
            .execute(&mut *tx)
            .await?;
        for rid in role_ids {
            sqlx::query("INSERT INTO user_roles (user_id, role_id) VALUES (?, ?)")
                .bind(user_id)
                .bind(rid)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}
