<script setup lang="ts">
import { Store } from '@tauri-apps/plugin-store'
import router from '@/router'
import { onMounted, ref } from 'vue'
import { global_notification } from '@/utils'
import { settings } from '@/constants'

const store = new Store(settings.FILE_NAME)
const backendHost = ref('')
const backendPort = ref('')
const tsGenDir = ref('')

onMounted(async () => {
  backendHost.value = (await store.get(settings.KEY_BACKEND_HOST)) || ''
  backendPort.value = (await store.get(settings.KEY_BACKEND_PORT)) || ''
  tsGenDir.value = (await store.get(settings.KEY_TS_GEN_DIR)) || ''
})

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
        <a-form-item label="后端Host">
          <a-input title="要访问的后端Host" placeholder="127.0.0.1" v-model:value="backendHost" />
        </a-form-item>
        <a-form-item label="后端端口">
          <a-input title="要访问的后端端口" placeholder="8080" v-model:value="backendPort" />
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
          <a-button @click="saveAllHandler">保存</a-button>
        </a-form-item>
      </a-form>
    </a-layout-content>
  </a-layout>
</template>
