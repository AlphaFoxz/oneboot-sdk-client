import * as SdkResponseDto from '../dtos/SdkResponseDto'
import {
  requireHttpUtil as _http,
  requireJSON as _JSON,
  type Page as _Page,
  type HttpResult as _HttpResult
} from '../../../apis-util'

namespace f_SdkInfoApi {
  export async function rootPath(): _HttpResult<SdkResponseDto.SdkStringResponseDto> {
    return (await _http()).get(`/_sdk/info/rootPath`, {})
  }
  export async function checkThriftErr(): _HttpResult<SdkResponseDto.SdkListResponseDto> {
    return (await _http()).get(`/_sdk/info/checkThriftErr`, {})
  }
  export async function checkRestApiImplements(): _HttpResult<SdkResponseDto.SdkListResponseDto> {
    return (await _http()).get(`/_sdk/info/checkRestApiImplements`, {})
  }
  export async function checkRpcImplements(): _HttpResult<SdkResponseDto.SdkListResponseDto> {
    return (await _http()).get(`/_sdk/info/checkRpcImplements`, {})
  }
}
/**
 * Sdk模块基本信息接口
 */
export const SdkInfoApi = {
  /**
   * 获取当前项目的硬盘根目录
   */
  rootPath: f_SdkInfoApi.rootPath,
  /**
   * 检查项目错误
   */
  checkThriftErr: f_SdkInfoApi.checkThriftErr,
  /**
   * 检查RestApi实现情况
   */
  checkRestApiImplements: f_SdkInfoApi.checkRestApiImplements,
  /**
   * 检查Rpc实现情况
   */
  checkRpcImplements: f_SdkInfoApi.checkRpcImplements,
}
