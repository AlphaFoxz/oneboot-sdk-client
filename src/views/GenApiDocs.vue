<script setup lang="ts">
import { useRouter } from 'vue-router'
import Button from 'primevue/button'
import { SdkGenDocApi } from '@/api'

const router = useRouter()
const handleGenWordApi = (moduleName: string) => {
  SdkGenDocApi.generateWordApi(moduleName).then((res) => {
    console.warn(res)
  })
}
const handleGenExcelApi = (moduleName: string) => {
  SdkGenDocApi.generateExcelApi(moduleName)
}
</script>

<template>
  <div style="min-height: 100%">
    <div>
      <Button label="返回" class="text-white" @click="router.push({ name: 'Home' })"></Button>
      <label class="text-white">生成Api文档</label>
    </div>
    <div class="bg-white text-black">
      <h1 class="text-3xl">创建Word Api</h1>
      <Button label="preset_sys" class="text-black" @click="handleGenWordApi('preset_sys')"></Button>
      <Button label="app" class="text-black" @click="handleGenWordApi('preset_sys')"></Button>
      <Button label="sdk" class="text-black" @click="handleGenWordApi('sdk')"></Button>
      <h1 class="text-3xl">创建Excel Api</h1>
      <Button label="preset_sys" class="text-black" @click="handleGenExcelApi('preset_sys')"></Button>
      <Button label="app" class="text-black" @click="handleGenExcelApi('preset_sys')"></Button>
      <Button label="sdk" class="text-black" @click="handleGenExcelApi('sdk')"></Button>
    </div>
  </div>
</template>

<style lang="less" scoped>
h1 {
  font-weight: bold;
  margin: 1.5rem 0;
}
</style>
