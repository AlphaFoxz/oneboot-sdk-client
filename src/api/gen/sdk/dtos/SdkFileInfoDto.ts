import * as SdkFileTypeEnum from '../enums/SdkFileTypeEnum'

export type SdkFileInfoDto = {
  filePath: string
  parentDir: string
  fileName: string
  separator: string
  content: string | null
  ext: string | null
  fileType: SdkFileTypeEnum.SdkFileTypeEnum
  isReadOnly: boolean
  isEmpty: boolean
  children: SdkFileInfoDto[] | null
}
