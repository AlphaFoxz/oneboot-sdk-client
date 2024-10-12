import { onMounted, ref } from 'vue'

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
 * Prefetches the specified paths by adding <link> elements to the document head.
 *
 * @param {string[]} paths - An array of paths to be prefetched.
 */
export function prefetchLink(paths: string[]) {
  if (!paths.length) return
  paths.forEach((item) => {
    const dom = document.createElement('link')
    dom.rel = 'prefetch'
    dom.href = item
    dom.as = 'script'
    document.head.appendChild(dom)
  })
}

/**
 * Finds the longest common prefix among an array of strings.
 *
 * @param {string[]} strs - The array of strings to find the longest common prefix from.
 * @return {string} - The longest common prefix string.
 */
export function longestCommonPrefix(...strs: string[]): string {
  if (!strs.length) return ''
  let [a, ...b] = strs
  let result = ''
  for (let i = 0; i < a.length; i++) {
    let flag = b.every((item) => item[i] === a[i])
    if (flag) result += a[i]
    else break
  }
  return result
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
  return str.replace(/([A-Z])/g, '_$1').toUpperCase()
}

/**
 * Camel转SNAKE_CASE
 */
export function snakeToLowerCamel(str: string): string {
  if (!str) return str
  str = str.toLowerCase()
  return str.replace(/_(\w)/g, (_, c) => c.toUpperCase())
}
export function snakeToUpperCamel(str: string): string {
  if (!str) return str
  str = str.toLowerCase()
  str = str.replace(str[0], str[0].toUpperCase())
  return str.replace(/_(\w)/g, (_, c) => c.toUpperCase())
}
