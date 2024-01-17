<script setup lang="ts">
import { useRouter } from 'vue-router'
import hljs from 'highlight.js'
import 'highlight.js/styles/github.css'
import { ref, onMounted } from 'vue'
import { useToast } from 'primevue/usetoast'
import Button from 'primevue/button'

const toast = useToast()
const router = useRouter()
const highlightCode = ref('')
const copyRef = ref<HTMLTextAreaElement>()
const templateCodeRef = ref<HTMLElement>()
const handleCopy = () => {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(copyRef.value!.value)
    toast.add({ severity: 'success', summary: '复制成功', life: 2000 })
  } else {
    copyRef.value!.select()
    document.execCommand('copy')
    toast.add({ severity: 'success', summary: '复制成功', life: 2000 })
  }
}
onMounted(() => {
  const r = templateCodeRef.value || { innerText: '' }
  highlightCode.value = hljs.highlight(r.innerText, { language: 'typescript' }).value
  copyRef.value!.value = r.innerText
})
</script>

<template>
  <div>
    <div>
      <Button label="返回" @click="router.push({ name: 'Home' })"></Button>
      <label class="text-white">前端apis目录 自定义apis-util.ts代码示例</label>
      <Button label="复制" @click="handleCopy"></Button>
    </div>
    <div class="bg-white">
      <textarea ref="copyRef" style="display: none"></textarea>
      <pre
        class="inline-block w-full h-full text-black"
      ><code class="whitespace-pre-wrap overflow-ellipsis overflow-hidden" v-html="highlightCode"></code></pre>
      <pre
        class="inline-block w-full h-full text-black"
      ><code ref="templateCodeRef" class="hidden">import axios, { AxiosPromise } from 'axios'
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
}, error => {
  console.error('自定义错误请求回调：请求失败', error)
  // Do something
})

/**
 * 配置响应拦截
 */
axios.interceptors.response.use(config => {
  // Do something
  return config
}, error => {
  console.error('自定义错误响应回调：请求失败', error)
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
 * 但是如果下面提供http方法的时候取了.data，返回值就变为了Promise&lt;T>
 */
export type HttpResult&lt;T> = AxiosPromise&lt;T>

let http: HttpUtil
export async function requireHttpUtil(): Promise&lt;HttpUtil> {
  if (http) {
    return http
  }
  http = {
    get: async (url: string, param?: any) => await axiosInstance.get(url, param),
    post: async (url: string, param?: any) => await axiosInstance.post(url, param),
    patch: async (url: string, param?: any) => await axiosInstance.patch(url, param),
    delete: async (url: string, param?: any) => await axiosInstance.delete(url, param),
    put: async (url: string, param?: any) => await axiosInstance.put(url, param),
  }
  return http
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
    </div>
  </div>
</template>
