<script setup lang="ts">
import { SdkVersionApi } from '@/api/gen/sdk/apis/SdkVersionApi'
import { onMounted, ref } from 'vue'
import { global_message } from '@/utils'
import { useRouter } from 'vue-router'

const router = useRouter()
const msg = ref('')
const hideSuccessed = ref(false)
const check = async () => {
  let result = ''
  const res = await SdkVersionApi.checkRestfulJava()
  console.info('res', res)
  if (!res.success) {
    global_message.api.error('服务端代码检查失败，请检查后端服务和网络连接')
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
  check()
})
</script>

<template>
  <a-layout>
    <a-layout-header>
      <p class="text-white">
        <a-button class="text-white" @click="router.push({ name: 'Home' })">返回</a-button>
        服务端代码检查
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
    <pre class="text-black">{{ msg }}</pre>
  </a-layout>
</template>
