import * as api from '@/api'
import { Files, useMonaco } from 'monaco-tree-editor'
import { Ref } from 'vue'
import { SdkFileInfoDto } from '@/api/gen/sdk/dtos/SdkFileInfoDto'
import { SdkFileTypeEnum } from '@/api/gen/sdk/enums/SdkFileTypeEnum'
import * as monaco from 'monaco-editor'

let files: Ref<Files>
let valid: { checkErr: Function }
const monacoStore = useMonaco()

const loadFile = (file: SdkFileInfoDto, container: Files, level: number) => {
  container[file.filePath] = {
    path: file.filePath + (file.fileType === SdkFileTypeEnum.LOCAL_DIR ? '/' : ''),
    name: file.fileName,
    content: file.content || '',
    readonly: level < 3,
    isFolder: file.fileType === SdkFileTypeEnum.LOCAL_DIR,
    isFile: file.fileType === SdkFileTypeEnum.LOCAL_FILE,
  }
  if (file.children) {
    file.children.forEach((element) => {
      container = loadFile(element, container, level + 1)
    })
  }
  return container
}
const handleReloadRestfulDsl = (resolve: () => void, reject: (msg?: string) => void) => {
  api
    .getRestfulTemplateFileTree()
    .then((res) => {
      if (res.success && res.data) {
        console.debug('请求结果', res)
        const container = loadFile(res.data, {}, 0)
        resolve()
        files.value = container
      } else {
        reject('加载文件树失败，请检查服务端报错信息')
      }
    })
    .catch(() => {
      reject('加载文件树失败，请检查网络是否正常、后端服务是否正常')
    })
}
const handleReloadPuml = (resolve: () => void, reject: (msg?: string) => void) => {
  api
    .getPumlTemplateFileTree()
    .then((res) => {
      if (res.success && res.data) {
        console.debug('请求结果', res)
        const container = loadFile(res.data, {}, 0)
        resolve()
        files.value = container
      } else {
        reject('加载文件树失败，请检查服务端报错信息')
      }
    })
    .catch(() => {
      reject('加载文件树失败，请检查网络是否正常、后端服务是否正常')
    })
}
const handleNewFile = (path: string, resolve: () => void, reject: (msg?: string) => void) => {
  api.createOrUpdateFile(path, '').then((res) => {
    if (res) {
      resolve()
    } else {
      reject('保存失败，请检查服务端报错信息')
    }
  })
}
const handleNewFolder = (path: string, resolve: () => void, reject: (msg?: string) => void) => {
  api.createFolder(path).then((res) => {
    if (res) {
      resolve()
    } else {
      reject('保存失败，请检查服务端报错信息')
    }
  })
}
const handleSaveFile = async (path: string, value: string, resolve: () => void, reject: (msg?: string) => void) => {
  const currentPath =
    monacoStore._state.prefix.value +
    monacoStore.state.currentPath.value.replaceAll('/', monacoStore._state.fileSeparator.value)
  if (path.endsWith('.restl')) {
    valid.checkErr(monaco, value, currentPath === path ? monacoStore.action.getEditor() : undefined)
  }

  api.createOrUpdateFile(path, value).then((res) => {
    if (res) {
      resolve()
    } else {
      reject('保存失败，请检查服务端报错信息')
    }
  })
}
const handleDeleteFile = (path: string, resolve: () => void, reject: (msg?: string) => void) => {
  api.deleteFile(path).then((res) => {
    if (res && res.success) {
      resolve()
    } else {
      reject('删除文件失败，请检查服务端报错信息')
    }
  })
}
const handleRename = (path: string, newPath: string, resolve: () => void, reject: (msg?: string) => void) => {
  api.renameFile(path, newPath).then((res) => {
    if (res && res.success) {
      resolve()
    } else {
      reject('重命名文件失败，请检查服务端保存信息')
    }
  })
}
export const useServerApi = (init: Ref<Files>, validIns: { checkErr: Function }) => {
  files = init
  valid = validIns
  return {
    handleReloadRestfulDsl,
    handleReloadPuml,
    handleNewFile,
    handleNewFolder,
    handleSaveFile,
    handleDeleteFile,
    handleRename,
  }
}
