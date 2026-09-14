//! 角色仓库

use sqlx::SqlitePool;

use crate::error::{AppError, AppResult};
use crate::models::role::{CreateRole, Role, UpdateRole};

pub struct RoleRepo;

impl RoleRepo {
    pub async fn find_by_id(pool: &SqlitePool, id: i64) -> AppResult<Option<Role>> {
        let role = sqlx::query_as::<_, Role>(
            "SELECT id, name, code, description, status, created_at, updated_at FROM roles WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(role)
    }

    pub async fn find_by_code(pool: &SqlitePool, code: &str) -> AppResult<Option<Role>> {
        let role = sqlx::query_as::<_, Role>(
            "SELECT id, name, code, description, status, created_at, updated_at FROM roles WHERE code = ?",
        )
        .bind(code)
        .fetch_optional(pool)
        .await?;
        Ok(role)
    }

    pub async fn list(pool: &SqlitePool) -> AppResult<Vec<Role>> {
        let roles = sqlx::query_as::<_, Role>(
            "SELECT id, name, code, description, status, created_at, updated_at
             FROM roles ORDER BY id ASC",
        )
        .fetch_all(pool)
        .await?;
        Ok(roles)
    }

    pub async fn create(
        pool: &SqlitePool,
        name: &str,
        code: &str,
        description: &str,
        permission_ids: &[i64],
    ) -> AppResult<Role> {
        let mut tx = pool.begin().await?;
        let result = sqlx::query(
            "INSERT INTO roles (name, code, description, status) VALUES (?, ?, ?, 1)",
        )
        .bind(name)
        .bind(code)
        .bind(description)
        .execute(&mut *tx)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db) if db.message().contains("UNIQUE") => {
                AppError::Conflict(format!("角色编码 {code} 已存在"))
            }
            _ => AppError::Database(e),
        })?;
        let id = result.last_insert_rowid();
        for pid in permission_ids {
            sqlx::query("INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES (?, ?)")
                .bind(id)
                .bind(pid)
                .execute(&mut *tx)
                .await?;
        }
        tx.commit().await?;

        Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("新建角色后未找到")))
    }

    pub async fn update(
        pool: &SqlitePool,
        id: i64,
        patch: &UpdateRole,
        permission_ids: Option<&[i64]>,
    ) -> AppResult<Role> {
        let role = Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("角色 {id} 不存在")))?;
        let name = patch.name.clone().unwrap_or(role.name);
        let description = patch.description.clone().unwrap_or(role.description);
        let status = patch.status.unwrap_or(role.status);

        let mut tx = pool.begin().await?;
        sqlx::query(
            "UPDATE roles SET name = ?, description = ?, status = ?, updated_at = datetime('now') WHERE id = ?",
        )
        .bind(&name)
        .bind(&description)
        .bind(status)
        .bind(id)
        .execute(&mut *tx)
        .await?;
        if let Some(pids) = permission_ids {
            sqlx::query("DELETE FROM role_permissions WHERE role_id = ?")
                .bind(id)
                .execute(&mut *tx)
                .await?;
            for pid in pids {
                sqlx::query("INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES (?, ?)")
                    .bind(id)
                    .bind(pid)
                    .execute(&mut *tx)
                    .await?;
            }
        }
        tx.commit().await?;

        Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("角色 {id} 不存在")))
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> AppResult<()> {
        // 不允许删 admin 角色(简单防护)
        let role = Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("角色 {id} 不存在")))?;
        if role.code == "admin" {
            return Err(AppError::BadRequest("不能删除 admin 角色".into()));
        }
        let mut tx = pool.begin().await?;
        sqlx::query("DELETE FROM role_permissions WHERE role_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM user_roles WHERE role_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM roles WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_permission_ids(pool: &SqlitePool, role_id: i64) -> AppResult<Vec<i64>> {
        let ids: Vec<i64> = sqlx::query_scalar(
            "SELECT permission_id FROM role_permissions WHERE role_id = ?",
        )
        .bind(role_id)
        .fetch_all(pool)
        .await?;
        Ok(ids)
    }

    // 兼容 create_role 入参的便利函数
    pub async fn create_from(
        pool: &SqlitePool,
        create: &CreateRole,
        password_hash_fn: impl Fn(&str) -> AppResult<String>,
    ) -> AppResult<Role> {
        // 这里 password_hash_fn 是占位,实际 role 创建不需要密码
        let _ = password_hash_fn;
        Self::create(
            pool,
            &create.name,
            &create.code,
            create.description.as_deref().unwrap_or(""),
            create.permission_ids.as_deref().unwrap_or(&[]),
        )
        .await
    }
}
