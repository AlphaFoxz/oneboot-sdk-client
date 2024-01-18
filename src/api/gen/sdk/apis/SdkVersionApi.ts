import * as SdkVersionDto from '../dtos/SdkVersionDto'
import {
  requireHttpUtil as _http,
  requireJSON as _JSON,
  type Page as _Page,
  type HttpResult as _HttpResult
} from '../../../apis-util'

namespace f_SdkVersionApi {
  export async function getRestfulTemplateHash(): _HttpResult<SdkVersionDto.SdkVersionCheckResponse> {
    return (await _http()).get(`/_sdk/version/getRestfulTemplateHash`, {})
  }
  export async function checkRestfulJava(): _HttpResult<SdkVersionDto.SdkVersionCheckResponse> {
    return (await _http()).get(`/_sdk/version/checkRestfulJava`, {})
  }
}
/**
 * 版本检查接口
 */
export const SdkVersionApi = {
  /**
   * 获取restful模板的hash值
   */
  getRestfulTemplateHash: f_SdkVersionApi.getRestfulTemplateHash,
  /**
   * 检查java已生成代码和模板的版本差别
   */
  checkRestfulJava: f_SdkVersionApi.checkRestfulJava,
}
