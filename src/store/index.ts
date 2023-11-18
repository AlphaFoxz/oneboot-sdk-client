//利用pinia实现状态管理（数据总线）
import { createPinia, SubscriptionCallback } from 'pinia'
import { configStore } from './ConfigStore'
import { hotkeyStore, HotkeyState } from './HotkeyStore'
import { editorStore } from './EditorStore'
import { onUnmounted } from 'vue'

/**
 * init pinia
 * 初始化pinia
 */
const pinia = createPinia()

function getEditorStore(subscribeFn?: SubscriptionCallback<any>) {
  const store = editorStore(pinia)
  if (subscribeFn) {
    store.$subscribe(subscribeFn)
  }
  return store
}

function getConfigStore(subscribeFn?: SubscriptionCallback<any>) {
  const store = configStore(pinia)
  if (subscribeFn) {
    store.$subscribe(subscribeFn)
  }
  return store
}

function getHotkeyStore(dom: HTMLElement, subscribeFn: SubscriptionCallback<HotkeyState>) {
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

export { getConfigStore, getHotkeyStore, getEditorStore }
