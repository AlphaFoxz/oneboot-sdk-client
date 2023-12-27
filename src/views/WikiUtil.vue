<script setup lang="ts">
const handleCreteWikiDir = () => {}
</script>

<template>
  <a-layout class="h-full" theme="dark">
    <a-layout-content class="overflow-x-hidden">
      <a-space>
        <label>项目根目录缺少.wiki文件夹</label>
        <a-button @click="handleCreteWikiDir">创建wiki目录</a-button>
      </a-space>
    </a-layout-content>
  </a-layout>
</template>
