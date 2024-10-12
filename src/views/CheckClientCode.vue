<script setup lang="ts">
import * as api from '@/api'
import { useToast } from 'primevue/usetoast'
import { onMounted, ref } from 'vue'
import { createStore } from '@tauri-apps/plugin-store'
import { useRouter } from 'vue-router'
import Button from 'primevue/button'
import InputSwitch from 'primevue/inputswitch'

const toast = useToast()
const router = useRouter()
const msg = ref('')
const hideSuccessed = ref(false)
const getStore = async () => {
  const store = await createStore('.ts_code_version.dat')
  await store.save()
  return store
}
const check = async () => {
  type CheckResult = { same: boolean; message: string; path: string }
  let result: CheckResult[] = []
  // TODO
  const res = (await api.getBasePackage()) as any
  console.info('res', res)
  if (!res.data.success) {
    toast.add({ severity: 'error', summary: '服务端代码检查失败，请检查后端服务和网络连接' })
    return
  }
  const store = await getStore()
  for (let dto of res.data.data) {
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
  check().catch((e) => {
    toast.add({ severity: 'error', summary: '服务端代码检查失败，请检查后端服务和网络连接：' + e.message })
  })
})
</script>

<template>
  <div>
    <div>
      <p class="text-white">
        <Button class="text-white" label="返回" @click="router.push({ name: 'Home' })"></Button>
        <label>前端代码检查</label>
        <br />
        <label>只显示异常记录</label>
        <InputSwitch class="text-white" v-model:checked="hideSuccessed" @change="check" />
      </p>
    </div>
    <div class="bg-white text-black">
      <pre class="text-black">{{ msg }}</pre>
    </div>
  </div>
</template>
