import { invoke } from '@tauri-apps/api/core'
import { SdkStringResponseDto } from './gen/sdk/dtos/SdkResponseDto'
import { SdkVersionCheckResponse } from './gen/sdk/dtos/SdkVersionDto'
import { SdkFileTreeResponseDto, SdkMapResponseDto } from './gen/sdk/dtos/SdkResponseDto'
import { SdkCrudServiceTypeEnum } from './gen/sdk/enums/SdkCrudServiceTypeEnum'
export { SdkCrudServiceTypeEnum }

export async function getRestfulTemplateFileTree(): Promise<SdkFileTreeResponseDto> {
  return invoke('get_restful_template_file_tree')
}
export async function getPumlTemplateFileTree(): Promise<SdkFileTreeResponseDto> {
  return invoke('get_puml_template_file_tree')
}

export async function getTemplateContentByPath(filePath: string): Promise<any> {
  return invoke('get_template_content_by_path', { filePath })
}

export async function createOrUpdateFile(filePath: string, content: string): Promise<any> {
  return invoke('create_or_update_file', { filePath, content })
}

export async function createFolder(folderPath: string): Promise<any> {
  return invoke('create_folder', { folderPath })
}

export async function deleteFile(filePath: string): Promise<any> {
  return invoke('delete_file', { filePath })
}

export async function renameFile(filePath: string, newPath: string): Promise<any> {
  return invoke('rename_file', { filePath, newPath })
}

export async function generateJavaServerApi(filePath: string): Promise<void> {
  return invoke('generate_java_server_api', { filePath })
}

export async function generateJavaServerMockService(filePath: string): Promise<void> {
  return invoke('generate_java_server_mock_service', { filePath })
}

export async function generateTsClientApi(filePath: string): Promise<void> {
  return invoke('generate_ts_client_api', { filePath })
}

export async function generateRustClientApi(filePath: string): Promise<void> {
  return invoke('generate_rust_client_api', { filePath })
}

export async function generateSql(filePath: string): Promise<SdkMapResponseDto> {
  return invoke('generate_sql', { filePath })
}

export async function getBasePackage(): Promise<SdkStringResponseDto> {
  return invoke('get_base_package')
}

export async function checkRestfulFileVersion(): Promise<SdkVersionCheckResponse> {
  return invoke('check_restful_file_version')
}

export async function getServerLanguageType(): Promise<SdkVersionCheckResponse> {
  return invoke('get_server_language_type')
}

export async function generateTableCrud(
  moduleName: string,
  poName: String,
  type: SdkCrudServiceTypeEnum,
  force: boolean
): Promise<void> {
  return invoke('generate_table_crud', { moduleName, poName, type, force })
}

export async function generateModuleCrud(
  moduleName: string,
  type: SdkCrudServiceTypeEnum,
  force: boolean
): Promise<void> {
  return invoke('generate_module_crud', { moduleName, type, force })
}

export async function generateJavaServerDomain(path: string) {
  return invoke('generate_java_server_domain', { path })
}

export async function generateWordApi(moduleName: string) {
  return invoke('generate_word_api', { moduleName })
}

export async function readFolderContent(
  parentFolder: string,
  folderPathPattern?: RegExp | string,
  filePathPattern?: RegExp | string
): Promise<FileInfo[]> {
  const result: FileInfo[] = await invoke('read_folder_content', {
    parentFolder,
    folderPathPattern: folderPathPattern ? folderPathPattern.toString() : undefined,
    filePathPattern: filePathPattern ? filePathPattern.toString() : undefined,
  })
  return result
}

export async function writeCodeFiles(files: FileInfo[]): Promise<void> {
  return invoke('write_code_files', { files })
}
