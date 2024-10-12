import './assets/styles/global.css'

import { createApp } from 'vue'
import App from './App.vue'
import { applyRouterPlugin } from './plugins/router'
import { applyPrimeVuePlugin } from './plugins/primevue'

const app = createApp(App)
applyRouterPlugin(app)
applyPrimeVuePlugin(app)
app.mount('#app')
