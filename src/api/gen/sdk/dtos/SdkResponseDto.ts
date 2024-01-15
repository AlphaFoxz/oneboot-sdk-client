import * as SdkCodeTemplateDto from './SdkCodeTemplateDto'
import * as SdkFileInfoDto from './SdkFileInfoDto'
// 响应体的data字段是list
/**
 * 字符串列表响应实体
 */
export type SdkListResponseDto = {
  /**
   * 主键
   */
  id: bigint
  /**
   * 任务id
   */
  taskId: bigint
  /**
   * 是否成功
   */
  success: boolean
  /**
   * 消息
   */
  message: string | undefined
  /**
   * 数据内容
   */
  data: string[] | undefined
}
// 响应体的data字段是map
/**
 * 字符串map响应实体
 */
export type SdkMapResponseDto = {
  /**
   * 主键
   */
  id: bigint
  /**
   * 任务id
   */
  taskId: bigint
  /**
   * 是否成功
   */
  success: boolean
  /**
   * 消息
   */
  message: string | undefined
  /**
   * 数据内容
   */
  data: Record<string, string> | undefined
}
// 响应体的data字段是long
/**
 * 长整型响应实体
 */
export type SdkLongResponseDto = {
  /**
   * 主键
   */
  id: bigint
  /**
   * 任务id
   */
  taskId: bigint
  /**
   * 是否成功
   */
  success: boolean
  /**
   * 消息
   */
  message: string | undefined
  /**
   * 数据内容
   */
  data: bigint | undefined
}
// 响应体的data字段是double
/**
 * 双精度小数响应实体
 */
export type SdkDoubleResponseDto = {
  /**
   * 主键
   */
  id: bigint
  /**
   * 任务id
   */
  taskId: bigint
  /**
   * 是否成功
   */
  success: boolean
  /**
   * 消息
   */
  message: string | undefined
  /**
   * 数据内容
   */
  data: number | undefined
}
// 响应体的data字段是binary
/**
 * 二进制响应实体
 */
export type SdkBinaryResponseDto = {
  /**
   * 主键
   */
  id: bigint
  /**
   * 任务id
   */
  taskId: bigint
  /**
   * 是否成功
   */
  success: boolean
  /**
   * 消息
   */
  message: string | undefined
  /**
   * 数据内容
   */
  data: string | undefined
}
// 响应体的data字段是string
/**
 * 字符串响应实体
 */
export type SdkStringResponseDto = {
  /**
   * 主键
   */
  id: bigint
  /**
   * 任务id
   */
  taskId: bigint
  /**
   * 是否成功
   */
  success: boolean
  /**
   * 消息
   */
  message: string | undefined
  /**
   * 数据内容
   */
  data: string | undefined
}
// 响应体的data字段是SdkCodeTemplateDto
/**
 * 代码模板响应实体
 */
export type SdkCodeTemplateResponseDto = {
  /**
   * 主键
   */
  id: bigint
  /**
   * 任务id
   */
  taskId: bigint
  /**
   * 是否成功
   */
  success: boolean
  /**
   * 消息
   */
  message: string | undefined
  /**
   * 数据内容
   */
  data: SdkCodeTemplateDto.SdkCodeTemplateDto | undefined
}
// 响应体的data字段是SdkFileInfoDto
/**
 * 文件树响应实体
 */
export type SdkFileTreeResponseDto = {
  /**
   * 主键
   */
  id: bigint
  /**
   * 任务id
   */
  taskId: bigint
  /**
   * 是否成功
   */
  success: boolean
  /**
   * 消息
   */
  message: string | undefined
  /**
   * 数据内容
   */
  data: SdkFileInfoDto.SdkFileInfoDto | undefined
}
