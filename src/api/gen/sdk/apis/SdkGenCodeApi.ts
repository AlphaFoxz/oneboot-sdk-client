import * as SdkResponseDto from '../dtos/SdkResponseDto'
import { requireAxios as _requireAxios, requireJSON as _requireJSON, type Page as _Page } from '../../../apisUtil'

let _axiosInstance: any
const _axios = async () => {
  if (_axiosInstance) {
    return _axiosInstance
  }
  _axiosInstance = await _requireAxios()
  return _axiosInstance
}

/**
 * Sdk模块代码生成接口
 */
export const SdkGenCodeApi = {
  /**
   * 创建所有Java rpc代码
   */
  generateJavaRpc: async (): Promise<SdkResponseDto.SdkListResponseDto> => {
    return (await (await _axios()).get(`/_sdk/genCode/generateJavaRpc`)).data
  },
}

