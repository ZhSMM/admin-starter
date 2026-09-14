/**
 * 权限指令 v-permission
 *
 * 用法:
 *   <el-button v-permission="'system:user:create'">新增</el-button>
 *   <el-button v-permission="['system:user:create', 'system:user:update']">编辑</el-button>
 *
 * 没有权限时直接从 DOM 移除节点(比 display:none 更彻底)。
 */

import type { App, Directive, DirectiveBinding } from 'vue'
import { useUserStore } from '@/stores/user'

function check(value: any): boolean {
  const store = useUserStore()
  if (store.isSuper) return true
  if (typeof value === 'string') return store.hasPermission(value)
  if (Array.isArray(value)) return value.some((v) => store.hasPermission(v))
  return false
}

const permission: Directive = {
  mounted(el: HTMLElement, binding: DirectiveBinding) {
    if (!check(binding.value)) el.parentNode?.removeChild(el)
  },
  updated(el: HTMLElement, binding: DirectiveBinding) {
    if (!check(binding.value)) el.parentNode?.removeChild(el)
  },
}

export function setupPermissionDirective(app: App) {
  app.directive('permission', permission)
}
