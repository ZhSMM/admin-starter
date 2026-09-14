# admin-starter

一个用于学习 **Rust + Axum + SQLite + Vue 3** 全栈开发的后台管理脚手架。

聚焦三件事:
- **可扩展** —— 分层清晰(Handler / Service / Repository),新增一个业务模块只需在固定位置加文件
- **可读** —— 大量中文注释,关键概念都点出为什么这么写
- **够用** —— 自带一套完整的 RBAC 权限管理(用户 / 角色 / 权限),开箱即用

## 目录结构

```
admin-starter/
├── backend/                  # Rust + Axum 后端
│   ├── Cargo.toml
│   ├── .env.example          # 环境变量样例
│   ├── migrations/           # SQL 迁移(SQLite)
│   └── src/
│       ├── main.rs           # 入口
│       ├── config.rs         # 配置加载
│       ├── error.rs          # 统一错误类型
│       ├── db.rs             # 数据库连接池
│       ├── auth/             # 认证:JWT、密码、中间件
│       ├── models/           # 数据模型(serde + sqlx)
│       ├── repository/       # 数据访问层(每个实体一个文件)
│       ├── service/          # 业务层(每个实体一个文件)
│       ├── handler/          # HTTP 层(每个实体一个文件)
│       └── routes.rs         # 路由聚合
└── frontend/                 # Vue 3 + Vite 前端
    ├── package.json
    ├── vite.config.ts
    ├── index.html
    └── src/
        ├── main.ts
        ├── api/              # axios 封装 + 各模块 API
        ├── stores/           # Pinia 状态
        ├── router/           # 路由 + 权限守卫
        ├── layouts/          # 布局组件
        ├── views/            # 页面
        │   ├── login/
        │   ├── dashboard/
        │   └── system/       # 用户/角色/权限管理
        ├── directives/       # v-permission 权限指令
        └── utils/
```

## 快速开始

### 1. 启动后端

```bash
cd backend
cp .env.example .env        # 可按需修改端口、JWT 密钥等
cargo run
```

第一次启动会自动在 `data/admin.db` 创建 SQLite 数据库,并写入默认数据:
- 超级管理员 `admin` / `admin123`
- 普通用户 `user` / `user123`
- 三个基础角色 + 一棵权限树

### 2. 启动前端

```bash
cd frontend
npm install
npm run dev
```

浏览器打开 `http://localhost:5173`,用 `admin / admin123` 登录。

## 权限模型

- **用户 (user)** 拥有若干 **角色 (role)**
- **角色** 拥有若干 **权限 (permission)**
- **权限** 分为三类:`menu` (菜单) / `button` (按钮) / `api` (接口)
- 路由守卫 + 中间件 + 前端 `v-permission` 指令 三处共同把控

新增一个受控菜单/按钮的流程:
1. 后端 `POST /api/permissions` 写入一条 permission 记录
2. 把该权限分配给某个角色
3. 前端路由 `meta.permission` 写上对应 code
4. 按钮上写 `v-permission="'user:create'"`

## 新增一个业务模块的步骤(以"商品管理"为例)

后端:
1. `migrations/000X_product.sql` 加表
2. `models/product.rs` 定义结构体
3. `repository/product.rs` 写 SQL
4. `service/product.rs` 写业务逻辑
5. `handler/product.rs` 写 HTTP 接口
6. `routes.rs` 挂载路由

前端:
1. `api/product.ts` 封装请求
2. `views/system/Product.vue` 写页面
3. `router/index.ts` 加路由
4. 在权限管理里给角色分配 `product:*` 权限

—— 整个过程文件位置都是约定好的,不会乱。
