<script setup lang="ts">
import { ref } from 'vue'
import { Editor, type Files, useMessage, useMonaco } from 'monaco-tree-editor'
import 'monaco-tree-editor/index.css'
import * as utils from '@/utils'
import * as api from './api'
import { SdkFileTypeEnum } from '@/utils/rust_api'
import { registerRestful } from './restful'

// ================ 注册语言 ================
const monacoStore = useMonaco()
const monaco = monacoStore.monaco
registerRestful(monacoStore.monaco)

// ================ 加载文件 load files ================
const files = ref<Files>({})
const loadFile = (file: utils.rust_api.SdkFileInfoDto, container: Files, level: number) => {
  container[file.file_path] = {
    path: file.file_path + (file.file_type === SdkFileTypeEnum.LOCAL_DIR ? '/' : ''),
    name: file.file_name,
    content: file.content || '',
    readonly: level < 3,
    isDirectory: file.file_type === SdkFileTypeEnum.LOCAL_DIR,
    isFile: file.file_type === SdkFileTypeEnum.LOCAL_FILE,
  }
  if (file.children) {
    file.children.forEach((element) => {
      container = loadFile(element, container, level + 1)
    })
  }
  return container
}
const handleReload = (resolve: () => void, reject: (msg?: string) => void) => {
  utils.rust_api
    .getRestfulTemplateFileTree()
    .then((res) => {
      if (res.success && res.data) {
        console.debug('请求结果', res)
        const container = loadFile(res.data, {}, 0)
        files.value = container
        resolve()
      } else {
        reject('加载文件树失败，请检查服务端报错信息')
      }
    })
    .catch(() => {
      reject('加载文件树失败，请检查网络是否正常、后端服务是否正常')
    })
}

// ================ 调整尺寸 resize ================
const editorRef = ref()
window.onresize = () => {
  setTimeout(() => {
    editorRef.value.resize()
  }, 30)
}

// ================ 回调函数 callback ================
const messageStore = useMessage()
const handleNewFile = (path: string, resolve: () => void, reject: (msg?: string) => void) => {
  utils.rust_api.createOrUpdateFile(path, '').then((res) => {
    if (res) {
      resolve()
    } else {
      reject('保存失败，请检查服务端保存信息')
    }
  })
}
const handleNewFolder = (path: string, resolve: () => void, reject: (msg?: string) => void) => {
  utils.rust_api.createFolder(path).then((res) => {
    if (res) {
      resolve()
    } else {
      reject('保存失败，请检查服务端保存信息')
    }
  })
}
const handleSaveFile = (path: string, value: string, resolve: () => void, reject: (msg?: string) => void) => {
  utils.rust_api.createOrUpdateFile(path, value).then((res) => {
    if (res) {
      if (path.endsWith('.restful')) {
        api.checkErr(monaco, monacoStore.getEditor())
      }
      resolve()
    } else {
      reject('保存失败，请检查服务端保存信息')
    }
  })
}
const handleDeleteFile = (path: string, resolve: () => void, reject: (msg?: string) => void) => {
  utils.rust_api.deleteFile(path).then((res) => {
    if (res && res.success) {
      resolve()
    } else {
      reject('删除文件失败，请检查服务端保存信息')
    }
  })
}
const handleRename = (path: string, newPath: string, resolve: () => void, reject: (msg?: string) => void) => {
  utils.rust_api.renameFile(path, newPath).then((res) => {
    if (res && res.success) {
      resolve()
    } else {
      reject('重命名文件失败，请检查服务端保存信息')
    }
  })
}
const handleContextmenuSelect = async (path: string, item: { label: string; value: string }) => {
  if (item.value === 'generateTsCode') {
    const ok = await api.checkErr(monaco, monacoStore.getEditor())
    if (ok) {
      utils.rust_api.generateTsApi(path).then((res) => {
        if (res) {
          messageStore.success({
            content: '代码已生成，请稍后重新编译项目并验证',
            timeoutMs: 5000,
            closeable: true,
          })
        } else {
          messageStore.error({
            content: '保存失败，请检查是否有网络错误',
            closeable: true,
          })
        }
      })
    } else {
      messageStore.error({
        content: '存在语法错误，无法生成',
        closeable: true,
      })
    }
  } else if (item.value === 'generateJavaCode') {
    const ok = await api.checkErr(monaco, monacoStore.getEditor())
    if (ok) {
      utils.rust_api.generateJavaApi(path).then((res) => {
        if (res) {
          messageStore.success({
            content: '代码已生成，请稍后重新编译项目并验证',
            timeoutMs: 5000,
            closeable: true,
          })
        } else {
          messageStore.error({
            content: '保存失败，请检查是否有网络错误',
            closeable: true,
          })
        }
      })
    } else {
      messageStore.error({
        content: '存在语法错误，无法生成',
        closeable: true,
      })
    }
  }
}
</script>

<template>
  <Editor
    ref="editorRef"
    @reload="handleReload"
    :files="files"
    @new-file="handleNewFile"
    @new-folder="handleNewFolder"
    @save-file="handleSaveFile"
    @rename-file="handleRename"
    @rename-folder="handleRename"
    @delete-file="handleDeleteFile"
    @delete-folder="handleDeleteFile"
    :file-menu="[
      { label: 'Generate Java Code', value: 'generateJavaCode' },
      {
        label: 'Generate Ts Code',
        value: 'generateTsCode',
      },
    ]"
    :folder-menu="[]"
    @contextmenu-select="handleContextmenuSelect"
  />
</template>
<!--  -->
