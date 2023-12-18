import * as SdkVersionDto from '../dtos/SdkVersionDto'
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
 * 版本检查接口
 */
export const SdkVersionApi = {
  /**
   * 获取restful模板的hash值
   */
  getRestfulTemplateHash: async (): Promise<SdkVersionDto.SdkVersionCheckResponse> => {
    return (await (await _axios()).get(`/_sdk/version/getRestfulTemplateHash`)).data
  },
  /**
   * 检查java已生成代码和模板的版本差别
   */
  checkRestfulJava: async (): Promise<SdkVersionDto.SdkVersionCheckResponse> => {
    return (await (await _axios()).get(`/_sdk/version/checkRestfulJava`)).data
  },
}

