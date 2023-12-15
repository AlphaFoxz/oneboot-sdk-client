import * as SdkResponseDto from '../dtos/SdkResponseDto'
import { requireAxios as _requireAxios, requireJSON as _requireJSON, type Page as _Page } from '../../../apisUtil'
const _axios = _requireAxios()
const _jsonUtil = _requireJSON()

/**
 * Sdk模块代码生成接口
 */
export const SdkGenCodeApi = {
  /**
   * 创建所有Java rpc代码
   */
  generateJavaRpc: async (): Promise<SdkResponseDto.SdkListResponseDto> => {
    return (await _axios.get(`/_sdk/genCode/generateJavaRpc`)).data
  },
}

