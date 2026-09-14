//! 权限业务

use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::models::permission::{build_tree, CreatePermission, MenuNode, PermissionNode, UpdatePermission};
use crate::repository::permission::PermissionRepo;
use crate::repository::role::RoleRepo;
use crate::state::AppState;

pub struct PermissionService;

impl PermissionService {
    pub async fn tree(state: &AppState) -> AppResult<Vec<PermissionNode>> {
        let perms = PermissionRepo::list_all(&state.db).await?;
        Ok(build_tree(perms))
    }

    pub async fn create(state: &AppState, req: CreatePermission) -> AppResult<PermissionNode> {
        req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
        crate::models::permission::validate_permission_input(&req.code, &req.kind)
            .map_err(AppError::BadRequest)?;
        // 如果有 parent_id,确保父存在
        if let Some(pid) = req.parent_id {
            if PermissionRepo::find_by_id(&state.db, pid).await?.is_none() {
                return Err(AppError::BadRequest(format!("父权限 {pid} 不存在")));
            }
        }
        let p = PermissionRepo::create(&state.db, &req).await?;
        Ok(PermissionNode::from(p))
    }

    pub async fn update(state: &AppState, id: i64, req: UpdatePermission) -> AppResult<PermissionNode> {
        req.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
        if let Some(Some(pid)) = req.parent_id {
            if pid == id {
                return Err(AppError::BadRequest("不能把自己设为父节点".into()));
            }
        }
        let p = PermissionRepo::update(&state.db, id, &req).await?;
        Ok(PermissionNode::from(p))
    }

    pub async fn delete(state: &AppState, id: i64) -> AppResult<()> {
        PermissionRepo::delete(&state.db, id).await
    }

    /// 当前用户的菜单(只取 type=menu 且 status=1)
    pub async fn my_menus(state: &AppState, user_id: i64) -> AppResult<Vec<MenuNode>> {
        let perms = if is_super(state, user_id).await? {
            PermissionRepo::list_all(&state.db).await?
        } else {
            sqlx::query_as::<_, crate::models::permission::Permission>(
                r#"SELECT p.id, p.parent_id, p.name, p.code, p."type" as kind, p.path, p.component,
                          p.icon, p.sort, p.status, p.created_at, p.updated_at
                   FROM permissions p
                   INNER JOIN role_permissions rp ON rp.permission_id = p.id
                   INNER JOIN user_roles ur ON ur.role_id = rp.role_id
                   WHERE ur.user_id = ? AND p.status = 1 AND p."type" = 'menu'"#,
            )
            .bind(user_id)
            .fetch_all(&state.db)
            .await?
        };
        let tree = build_tree(perms);
        Ok(tree.iter().map(MenuNode::from_node).collect())
    }

    /// 当前用户的权限 code 列表(扁平)
    pub async fn my_permissions(state: &AppState, user_id: i64) -> AppResult<Vec<String>> {
        if is_super(state, user_id).await? {
            // 超管返回所有 status=1 的 code
            let codes: Vec<String> = sqlx::query_scalar(
                "SELECT code FROM permissions WHERE status = 1",
            )
            .fetch_all(&state.db)
            .await?;
            return Ok(codes);
        }
        let codes: Vec<String> = sqlx::query_scalar(
            r#"SELECT DISTINCT p.code
               FROM permissions p
               INNER JOIN role_permissions rp ON rp.permission_id = p.id
               INNER JOIN user_roles ur ON ur.role_id = rp.role_id
               WHERE ur.user_id = ? AND p.status = 1"#,
        )
        .bind(user_id)
        .fetch_all(&state.db)
        .await?;
        Ok(codes)
    }
}

async fn is_super(state: &AppState, user_id: i64) -> AppResult<bool> {
    // 通过 roles 中 code='admin' 判定
    let role_ids = crate::repository::user::UserRepo::get_role_ids(&state.db, user_id).await?;
    for rid in role_ids {
        if let Some(r) = RoleRepo::find_by_id(&state.db, rid).await? {
            if r.code == "admin" && r.status == 1 {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
