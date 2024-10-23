<script setup lang="ts">
import LayoutFlex from '@/components/layout/Flex.vue'
import Button from 'primevue/button'
import { settings } from '@/constants'
import { createVoMapperDomain } from '@/domains/vo-mapper'
import { Store } from '@tauri-apps/plugin-store'
import { onMounted, shallowRef } from 'vue'

const voMapperDomain = shallowRef<ReturnType<typeof createVoMapperDomain>>()
const store = Store.load(settings.FILE_NAME)
onMounted(async () => {
  const storeInst = await store
  voMapperDomain.value = createVoMapperDomain(
    (await storeInst.get(settings.KEY_PROJECT_ROOT_DIR)) || 'F:\\idea_projects\\oneboot',
    'domain-preset_sys',
    'preset_sys',
    'com.github.alphafoxz.oneboot'
  )
})

function gen() {
  voMapperDomain.value?.actions.generate()
}
</script>

<template>
  <div>
    <LayoutFlex wrapped>
      <Button label="try" @click="gen"></Button>
    </LayoutFlex>
  </div>
</template>
