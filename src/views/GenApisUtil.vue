<script setup lang="ts">
import { useRouter } from 'vue-router'
import hljs from 'highlight.js'
import 'highlight.js/styles/github.css'
import { ref, onMounted } from 'vue'
import { global_message } from '@/utils'
const router = useRouter()
const highlightCode = ref('')
const copyRef = ref<HTMLTextAreaElement>()
const templateCodeRef = ref<HTMLElement>()
const handleCopy = () => {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(copyRef.value!.value)
    global_message.api.success('复制成功')
  } else {
    copyRef.value!.select()
    document.execCommand('copy')
    global_message.api.success('复制成功')
  }
}
onMounted(() => {
  const r = templateCodeRef.value || { innerText: '' }
  highlightCode.value = hljs.highlight(r.innerText, { language: 'typescript' }).value
  copyRef.value!.value = r.innerText
})
</script>

<template>
  <a-layout>
    <a-layout-header>
      <a-button class="text-white" @click="router.push({ name: 'Home' })">返回</a-button>
      <a-space class="text-white">前段apis目录 自定义apisUtil.ts代码示例</a-space>
      <a-button class="text-white" @click="handleCopy">复制</a-button>
    </a-layout-header>
    <a-layout-content>
      <textarea ref="copyRef" style="display: none"></textarea>
      <pre
        class="inline-block w-full h-full text-black"
      ><code class="whitespace-pre-wrap overflow-ellipsis overflow-hidden" v-html="highlightCode"></code></pre>
      <pre
        class="inline-block w-full h-full text-black"
      ><code ref="templateCodeRef" class="hidden">import axios, { type AxiosInstance } from 'axios'
import * as losslessJson from 'lossless-json'

/**
 * 向gen各个模块提供JSON序列化工具，解决JSON.parse大整数精度丢失问题
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
axiosInstance.defaults.baseURL = '/api'

/**
 * 配置请求拦截
 */
axios.interceptors.request.use(config => {
  // Do something
  return config
}, _error => {
  // Do something
})

/**
 * 配置响应拦截
 */
axios.interceptors.response.use(config => {
  // Do something
  return config
}, error => {
  console.error('自定义错误回调：请求失败', error)
  // Do something
})

/**
 * 向gen各个模块提供axios实例，方法内可改，方法本体勿删
 */
type HttpUtil = {
  post: (...arg0: any[]) => Promise&lt;any>
  get: (...arg0: any[]) => Promise&lt;any>
  put: (...arg0: any[]) => Promise&lt;any>
  delete: (...arg0: any[]) => Promise&lt;any>
  patch: (...arg0: any[]) => Promise&lt;any>
}
/**
 * api方法的返回值类型，正常带响应码的axios应该返回的是AxiosPromise，像这样：
 * export type HttpResult&lt;T> = AxiosPromise&lt;T>
 * 但是下面提供http方法的时候取了.data，所以返回值就变为了Promise&lt;T>
 */
export type HttpResult&lt;T> = Promise&lt;T>

let http: HttpUtil
export async function requireHttpUtil(): AxiosInstance {
  if (http) {
    return http
  }
  // 可以等待请求一些异步配置之类的 ...
  // const port = await getPrefix()
  // axios.defaults.baseURL = `http://localhost:${port}`
  http = {
    // 如果本项目只要response里的data，直接在这里处理就行
    get: async (url: string) => (await axiosInstance.get(url)).data,
    post: async (url: string, param?: any) => (await axiosInstance.post(url, param)).data,
    patch: async (url: string, param?: any) => (await axiosInstance.patch(url, param)).data,
    delete: async (url: string) => (await axiosInstance.delete(url)).data,
    put: async (url: string, param?: any) => (await axiosInstance.put(url, param)).data,
  }
  return axiosInstance
}

/**
 * 分页信息
 */
export type Page&lt;T> = {
  content: Array&lt;T>,
  pageable: {
    sort: {
      empty: boolean,
      unsorted: boolean,
      sorted: boolean,
    },
    offset: number,
    pageNumber: number,
    pageSize: number,
    unpaged: boolean
    paged: boolean,
  },
  last: boolean,
  totalPage: number,
  totalElements: number,
  size: number,
  number: number,
  sort: {
    empty: boolean,
    unsorted: boolean,
    sorted: boolean,
  },
  first: boolean,
  numberOfElements: number,
  empty: boolean
}
</code>
</pre>
    </a-layout-content>
  </a-layout>
</template>
