import * as SdkFileTypeEnum from '../enums/SdkFileTypeEnum'
export type SdkFileInfoDto = {
  filePath: string
  parentDir: string
  fileName: string
  separator: string
  content: string | undefined
  ext: string | undefined
  fileType: SdkFileTypeEnum.SdkFileTypeEnum
  isReadOnly: boolean
  isEmpty: boolean
  children: SdkFileInfoDto[] | undefined
}
