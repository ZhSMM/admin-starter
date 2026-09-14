<template>
  <div>
    <el-card>
      <!-- 工具栏 -->
      <div class="toolbar">
        <el-input v-model="query.keyword" placeholder="搜索用户名/昵称/邮箱" style="width: 240px" clearable @keyup.enter="reload" />
        <el-button type="primary" @click="reload">搜索</el-button>
        <div class="spacer" />
        <el-button v-permission="'system:user:create'" type="primary" @click="openCreate">新增用户</el-button>
      </div>

      <!-- 表格 -->
      <el-table v-loading="loading" :data="list" border style="margin-top: 12px">
        <el-table-column prop="id" label="ID" width="70" />
        <el-table-column prop="username" label="用户名" />
        <el-table-column prop="nickname" label="昵称" />
        <el-table-column prop="email" label="邮箱" />
        <el-table-column label="角色">
          <template #default="{ row }">
            <el-tag v-for="rid in row.role_ids" :key="rid" type="info" style="margin-right: 4px">
              {{ roleName(rid) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="状态" width="80">
          <template #default="{ row }">
            <el-tag :type="row.status === 1 ? 'success' : 'danger'">
              {{ row.status === 1 ? '启用' : '禁用' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="created_at" label="创建时间" width="170" />
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button v-permission="'system:user:update'" link type="primary" @click="openEdit(row)">编辑</el-button>
            <el-popconfirm
              v-if="row.id !== 1"
              title="确定删除该用户?"
              @confirm="onDelete(row.id)"
            >
              <template #reference>
                <el-button v-permission="'system:user:delete'" link type="danger">删除</el-button>
              </template>
            </el-popconfirm>
          </template>
        </el-table-column>
      </el-table>

      <!-- 分页 -->
      <el-pagination
        v-model:current-page="query.page"
        v-model:page-size="query.page_size"
        :total="total"
        :page-sizes="[10, 20, 50, 100]"
        layout="total, sizes, prev, pager, next, jumper"
        style="margin-top: 12px; justify-content: flex-end; display: flex"
        @size-change="reload"
        @current-change="reload"
      />
    </el-card>

    <!-- 新增/编辑弹窗 -->
    <el-dialog v-model="dialog" :title="form.id ? '编辑用户' : '新增用户'" width="500px">
      <el-form :model="form" label-width="80px">
        <el-form-item label="用户名">
          <el-input v-model="form.username" :disabled="!!form.id" />
        </el-form-item>
        <el-form-item v-if="!form.id" label="密码">
          <el-input v-model="form.password" type="password" show-password />
        </el-form-item>
        <el-form-item v-else label="重置密码">
          <el-input v-model="form.password" type="password" show-password placeholder="留空表示不修改" />
        </el-form-item>
        <el-form-item label="昵称">
          <el-input v-model="form.nickname" />
        </el-form-item>
        <el-form-item label="邮箱">
          <el-input v-model="form.email" />
        </el-form-item>
        <el-form-item label="角色">
          <el-select v-model="form.role_ids" multiple style="width: 100%">
            <el-option v-for="r in allRoles" :key="r.id" :label="r.name" :value="r.id" />
          </el-select>
        </el-form-item>
        <el-form-item v-if="form.id" label="状态">
          <el-switch v-model="form.status" :active-value="1" :inactive-value="0" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialog = false">取消</el-button>
        <el-button type="primary" :loading="submitting" @click="onSubmit">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'
import { ElMessage } from 'element-plus'
import { userApi, type CreateUserReq, type UpdateUserReq } from '@/api/user'
import { roleApi, type RoleDto } from '@/api/role'
import type { UserDto } from '@/api/auth'

const query = reactive({ page: 1, page_size: 10, keyword: '' })
const list = ref<UserDto[]>([])
const total = ref(0)
const loading = ref(false)

const allRoles = ref<RoleDto[]>([])
const roleMap = ref<Record<number, string>>({})

const dialog = ref(false)
const submitting = ref(false)
const form = reactive<any>({
  id: undefined,
  username: '',
  password: '',
  nickname: '',
  email: '',
  status: 1,
  role_ids: [] as number[],
})

async function reload() {
  loading.value = true
  try {
    const r = await userApi.page({ ...query })
    list.value = r.data.list
    total.value = r.data.total
  } finally {
    loading.value = false
  }
}

function roleName(id: number) {
  return roleMap.value[id] || `#${id}`
}

function openCreate() {
  Object.assign(form, { id: undefined, username: '', password: '', nickname: '', email: '', status: 1, role_ids: [] })
  dialog.value = true
}

function openEdit(row: UserDto) {
  Object.assign(form, { ...row, password: '' })
  dialog.value = true
}

async function onSubmit() {
  if (!form.username) return ElMessage.warning('请填写用户名')
  if (!form.id && !form.password) return ElMessage.warning('请填写密码')
  submitting.value = true
  try {
    if (form.id) {
      const data: UpdateUserReq = {
        nickname: form.nickname,
        email: form.email,
        status: form.status,
        role_ids: form.role_ids,
      }
      if (form.password) data.password = form.password
      await userApi.update(form.id, data)
    } else {
      const data: CreateUserReq = {
        username: form.username,
        password: form.password,
        nickname: form.nickname,
        email: form.email,
        role_ids: form.role_ids,
      }
      await userApi.create(data)
    }
    ElMessage.success('保存成功')
    dialog.value = false
    reload()
  } finally {
    submitting.value = false
  }
}

async function onDelete(id: number) {
  await userApi.remove(id)
  ElMessage.success('删除成功')
  reload()
}

onMounted(async () => {
  await reload()
  const r = await roleApi.list()
  allRoles.value = r.data
  roleMap.value = Object.fromEntries(r.data.map((x) => [x.id, x.name]))
})
</script>

<style scoped>
.toolbar { display: flex; gap: 8px; align-items: center; }
.spacer { flex: 1; }
</style>
