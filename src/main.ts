import app from './app'
import router from './router'
import PrimeModel from './primevue'
import './styles.css'

app.use(router)
PrimeModel.mountTo(app)
app.mount('#app')
