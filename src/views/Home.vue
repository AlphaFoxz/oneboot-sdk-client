<script setup lang="ts">
import Button from 'primevue/button'
import { router } from '@/plugins/router'
import { nanoid } from 'nanoid'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

const routerLink = (name: string) => {
  router.push({ name })
}
const newWindowFn = (url: string, dragDropEnabled = false) => {
  const w = new WebviewWindow('main-' + nanoid(), {
    url,
    minHeight: 600,
    minWidth: 800,
    width: 1366,
    height: 768,
    dragDropEnabled,
  })
  w.once('tauri://error', (e) => {
    console.error(e)
  })
}
</script>

<template>
  <h1>Oneboot SDK模块 开发工具</h1>
  <Button label="RESTful-DSL接口设计器" @click="routerLink('GenRestfulApi')"></Button>
  <Button label="[新窗口]" @click="newWindowFn('/genRestfulApi')"></Button>
  <br />
  <Button label="RESTful-DSL语法说明" @click="routerLink('RulesOfRestfulLanguage')"></Button>
  <Button label="[新窗口]" @click="newWindowFn('/rulesOfRestfulLanguage')"></Button>
  <br />
  <Button label="RESTful-DSL服务端代码检查" @click="routerLink('CheckServerRestfulCode')"></Button>
  <Button label="[新窗口]" @click="newWindowFn('/checkServerRestfulCode')"></Button>
  <br />
  <Button label="RESTful-DSL前端代码检查（待完成）" @click="routerLink('CheckClientCode')"></Button>
  <br />
  <Button label="领域设计生成器" @click="routerLink('GenDomain')"></Button>
  <br />
  <Button label="后端CrudService代码生成" @click="routerLink('GenCrudService')"></Button>
  <Button label="[新窗口]" @click="newWindowFn('/genCrudService')"></Button>
  <br />
  <Button label="生成Api文档" @click="routerLink('GenApiDocs')"></Button>
  <br />
  <Button label="自定义apis-util.ts代码示例" @click="routerLink('GenApisUtil')"></Button>
  <br />
  <Button label="数据库设计原则" @click="routerLink('RulesOfDatabase')"></Button>
  <Button label="[新窗口]" @click="newWindowFn('/rulesOfDatabase')"></Button>
  <br />
  <Button label="Java编写原则" @click="routerLink('RulesOfJava')"></Button>
  <Button label="[新窗口]" @click="newWindowFn('/rulesOfJava')"></Button>
  <br />
  <Button label="服务状态检查（待完成）" @click="routerLink('CheckServiceStatus')"></Button>
  <br />
  <Button label="设置" @click="routerLink('Settings')"></Button>
</template>
