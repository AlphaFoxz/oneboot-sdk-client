<script setup lang="ts">
import Button from 'primevue/button'

const handleCreteWikiDir = () => {}
</script>

<template>
  <label>项目根目录缺少.wiki文件夹</label>
  <Button label="创建wiki目录" @click="handleCreteWikiDir"></Button>
</template>
