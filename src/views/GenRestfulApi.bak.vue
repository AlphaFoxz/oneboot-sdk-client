<script setup lang="ts">
import VMonacoEditor from "@/components/v-monaco-editor/Index.vue"
import { onMounted, ref } from "vue"
import * as utils from "@/utils"

const codeStr = ref('')
let dto = ref<any>({})
utils.rust_api.getRestfulTemplateFileTree().then(res => {
  dto.value = res
})
onMounted(() => {
})
</script>

<template>
  <v-monaco-editor :model-value="codeStr" :file-info="dto" language="restful" />
</template>