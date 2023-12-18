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
 * thrift功能接口（rpc跨语言通信）
 */
export const SdkThriftApi = {
  /**
   * 获取当前thrift服务端口
   */
  getServerPort: async (): Promise<SdkResponseDto.SdkLongResponseDto> => {
    return (await (await _axios()).get(`/_sdk/thrift/getServerPort`)).data
  },
  /**
   * 获取thrift可执行文件路径
   */
  getExecutableFilePath: async (): Promise<SdkResponseDto.SdkStringResponseDto> => {
    return (await (await _axios()).get(`/_sdk/thrift/getExecutableFilePath`)).data
  },
  /**
   * 根据文件路径获取模板内容
   */
  getTemplateContentByPath: async (p_filePath: string): Promise<SdkResponseDto.SdkCodeTemplateResponseDto> => {
    return (await (await _axios()).get(`/_sdk/thrift/getTemplateContentByPath?filePath=${encodeURI(p_filePath)}`)).data
  },
}

