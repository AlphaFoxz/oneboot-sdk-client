<script setup lang="ts">
import Toast from 'primevue/toast'
import LayoutSpace from '@/components/layout/Space.vue'
import DynamicDialog from 'primevue/dynamicdialog'
import ThemeAndLocale from './components/ThemeAndLocale.vue'
</script>

<template>
  <LayoutSpace>
    <DynamicDialog />
    <Toast />
    <Toast position="bottom-right" group="br" />

    <LayoutSpace class="header">
      <ThemeAndLocale />
    </LayoutSpace>
    <LayoutSpace><router-view></router-view></LayoutSpace>
  </LayoutSpace>
</template>

<style lang="scss" scoped>
.header {
  width: 100%;
}
</style>
