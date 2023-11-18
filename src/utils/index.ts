import { notification, message } from "ant-design-vue"

const [notificationApi, notificationContextHolder] = notification.useNotification()
/**
 * 全局通知
*/
export const global_notification = { api: notificationApi, contextHolder: notificationContextHolder }

const [messageApi, messageContextHolder] = message.useMessage()
/**
 * 全局消息
 */
export const global_message = { api: messageApi, contextHolder: messageContextHolder }

/**
 * 函数防抖（间隔不超过delay毫秒的多次调用会被合并为最后一次调用，最终延迟时间为[delay]毫秒）
 * @param fn 目标函数
 */
export function debounce(fn: Function, delay: number = 500): Function {
  let timer: NodeJS.Timeout
  return function () {
    if (timer) {
      clearTimeout(timer)
    }
    timer = setTimeout(() => {
      fn()
    }, delay)
  }
}

/**
 * 函数节流（每次触发后，在delay毫秒内不再触发）
 * @param fn 目标函数
 */
export function throttle(fn: Function, delay: number = 500): Function {
  let refuse = false
  return () => {
    if (refuse) return
    refuse = true
    fn()
    setTimeout(() => {
      refuse = false
    }, delay)
  }
}

/**
 * 生成一个defer方法用于控制模板组件的加载顺序（通过传入的帧数判断）
 * @param maxCount 最大帧数，传入负数将在mounted之前一直循环
 * @returns defer方法，如：v-if="defer(1)" 表示从第一帧开始加载组件
 */
export function useDefer(maxCount = 100) {
  const frameCount = ref(0)
  let refId: number
  function updateFrameCount() {
    refId = requestAnimationFrame(() => {
      frameCount.value++
      maxCount--
      if (maxCount !== 0) {
        updateFrameCount()
      }
    })
  }
  if (maxCount !== 0) {
    updateFrameCount()
  }
  onMounted(() => cancelAnimationFrame(refId))
  return (n: number) => {
    return frameCount.value >= n
  }
}

/**
 * Camel转SNAKE_CASE
 */
export function camelToUpperSnake(str: string): string {
  if (!str) return str
  str = str.replace(str[0], str[0].toLowerCase())
  return str.replace(/([A-Z])/g, "_$1").toUpperCase()
}

/**
 * Camel转SNAKE_CASE
 */
export function snakeToUpperCamel(str: string): string {
  if (!str) return str
  str = str.toLowerCase()
  str = str.replace(str[0], str[0].toUpperCase())
  return str.replace(/_(\w)/g, (_, c) => c.toUpperCase())
}

import * as rust_api from './rust_api'
import * as dyn_component from './dyn_component'
import { onMounted, ref } from 'vue'
export {
  rust_api,
  dyn_component,
}
