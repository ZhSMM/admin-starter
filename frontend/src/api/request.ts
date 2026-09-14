/**
 * axios 封装
 *
 * 关键点:
 * 1. 请求拦截器自动带 token
 * 2. 响应拦截器统一处理业务错误(后端 200 + code != 0 也算业务错误)
 * 3. 401 跳登录
 * 4. 后端统一返回: { code: 0, message: 'ok', data: ... }
 */

import axios, { AxiosError, type AxiosResponse, type InternalAxiosRequestConfig } from 'axios'
import { ElMessage } from 'element-plus'

const request = axios.create({
  baseURL: '/',
  timeout: 15000,
})

request.interceptors.request.use((config: InternalAxiosRequestConfig) => {
  const token = localStorage.getItem('token')
  if (token) {
    config.headers.set('Authorization', `Bearer ${token}`)
  }
  return config
})

request.interceptors.response.use(
  (resp: AxiosResponse) => {
    const data = resp.data
    if (data && typeof data === 'object' && 'code' in data) {
      if (data.code === 0) return data
      ElMessage.error(data.message || '请求失败')
      return Promise.reject(data)
    }
    return data
  },
  (err: AxiosError<any>) => {
    const status = err.response?.status
    const msg = err.response?.data?.message || err.message
    if (status === 401) {
      // 清理登录态
      localStorage.removeItem('token')
      // 避免在登录页重复弹
      if (!location.pathname.startsWith('/login')) {
        location.href = '/login'
      }
    }
    ElMessage.error(msg || '网络错误')
    return Promise.reject(err)
  },
)

export default request

// 后端统一响应结构
export interface ApiResp<T = any> {
  code: number
  message: string
  data: T
}
