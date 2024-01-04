<script setup lang="ts">
import Button from 'primevue/button'
import router from '@/router'
import { Window } from '@tauri-apps/api/window'

const routerLink = (name: string) => {
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
  <Button label="Restful接口设计器" @click="routerLink('GenRestfulApi')"></Button>
  <Button label="[新窗口]" @click="newWindowFn('/genRestfulApi')"></Button>
  <br />
  <Button label="后端CrudService代码生成" @click="routerLink('GenCrudService')"></Button>
  <Button label="[新窗口]" @click="newWindowFn('/genCrudService')"></Button>
  <br />
  <Button label="自定义apis-util.ts代码示例" @click="routerLink('GenApisUtil')"></Button>
  <br />
  <Button label="数据库设计原则" @click="routerLink('RulesOfDatabase')"></Button>
  <Button label="[新窗口]" @click="newWindowFn('/rulesOfDatabase')"></Button>
  <br />
  <Button label="Java编写原则" @click="routerLink('RulesOfJava')"></Button>
  <Button label="[新窗口]" @click="newWindowFn('/rulesOfJava')"></Button>
  <br />
  <Button label="前端代码检查" @click="routerLink('CheckClientCode')"></Button>
  <br />
  <Button label="服务端代码检查" @click="routerLink('CheckServerCode')"></Button>
  <br />
  <Button label="服务状态检查（待完成）" @click="routerLink('CheckServiceStatus')"></Button>
  <br />
  <Button label="Wiki工具" @click="routerLink('WikiUtil')"></Button>
  <br />
  <Button label="设置" @click="routerLink('Settings')"></Button>
</template>
