<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useToast } from 'primevue/usetoast'
import { checkRestfulFileVersion } from '@/api/rust_api'
import { useRouter } from 'vue-router'
import Button from 'primevue/button'
import InputSwitch from 'primevue/inputswitch'

const toast = useToast()
const router = useRouter()
const msg = ref('')
const hideSuccessed = ref(false)
const check = async () => {
  let result = ''
  const res = await checkRestfulFileVersion()
  console.info('res', res)
  if (!res) {
    toast.add({ severity: 'error', summary: '服务端代码检查失败，请检查后端服务和网络连接' })
    return
  }
  for (let dto of res.data) {
    if (hideSuccessed.value && dto.same) {
      continue
    }
    result += `${dto.same ? '成功' : '异常'}，${dto.message}: ${dto.filePath}\n`
  }
  msg.value = result
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
        <Button class="text-white" @click="router.push({ name: 'Home' })">返回</Button>
        <label>服务端代码检查</label>
        <br />
        <label>只显示异常记录</label>
        <InputSwitch class="text-white" v-model="hideSuccessed" @change="check" />
      </p>
    </div>
    <div class="bg-white text-black">
      <pre class="text-black">{{ msg }}</pre>
    </div>
  </div>
</template>
