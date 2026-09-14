import request, { type ApiResp } from './request'

export interface LoginReq {
  username: string
  password: string
}

export interface UserDto {
  id: number
  username: string
  nickname: string
  email: string
  status: number
  role_ids: number[]
  created_at?: string
  updated_at?: string
}

export interface LoginResp {
  token: string
  user: UserDto
}

export interface MenuNode {
  id: number
  parent_id: number | null
  name: string
  path: string
  component: string
  icon: string
  sort: number
  children: MenuNode[]
}

export interface MeResp {
  user: { id: number; username: string; nickname: string; is_super: boolean }
  permissions: string[]
  menus: MenuNode[]
}

export const authApi = {
  login: (data: LoginReq) =>
    request.post<any, ApiResp<LoginResp>>('/api/auth/login', data),

  me: () => request.get<any, ApiResp<MeResp>>('/api/auth/me'),

  changePassword: (data: { old_password: string; new_password: string }) =>
    request.post<any, ApiResp<null>>('/api/auth/change-password', data),
}
