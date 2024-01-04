import PrimeVue from 'primevue/config'
import { App } from 'vue'
import ToastService from 'primevue/toastservice'
import 'primevue/resources/themes/lara-dark-amber/theme.css'

type Plugin = { plugin: any; options?: Record<string, any> }
PrimeVue.install
const toastPlugins: Plugin[] = [
  { plugin: ToastService, options: undefined },
  { plugin: PrimeVue, options: undefined },
]

export default {
  toastPlugins,
  mountTo(app: App<any>) {
    this.toastPlugins.forEach((item) => {
      app.use(item.plugin, item.options)
    })
  },
}
