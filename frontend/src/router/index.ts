/**
 * 路由 + 权限守卫
 *
 * 路由元信息约定:
 * - meta.title: 页面标题
 * - meta.permission: 需要的权限 code(单字符串,匹配 hasPermission)
 * - meta.hideInMenu: true 表示不进侧边栏
 *
 * 守卫规则:
 * 1. /login 不需要 token
 * 2. 受保护路由: 没 token → 跳 /login;有 token 但 store 没加载过 → 调 /me 加载
 * 3. 路由本身需要权限 → 没权限 → 跳 403 或 dashboard
 */

import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import { useUserStore } from '@/stores/user'

const routes: RouteRecordRaw[] = [
  { path: '/login', name: 'Login', component: () => import('@/views/login/Login.vue'), meta: { hideInMenu: true } },
  {
    path: '/',
    component: () => import('@/layouts/DefaultLayout.vue'),
    redirect: '/dashboard',
    children: [
      {
        path: 'dashboard',
        name: 'Dashboard',
        component: () => import('@/views/dashboard/Dashboard.vue'),
        meta: { title: '仪表盘', icon: 'Odometer', permission: 'dashboard' },
      },
      {
        path: 'system/user',
        name: 'SystemUser',
        component: () => import('@/views/system/User.vue'),
        meta: { title: '用户管理', icon: 'User', permission: 'system:user' },
      },
      {
        path: 'system/role',
        name: 'SystemRole',
        component: () => import('@/views/system/Role.vue'),
        meta: { title: '角色管理', icon: 'UserFilled', permission: 'system:role' },
      },
      {
        path: 'system/permission',
        name: 'SystemPermission',
        component: () => import('@/views/system/Permission.vue'),
        meta: { title: '权限管理', icon: 'Key', permission: 'system:permission' },
      },
    ],
  },
  { path: '/:pathMatch(.*)*', redirect: '/dashboard' },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach(async (to, _from, next) => {
  const userStore = useUserStore()

  // 登录页直接放行
  if (to.path === '/login') {
    if (userStore.isLogin) return next('/dashboard')
    return next()
  }

  // 其他页必须有 token
  if (!userStore.isLogin) {
    return next({ path: '/login', query: { redirect: to.fullPath } })
  }

  // 已登录但没加载过用户信息(刷新页面的情况)
  if (!userStore.loaded) {
    try {
      await userStore.fetchMe()
    } catch (e) {
      userStore.logout()
      return next({ path: '/login', query: { redirect: to.fullPath } })
    }
  }

  // 权限校验
  const perm = to.meta?.permission as string | undefined
  if (perm && !userStore.hasPermission(perm)) {
    // 没权限就跳首页,实际项目可加 /403
    return next('/dashboard')
  }

  next()
})

export default router
