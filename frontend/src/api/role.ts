import request, { type ApiResp } from './request'

export interface RoleDto {
  id: number
  name: string
  code: string
  description: string
  status: number
  permission_ids: number[]
}

export interface CreateRoleReq {
  name: string
  code: string
  description?: string
  permission_ids?: number[]
}

export interface UpdateRoleReq {
  name?: string
  description?: string
  status?: number
  permission_ids?: number[]
}

export const roleApi = {
  list: () => request.get<any, ApiResp<RoleDto[]>>('/api/roles'),
  detail: (id: number) => request.get<any, ApiResp<RoleDto>>(`/api/roles/${id}`),
  create: (data: CreateRoleReq) => request.post<any, ApiResp<RoleDto>>('/api/roles', data),
  update: (id: number, data: UpdateRoleReq) =>
    request.put<any, ApiResp<RoleDto>>(`/api/roles/${id}`, data),
  remove: (id: number) => request.delete<any, ApiResp<null>>(`/api/roles/${id}`),
}
