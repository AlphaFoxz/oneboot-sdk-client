<script setup lang="ts">
import { ref } from 'vue'
import ToggleSwitch from 'primevue/toggleswitch'
import Popover from 'primevue/popover'
import Listbox from 'primevue/listbox'
import Button from 'primevue/button'
import IconsSun from '@/components/icons/Sun.vue'
import IconsMoon from '@/components/icons/Moon.vue'
import IconsLocale from '@/components/icons/Locale.vue'
import { useUserPreferenceStore } from '@/stores/user-preference-store'

const opRef = ref()
const userPreferenceStore = useUserPreferenceStore()
const isLightTheme = ref(userPreferenceStore.state.colorMode.value === 'light')
const locale = ref()
const handleThemeChange = () => {
  userPreferenceStore.action.setColorMode(isLightTheme.value ? 'light' : 'dark')
}
function toggle(event: MouseEvent) {
  opRef.value.toggle(event)
}
</script>

<template>
  <div class="theme-lang">
    <ToggleSwitch v-model="isLightTheme" @change="handleThemeChange">
      <template #handle="{ checked }">
        <IconsSun v-if="checked" style="width: 14px; height: 22px" />
        <IconsMoon v-else style="width: 14px; height: 22px" />
      </template>
    </ToggleSwitch>
    <Button text @click="toggle">
      <IconsLocale style="width: 20px; height: 20px; margin-left: 6px"></IconsLocale>
    </Button>
    <Popover ref="opRef" style="background-color: rgba(0, 0, 0, 0)">
      <Listbox
        v-model="locale"
        :options="[
          { label: '简体中文', value: 'zh-CN' },
          { label: 'English', value: 'en-US' },
        ]"
        optionLabel="label"
        optionValue="value"
      ></Listbox>
    </Popover>
  </div>
</template>

<style scoped>
.theme-lang {
  position: absolute;
  float: right;
  top: 0.75rem;
  right: 1.25rem;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
