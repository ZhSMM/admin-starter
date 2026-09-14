-- 初始化表结构
-- SQLite 不支持 IF NOT EXISTS 在 CREATE TABLE 里同时定义约束,这里先建表再加索引/外键

-- 用户
CREATE TABLE IF NOT EXISTS users (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    username      TEXT    NOT NULL UNIQUE,
    password_hash TEXT    NOT NULL,
    nickname      TEXT    NOT NULL DEFAULT '',
    email         TEXT    NOT NULL DEFAULT '',
    status        INTEGER NOT NULL DEFAULT 1, -- 1=启用, 0=禁用
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- 角色
CREATE TABLE IF NOT EXISTS roles (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT    NOT NULL,
    code        TEXT    NOT NULL UNIQUE,
    description TEXT    NOT NULL DEFAULT '',
    status      INTEGER NOT NULL DEFAULT 1,
    created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- 权限(树形)
-- type: menu(菜单) / button(按钮) / api(接口)
CREATE TABLE IF NOT EXISTS permissions (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_id  INTEGER,                          -- NULL = 顶级
    name       TEXT    NOT NULL,
    code       TEXT    NOT NULL UNIQUE,          -- 例如 system:user:create
    type       TEXT    NOT NULL DEFAULT 'menu',  -- menu / button / api
    path       TEXT    NOT NULL DEFAULT '',       -- 前端路由 path
    component  TEXT    NOT NULL DEFAULT '',      -- 前端组件路径
    icon       TEXT    NOT NULL DEFAULT '',
    sort       INTEGER NOT NULL DEFAULT 0,
    status     INTEGER NOT NULL DEFAULT 1,
    created_at TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT    NOT NULL DEFAULT (datetime('now'))
);

-- 用户-角色(多对多)
CREATE TABLE IF NOT EXISTS user_roles (
    user_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, role_id)
);

-- 角色-权限(多对多)
CREATE TABLE IF NOT EXISTS role_permissions (
    role_id       INTEGER NOT NULL,
    permission_id INTEGER NOT NULL,
    PRIMARY KEY (role_id, permission_id)
);

CREATE INDEX IF NOT EXISTS idx_permissions_parent ON permissions(parent_id);
CREATE INDEX IF NOT EXISTS idx_user_roles_user ON user_roles(user_id);
CREATE INDEX IF NOT EXISTS idx_user_roles_role ON user_roles(role_id);
CREATE INDEX IF NOT EXISTS idx_role_permissions_role ON role_permissions(role_id);
CREATE INDEX IF NOT EXISTS idx_role_permissions_perm ON role_permissions(permission_id);
