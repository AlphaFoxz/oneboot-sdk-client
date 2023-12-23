<script setup lang="ts">
import { Store } from '@tauri-apps/plugin-store'
import router from '@/router'
import { onMounted, ref } from 'vue'
import { global_notification } from '@/utils'
import { settings } from '@/constants'
import axios, { AxiosError } from 'axios'

const store = new Store(settings.FILE_NAME)
const backendHost = ref('')
const backendPort = ref('')
const tsGenDir = ref('')
const isTestingUrl = ref(false)

onMounted(async () => {
  backendHost.value = (await store.get(settings.KEY_BACKEND_HOST)) || ''
  backendPort.value = (await store.get(settings.KEY_BACKEND_PORT)) || ''
  tsGenDir.value = (await store.get(settings.KEY_TS_GEN_DIR)) || ''
})

const handleTestUrl = async () => {
  isTestingUrl.value = true
  axios
    .get(`http://${backendHost.value}:${backendPort.value}/_sdk`)
    .then(() => {
      openSuccessNotification('连通性测试成功')
      isTestingUrl.value = false
    })
    .catch((e: AxiosError) => {
      if (e.response && e.response.status) {
        openErrorNotification('已连通，但后端可能没有以开发模式运行')
      } else {
        console.error('catch', e)
        openErrorNotification('连通性测试失败，服务不可访问')
      }
      isTestingUrl.value = false
    })
}
const saveAllHandler = async () => {
  await store.set(settings.KEY_BACKEND_HOST, backendHost.value)
  await store.set(settings.KEY_BACKEND_PORT, backendPort.value)
  await store.set(settings.KEY_TS_GEN_DIR, tsGenDir.value)
  store
    .save()
    .then(() => {
      openSuccessNotification('配置已保存')
    })
    .catch(() => {
      openErrorNotification('保存失败，请检查是否有网络错误')
    })
}
const routerGo = (name: string) => {
  router.push({ name })
}
const openSuccessNotification = (content: string) => {
  global_notification.api.success({
    message: `操作成功`,
    description: content,
    placement: 'bottomRight',
  })
}
const openErrorNotification = (content: string) => {
  global_notification.api.error({
    message: `操作失败`,
    description: content,
    placement: 'bottomRight',
  })
}
</script>

<template>
  <a-layout class="h-full">
    <a-layout-content class="overflow-x-hidden">
      <a-form>
        <a-form-item label="服务端http地址">
          <label class="font-bold">http://</label>
          <a-input class="w-1/3" title="要访问的后端Host" placeholder="127.0.0.1" v-model:value="backendHost" />
          <label class="font-bold">:</label>
          <a-input class="w-1/12" title="要访问的后端端口" placeholder="8080" v-model:value="backendPort" />
          <a-button type="text" @click="handleTestUrl" :loading="isTestingUrl">测试连接</a-button>
        </a-form-item>
        <a-form-item label="ts代码生成目录">
          <a-input
            title="ts代码会在该目录生成，如：【配置的目录】/gen/app/AppTestApi.ts"
            placeholder="D:\projects\oneboot_front\src\apis"
            v-model:value="tsGenDir"
          />
        </a-form-item>
        <a-form-item>
          <a-button @click="routerGo('Home')">返回</a-button>
          &emsp;
          <a-button type="link" @click="saveAllHandler">保存</a-button>
        </a-form-item>
      </a-form>
    </a-layout-content>
  </a-layout>
</template>
