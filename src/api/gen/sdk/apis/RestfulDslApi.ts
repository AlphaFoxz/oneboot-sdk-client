import * as SdkResponseDto from '../dtos/SdkResponseDto'
import * as SdkRequestDto from '../dtos/SdkRequestDto'
import * as SdkCodeTemplateDto from '../dtos/SdkCodeTemplateDto'
import * as SdkFileInfoDto from '../dtos/SdkFileInfoDto'
import {
  requireHttpUtil as _http,
  requireJSON as _JSON,
  type Page as _Page,
  type HttpResult as _HttpResult
} from '../../../apis-util'

namespace f_RestfulDslApi {
  export async function generateTsClientApi(p_templateDto: SdkRequestDto.SdkCodeTemplateRequestDto, p_genDir: string): _HttpResult<SdkResponseDto.SdkMapResponseDto> {
    return (await _http()).post(`/_restfulDsl/generateTsClientApi`, { templateDto: p_templateDto, genDir: p_genDir }, {})
  }
  export async function generateRustClientApi(p_templateDto: SdkRequestDto.SdkCodeTemplateRequestDto, p_genDir: string): _HttpResult<SdkResponseDto.SdkMapResponseDto> {
    return (await _http()).post(`/_restfulDsl/generateRustClientApi`, { templateDto: p_templateDto, genDir: p_genDir }, {})
  }
  export async function generateJavaServerApi(p_templateDto: SdkRequestDto.SdkCodeTemplateRequestDto): _HttpResult<SdkResponseDto.SdkListResponseDto> {
    return (await _http()).post(`/_restfulDsl/generateJavaServerApi`, p_templateDto, {})
  }
  export async function generateSql(p_templateDto: SdkRequestDto.SdkCodeTemplateRequestDto): _HttpResult<SdkResponseDto.SdkMapResponseDto> {
    return (await _http()).post(`/_restfulDsl/generateSql`, p_templateDto, {})
  }
  export async function createOrUpdateFile(p_filePath: string, p_content: string): _HttpResult<SdkResponseDto.SdkStringResponseDto> {
    return (await _http()).post(`/_restfulDsl/createOrUpdateFile`, { filePath: p_filePath, content: p_content }, {})
  }
  export async function createFolder(p_folderPath: string): _HttpResult<SdkResponseDto.SdkStringResponseDto> {
    return (await _http()).post(`/_restfulDsl/createFolder`, { folderPath: p_folderPath }, {})
  }
  export async function renameFile(p_filePath: string, p_newPath: string): _HttpResult<SdkResponseDto.SdkStringResponseDto> {
    return (await _http()).post(`/_restfulDsl/renameFile`, { filePath: p_filePath, newPath: p_newPath }, {})
  }
  export async function deleteFile(p_filePath: string): _HttpResult<SdkResponseDto.SdkListResponseDto> {
    return (await _http()).post(`/_restfulDsl/deleteFile`, { filePath: p_filePath }, {})
  }
  export async function getTemplateContentByPath(p_filePath: string): _HttpResult<SdkResponseDto.SdkCodeTemplateResponseDto> {
    return (await _http()).post(`/_restfulDsl/getTemplateContentByPath`, { filePath: p_filePath }, {})
  }
  export async function getRestfulTemplateFileTree(): _HttpResult<SdkResponseDto.SdkFileTreeResponseDto> {
    return (await _http()).get(`/_restfulDsl/getRestfulTemplateFileTree`, {})
  }
  export async function getTemplateContentByImportPath(p_temp_path: string, p_import_path: string): _HttpResult<SdkResponseDto.SdkCodeTemplateResponseDto> {
    return (await _http()).get(`/_restfulDsl/getTemplateContentByImportPath?temp_path=${encodeURI(p_temp_path.toString())}&import_path=${encodeURI(p_import_path.toString())}`, {})
  }
}
/**
 * restful-dsl接口
 */
export const RestfulDslApi = {
  /**
   * 创建Ts client的Api代码
   */
  generateTsClientApi: f_RestfulDslApi.generateTsClientApi,
  /**
   * 创建rust client的Api代码
   */
  generateRustClientApi: f_RestfulDslApi.generateRustClientApi,
  /**
   * 创建java server的Api代码
   */
  generateJavaServerApi: f_RestfulDslApi.generateJavaServerApi,
  /**
   * 生成sql语句
   */
  generateSql: f_RestfulDslApi.generateSql,
  /**
   * 创建或更新文件
   */
  createOrUpdateFile: f_RestfulDslApi.createOrUpdateFile,
  /**
   * 创建文件夹
   */
  createFolder: f_RestfulDslApi.createFolder,
  /**
   * 重命名文件
   */
  renameFile: f_RestfulDslApi.renameFile,
  /**
   * 删除文件
   */
  deleteFile: f_RestfulDslApi.deleteFile,
  /**
   * 根据路径获取内容
   */
  getTemplateContentByPath: f_RestfulDslApi.getTemplateContentByPath,
  /**
   * 获取restful模板文件树
   */
  getRestfulTemplateFileTree: f_RestfulDslApi.getRestfulTemplateFileTree,

  getTemplateContentByImportPath: f_RestfulDslApi.getTemplateContentByImportPath,
}
