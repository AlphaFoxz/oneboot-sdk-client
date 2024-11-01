import { readFolderContent, writeCodeFiles } from '@/api'
import { forTimes } from './fun'

export async function writeCodeFile(...files: FileInfo[]) {
  await writeCodeFiles(files)
}

export async function recursivelyReadFolderContent(
  parentDir: string,
  callback: (fileInfo: FileInfo) => void,
  folderPathPattern?: RegExp | string,
  filePathPattern?: RegExp | string
) {
  const fileInfoArr: FileInfo[] = await readFolderContent(parentDir, folderPathPattern, filePathPattern)
  for (const fileInfo of fileInfoArr) {
    if (fileInfo.isFile) {
      callback(fileInfo)
    } else if (fileInfo.isFolder) {
      await recursivelyReadFolderContent(fileInfo.path, callback, folderPathPattern, filePathPattern)
    }
  }
  return
}

export async function readFolder(parentFolder: string): Promise<FileInfo[]> {
  const fileInfoArr: FileInfo[] = await readFolderContent(parentFolder, undefined, undefined)
  return forTimes(fileInfoArr.length).reduce((acc, index) => {
    if (fileInfoArr[index].isFolder) {
      acc.push(fileInfoArr[index])
    }
    return acc
  }, [] as FileInfo[])
}
