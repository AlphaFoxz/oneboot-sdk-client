<script setup lang="ts">
import { ref, watchEffect } from 'vue'

const props = defineProps({
  wrapFlex: {
    type: Boolean,
    default: false,
  },
  wrapGrid: {
    type: Boolean,
    default: false,
  },
  margin: {
    type: String,
    default: '0.5rem',
  },
  padding: {
    type: String,
    default: '0',
  },
})

const clazz = ref('')
watchEffect(() => {
  const defaultClazz = ['margin', 'padding']
  if (props.wrapFlex) {
    defaultClazz.push('flex')
  } else if (props.wrapGrid) {
    defaultClazz.push('grid')
  }
  clazz.value = defaultClazz.join(' ')
})
</script>

<template>
  <template v-if="wrapFlex">
    <div :class="clazz">
      <slot></slot>
    </div>
  </template>
  <template v-else-if="wrapGrid">
    <div :class="clazz">
      <slot></slot>
    </div>
  </template>
  <template v-else>
    <slot :class="clazz"></slot>
  </template>
</template>

<style scoped lang="scss">
.margin {
  margin: v-bind(margin);
}
.padding {
  padding: v-bind(padding);
}
.flex {
  display: flex;
  justify-content: center;
  align-items: center;
}
.grid {
  display: grid;
  grid-auto-columns: auto;
  grid-auto-rows: auto;
}
</style>
