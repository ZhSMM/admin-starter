//! 权限模型
//!
//! 权限是树形结构:parent_id 为 NULL 的就是顶级,常见为目录/菜单。
//! code 唯一,推荐格式: `<module>:<resource>:<action>`,例如 `system:user:create`。

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Permission {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub code: String,
    /// menu / button / api
    /// 数据库列名是 `type`(SQL 保留字),SQL 中用 `"type" as kind` 别名,
    /// 这里字段就叫 `kind`,这样 FromRow 不用 rename。
    pub kind: String,
    pub path: String,
    pub component: String,
    pub icon: String,
    pub sort: i64,
    pub status: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// 树形 DTO:给前端做菜单/权限树展示
#[derive(Debug, Clone, Serialize)]
pub struct PermissionNode {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub code: String,
    pub kind: String,
    pub path: String,
    pub component: String,
    pub icon: String,
    pub sort: i64,
    pub status: i64,
    pub children: Vec<PermissionNode>,
}

impl PermissionNode {
    pub fn from(p: Permission) -> Self {
        Self {
            id: p.id,
            parent_id: p.parent_id,
            name: p.name,
            code: p.code,
            kind: p.kind,
            path: p.path,
            component: p.component,
            icon: p.icon,
            sort: p.sort,
            status: p.status,
            children: vec![],
        }
    }
}

/// 把扁平列表组装成树。简单清晰,O(n)。
pub fn build_tree(items: Vec<Permission>) -> Vec<PermissionNode> {
    use std::collections::HashMap;
    let mut nodes: HashMap<i64, PermissionNode> = items
        .into_iter()
        .map(|p| (p.id, PermissionNode::from(p)))
        .collect();
    let mut roots = Vec::new();
    let ids: Vec<i64> = nodes.keys().copied().collect();
    for id in ids {
        let node = nodes.get(&id).cloned().unwrap();
        if let Some(parent_id) = node.parent_id {
            if let Some(parent) = nodes.get_mut(&parent_id) {
                parent.children.push(node);
            } else {
                roots.push(node);
            }
        } else {
            roots.push(node);
        }
    }
    // 排序
    fn sort_rec(nodes: &mut [PermissionNode]) {
        nodes.sort_by_key(|n| n.sort);
        for n in nodes.iter_mut() {
            sort_rec(&mut n.children);
        }
    }
    sort_rec(&mut roots);
    roots
}

/// 菜单 DTO:登录后前端要拿到的精简结构
#[derive(Debug, Clone, Serialize)]
pub struct MenuNode {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub name: String,
    pub path: String,
    pub component: String,
    pub icon: String,
    pub sort: i64,
    pub children: Vec<MenuNode>,
}

impl MenuNode {
    pub fn from_node(n: &PermissionNode) -> Self {
        Self {
            id: n.id,
            parent_id: n.parent_id,
            name: n.name.clone(),
            path: n.path.clone(),
            component: n.component.clone(),
            icon: n.icon.clone(),
            sort: n.sort,
            children: n.children.iter().map(MenuNode::from_node).collect(),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePermission {
    pub parent_id: Option<i64>,
    #[validate(length(min = 1, max = 32, message = "名称长度 1-32"))]
    pub name: String,
    #[validate(length(min = 1, max = 64, message = "编码长度 1-64"))]
    pub code: String,
    /// menu / button / api
    pub kind: String,
    pub path: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub sort: Option<i64>,
    pub status: Option<i64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePermission {
    pub parent_id: Option<Option<i64>>,
    #[validate(length(min = 1, max = 32, message = "名称长度 1-32"))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 64, message = "编码长度 1-64"))]
    pub code: Option<String>,
    pub kind: Option<String>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub sort: Option<i64>,
    pub status: Option<i64>,
}

/// 手动校验 permission 入参。validator crate 0.18 的 derive 不支持
/// regex 字面量,这里把业务规则抽出来,service 层调用。
pub fn validate_permission_input(code: &str, kind: &str) -> Result<(), String> {
    fn is_valid_code(s: &str) -> bool {
        !s.is_empty()
            && s.bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b':' | b'*'))
    }
    if !is_valid_code(code) {
        return Err("编码只能包含小写字母、数字、下划线、冒号、星号".into());
    }
    if !matches!(kind, "menu" | "button" | "api") {
        return Err("类型必须是 menu/button/api".into());
    }
    Ok(())
}
