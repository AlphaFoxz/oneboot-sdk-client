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
  <a-button type="primary" @click="routerGo('HelpApisUtil')">自定义apisUtil.ts代码示例</a-button>
  <br />
  <a-button type="primary" @click="routerGo('HelpDatabase')">数据库设计准则</a-button>
  <br />
  <a-button type="primary" @click="routerGo('Options')">设置</a-button>
</template>
