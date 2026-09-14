import request, { type ApiResp } from './request'

export type PermissionKind = 'menu' | 'button' | 'api'

export interface PermissionNode {
  id: number
  parent_id: number | null
  name: string
  code: string
  kind: PermissionKind
  path: string
  component: string
  icon: string
  sort: number
  status: number
  children: PermissionNode[]
}

export interface CreatePermissionReq {
  parent_id?: number | null
  name: string
  code: string
  kind: PermissionKind
  path?: string
  component?: string
  icon?: string
  sort?: number
  status?: number
}

export interface UpdatePermissionReq {
  parent_id?: number | null | undefined
  name?: string
  code?: string
  kind?: PermissionKind
  path?: string
  component?: string
  icon?: string
  sort?: number
  status?: number
}

export const permissionApi = {
  tree: () => request.get<any, ApiResp<PermissionNode[]>>('/api/permissions'),
  create: (data: CreatePermissionReq) =>
    request.post<any, ApiResp<PermissionNode>>('/api/permissions', data),
  update: (id: number, data: UpdatePermissionReq) =>
    request.put<any, ApiResp<PermissionNode>>(`/api/permissions/${id}`, data),
  remove: (id: number) =>
    request.delete<any, ApiResp<null>>(`/api/permissions/${id}`),
}
