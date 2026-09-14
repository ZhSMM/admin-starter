import request, { type ApiResp } from './request'
import type { UserDto } from './auth'

export interface PageResp<T> {
  list: T[]
  total: number
  page: number
  page_size: number
}

export interface CreateUserReq {
  username: string
  password: string
  nickname?: string
  email?: string
  role_ids?: number[]
}

export interface UpdateUserReq {
  username?: string
  nickname?: string
  email?: string
  status?: number
  password?: string
  role_ids?: number[]
}

export const userApi = {
  page: (params: { page?: number; page_size?: number; keyword?: string }) =>
    request.get<any, ApiResp<PageResp<UserDto>>>('/api/users', { params }),

  detail: (id: number) => request.get<any, ApiResp<UserDto>>(`/api/users/${id}`),

  create: (data: CreateUserReq) =>
    request.post<any, ApiResp<UserDto>>('/api/users', data),

  update: (id: number, data: UpdateUserReq) =>
    request.put<any, ApiResp<UserDto>>(`/api/users/${id}`, data),

  remove: (id: number) =>
    request.delete<any, ApiResp<null>>(`/api/users/${id}`),
}
