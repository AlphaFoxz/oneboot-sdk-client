import { ref, readonly } from 'vue'

type ColorModeType = 'light' | 'dark'
const currentColorMode = ref('dark')

const api = {
  state: {
    currentColorMode: readonly(currentColorMode),
  },
  action: {
    setCurrentColorMode(mode: ColorModeType) {
      currentColorMode.value = mode
    },
  },
}
export function useUserPreferenceStore() {
  return api
}
