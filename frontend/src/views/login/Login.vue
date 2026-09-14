<template>
  <div class="login-page">
    <el-card class="login-card">
      <h2 class="title">Admin Starter</h2>
      <p class="subtitle">Rust + Axum + SQLite + Vue 3</p>
      <el-form :model="form" label-position="top" @submit.prevent="onSubmit">
        <el-form-item label="用户名">
          <el-input v-model="form.username" placeholder="admin / user" autofocus />
        </el-form-item>
        <el-form-item label="密码">
          <el-input v-model="form.password" type="password" show-password placeholder="admin123" @keyup.enter="onSubmit" />
        </el-form-item>
        <el-button type="primary" :loading="loading" style="width: 100%" @click="onSubmit">登录</el-button>
      </el-form>
      <p class="hint">默认账号: admin / admin123  ·  user / user123</p>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { useUserStore } from '@/stores/user'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()

const form = reactive({ username: 'admin', password: 'admin123' })
const loading = ref(false)

async function onSubmit() {
  if (!form.username || !form.password) return
  loading.value = true
  try {
    await userStore.login(form.username, form.password)
    await userStore.fetchMe()
    ElMessage.success('登录成功')
    const redirect = (route.query.redirect as string) || '/dashboard'
    router.push(redirect)
  } catch (e) {
    // request.ts 已经弹了错误
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-page {
  min-height: 100vh;
  display: flex; align-items: center; justify-content: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}
.login-card { width: 380px; padding: 24px 12px; border-radius: 12px; }
.title { text-align: center; margin: 0 0 4px; }
.subtitle { text-align: center; color: #999; font-size: 12px; margin: 0 0 16px; }
.hint { text-align: center; color: #aaa; font-size: 12px; margin-top: 12px; }
</style>
