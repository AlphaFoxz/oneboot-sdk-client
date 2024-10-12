<script setup lang="ts">
import Button from 'primevue/button'
import LayoutSpace from '@/components/layout/Space.vue'
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
  <LayoutSpace wrap-flex class="item">
    <h1>Oneboot SDK模块 开发工具</h1>
  </LayoutSpace>
  <LayoutSpace wrap-flex class="item">
    <Button label="DDD设计器" @click="routerLink('DddDesigner')"></Button>
    <Button label="[新窗口]" @click="newWindowFn('/dddDesigner')"></Button>
  </LayoutSpace>
  <LayoutSpace wrap-flex class="item">
    <Button label="RESTful-DSL接口设计器" @click="routerLink('GenRestfulApi')"></Button>
    <Button label="[新窗口]" @click="newWindowFn('/genRestfulApi')"></Button>
  </LayoutSpace>
  <LayoutSpace wrap-flex class="item">
    <Button label="RESTful-DSL语法说明" @click="routerLink('RulesOfRestfulLanguage')"></Button>
    <Button label="[新窗口]" @click="newWindowFn('/rulesOfRestfulLanguage')"></Button>
  </LayoutSpace>
  <LayoutSpace wrap-flex class="item">
    <Button label="RESTful-DSL服务端代码检查" @click="routerLink('CheckServerRestfulCode')"></Button>
    <Button label="[新窗口]" @click="newWindowFn('/checkServerRestfulCode')"></Button>
  </LayoutSpace>
  <LayoutSpace wrap-flex class="item">
    <Button label="RESTful-DSL前端代码检查（待完成）" @click="routerLink('CheckClientCode')"></Button>
  </LayoutSpace>
  <LayoutSpace wrap-flex class="item">
    <Button label="后端CrudService代码生成" @click="routerLink('GenCrudService')"></Button>
    <Button label="[新窗口]" @click="newWindowFn('/genCrudService')"></Button>
  </LayoutSpace>
  <LayoutSpace wrap-flex class="item">
    <Button label="生成Api文档" @click="routerLink('GenApiDocs')"></Button>
  </LayoutSpace>
  <LayoutSpace wrap-flex class="item">
    <Button label="自定义apis-util.ts代码示例" @click="routerLink('GenApisUtil')"></Button>
  </LayoutSpace>
  <LayoutSpace wrap-flex class="item">
    <Button label="数据库设计原则" @click="routerLink('RulesOfDatabase')"></Button>
    <Button label="[新窗口]" @click="newWindowFn('/rulesOfDatabase')"></Button>
  </LayoutSpace>
  <LayoutSpace wrap-flex class="item">
    <Button label="Java编写原则" @click="routerLink('RulesOfJava')"></Button>
    <Button label="[新窗口]" @click="newWindowFn('/rulesOfJava')"></Button>
  </LayoutSpace>
  <LayoutSpace wrap-flex class="item">
    <Button label="服务状态检查（待完成）" @click="routerLink('CheckServiceStatus')"></Button>
  </LayoutSpace>
  <LayoutSpace wrap-flex class="item">
    <Button label="设置" @click="routerLink('Settings')"></Button>
  </LayoutSpace>
</template>

<style lang="scss" scoped>
.item {
  justify-content: center;
}
</style>
