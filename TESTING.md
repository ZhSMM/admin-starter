# 测试与验证

跑通整个项目需要做四步:启动后端 → 验证后端 → 启动前端 → 浏览器登录。

## 1. 启动后端

```bash
cd backend
cp .env.example .env       # Windows 用 copy
cargo run
```

第一次启动会自动:
- 在 `data/admin.db` 创建 SQLite 数据库
- 跑迁移建表
- 写入种子数据(3 个角色、1 棵权限树、2 个用户)

控制台会看到:
```
INFO 配置加载完成,server_addr = 0.0.0.0:8080
INFO 连接数据库: ./data/admin.db
INFO 初始化数据写入完成,默认账号: admin/admin123, user/user123
INFO 数据库初始化完成
INFO 🚀 listening on http://0.0.0.0:8080
```

## 2. 验证后端 API(用 curl / PowerShell)

### 登录拿到 token

```bash
# curl
curl -X POST http://127.0.0.1:8080/api/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"username":"admin","password":"admin123"}'
```

```powershell
# PowerShell
$r = Invoke-RestMethod -Method POST -Uri "http://127.0.0.1:8080/api/auth/login" `
  -ContentType "application/json" -Body '{"username":"admin","password":"admin123"}'
$token = $r.data.token
```

### 用 token 调受保护接口

```bash
# /me: 当前用户、权限、菜单
curl -H "Authorization: Bearer $token" http://127.0.0.1:8080/api/auth/me

# /users: 用户列表(分页)
curl -H "Authorization: Bearer $token" "http://127.0.0.1:8080/api/users?page=1&page_size=10"

# /roles: 角色列表
curl -H "Authorization: Bearer $token" http://127.0.0.1:8080/api/roles

# /permissions: 权限树
curl -H "Authorization: Bearer $token" http://127.0.0.1:8080/api/permissions
```

### 验证权限隔离

用 `user` 登录(普通用户),调 `/api/users` 会返回 `403` —— 因为它没有 `system:user:list` 权限。

```bash
TOKEN=$(curl -s -X POST http://127.0.0.1:8080/api/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"username":"user","password":"user123"}' | jq -r .data.token)

curl -i -H "Authorization: Bearer $TOKEN" http://127.0.0.1:8080/api/users
# HTTP/1.1 403 Forbidden
# {"code":"forbidden","message":"缺少权限: system:user:list"}
```

## 3. 启动前端

```bash
cd frontend
npm install
npm run dev
```

打开 `http://localhost:5173`,登录页提示:
- 超级管理员:`admin / admin123` → 看到所有菜单和按钮
- 普通用户:`user / user123` → 只能看到菜单,点创建/删除按钮会报 403

## 4. 完整端到端冒烟

1. 用 admin 登录 → 进系统管理 → 用户管理 → 新增一个测试用户 → 给它分配"普通用户"角色
2. 退出,用新用户登录 → 验证侧边栏只能看到仪表盘 + 三个菜单(没有按钮)
3. 用 admin 进权限管理 → 新建一个"商品管理"菜单,code=`product:list`
4. 进角色管理 → 给"普通用户"分配该权限
5. 退出,用 user 登录 → 应该能看到新菜单

## 5. 新增模块的脚手架约定

新增一个业务模块,文件落点固定:

```
backend/
├── migrations/0002_xxx.sql      ← 新表
├── src/models/xxx.rs            ← 数据结构
├── src/repository/xxx.rs        ← SQL
├── src/service/xxx.rs           ← 业务
├── src/handler/xxx.rs           ← HTTP
└── src/{models,repository,service,handler}/mod.rs   ← 加 pub mod xxx;
src/routes.rs                    ← 加路由

frontend/
├── src/api/xxx.ts               ← API
├── src/views/xxx/Xxx.vue        ← 页面
└── src/router/index.ts          ← 加路由 + meta.permission
```

把 `permission` 表加几条对应 code,然后给某个角色分配,前端 `v-permission` 和路由守卫就自动生效了。

## 6. 常见问题

| 现象 | 原因 / 修法 |
|---|---|
| `unable to open database file` | 路径里不要有 URL 里 `://` 这种语法,直接 `sqlite:./data/admin.db` 或 `sqlite:C:\...\admin.db` |
| `JWT_SECRET 未设置` | 复制 `.env.example` 为 `.env` |
| 登录提示 401 | 用户名/密码错,默认 admin/admin123 |
| 看到菜单但操作报 403 | 没分配对应权限,去"权限管理"加 |
| 端口 8080 被占 | 改 `.env` 里的 `SERVER_ADDR` |
