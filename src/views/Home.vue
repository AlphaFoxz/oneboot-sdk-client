<script setup lang="ts">
import router from '@/router'
import { Window } from '@tauri-apps/api/window'

const routerGo = (name: string) => {
  router.push({ name })
}
const newWindowFn = (url: string) => {
  new Window('新窗口', {
    url,
    minHeight: 600,
    minWidth: 800,
    width: 1366,
    height: 768,
    fileDropEnabled: false,
  })
}
</script>

<template>
  <h1>Oneboot SDK模块 开发工具</h1>
  <a-button type="primary" @click="routerGo('GenRestfulApi')">Restful接口设计器</a-button>
  <a-button type="primary" @click="newWindowFn('/genRestfulApi')">[新窗口]</a-button>
  <br />
  <a-button type="primary" @click="routerGo('GenCrudService')">后端CrudService代码生成</a-button>
  <a-button type="primary" @click="newWindowFn('/genCrudService')">[新窗口]</a-button>
  <br />
  <a-button type="primary" @click="routerGo('GenApisUtil')">自定义apisUtil.ts代码示例</a-button>
  <br />
  <a-button type="primary" @click="routerGo('RulesOfDatabase')">数据库设计原则</a-button>
  <a-button type="primary" @click="newWindowFn('/rulesOfDatabase')">[新窗口]</a-button>
  <br />
  <a-button type="primary" @click="routerGo('RulesOfJava')">Java编写原则</a-button>
  <a-button type="primary" @click="newWindowFn('/rulesOfJava')">[新窗口]</a-button>
  <br />
  <a-button type="primary" @click="routerGo('CheckClientCode')">前端代码检查</a-button>
  <br />
  <a-button type="primary" @click="routerGo('CheckServerCode')">服务端代码检查</a-button>
  <br />
  <a-button type="primary" @click="routerGo('CheckServiceStatus')">服务状态检查（待完成）</a-button>
  <br />
  <a-button type="primary" @click="routerGo('WikiUtil')">Wiki工具</a-button>
  <br />
  <a-button type="primary" @click="routerGo('Settings')">设置</a-button>
</template>
