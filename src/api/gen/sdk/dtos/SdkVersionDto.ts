/**
 * 版本检查响应
 */
export type SdkVersionCheckResponse = {
  id: bigint | undefined
  taskId: bigint
  success: boolean
  message: string | undefined
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
