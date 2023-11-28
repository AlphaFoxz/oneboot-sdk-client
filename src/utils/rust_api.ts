import { primitives } from '@tauri-apps/api'
const invoke = primitives.invoke
export enum SdkFileTypeEnum {
  LOCAL_FILE = 0,
  LOCAL_DIR = 1,
}
export interface SdkFileInfoDto {
  file_path: string
  parent_dir: string
  file_name: string
  separator: string
  content: string | null
  ext: string
  file_type: SdkFileTypeEnum
  is_read_only: boolean
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
  return (await invoke('get_restful_template_file_tree')) as SdkFileTreeResponseDtoImpl
}

export async function getTemplateContentByPath(filePath: string): Promise<any> {
  return await invoke('get_template_content_by_path', { filePath })
}

export async function createOrUpdateFile(filePath: string, content: string): Promise<any> {
  return await invoke('create_or_update_file', { filePath, content })
}

export async function createFolder(folderPath: string): Promise<any> {
  return await invoke('create_folder', { folderPath })
}

export async function deleteFile(filePath: string): Promise<any> {
  return await invoke('delete_file', { filePath })
}

export async function renameFile(filePath: string, newPath: string): Promise<any> {
  return await invoke('rename_file', { filePath, newPath })
}

export async function generateJavaApi(filePath: string): Promise<boolean> {
  return await invoke('generate_java_api', { filePath })
}

export async function generateTsApi(filePath: string): Promise<boolean> {
  return await invoke('generate_ts_api', { filePath })
}

export async function generateSql(
  filePath: string
): Promise<{ success: boolean; data: { [k: string]: string } | undefined }> {
  return await invoke('generate_sql', { filePath })
}
