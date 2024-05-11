<script setup lang="ts">
import { useRouter } from 'vue-router'
import Button from 'primevue/button'
import * as api from '@/api'

const router = useRouter()
const download = (data: Blob, fileName: string) => {
  const url = window.URL.createObjectURL(data)
  const tmpLink = document.createElement('a')
  tmpLink.download = fileName || ''
  tmpLink.style.display = 'none'
  tmpLink.href = url
  document.body.appendChild(tmpLink)
  tmpLink.click()
  window.URL.revokeObjectURL(tmpLink.href) // 释放URL 对象
  document.body.removeChild(tmpLink)
}
const handleGenWordApi = (moduleName: string) => {
  api.generateWordApi(moduleName).then((res: any) => {
    const data = new Blob([res.data], { type: 'application/octet-stream;charset=utf-8' })
    const fileName = decodeURI(res.headers['content-disposition'].split('=')[1])
    download(data, fileName)
  })
}
</script>

<template>
  <div style="min-height: 100%">
    <div>
      <Button label="返回" class="text-white" @click="router.push({ name: 'Home' })"></Button>
      <label class="text-white">生成Api文档</label>
    </div>
    <div class="bg-white text-black">
      <h1 class="text-3xl">生成Word Api（docx）</h1>
      <Button label="preset_sys" class="text-black" @click="handleGenWordApi('preset_sys')"></Button>
      <Button label="app" class="text-black" @click="handleGenWordApi('preset_sys')"></Button>
      <Button label="sdk" class="text-black" @click="handleGenWordApi('sdk')"></Button>
    </div>
  </div>
</template>

<style lang="less" scoped>
h1 {
  font-weight: bold;
  margin: 1.5rem 0;
}
</style>
