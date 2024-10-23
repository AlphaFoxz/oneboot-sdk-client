<script setup lang="ts">
import Toast from 'primevue/toast'
import Button from 'primevue/button'
import LayoutFlex from '@/components/layout/Flex.vue'
import LayoutGrid from '@/components/layout/Grid.vue'
import DynamicDialog from 'primevue/dynamicdialog'
import ThemeAndLocale from './components/ThemeAndLocale.vue'
import { useRouter } from 'vue-router'

const router = useRouter()
</script>

<template>
  <LayoutGrid wrapped class="container">
    <DynamicDialog />
    <Toast />
    <LayoutGrid wrapped margin="0" class="header">
      <Button label="返回" text @click="router.back()" class="back-btn"></Button>
      <LayoutFlex wrapped>
        <Button
          label="主页"
          text
          size="large"
          security="info"
          @click="router.push({ name: 'Home' })"
          class="home-btn"
        ></Button>
      </LayoutFlex>
      <div>
        <ThemeAndLocale />
      </div>
    </LayoutGrid>
    <LayoutFlex><router-view></router-view></LayoutFlex>
  </LayoutGrid>
</template>

<style lang="scss" scoped>
.container {
  width: 100%;
  height: 100%;
  grid-template-rows: 2.5rem 1fr;

  .header {
    width: 100%;
    height: 100%;
    grid-template-columns: 200px auto 200px;

    .back-btn {
      width: 200px;
    }
    .home-btn {
      width: 200px;
    }
  }
}
</style>
