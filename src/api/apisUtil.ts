import axios, { type AxiosInstance } from 'axios'
import * as losslessJson from 'lossless-json'
import { Store } from '@tauri-apps/plugin-store'
import { settings } from '@/constants'

const settingsStore = new Store(settings.FILE_NAME)

/**
 * 向gen各个模块提供JSON序列化工具，方法内可改，方法本体勿删
 */
type JsonLike = {
  parse: typeof JSON.parse
  stringify:
    | typeof JSON.stringify
    | ((
        value: any,
        replacer?: (this: any, key: string, value: any) => any,
        space?: string | number
      ) => string | undefined)
}
export function requireJSON(): JsonLike {
  return losslessJson
}

/**
 * 创建axios实例
 */
const axiosInstance = axios.create({
  timeout: 30_000,
  headers: {
    'Content-Type': 'application/json',
  },
  transformResponse: [
    (data) => {
      try {
        return losslessJson.parse(data)
      } catch (e) {
        return data
      }
    },
  ],
})

/**
 * 默认前缀，对应proxy跨域代理中的前缀
 */
// axiosInstance.defaults.baseURL = `http://127.0.0.1:8080`

/**
 * 配置请求拦截
 */
axios.interceptors.request.use(
  (config) => {
    // Do something
    return config
  },
  (_error) => {
    // Do something
  }
)

/**
 * 配置响应拦截
 */
axios.interceptors.response.use(
  (config) => {
    // Do something
    return config
  },
  (error) => {
    console.error('自定义错误回调：请求失败', error)
    // Do something
  }
)

/**
 * 向gen各个模块提供axios实例，方法内可改，方法本体勿删
 */
export async function requireAxios(): Promise<AxiosInstance> {
  const host = await settingsStore.get(settings.KEY_BACKEND_HOST)
  const port = await settingsStore.get(settings.KEY_BACKEND_PORT)
  axiosInstance.defaults.baseURL = `http://${host}:${port}`
  return axiosInstance
}

/**
 * 分页信息
 */
export type Page<T> = {
  content: Array<T>
  pageable: {
    sort: {
      empty: boolean
      unsorted: boolean
      sorted: boolean
    }
    offset: number
    pageNumber: number
    pageSize: number
    unpaged: boolean
    paged: boolean
  }
  last: boolean
  totalPage: number
  totalElements: number
  size: number
  number: number
  sort: {
    empty: boolean
    unsorted: boolean
    sorted: boolean
  }
  first: boolean
  numberOfElements: number
  empty: boolean
}
