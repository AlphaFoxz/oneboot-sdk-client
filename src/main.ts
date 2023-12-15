import { createApp } from 'vue'
import './styles.css'
import App from './App.vue'
import router from './router'

import {
  ConfigProvider,
  Layout,
  Space,
  Divider,
  Anchor,
  Menu,
  Tree,
  Form,
  FloatButton,
  Switch,
  Input,
  Select,
  Button,
} from 'ant-design-vue'

import { global_notification, global_message } from './utils'

const app = createApp(App)
app
  .use(ConfigProvider)
  .use(Layout)
  .use(Space)
  .use(Divider)
  .use(Anchor)
  .use(Menu)
  .use(Tree)
  .use(Form)
  .use(FloatButton)
  .use(Switch)
  .use(Input)
  .use(Select)
  .use(Button)

  .component('a-notification', global_notification.contextHolder)
  .component('a-message', global_message.contextHolder)

  .use(router)
  .mount('#app')
