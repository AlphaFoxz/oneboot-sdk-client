<script setup lang="ts">
import { createStore, Store } from '@tauri-apps/plugin-store'
import { router } from '@/plugins/router'
import { onMounted, ref } from 'vue'
import { useToast } from 'primevue/usetoast'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import { settings } from '@/constants'
import { getBasePackage } from '@/api'

const toast = useToast()
const store = ref<Store>()
async function untilReady(): Promise<void> {
  return new Promise((resolve) => {
    if (store.value) {
      resolve()
    } else {
      setTimeout(() => {
        untilReady().then(resolve)
      }, 100)
    }
  })
}
const backendHost = ref('')
const backendPort = ref('')
const tsClientGenDir = ref('')
const rustClientGenDir = ref('')
const isTestingUrl = ref(false)

onMounted(async () => {
  const storeInst = await createStore(settings.FILE_NAME)
  store.value = storeInst
  backendHost.value = (await storeInst.get(settings.KEY_BACKEND_HOST)) || '127.0.0.1'
  backendPort.value = (await storeInst.get(settings.KEY_BACKEND_PORT)) || '8080'
  tsClientGenDir.value = (await storeInst.get(settings.KEY_TS_CLIENT_GEN_DIR)) || ''
  rustClientGenDir.value = (await storeInst.get(settings.KEY_RUST_CLIENT_GEN_DIR)) || ''
})

const handleTestUrl = async () => {
  isTestingUrl.value = true

  getBasePackage()
    .then(() => {
      openSuccessNotification('连通性测试成功')
      isTestingUrl.value = false
    })
    .catch((e: Error) => {
      console.error('catch', e)
      openErrorNotification(`连通性测试失败，访问失败 http://${backendHost.value}:${backendPort.value}`)
      isTestingUrl.value = false
    })
}
const handleSaveAll = async () => {
  await untilReady()
  const storeInst = store.value!
  // FIXME https://github.com/tauri-apps/plugins-workspace/issues/1865
  await storeInst.set(settings.KEY_BACKEND_HOST, backendHost.value)
  await storeInst.set(settings.KEY_BACKEND_PORT, backendPort.value)
  await storeInst.set(settings.KEY_TS_CLIENT_GEN_DIR, tsClientGenDir.value)
  await storeInst.set(settings.KEY_RUST_CLIENT_GEN_DIR, rustClientGenDir.value)
  storeInst
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
      <label for="tsGenDir">TS客户端代码生成目录</label>
      <InputText
        id="tsGenDir"
        title="ts代码会在该目录生成，如：【配置的目录】/gen/app/AppTestApi.ts"
        placeholder="D:\projects\oneboot_front\src\apis"
        style="min-width: 40%"
        v-model="tsClientGenDir"
      />
    </div>
    <div class="flex flex-column gap-2">
      <label for="rustGenDir">Rust客户端代码生成目录</label>
      <InputText
        id="rustGenDir"
        title="ts代码会在该目录生成，如：【配置的目录】/gen/app/AppTestApi.ts"
        placeholder="D:\projects\oneboot_front\src\apis"
        style="min-width: 40%"
        v-model="rustClientGenDir"
      />
    </div>
    <div class="flex flex-column gap-2">
      <Button @click="routerLink('Home')">返回</Button>
      &emsp;
      <Button @click="handleSaveAll">保存</Button>
    </div>
  </div>
</template>
