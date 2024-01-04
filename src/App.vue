<script setup lang="ts">
import Toast from 'primevue/toast'
import DynamicDialog from 'primevue/dynamicdialog'
import 'primevue/resources/themes/lara-dark-amber/theme.css'
</script>

<template>
  <router-view></router-view>
  <DynamicDialog />
  <Toast />
  <Toast position="bottom-right" group="br" />
</template>
