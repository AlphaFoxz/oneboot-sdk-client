<script setup lang="ts">
import { computed, ref } from 'vue'

const props = defineProps({
  margin: {
    type: String,
    default: '0',
  },
  padding: {
    type: String,
    default: '0',
  },
  wrapped: {
    type: Boolean,
    default: false,
    validator: (wrappedHtmlTag: string | boolean) => {
      if (typeof wrappedHtmlTag === 'string') {
        const reg = /^[a-z]+$/
        return reg.test(wrappedHtmlTag)
      }
      return true
    },
  },
})

const clazz = ref(props.wrapped ? 'flex' : 'flex margin padding')
const wrappedType = computed(() => {
  let tag = 'div'
  if (props.wrapped === false) {
    return null
  } else if (typeof props.wrapped === 'string') {
    tag = props.wrapped
  }
  return tag
})
</script>

<template>
  <template v-if="!wrapped" :class="`${clazz} ${$attrs.class}`"><slot></slot></template>
  <component v-else :is="wrappedType" :class="`${clazz} ${$attrs.class}`"><slot></slot></component>
</template>

<style scoped>
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
</style>
