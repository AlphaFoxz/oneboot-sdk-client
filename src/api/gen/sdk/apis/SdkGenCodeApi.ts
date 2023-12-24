import * as SdkResponseDto from '../dtos/SdkResponseDto'
import {
  requireHttpUtil as _http,
  requireJSON as _JSON,
  type Page as _Page,
  type HttpResult as _HttpResult
} from '../../../apisUtil'

/**
 * Sdk模块代码生成接口
 */
export const SdkGenCodeApi = {
  /**
   * 创建所有Java rpc代码
   */
  generateJavaRpc: async (): _HttpResult<SdkResponseDto.SdkListResponseDto> => {
    return (await _http()).get(`/_sdk/genCode/generateJavaRpc`)
  },
}

