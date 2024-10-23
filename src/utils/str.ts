export function upperFirst(str: string | String): string {
  return str.charAt(0).toUpperCase() + str.slice(1)
}

export function lowerFirst(str: string | String): string {
  return str.charAt(0).toLowerCase() + str.slice(1)
}

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

export function camelToUpperSnake(str: string): string {
  if (!str) return str
  str = str.replace(str[0], str[0].toLowerCase())
  return str.replace(/([A-Z])/g, '_$1').toUpperCase()
}
