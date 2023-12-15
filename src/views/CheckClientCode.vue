<script setup lang="ts">
import { SdkVersionApi } from '@/api/gen/sdk/apis/SdkVersionApi'
import { global_message } from '@/utils'
import { onMounted, ref } from 'vue'
import { Store } from '@tauri-apps/plugin-store'
import { useRouter } from 'vue-router'

const router = useRouter()
const msg = ref('')
const hideSuccessed = ref(false)
const getStore = async () => {
  const store = new Store('.ts_code_version.dat')
  await store.save()
  return store
}
const check = async () => {
  type CheckResult = { same: boolean; message: string; path: string }
  let result: CheckResult[] = []
  const res = await SdkVersionApi.getRestfulTemplateHash()
  console.info('res', res)
  if (!res.success) {
    global_message.api.error('服务端代码检查失败，请检查后端服务和网络连接')
    return
  }
  const store = await getStore()
  for (let dto of res.data) {
    const tmpResult: CheckResult = {
      same: false,
      message: '',
      path: '',
    }
    const generatedHash = await store.get(dto.filePath)
    tmpResult.same = false
    if (generatedHash === dto.sha256) {
      if (hideSuccessed.value) {
        continue
      }
      tmpResult.message = '校验成功'
      tmpResult.same = true
    } else if (generatedHash === null) {
      tmpResult.message = '未检测到版本信息，该文件可能从未生成过ts代码'
    } else if (generatedHash !== dto.sha256) {
      tmpResult.message = '版本不一致，请重新生成ts代码'
    }
    tmpResult.path = dto.filePath
    result.push(tmpResult)
  }
  let tmpMsg = ''
  result.forEach((r) => {
    tmpMsg += `${r.same ? '成功' : '异常'}，${r.message}: ${r.path}\n`
  })
  msg.value = tmpMsg
}
onMounted(async () => {
  check()
})
</script>

<template>
  <a-layout>
    <a-layout-header>
      <p class="text-white">
        <a-button class="text-white" @click="router.push({ name: 'Home' })">返回</a-button>
        前端代码检查
        <a-switch
          class="text-white"
          v-model:checked="hideSuccessed"
          checked-children="过滤一致记录"
          un-checked-children="显示全部结果"
          @change="check"
        >
        </a-switch>
      </p>
    </a-layout-header>
    <a-layout-content>
      <pre class="text-black">{{ msg }}</pre>
    </a-layout-content>
  </a-layout>
</template>
