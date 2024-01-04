//利用pinia实现状态管理（数据总线）
import { createPinia, SubscriptionCallback } from 'pinia'
import { hotkeyStore, HotkeyState } from './HotkeyStore'
import { editorStore } from './EditorStore'
import { onUnmounted } from 'vue'

/**
 * init pinia
 * 初始化pinia
 */
const pinia = createPinia()

export function getEditorStore(subscribeFn?: SubscriptionCallback<any>) {
  const store = editorStore(pinia)
  if (subscribeFn) {
    store.$subscribe(subscribeFn)
  }
  return store
}

export function getHotkeyStore(dom: HTMLElement, subscribeFn: SubscriptionCallback<HotkeyState>) {
  const store = hotkeyStore(pinia)
  store.mounted(dom)
  onUnmounted(() => {
    store.unmount(dom)
  })
  if (subscribeFn) {
    store.$subscribe(subscribeFn)
  }
  return store
}
