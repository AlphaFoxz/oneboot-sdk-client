import * as SdkResponseDto from '../dtos/SdkResponseDto'
import { requireAxios as _requireAxios, requireJSON as _requireJSON, type Page as _Page } from '../../../apisUtil'
const _axios = _requireAxios()
const _jsonUtil = _requireJSON()

/**
 * Sdk模块基本信息接口
 */
export const SdkInfoApi = {
  /**
   * 获取当前项目的硬盘根目录
   */
  rootPath: async (): Promise<SdkResponseDto.SdkStringResponseDto> => {
    return (await _axios.get(`/_sdk/info/rootPath`)).data
  },
  /**
   * 检查项目错误
   */
  checkThriftErr: async (): Promise<SdkResponseDto.SdkListResponseDto> => {
    return (await _axios.get(`/_sdk/info/checkThriftErr`)).data
  },
  /**
   * 检查RestApi实现情况
   */
  checkRestApiImplements: async (): Promise<SdkResponseDto.SdkListResponseDto> => {
    return (await _axios.get(`/_sdk/info/checkRestApiImplements`)).data
  },
  /**
   * 检查Rpc实现情况
   */
  checkRpcImplements: async (): Promise<SdkResponseDto.SdkListResponseDto> => {
    return (await _axios.get(`/_sdk/info/checkRpcImplements`)).data
  },
}

