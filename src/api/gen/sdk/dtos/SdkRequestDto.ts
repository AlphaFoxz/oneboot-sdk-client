import * as SdkCodeTemplateDto from './SdkCodeTemplateDto'

export type SdkListRequestDto = {
  id: bigint
  taskId: bigint
  data: string[]
}
export type SdkMapRequestDto = {
  id: bigint
  taskId: bigint
  data: Record<string, string>
}
export type SdkStringRequestDto = {
  id: bigint
  taskId: bigint
  data: string
}
export type SdkLongRequestDto = {
  id: bigint
  taskId: bigint
  data: bigint
}
export type SdkCodeTemplateRequestDto = {
  id: bigint
  taskId: bigint
  data: SdkCodeTemplateDto.SdkCodeTemplateDto
}
