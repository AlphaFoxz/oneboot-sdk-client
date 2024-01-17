import {
  requireHttpUtil as _http,
  requireJSON as _JSON,
  type Page as _Page,
  type HttpResult as _HttpResult
} from '../../../apis-util'

/**
 * 文档生成接口
 */
export const SdkGenDocApi = {
  /**
   * 生成word Api文档
   * @param p_moduleName 模块名称
   */
  generateWordApi: async (p_moduleName: string): _HttpResult<string> => {
    return (await _http()).get(`/_sdk/genDoc/generateWordApi?moduleName=${encodeURI(p_moduleName.toString())}`, {
      responseType: 'blob',
    })
  },
  /**
   * 生成Excel Api文档
   * @param p_moduleName 模块名称
   */
  generateExcelApi: async (p_moduleName: string): _HttpResult<string> => {
    return (await _http()).get(`/_sdk/genDoc/generateExcelApi?moduleName=${encodeURI(p_moduleName.toString())}`, {
      responseType: 'blob',
    })
  },
}
