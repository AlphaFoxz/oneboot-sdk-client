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
import JSONBigFun from 'json-bigint'

const JSONBig = JSONBigFun({ useNativeBigInt: true })

/**
 * 向gen各个模块提供JSON序列化工具，方法内可改，方法本体勿删
 */
export function requireJSON(): { parse: typeof JSON.parse; stringify: typeof JSON.stringify } {
  return JSONBig
}

/**
 * 创建axios实例
 */
const axiosInstance = axios.create({
  timeout: 30_000,
  headers: {
    'Content-Type': 'application/json'
  },
  transformResponse: [data => {
    try {
      return JSONBig.parse(data)
    } catch (e) {
      return data
    }
  }]
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
export async function requireAxios(): AxiosInstance {
  // 可以等待请求一些异步配置 ...
  // await some async configure ...
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
