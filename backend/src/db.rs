//! 数据库初始化
//!
//! 启动流程:
//! 1. 确保 SQLite 文件所在目录存在
//! 2. 建连接池
//! 3. 跑迁移(`migrations/*.sql`,按文件名升序,只跑一次)
//! 4. 如果库里没有任何用户,塞入默认管理员和示例数据(便于第一次跑通)

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{path::Path, time::Duration};

use crate::config::Config;

pub type Db = SqlitePool;

pub async fn init_pool(cfg: &Config) -> anyhow::Result<Db> {
    use sqlx::sqlite::SqliteConnectOptions;

    // 解析出物理路径,以便必要时创建父目录
    let path_str = cfg
        .database_url
        .strip_prefix("sqlite://")
        .or_else(|| cfg.database_url.strip_prefix("sqlite:"))
        .unwrap_or(&cfg.database_url)
        .trim_start_matches('/')
        .trim_start_matches('\\')
        .to_string();
    let in_memory = path_str.contains(":memory:");

    if !in_memory {
        if let Some(parent) = Path::new(&path_str).parent() {
            let s = parent.as_os_str();
            if !s.is_empty() && s != ":" {
                std::fs::create_dir_all(parent)?;
            }
        }
    }

    // 关键:用 SqliteConnectOptions 构造,而不是 URL parse
    // (Windows 上 URL parser 会把 "C:" 误识为 authority,导致打开失败)
    let opts = SqliteConnectOptions::new()
        .filename(&path_str)
        .create_if_missing(true);

    tracing::info!("连接数据库: {}", path_str);
    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(opts)
        .await?;

    // 启动时跑迁移
    sqlx::migrate!("./migrations").run(&pool).await?;

    // 首次启动塞入种子数据
    seed_if_empty(&pool, cfg).await?;

    Ok(pool)
}

async fn seed_if_empty(pool: &Db, cfg: &Config) -> anyhow::Result<()> {
    use bcrypt::{hash, DEFAULT_COST};

    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    if user_count > 0 {
        return Ok(());
    }

    let mut tx = pool.begin().await?;

    // 1) 三个基础角色
    sqlx::query("INSERT INTO roles (id, name, code, description, status) VALUES (?, ?, ?, ?, 1)")
        .bind(1)
        .bind("超级管理员")
        .bind("admin")
        .bind("拥有所有权限")
        .execute(&mut *tx)
        .await?;
    sqlx::query("INSERT INTO roles (id, name, code, description, status) VALUES (?, ?, ?, ?, 1)")
        .bind(2)
        .bind("普通用户")
        .bind("user")
        .bind("只能查看")
        .execute(&mut *tx)
        .await?;
    sqlx::query("INSERT INTO roles (id, name, code, description, status) VALUES (?, ?, ?, ?, 1)")
        .bind(3)
        .bind("访客")
        .bind("guest")
        .bind("只读")
        .execute(&mut *tx)
        .await?;

    // 2) 权限树
    // 顶级:系统管理
    sqlx::query(
        "INSERT INTO permissions (id, parent_id, name, code, type, path, component, icon, sort, status)
         VALUES (1, NULL, '系统管理', 'system', 'menu', '/system', 'Layout', 'setting', 1, 1)",
    )
    .execute(&mut *tx)
    .await?;
    // 子菜单:用户管理 / 角色管理 / 权限管理
    for (id, name, code, path, component, icon, sort) in [
        (2, "用户管理", "system:user", "/system/user", "system/User", "user", 1),
        (3, "角色管理", "system:role", "/system/role", "system/Role", "peoples", 2),
        (4, "权限管理", "system:permission", "/system/permission", "system/Permission", "tree", 3),
    ] {
        sqlx::query(
            "INSERT INTO permissions (id, parent_id, name, code, type, path, component, icon, sort, status)
             VALUES (?, 1, ?, ?, 'menu', ?, ?, ?, ?, 1)",
        )
        .bind(id)
        .bind(name)
        .bind(code)
        .bind(path)
        .bind(component)
        .bind(icon)
        .bind(sort)
        .execute(&mut *tx)
        .await?;
    }
    // 用户管理下的按钮权限
    for (id, name, code, sort) in [
        (10, "新增用户", "system:user:create", 1),
        (11, "编辑用户", "system:user:update", 2),
        (12, "删除用户", "system:user:delete", 3),
    ] {
        sqlx::query(
            "INSERT INTO permissions (id, parent_id, name, code, type, sort, status)
             VALUES (?, 2, ?, ?, 'button', ?, 1)",
        )
        .bind(id)
        .bind(name)
        .bind(code)
        .bind(sort)
        .execute(&mut *tx)
        .await?;
    }
    // 角色管理下的按钮权限
    for (id, name, code, sort) in [
        (20, "新增角色", "system:role:create", 1),
        (21, "编辑角色", "system:role:update", 2),
        (22, "删除角色", "system:role:delete", 3),
        (23, "分配权限", "system:role:assign", 4),
    ] {
        sqlx::query(
            "INSERT INTO permissions (id, parent_id, name, code, type, sort, status)
             VALUES (?, 3, ?, ?, 'button', ?, 1)",
        )
        .bind(id)
        .bind(name)
        .bind(code)
        .bind(sort)
        .execute(&mut *tx)
        .await?;
    }
    // 权限管理下的按钮权限
    for (id, name, code, sort) in [
        (30, "新增权限", "system:permission:create", 1),
        (31, "编辑权限", "system:permission:update", 2),
        (32, "删除权限", "system:permission:delete", 3),
    ] {
        sqlx::query(
            "INSERT INTO permissions (id, parent_id, name, code, type, sort, status)
             VALUES (?, 4, ?, ?, 'button', ?, 1)",
        )
        .bind(id)
        .bind(name)
        .bind(code)
        .bind(sort)
        .execute(&mut *tx)
        .await?;
    }

    // 3) 仪表盘菜单,所有登录用户都能看
    sqlx::query(
        "INSERT INTO permissions (id, parent_id, name, code, type, path, component, icon, sort, status)
         VALUES (40, NULL, '仪表盘', 'dashboard', 'menu', '/dashboard', 'Dashboard', 'dashboard', 0, 1)",
    )
    .execute(&mut *tx)
    .await?;

    // 4) 默认用户
    let admin_pwd = hash("admin123", cfg.bcrypt_cost).unwrap_or_else(|_| {
        // 兜底,实际上 bcrypt::hash 不会失败除非 cost 不合法
        hash("admin123", DEFAULT_COST).unwrap()
    });
    let user_pwd = hash("user123", cfg.bcrypt_cost).unwrap_or_else(|_| {
        hash("user123", DEFAULT_COST).unwrap()
    });

    sqlx::query(
        "INSERT INTO users (id, username, password_hash, nickname, email, status)
         VALUES (1, 'admin', ?, '超级管理员', 'admin@example.com', 1)",
    )
    .bind(&admin_pwd)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        "INSERT INTO users (id, username, password_hash, nickname, email, status)
         VALUES (2, 'user', ?, '普通用户', 'user@example.com', 1)",
    )
    .bind(&user_pwd)
    .execute(&mut *tx)
    .await?;

    // 5) 给用户分配角色
    sqlx::query("INSERT INTO user_roles (user_id, role_id) VALUES (1, 1)")
        .execute(&mut *tx)
        .await?;
    sqlx::query("INSERT INTO user_roles (user_id, role_id) VALUES (2, 2)")
        .execute(&mut *tx)
        .await?;

    // 6) 给角色分配权限
    // admin -> 所有权限
    let all_perm_ids: Vec<i64> =
        sqlx::query_scalar("SELECT id FROM permissions").fetch_all(&mut *tx).await?;
    for pid in all_perm_ids.iter() {
        sqlx::query("INSERT INTO role_permissions (role_id, permission_id) VALUES (1, ?)")
            .bind(pid)
            .execute(&mut *tx)
            .await?;
    }
    // user -> 仪表盘 + 查看类权限(用 code 前缀简单模拟)
    for code in ["dashboard", "system:user", "system:role", "system:permission"] {
        let pid: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM permissions WHERE code = ? OR code LIKE ?",
        )
        .bind(code)
        .bind(format!("{code}:%"))
        .fetch_optional(&mut *tx)
        .await?;
        if let Some(pid) = pid {
            sqlx::query("INSERT INTO role_permissions (role_id, permission_id) VALUES (2, ?)")
                .bind(pid)
                .execute(&mut *tx)
                .await?;
        }
    }
    // guest -> 仪表盘
    let dash_id: Option<i64> = sqlx::query_scalar("SELECT id FROM permissions WHERE code = 'dashboard'")
        .fetch_optional(&mut *tx)
        .await?;
    if let Some(dash_id) = dash_id {
        sqlx::query("INSERT INTO role_permissions (role_id, permission_id) VALUES (3, ?)")
            .bind(dash_id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;
    tracing::info!("初始化数据写入完成,默认账号: admin/admin123, user/user123");
    Ok(())
}
