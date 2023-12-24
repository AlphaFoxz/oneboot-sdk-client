import * as SdkResponseDto from '../dtos/SdkResponseDto'
import {
  requireHttpUtil as _http,
  requireJSON as _JSON,
  type Page as _Page,
  type HttpResult as _HttpResult
} from '../../../apisUtil'

/**
 * thrift功能接口（rpc跨语言通信）
 */
export const SdkThriftApi = {
  /**
   * 获取当前thrift服务端口
   */
  getServerPort: async (): _HttpResult<SdkResponseDto.SdkLongResponseDto> => {
    return (await _http()).get(`/_sdk/thrift/getServerPort`)
  },
  /**
   * 获取thrift可执行文件路径
   */
  getExecutableFilePath: async (): _HttpResult<SdkResponseDto.SdkStringResponseDto> => {
    return (await _http()).get(`/_sdk/thrift/getExecutableFilePath`)
  },
  /**
   * 根据文件路径获取模板内容
   */
  getTemplateContentByPath: async (p_filePath: string): _HttpResult<SdkResponseDto.SdkCodeTemplateResponseDto> => {
    return (await _http()).get(`/_sdk/thrift/getTemplateContentByPath?filePath=${encodeURI(p_filePath.toString())}`)
  },
}

