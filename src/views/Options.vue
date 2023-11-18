<script setup lang="ts">
import { invoke } from '@tauri-apps/api/primitives'
import router from '@/router'
import { ref } from 'vue'
import { global_notification } from '@/utils'

const backendHost = ref('')
invoke('get_option_tree_value_command', { key: 'OPTIONS:BACKEND_HOST' }).then(
  (str) => (backendHost.value = str as string)
)
const backendPort = ref('')
invoke('get_option_tree_value_command', { key: 'OPTIONS:BACKEND_PORT' }).then(
  (str) => (backendPort.value = str as string)
)
const tsGenDir = ref('')
invoke('get_option_tree_value_command', { key: 'OPTIONS:TS_GEN_DIR' }).then((str) => (tsGenDir.value = str as string))

const saveAllHandler = async () => {
  const result =
    (await invoke('set_option_tree_value_command', { key: 'OPTIONS:BACKEND_HOST', value: backendHost.value })) &&
    (await invoke('set_option_tree_value_command', { key: 'OPTIONS:BACKEND_PORT', value: backendPort.value })) &&
    (await invoke('set_option_tree_value_command', { key: 'OPTIONS:TS_GEN_DIR', value: tsGenDir.value }))
  if (result) {
    openSuccessNotification('配置已保存')
  } else {
    openErrorNotification('保存失败，请检查是否有网络错误')
  }
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
