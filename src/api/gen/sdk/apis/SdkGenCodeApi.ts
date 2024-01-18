import * as SdkResponseDto from '../dtos/SdkResponseDto'
import * as SdkCrudServiceTypeEnum from '../enums/SdkCrudServiceTypeEnum'
import {
  requireHttpUtil as _http,
  requireJSON as _JSON,
  type Page as _Page,
  type HttpResult as _HttpResult
} from '../../../apis-util'

namespace f_SdkGenCodeApi {
  export async function generateJavaRpc(): _HttpResult<SdkResponseDto.SdkListResponseDto> {
    return (await _http()).get(`/_sdk/genCode/generateJavaRpc`, {})
  }
  export async function generateTableCrud(p_moduleName: string, p_poName: string, p_serviceType: SdkCrudServiceTypeEnum.SdkCrudServiceTypeEnum, p_force: boolean): _HttpResult<null> {
    return (await _http()).get(`/_sdk/genCode/generateTableCrud?moduleName=${encodeURI(p_moduleName.toString())}&poName=${encodeURI(p_poName.toString())}&serviceType=${encodeURI(p_serviceType.toString())}&force=${encodeURI(p_force.toString())}`, {})
  }
  export async function generateModuleCrud(p_moduleName: string, p_serviceType: SdkCrudServiceTypeEnum.SdkCrudServiceTypeEnum, p_force: boolean): _HttpResult<null> {
    return (await _http()).get(`/_sdk/genCode/generateModuleCrud?moduleName=${encodeURI(p_moduleName.toString())}&serviceType=${encodeURI(p_serviceType.toString())}&force=${encodeURI(p_force.toString())}`, {})
  }
}
/**
 * Sdk模块代码生成接口
 */
export const SdkGenCodeApi = {
  /**
   * 创建所有Java rpc代码
   */
  generateJavaRpc: f_SdkGenCodeApi.generateJavaRpc,
  /**
   * 创建单表CRUD代码
   * @param p_moduleName 模块名称
   * @param p_poName 表名
   * @param p_serviceType 生成类型
   * @param p_force 是否覆盖已有代码
   */
  generateTableCrud: f_SdkGenCodeApi.generateTableCrud,
  /**
   * 创建整个模块的CRUD代码
   * @param p_moduleName 模块名称
   * @param p_serviceType 生成类型
   * @param p_force 是否覆盖已有代码
   */
  generateModuleCrud: f_SdkGenCodeApi.generateModuleCrud,
}
