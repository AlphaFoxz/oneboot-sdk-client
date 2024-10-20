<script setup lang="ts">
import Button from 'primevue/button'
import LayoutFlex from '@/components/layout/Flex.vue'
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
  <div class="container">
    <LayoutFlex wrapped class="item">
      <Button label="DDD设计器" @click="routerLink('DddDesigner')"></Button>
      <Button label="[新窗口]" @click="newWindowFn('/dddDesigner')"></Button>
    </LayoutFlex>
    <LayoutFlex wrapped class="item">
      <Button label="RESTful-DSL接口设计器" @click="routerLink('GenRestfulApi')"></Button>
      <Button label="[新窗口]" @click="newWindowFn('/genRestfulApi')"></Button>
    </LayoutFlex>
    <LayoutFlex wrapped class="item">
      <Button label="RESTful-DSL语法说明" @click="routerLink('RulesOfRestfulLanguage')"></Button>
      <Button label="[新窗口]" @click="newWindowFn('/rulesOfRestfulLanguage')"></Button>
    </LayoutFlex>
    <LayoutFlex wrapped class="item">
      <Button label="RESTful-DSL服务端代码检查" @click="routerLink('CheckServerRestfulCode')"></Button>
      <Button label="[新窗口]" @click="newWindowFn('/checkServerRestfulCode')"></Button>
    </LayoutFlex>
    <LayoutFlex wrapped class="item">
      <Button label="RESTful-DSL前端代码检查（待完成）" @click="routerLink('CheckClientCode')"></Button>
    </LayoutFlex>
    <LayoutFlex wrapped class="item">
      <Button label="后端CrudService代码生成" @click="routerLink('GenCrudService')"></Button>
      <Button label="[新窗口]" @click="newWindowFn('/genCrudService')"></Button>
    </LayoutFlex>
    <LayoutFlex wrapped class="item">
      <Button label="生成Api文档" @click="routerLink('GenApiDocs')"></Button>
    </LayoutFlex>
    <LayoutFlex wrapped class="item">
      <Button label="生成值对象映射（MapStruct）" @click="routerLink('GenMapStruct')"></Button>
    </LayoutFlex>
    <LayoutFlex wrapped class="item">
      <Button label="自定义apis-util.ts代码示例" @click="routerLink('GenApisUtil')"></Button>
    </LayoutFlex>
    <LayoutFlex wrapped class="item">
      <Button label="数据库设计原则" @click="routerLink('RulesOfDatabase')"></Button>
      <Button label="[新窗口]" @click="newWindowFn('/rulesOfDatabase')"></Button>
    </LayoutFlex>
    <LayoutFlex wrapped class="item">
      <Button label="Java编写原则" @click="routerLink('RulesOfJava')"></Button>
      <Button label="[新窗口]" @click="newWindowFn('/rulesOfJava')"></Button>
    </LayoutFlex>
    <LayoutFlex wrapped class="item">
      <Button label="服务状态检查（待完成）" @click="routerLink('CheckServiceStatus')"></Button>
    </LayoutFlex>
    <LayoutFlex wrapped class="item">
      <Button label="设置" @click="routerLink('Settings')"></Button>
    </LayoutFlex>
  </div>
</template>

<style lang="scss" scoped>
.container {
  height: 100%;
  width: 100%;
  margin: auto 0;

  .item {
    margin: 0.5rem;
    flex-direction: unset;
    justify-content: center;
  }
}
</style>
