//! 用户模型

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// 数据库实体
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub nickname: String,
    pub email: String,
    pub status: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// 对外响应:绝不暴露 password_hash
#[derive(Debug, Clone, Serialize)]
pub struct UserDto {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub status: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub role_ids: Vec<i64>,
}

impl UserDto {
    pub fn from(user: User, role_ids: Vec<i64>) -> Self {
        Self {
            id: user.id,
            username: user.username,
            nickname: user.nickname,
            email: user.email,
            status: user.status,
            created_at: user.created_at,
            updated_at: user.updated_at,
            role_ids,
        }
    }
}

/// 创建用户入参
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 32, message = "用户名长度 3-32"))]
    pub username: String,
    #[validate(length(min = 6, max = 64, message = "密码长度 6-64"))]
    pub password: String,
    #[validate(length(max = 32, message = "昵称最多 32 字符"))]
    pub nickname: Option<String>,
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
    pub role_ids: Option<Vec<i64>>,
}

/// 更新用户入参
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(length(min = 3, max = 32, message = "用户名长度 3-32"))]
    pub username: Option<String>,
    #[validate(length(max = 32, message = "昵称最多 32 字符"))]
    pub nickname: Option<String>,
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
    pub status: Option<i64>,
    pub password: Option<String>,
    pub role_ids: Option<Vec<i64>>,
}

/// 修改自己密码
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePassword {
    pub old_password: String,
    #[validate(length(min = 6, max = 64, message = "密码长度 6-64"))]
    pub new_password: String,
}

/// 登录入参
#[derive(Debug, Deserialize, Validate)]
pub struct LoginReq {
    #[validate(length(min = 1, message = "用户名不能为空"))]
    pub username: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Serialize)]
pub struct LoginResp {
    pub token: String,
    pub user: UserDto,
}

/// 分页响应
#[derive(Debug, Serialize)]
pub struct PageResp<T> {
    pub list: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

/// 分页查询参数
#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub keyword: Option<String>,
}

impl PageQuery {
    pub fn normalize(&self) -> (i64, i64) {
        let page = self.page.unwrap_or(1).max(1);
        let page_size = self.page_size.unwrap_or(20).clamp(1, 200);
        (page, page_size)
    }
    pub fn offset(&self) -> i64 {
        let (page, page_size) = self.normalize();
        (page - 1) * page_size
    }
}
