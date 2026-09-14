<template>
  <div>
    <el-card>
      <div class="toolbar">
        <div class="spacer" />
        <el-button v-permission="'system:role:create'" type="primary" @click="openCreate">新增角色</el-button>
      </div>

      <el-table v-loading="loading" :data="list" border style="margin-top: 12px">
        <el-table-column prop="id" label="ID" width="70" />
        <el-table-column prop="name" label="名称" />
        <el-table-column prop="code" label="编码" />
        <el-table-column prop="description" label="描述" />
        <el-table-column label="权限数" width="100">
          <template #default="{ row }">{{ row.permission_ids.length }}</template>
        </el-table-column>
        <el-table-column label="状态" width="80">
          <template #default="{ row }">
            <el-tag :type="row.status === 1 ? 'success' : 'danger'">
              {{ row.status === 1 ? '启用' : '禁用' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="240" fixed="right">
          <template #default="{ row }: { row: RoleDto }">
            <el-button v-permission="'system:role:assign'" link type="primary" @click="openPerm(row)">分配权限</el-button>
            <el-button v-permission="'system:role:update'" link type="primary" @click="openEdit(row)">编辑</el-button>
            <el-popconfirm
              v-if="row.code !== 'admin'"
              title="确定删除该角色?"
              @confirm="onDelete(row.id)"
            >
              <template #reference>
                <el-button v-permission="'system:role:delete'" link type="danger">删除</el-button>
              </template>
            </el-popconfirm>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 新增/编辑弹窗 -->
    <el-dialog v-model="dialog" :title="form.id ? '编辑角色' : '新增角色'" width="500px">
      <el-form :model="form" label-width="80px">
        <el-form-item label="名称"><el-input v-model="form.name" /></el-form-item>
        <el-form-item label="编码">
          <el-input v-model="form.code" :disabled="!!form.id" />
        </el-form-item>
        <el-form-item label="描述"><el-input v-model="form.description" type="textarea" /></el-form-item>
        <el-form-item v-if="form.id" label="状态">
          <el-switch v-model="form.status" :active-value="1" :inactive-value="0" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialog = false">取消</el-button>
        <el-button type="primary" :loading="submitting" @click="onSubmit">确定</el-button>
      </template>
    </el-dialog>

    <!-- 分配权限弹窗 -->
    <el-dialog v-model="permDialog" title="分配权限" width="500px">
      <el-tree
        ref="treeRef"
        :data="permTree"
        node-key="id"
        show-checkbox
        :default-checked-keys="form.permission_ids"
        :props="{ label: 'name', children: 'children' }"
      />
      <template #footer>
        <el-button @click="permDialog = false">取消</el-button>
        <el-button type="primary" :loading="submitting" @click="onSubmitPerm">确定</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'
import { ElMessage } from 'element-plus'
import type { ElTree } from 'element-plus'
import { roleApi, type CreateRoleReq, type RoleDto, type UpdateRoleReq } from '@/api/role'
import { permissionApi, type PermissionNode } from '@/api/permission'

const list = ref<RoleDto[]>([])
const loading = ref(false)

const permTree = ref<PermissionNode[]>([])
const treeRef = ref<InstanceType<typeof ElTree>>()

const dialog = ref(false)
const permDialog = ref(false)
const submitting = ref(false)

const form = reactive<any>({
  id: undefined,
  name: '',
  code: '',
  description: '',
  status: 1,
  permission_ids: [] as number[],
})

async function reload() {
  loading.value = true
  try {
    const r = await roleApi.list()
    list.value = r.data
  } finally {
    loading.value = false
  }
}

function openCreate() {
  Object.assign(form, { id: undefined, name: '', code: '', description: '', status: 1, permission_ids: [] })
  dialog.value = true
}

function openEdit(row: RoleDto) {
  Object.assign(form, row, { permission_ids: [...row.permission_ids] })
  dialog.value = true
}

async function openPerm(row: RoleDto) {
  Object.assign(form, { id: row.id, name: row.name, code: row.code, description: row.description, status: row.status, permission_ids: [...row.permission_ids] })
  // 加载权限树(若已加载则跳过)
  if (permTree.value.length === 0) {
    const r = await permissionApi.tree()
    permTree.value = r.data
  }
  permDialog.value = true
}

async function onSubmit() {
  if (!form.name || !form.code) return ElMessage.warning('请填写名称和编码')
  submitting.value = true
  try {
    if (form.id) {
      const data: UpdateRoleReq = { name: form.name, description: form.description, status: form.status }
      await roleApi.update(form.id, data)
    } else {
      const data: CreateRoleReq = { name: form.name, code: form.code, description: form.description }
      await roleApi.create(data)
    }
    ElMessage.success('保存成功')
    dialog.value = false
    reload()
  } finally {
    submitting.value = false
  }
}

async function onSubmitPerm() {
  if (!form.id) return
  const checked = treeRef.value?.getCheckedKeys() as number[]
  const halfChecked = treeRef.value?.getHalfCheckedKeys() as number[]
  // 半选中的父节点不算(只勾选叶子),这里把两者合并
  const all = [...new Set([...checked, ...halfChecked])]
  submitting.value = true
  try {
    await roleApi.update(form.id, { permission_ids: all })
    ElMessage.success('权限已更新')
    permDialog.value = false
    reload()
  } finally {
    submitting.value = false
  }
}

async function onDelete(id: number) {
  await roleApi.remove(id)
  ElMessage.success('删除成功')
  reload()
}

onMounted(reload)
</script>

<style scoped>
.toolbar { display: flex; gap: 8px; align-items: center; }
.spacer { flex: 1; }
</style>
