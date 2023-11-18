<script setup lang="ts">
import { ref } from 'vue'
import { getConfigStore } from '@/store'
class AntdvOptions {
  componentSize: string = 'large'
  theme = {
    token: {},
  }
  space = {
    align: <'start' | 'end' | 'center' | 'baseline'>'start',
    direction: <'vertical' | 'horizontal'>'horizontal',
    size: <'small' | 'middle' | 'large' | number>'middle',
  }
}
const options = ref(new AntdvOptions())
const configStore = getConfigStore((_, state) => {
  options.value.componentSize = state.componentSize
})
options.value.componentSize = configStore.$state.componentSize
</script>

<template>
  <a-config-provider :componentSize="options.componentSize" :space="options.space" :theme="options.theme">
    <router-view></router-view>
    <a-notification></a-notification>
    <a-message></a-message>
  </a-config-provider>
</template>
