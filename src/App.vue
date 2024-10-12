<script setup lang="ts">
import Toast from 'primevue/toast'
import DynamicDialog from 'primevue/dynamicdialog'
import ThemeAndLocale from './components/ThemeAndLocale.vue'
</script>

<template>
  <DynamicDialog />
  <Toast />
  <Toast position="bottom-right" group="br" />
  <router-view></router-view>
  <ThemeAndLocale />
</template>
