/**
 * 用户 store
 *
 * 维护: token、当前用户信息、权限 code 集合、菜单树
 * 提供: login / fetchMe / logout / hasPermission
 *
 * 权限集合用 Set,hasPermission 是 O(1)。
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi, type MeResp, type MenuNode } from '@/api/auth'

export const useUserStore = defineStore('user', () => {
  const token = ref<string>(localStorage.getItem('token') || '')
  const user = ref<MeResp['user'] | null>(null)
  const permissions = ref<Set<string>>(new Set())
  const menus = ref<MenuNode[]>([])
  const loaded = ref(false)

  const isLogin = computed(() => !!token.value)
  const isSuper = computed(() => user.value?.is_super ?? false)

  function setToken(t: string) {
    token.value = t
    if (t) localStorage.setItem('token', t)
    else localStorage.removeItem('token')
  }

  async function login(username: string, password: string) {
    const r = await authApi.login({ username, password })
    setToken(r.data.token)
    return r.data
  }

  async function fetchMe() {
    const r = await authApi.me()
    user.value = r.data.user
    permissions.value = new Set(r.data.permissions)
    menus.value = r.data.menus
    loaded.value = true
    return r.data
  }

  function logout() {
    setToken('')
    user.value = null
    permissions.value = new Set()
    menus.value = []
    loaded.value = false
  }

  /**
   * 判断是否拥有某权限 code。
   * 超管直接放行;支持 :* 通配,例如 system:user:* 匹配 system:user:create。
   */
  function hasPermission(code: string): boolean {
    if (isSuper.value) return true
    if (permissions.value.has(code)) return true
    // 通配符:把最后一个段换成 *
    if (code.includes(':')) {
      const wildcard = code.replace(/:[^:]+$/, ':*')
      if (permissions.value.has(wildcard)) return true
    }
    return false
  }

  return {
    token,
    user,
    permissions,
    menus,
    loaded,
    isLogin,
    isSuper,
    setToken,
    login,
    fetchMe,
    logout,
    hasPermission,
  }
})
