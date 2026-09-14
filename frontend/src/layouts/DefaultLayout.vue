<template>
  <el-container class="layout">
    <!-- 侧边栏 -->
    <el-aside :width="collapsed ? '64px' : '220px'" class="aside">
      <div class="logo">
        <span v-if="!collapsed">Admin Starter</span>
        <span v-else>A</span>
      </div>
      <el-menu
        :default-active="activeMenu"
        :collapse="collapsed"
        :collapse-transition="false"
        background-color="#001529"
        text-color="rgba(255,255,255,0.85)"
        active-text-color="#fff"
        router
      >
        <!-- 渲染菜单(只展到二级,够覆盖大多数后台) -->
        <template v-for="item in menus" :key="item.id">
          <el-sub-menu
            v-if="item.children && item.children.length > 0"
            :index="`m-${item.id}`"
          >
            <template #title>
              <el-icon><component :is="item.icon || 'Folder'" /></el-icon>
              <span>{{ item.name }}</span>
            </template>
            <template v-for="child in item.children" :key="child.id">
              <el-sub-menu
                v-if="child.children && child.children.length > 0"
                :index="`m-${item.id}-${child.id}`"
              >
                <template #title>
                  <el-icon><component :is="child.icon || 'Folder'" /></el-icon>
                  <span>{{ child.name }}</span>
                </template>
                <el-menu-item
                  v-for="leaf in child.children"
                  :key="leaf.id"
                  :index="leaf.path"
                >
                  <el-icon><component :is="leaf.icon || 'Document'" /></el-icon>
                  <template #title>{{ leaf.name }}</template>
                </el-menu-item>
              </el-sub-menu>
              <el-menu-item v-else :index="child.path">
                <el-icon><component :is="child.icon || 'Document'" /></el-icon>
                <template #title>{{ child.name }}</template>
              </el-menu-item>
            </template>
          </el-sub-menu>
          <el-menu-item v-else :index="item.path">
            <el-icon><component :is="item.icon || 'Document'" /></el-icon>
            <template #title>{{ item.name }}</template>
          </el-menu-item>
        </template>
      </el-menu>
    </el-aside>

    <el-container>
      <!-- 顶栏 -->
      <el-header class="header">
        <el-icon class="collapse-btn" @click="collapsed = !collapsed">
          <component :is="collapsed ? 'Expand' : 'Fold'" />
        </el-icon>
        <div class="spacer" />
        <el-dropdown @command="onCommand">
          <span class="user-trigger">
            <el-avatar :size="28">{{ userStore.user?.nickname?.slice(0, 1) || 'U' }}</el-avatar>
            <span class="username">{{ userStore.user?.nickname || userStore.user?.username }}</span>
            <el-icon><ArrowDown /></el-icon>
          </span>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="changePwd">修改密码</el-dropdown-item>
              <el-dropdown-item command="logout" divided>退出登录</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </el-header>

      <el-main class="main">
        <router-view v-slot="{ Component, route }">
          <transition name="fade" mode="out-in">
            <component :is="Component" :key="route.fullPath" />
          </transition>
        </router-view>
      </el-main>
    </el-container>

    <!-- 修改密码弹窗 -->
    <el-dialog v-model="pwdDialog" title="修改密码" width="420px">
      <el-form :model="pwdForm" label-width="80px">
        <el-form-item label="原密码">
          <el-input v-model="pwdForm.old_password" type="password" show-password />
        </el-form-item>
        <el-form-item label="新密码">
          <el-input v-model="pwdForm.new_password" type="password" show-password />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="pwdDialog = false">取消</el-button>
        <el-button type="primary" :loading="pwdLoading" @click="submitPwd">确定</el-button>
      </template>
    </el-dialog>
  </el-container>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessageBox } from 'element-plus'
import { useUserStore } from '@/stores/user'
import { authApi } from '@/api/auth'

const route = useRoute()
const router = useRouter()
const userStore = useUserStore()

const collapsed = ref(false)
const activeMenu = computed(() => route.path)

// 过滤掉没有 path 也没有 children 的节点(空节点)
const menus = computed(() => filterMenus(userStore.menus))

function filterMenus(nodes: any[]): any[] {
  return nodes
    .map((n) => ({ ...n, children: filterMenus(n.children || []) }))
    .filter((n) => n.path || (n.children && n.children.length > 0))
}

const pwdDialog = ref(false)
const pwdLoading = ref(false)
const pwdForm = ref({ old_password: '', new_password: '' })

function onCommand(cmd: string) {
  if (cmd === 'logout') {
    ElMessageBox.confirm('确定退出登录吗?', '提示', { type: 'warning' })
      .then(() => {
        userStore.logout()
        router.push('/login')
      })
      .catch(() => {})
  } else if (cmd === 'changePwd') {
    pwdForm.value = { old_password: '', new_password: '' }
    pwdDialog.value = true
  }
}

async function submitPwd() {
  if (!pwdForm.value.old_password || !pwdForm.value.new_password) {
    return ElMessageBox.alert('请填写完整', '提示')
  }
  pwdLoading.value = true
  try {
    await authApi.changePassword(pwdForm.value)
    pwdDialog.value = false
    ElMessageBox.alert('密码已修改,请重新登录', '提示').then(() => {
      userStore.logout()
      router.push('/login')
    })
  } finally {
    pwdLoading.value = false
  }
}
</script>

<style scoped>
.layout { height: 100vh; }
.aside { background: #001529; transition: width .2s; overflow: hidden; }
.logo { height: 56px; color: #fff; display: flex; align-items: center; justify-content: center; font-weight: 600; font-size: 16px; }
.header { background: #fff; border-bottom: 1px solid #eee; display: flex; align-items: center; padding: 0 16px; }
.collapse-btn { font-size: 20px; cursor: pointer; }
.spacer { flex: 1; }
.user-trigger { display: flex; align-items: center; gap: 8px; cursor: pointer; padding: 0 8px; }
.username { font-size: 14px; }
.main { background: #f5f7fa; padding: 16px; overflow: auto; }
.fade-enter-active, .fade-leave-active { transition: opacity .15s; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
