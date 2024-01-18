import * as SdkResponseDto from '../dtos/SdkResponseDto'
import {
  requireHttpUtil as _http,
  requireJSON as _JSON,
  type Page as _Page,
  type HttpResult as _HttpResult
} from '../../../apis-util'

namespace f_SdkThriftApi {
  export async function getServerPort(): _HttpResult<SdkResponseDto.SdkLongResponseDto> {
    return (await _http()).get(`/_sdk/thrift/getServerPort`, {})
  }
  export async function getExecutableFilePath(): _HttpResult<SdkResponseDto.SdkStringResponseDto> {
    return (await _http()).get(`/_sdk/thrift/getExecutableFilePath`, {})
  }
  export async function getTemplateContentByPath(p_filePath: string): _HttpResult<SdkResponseDto.SdkCodeTemplateResponseDto> {
    return (await _http()).get(`/_sdk/thrift/getTemplateContentByPath?filePath=${encodeURI(p_filePath.toString())}`, {})
  }
}
/**
 * thrift功能接口（rpc跨语言通信）
 */
export const SdkThriftApi = {
  /**
   * 获取当前thrift服务端口
   */
  getServerPort: f_SdkThriftApi.getServerPort,
  /**
   * 获取thrift可执行文件路径
   */
  getExecutableFilePath: f_SdkThriftApi.getExecutableFilePath,
  /**
   * 根据文件路径获取模板内容
   */
  getTemplateContentByPath: f_SdkThriftApi.getTemplateContentByPath,
}
