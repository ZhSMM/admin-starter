//! 权限仓库

use sqlx::SqlitePool;

use crate::error::{AppError, AppResult};
use crate::models::permission::{CreatePermission, Permission, UpdatePermission};

pub struct PermissionRepo;

impl PermissionRepo {
    pub async fn list_all(pool: &SqlitePool) -> AppResult<Vec<Permission>> {
        let perms = sqlx::query_as::<_, Permission>(
            r#"SELECT id, parent_id, name, code, "type" as kind, path, component, icon, sort, status,
                      created_at, updated_at
               FROM permissions ORDER BY sort ASC, id ASC"#,
        )
        .fetch_all(pool)
        .await?;
        Ok(perms)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: i64) -> AppResult<Option<Permission>> {
        let perm = sqlx::query_as::<_, Permission>(
            r#"SELECT id, parent_id, name, code, "type" as kind, path, component, icon, sort, status,
                      created_at, updated_at
               FROM permissions WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(perm)
    }

    pub async fn find_by_code(pool: &SqlitePool, code: &str) -> AppResult<Option<Permission>> {
        let perm = sqlx::query_as::<_, Permission>(
            r#"SELECT id, parent_id, name, code, "type" as kind, path, component, icon, sort, status,
                      created_at, updated_at
               FROM permissions WHERE code = ?"#,
        )
        .bind(code)
        .fetch_optional(pool)
        .await?;
        Ok(perm)
    }

    pub async fn create(pool: &SqlitePool, p: &CreatePermission) -> AppResult<Permission> {
        let result = sqlx::query(
            r#"INSERT INTO permissions
               (parent_id, name, code, "type", path, component, icon, sort, status)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(p.parent_id)
        .bind(&p.name)
        .bind(&p.code)
        .bind(&p.kind)
        .bind(p.path.as_deref().unwrap_or(""))
        .bind(p.component.as_deref().unwrap_or(""))
        .bind(p.icon.as_deref().unwrap_or(""))
        .bind(p.sort.unwrap_or(0))
        .bind(p.status.unwrap_or(1))
        .execute(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db) if db.message().contains("UNIQUE") => {
                AppError::Conflict(format!("权限编码 {} 已存在", p.code))
            }
            _ => AppError::Database(e),
        })?;
        let id = result.last_insert_rowid();
        Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::Internal(anyhow::anyhow!("新建权限后未找到")))
    }

    pub async fn update(pool: &SqlitePool, id: i64, p: &UpdatePermission) -> AppResult<Permission> {
        let cur = Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("权限 {id} 不存在")))?;

        let parent_id = match &p.parent_id {
            // 显式传了 Some(Some(x)) 表示要改成 x;Some(None) 表示改成 NULL;None 表示不动
            Some(v) => v.clone(),
            None => cur.parent_id,
        };
        let name = p.name.clone().unwrap_or(cur.name);
        let code = p.code.clone().unwrap_or(cur.code);
        let kind = p.kind.clone().unwrap_or(cur.kind);
        let path = p.path.clone().unwrap_or(cur.path);
        let component = p.component.clone().unwrap_or(cur.component);
        let icon = p.icon.clone().unwrap_or(cur.icon);
        let sort = p.sort.unwrap_or(cur.sort);
        let status = p.status.unwrap_or(cur.status);

        sqlx::query(
            r#"UPDATE permissions
               SET parent_id = ?, name = ?, code = ?, "type" = ?, path = ?,
                   component = ?, icon = ?, sort = ?, status = ?,
                   updated_at = datetime('now')
               WHERE id = ?"#,
        )
        .bind(parent_id)
        .bind(&name)
        .bind(&code)
        .bind(&kind)
        .bind(&path)
        .bind(&component)
        .bind(&icon)
        .bind(sort)
        .bind(status)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db) if db.message().contains("UNIQUE") => {
                AppError::Conflict(format!("权限编码 {code} 已存在"))
            }
            _ => AppError::Database(e),
        })?;
        Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("权限 {id} 不存在")))
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> AppResult<()> {
        // 不允许删核心权限
        let perm = Self::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("权限 {id} 不存在")))?;
        if perm.code == "dashboard" || perm.code == "system" {
            return Err(AppError::BadRequest("不能删除核心权限".into()));
        }
        // 简单防护:有子节点就拒绝
        let child_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM permissions WHERE parent_id = ?")
            .bind(id)
            .fetch_one(pool)
            .await?;
        if child_count > 0 {
            return Err(AppError::BadRequest("请先删除子权限".into()));
        }
        let mut tx = pool.begin().await?;
        sqlx::query("DELETE FROM role_permissions WHERE permission_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM permissions WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }
}
