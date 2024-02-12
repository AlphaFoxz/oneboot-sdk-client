<script setup lang="ts">
import { useRouter } from 'vue-router'
import Button from 'primevue/button'

const router = useRouter()
</script>

<template>
  <div style="min-height: 100%">
    <div style="min-height: 7vh">
      <Button label="返回" class="text-white" @click="router.push({ name: 'Home' })"></Button>
      <label class="text-white">Restful语法说明</label>
    </div>
    <iframe
      width="100%"
      style="min-height: 93vh"
      frameborder="0"
      src="https://alphafoxz.github.io/restful-dsl-java/"
    ></iframe>
  </div>
</template>

<style scoped>
p {
  font-size: 1rem;
  padding: 0.7rem;
}
h1 {
  font-weight: bold;
  padding: 1.5rem 0;
}
h2 {
  font-size: 1.2rem;
  font-weight: bold;
  text-align: center;
}
</style>
