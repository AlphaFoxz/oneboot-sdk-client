
/**
 * 版本检查响应
 */
export type SdkVersionCheckResponse = {
  id: bigint | null
  taskId: bigint
  success: boolean
  message: string | null
  data: SdkVersionCheckDto[]
}
/**
 * 版本检查结果
 */
export type SdkVersionCheckDto = {
  filePath: string
  sha256: string
  same: boolean
  message: string
}
