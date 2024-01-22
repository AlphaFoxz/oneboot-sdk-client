import { invoke } from '@tauri-apps/api/core'
import { SdkStringResponseDto } from '@/api/gen/sdk/dtos/SdkResponseDto'
import { SdkVersionCheckResponse } from '@/api/gen/sdk/dtos/SdkVersionDto'
export enum SdkFileTypeEnum {
  LOCAL_FILE = 0,
  LOCAL_DIR = 1,
}
export interface SdkFileInfoDto {
  filePath: string
  parentDir: string
  fileName: string
  separator: string
  content: string | null
  ext: string
  fileType: SdkFileTypeEnum
  isReadOnly: boolean
  isEmpty: boolean
  children: Array<SdkFileInfoDto>
}

export class SdkFileTreeResponseDtoImpl implements SdkFileTreeResponseDto {
  id = 0
  taskId = 0
  success = true
  message = ''
  data = {} as SdkFileInfoDto
}
export interface SdkFileTreeResponseDto {
  id: number
  taskId: number
  success: boolean
  message: string | undefined
  data: SdkFileInfoDto | undefined
}

export async function getRestfulTemplateFileTree(): Promise<SdkFileTreeResponseDto> {
  return invoke('get_restful_template_file_tree')
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

export async function generateTsClientApi(filePath: string): Promise<void> {
  return invoke('generate_ts_client_api', { filePath })
}

export async function generateRustClientApi(filePath: string): Promise<void> {
  return invoke('generate_rust_client_api', { filePath })
}

export async function generateSql(
  filePath: string
): Promise<{ success: boolean; data: { [k: string]: string } | undefined }> {
  return invoke('generate_sql', { filePath })
}

export async function getBasePackage(): Promise<SdkStringResponseDto> {
  return invoke('get_base_package')
}

export async function checkRestfulFileVersion(): Promise<SdkVersionCheckResponse> {
  return invoke('check_restful_file_version')
}
