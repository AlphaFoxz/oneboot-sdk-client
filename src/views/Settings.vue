<script setup lang="ts">
import { Store } from '@tauri-apps/plugin-store'
import router from '@/router'
import { onMounted, ref } from 'vue'
import { useToast } from 'primevue/usetoast'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import { settings } from '@/constants'
import axios, { AxiosError } from 'axios'

const toast = useToast()
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
        openErrorNotification(`连通性测试失败，服务不可访问 http://${backendHost.value}:${backendPort.value}`)
      }
      isTestingUrl.value = false
    })
}
const handleSaveAll = async () => {
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
const routerLink = (name: string) => {
  router.push({ name })
}
const openSuccessNotification = (content: string) => {
  toast.add({ severity: 'success', summary: '操作成功', detail: content, life: 2000 })
}
const openErrorNotification = (content: string) => {
  toast.add({ severity: 'error', summary: '操作失败', detail: content, life: 5000 })
}
</script>

<template>
  <div class="h-full overflow-x-hidden">
    <div class="flex flex-column gap-2">
      <label class="font-bold">http://</label>
      <InputText
        class="w-1/3"
        title="要访问的后端Host"
        mask="9(99).9(99).9(99).9(99)"
        placeholder="127.0.0.1"
        v-model="backendHost"
      />
      <InputText class="w-1/12" title="要访问的后端端口" placeholder="8080" v-model="backendPort" />
      <Button @click="handleTestUrl" :loading="isTestingUrl">测试连接</Button>
    </div>
    <div class="flex flex-column gap-2">
      <label for="tsGenDir">TS代码生成目录</label>
      <InputText
        id="tsGenDir"
        title="ts代码会在该目录生成，如：【配置的目录】/gen/app/AppTestApi.ts"
        placeholder="D:\projects\oneboot_front\src\apis"
        style="min-width: 40%"
        v-model="tsGenDir"
      />
    </div>
    <div class="flex flex-column gap-2">
      <Button @click="routerLink('Home')">返回</Button>
      &emsp;
      <Button @click="handleSaveAll">保存</Button>
    </div>
  </div>
</template>
