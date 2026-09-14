<template>
  <div>
    <el-row :gutter="16">
      <el-col v-for="card in cards" :key="card.title" :span="6">
        <el-card shadow="hover">
          <div class="stat">
            <div class="stat-icon" :style="{ background: card.color }">
              <el-icon :size="24"><component :is="card.icon" /></el-icon>
            </div>
            <div>
              <div class="stat-title">{{ card.title }}</div>
              <div class="stat-value">{{ card.value }}</div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>

    <el-card style="margin-top: 16px">
      <template #header><b>欢迎回来, {{ userStore.user?.nickname }}</b></template>
      <p>这是一个用于学习的全栈后台管理脚手架。</p>
      <ul>
        <li>后端:Rust + Axum + SQLite,分层架构(handler / service / repository)</li>
        <li>前端:Vue 3 + Vite + Pinia + Element Plus</li>
        <li>权限:RBAC 模型(用户 - 角色 - 权限),JWT 鉴权</li>
        <li>扩展:新增业务模块只需在固定目录加文件</li>
      </ul>
      <p>当前账号: <el-tag>{{ userStore.user?.username }}</el-tag>
         <el-tag v-if="userStore.isSuper" type="danger" style="margin-left:8px">超级管理员</el-tag></p>
      <p>拥有权限数: <el-tag type="info">{{ userStore.permissions.size }}</el-tag></p>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useUserStore } from '@/stores/user'
import { userApi } from '@/api/user'
import { roleApi } from '@/api/role'
import { permissionApi } from '@/api/permission'

const userStore = useUserStore()

const userCount = ref(0)
const roleCount = ref(0)
const permCount = ref(0)

const cards = ref<any[]>([
  { title: '用户数', value: 0, icon: 'User', color: '#409eff' },
  { title: '角色数', value: 0, icon: 'UserFilled', color: '#67c23a' },
  { title: '权限数', value: 0, icon: 'Key', color: '#e6a23c' },
  { title: '在线状态', value: '在线', icon: 'Connection', color: '#f56c6c' },
])

onMounted(async () => {
  try {
    const [u, r, p] = await Promise.all([
      userApi.page({ page: 1, page_size: 1 }),
      roleApi.list(),
      permissionApi.tree(),
    ])
    userCount.value = u.data.total
    roleCount.value = r.data.length
    permCount.value = countTree(p.data)
    cards.value[0].value = userCount.value
    cards.value[1].value = roleCount.value
    cards.value[2].value = permCount.value
  } catch (e) {
    // 静默
  }
})

function countTree(nodes: any[]): number {
  return nodes.reduce((sum, n) => sum + 1 + countTree(n.children || []), 0)
}
</script>

<style scoped>
.stat { display: flex; align-items: center; gap: 16px; }
.stat-icon { width: 48px; height: 48px; border-radius: 8px; color: #fff; display: flex; align-items: center; justify-content: center; }
.stat-title { color: #999; font-size: 13px; }
.stat-value { font-size: 22px; font-weight: 600; }
ul { padding-left: 20px; line-height: 1.9; }
</style>
