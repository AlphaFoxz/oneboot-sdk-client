import { ref, readonly, watchEffect } from 'vue'

type ColorModeType = 'light' | 'dark'
const currentColorMode = ref<ColorModeType>('dark')
const svgColor = ref('#fff')
function setColorMode(mode: ColorModeType) {
  currentColorMode.value = mode
}
watchEffect(() => {
  const v = currentColorMode.value
  if (v === 'dark') {
    document.documentElement.classList.add('dark-mode')
    svgColor.value = '#fff'
  } else if (v === 'light') {
    document.documentElement.classList.remove('dark-mode')
    svgColor.value = '#000'
  } else {
    isNever(v)
  }
})

const api = {
  state: {
    colorMode: readonly(currentColorMode),
    svgColor: readonly(svgColor),
  },
  action: {
    setColorMode,
  },
}
export function useUserPreferenceStore() {
  return api
}
