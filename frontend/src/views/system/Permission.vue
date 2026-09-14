<template>
  <div>
    <el-card>
      <div class="toolbar">
        <el-button @click="reload">刷新</el-button>
        <div class="spacer" />
        <el-button v-permission="'system:permission:create'" type="primary" @click="openCreate(null)">新增权限</el-button>
      </div>

      <el-table
        v-loading="loading"
        :data="flatTree"
        row-key="id"
        :tree-props="{ children: 'children' }"
        default-expand-all
        border
        style="margin-top: 12px"
      >
        <el-table-column prop="name" label="名称" />
        <el-table-column prop="code" label="编码" width="200" />
        <el-table-column label="类型" width="90">
          <template #default="{ row }">
            <el-tag :type="kindTag(row.kind)">{{ row.kind }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="path" label="路径" />
        <el-table-column prop="component" label="组件" />
        <el-table-column prop="icon" label="图标" width="100" />
        <el-table-column prop="sort" label="排序" width="80" />
        <el-table-column label="状态" width="80">
          <template #default="{ row }">
            <el-tag :type="row.status === 1 ? 'success' : 'danger'">
              {{ row.status === 1 ? '启用' : '禁用' }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="220" fixed="right">
          <template #default="{ row }: { row: PermissionNode }">
            <el-button v-permission="'system:permission:create'" link type="primary" @click="openCreate(row)">新增子项</el-button>
            <el-button v-permission="'system:permission:update'" link type="primary" @click="openEdit(row)">编辑</el-button>
            <el-popconfirm
              v-if="!['dashboard', 'system'].includes(row.code)"
              title="确定删除?"
              @confirm="onDelete(row.id)"
            >
              <template #reference>
                <el-button v-permission="'system:permission:delete'" link type="danger">删除</el-button>
              </template>
            </el-popconfirm>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <el-dialog v-model="dialog" :title="form.id ? '编辑权限' : '新增权限'" width="500px">
      <el-form :model="form" label-width="80px">
        <el-form-item label="父级">
          <el-tree-select
            v-model="form.parent_id"
            :data="parentOptions"
            :props="{ label: 'name', value: 'id', children: 'children' }"
            check-strictly
            clearable
            placeholder="顶级"
            style="width: 100%"
          />
        </el-form-item>
        <el-form-item label="名称"><el-input v-model="form.name" /></el-form-item>
        <el-form-item label="编码"><el-input v-model="form.code" :disabled="!!form.id" /></el-form-item>
        <el-form-item label="类型">
          <el-select v-model="form.kind" style="width: 100%">
            <el-option label="菜单 (menu)" value="menu" />
            <el-option label="按钮 (button)" value="button" />
            <el-option label="接口 (api)" value="api" />
          </el-select>
        </el-form-item>
        <el-form-item label="路径"><el-input v-model="form.path" placeholder="/system/user" /></el-form-item>
        <el-form-item label="组件"><el-input v-model="form.component" placeholder="system/User" /></el-form-item>
        <el-form-item label="图标"><el-input v-model="form.icon" placeholder="User" /></el-form-item>
        <el-form-item label="排序"><el-input-number v-model="form.sort" :min="0" /></el-form-item>
        <el-form-item label="状态">
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
import { computed, onMounted, reactive, ref } from 'vue'
import { ElMessage } from 'element-plus'
import { permissionApi, type CreatePermissionReq, type PermissionKind, type PermissionNode, type UpdatePermissionReq } from '@/api/permission'

const tree = ref<PermissionNode[]>([])
const loading = ref(false)
const dialog = ref(false)
const submitting = ref(false)

const form = reactive<any>({
  id: undefined,
  parent_id: null as number | null,
  name: '',
  code: '',
  kind: 'menu' as PermissionKind,
  path: '',
  component: '',
  icon: '',
  sort: 0,
  status: 1,
})

const flatTree = computed(() => tree.value)
const parentOptions = computed(() => tree.value)

function kindTag(k: string): 'success' | 'warning' | 'info' {
  return k === 'menu' ? 'success' : k === 'button' ? 'warning' : 'info'
}

async function reload() {
  loading.value = true
  try {
    const r = await permissionApi.tree()
    tree.value = r.data
  } finally {
    loading.value = false
  }
}

function openCreate(parent: PermissionNode | null) {
  Object.assign(form, {
    id: undefined,
    parent_id: parent?.id ?? null,
    name: '',
    code: '',
    kind: 'menu',
    path: '',
    component: '',
    icon: '',
    sort: 0,
    status: 1,
  })
  dialog.value = true
}

function openEdit(row: PermissionNode) {
  Object.assign(form, {
    id: row.id,
    parent_id: row.parent_id,
    name: row.name,
    code: row.code,
    kind: row.kind,
    path: row.path,
    component: row.component,
    icon: row.icon,
    sort: row.sort,
    status: row.status,
  })
  dialog.value = true
}

async function onSubmit() {
  if (!form.name || !form.code) return ElMessage.warning('请填写名称和编码')
  submitting.value = true
  try {
    if (form.id) {
      const data: UpdatePermissionReq = {
        parent_id: form.parent_id,
        name: form.name,
        code: form.code,
        kind: form.kind,
        path: form.path,
        component: form.component,
        icon: form.icon,
        sort: form.sort,
        status: form.status,
      }
      await permissionApi.update(form.id, data)
    } else {
      const data: CreatePermissionReq = {
        parent_id: form.parent_id,
        name: form.name,
        code: form.code,
        kind: form.kind,
        path: form.path,
        component: form.component,
        icon: form.icon,
        sort: form.sort,
        status: form.status,
      }
      await permissionApi.create(data)
    }
    ElMessage.success('保存成功')
    dialog.value = false
    reload()
  } finally {
    submitting.value = false
  }
}

async function onDelete(id: number) {
  await permissionApi.remove(id)
  ElMessage.success('删除成功')
  reload()
}

onMounted(reload)
</script>

<style scoped>
.toolbar { display: flex; gap: 8px; align-items: center; }
.spacer { flex: 1; }
</style>
