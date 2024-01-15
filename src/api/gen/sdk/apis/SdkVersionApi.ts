import * as SdkVersionDto from '../dtos/SdkVersionDto'
import {
  requireHttpUtil as _http,
  requireJSON as _JSON,
  type Page as _Page,
  type HttpResult as _HttpResult
} from '../../../apis-util'

/**
 * 版本检查接口
 */
export const SdkVersionApi = {
  /**
   * 获取restful模板的hash值
   */
  getRestfulTemplateHash: async (): _HttpResult<SdkVersionDto.SdkVersionCheckResponse> => {
    return (await _http()).get(`/_sdk/version/getRestfulTemplateHash`)
  },
  /**
   * 检查java已生成代码和模板的版本差别
   */
  checkRestfulJava: async (): _HttpResult<SdkVersionDto.SdkVersionCheckResponse> => {
    return (await _http()).get(`/_sdk/version/checkRestfulJava`)
  },
}
