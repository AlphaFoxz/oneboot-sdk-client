import * as SdkResponseDto from '../dtos/SdkResponseDto'
import {
  requireHttpUtil as _http,
  requireJSON as _JSON,
  type Page as _Page,
  type HttpResult as _HttpResult
} from '../../../apisUtil'

/**
 * Sdk模块基本信息接口
 */
export const SdkInfoApi = {
  /**
   * 获取当前项目的硬盘根目录
   */
  rootPath: async (): _HttpResult<SdkResponseDto.SdkStringResponseDto> => {
    return (await _http()).get(`/_sdk/info/rootPath`)
  },
  /**
   * 检查项目错误
   */
  checkThriftErr: async (): _HttpResult<SdkResponseDto.SdkListResponseDto> => {
    return (await _http()).get(`/_sdk/info/checkThriftErr`)
  },
  /**
   * 检查RestApi实现情况
   */
  checkRestApiImplements: async (): _HttpResult<SdkResponseDto.SdkListResponseDto> => {
    return (await _http()).get(`/_sdk/info/checkRestApiImplements`)
  },
  /**
   * 检查Rpc实现情况
   */
  checkRpcImplements: async (): _HttpResult<SdkResponseDto.SdkListResponseDto> => {
    return (await _http()).get(`/_sdk/info/checkRpcImplements`)
  },
}

