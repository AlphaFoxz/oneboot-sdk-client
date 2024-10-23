import './assets/styles/global.scss'

import { createApp } from 'vue'
import App from './App.vue'
import './plugins/global'
import { applyRouterPlugin } from './plugins/router'
import { applyPrimeVuePlugin } from './plugins/primevue'

const app = createApp(App)
applyPrimeVuePlugin(app)
applyRouterPlugin(app)
app.mount('#app')
