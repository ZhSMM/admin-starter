//! 角色模型

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: String,
    pub status: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoleDto {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub description: String,
    pub status: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub permission_ids: Vec<i64>,
}

impl RoleDto {
    pub fn from(role: Role, permission_ids: Vec<i64>) -> Self {
        Self {
            id: role.id,
            name: role.name,
            code: role.code,
            description: role.description,
            status: role.status,
            created_at: role.created_at,
            updated_at: role.updated_at,
            permission_ids,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRole {
    #[validate(length(min = 1, max = 32, message = "名称长度 1-32"))]
    pub name: String,
    #[validate(length(min = 1, max = 32, message = "编码长度 1-32"))]
    pub code: String,
    #[validate(length(max = 255, message = "描述最多 255 字符"))]
    pub description: Option<String>,
    pub permission_ids: Option<Vec<i64>>,
}

/// 校验角色编码(手写,因 validator 0.18 derive 不支持 regex 字面量)
pub fn validate_role_code(code: &str) -> Result<(), String> {
    let valid = !code.is_empty()
        && code.bytes().all(|b| {
            b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b':')
        });
    if !valid {
        return Err("编码只能包含小写字母、数字、下划线、冒号".into());
    }
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRole {
    #[validate(length(min = 1, max = 32, message = "名称长度 1-32"))]
    pub name: Option<String>,
    #[validate(length(max = 255, message = "描述最多 255 字符"))]
    pub description: Option<String>,
    pub status: Option<i64>,
    pub permission_ids: Option<Vec<i64>>,
}
