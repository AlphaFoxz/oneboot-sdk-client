import { App } from 'vue'
import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'
// import Lara from '@primevue/themes/lara'
import ToastService from 'primevue/toastservice'

export function applyPrimeVuePlugin(app: App) {
  app.use(PrimeVue, {
    theme: {
      preset: Aura,
      options: {
        prefix: 'p',
        darkModeSelector: 'system',
        cssLayer: false,
      },
    },
  })
  app.use(ToastService)
}