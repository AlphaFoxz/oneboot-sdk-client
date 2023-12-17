<script setup lang="ts">
import { useRouter } from 'vue-router'
import { Modal as AModal, Tabs as ATabs, TabPane as ATabPane } from 'ant-design-vue'
import { ref, watch } from 'vue'
import { Editor, type Files, useMessage, useMonaco, useHotkey } from 'monaco-tree-editor'
import 'monaco-tree-editor/index.css'
import * as utils from '@/utils'
import * as api from './api'
import { SdkFileTypeEnum } from '@/utils/rust_api'
import { registerRestful } from './restful'
import * as monaco from 'monaco-editor'
// import * as monaco from 'monaco-editor/esm/vs/editor/editor.api'
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
// import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
// import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker'
// import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker'
// import cssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker'

window.MonacoEnvironment = {
  getWorker: function (_moduleId, _label: string) {
    // if (label === 'json') {
    //   return new jsonWorker()
    // } else if (label === 'ts' || label === 'typescript') {
    //   return new tsWorker()
    // } else if (label === 'html' || label === 'handlebars' || label === 'razor') {
    //   return new htmlWorker()
    // } else if (label === 'css' || label === 'scss' || label === 'less') {
    //   return new cssWorker()
    // }
    return new editorWorker()
  },
  globalAPI: true,
}

// ================ 注册语言 ================
const monacoStore = useMonaco(monaco)
watch(monacoStore.isReady, () => {
  monacoStore.getEditor().onDidChangeModelContent(() => {})
})
registerRestful(monaco)

// ================ 快捷键 ================
const hotkeyStore = useHotkey()
hotkeyStore.listen('editor', (e) => {
  if (e.ctrlKey && e.altKey && !e.shiftKey) {
    if (e.key === 'ArrowLeft') {
      monacoStore.getEditor().trigger('keyboard', 'cursorUndo', null)
    } else if (e.key === 'ArrowRight') {
      monacoStore.getEditor().trigger('keyboard', 'cursorRedo', null)
    }
  } else if (e.ctrlKey && !e.altKey && e.shiftKey) {
    if (e.key === '?' || e.key === '/') {
      monacoStore.getEditor().trigger('keyboard', 'editor.action.blockComment', null)
    } else if (e.key === 'ArrowUp') {
      monacoStore.getEditor().trigger('keyboard', 'editor.action.moveLinesUpAction', null)
    } else if (e.key === 'ArrowDown') {
      monacoStore.getEditor().trigger('keyboard', 'editor.action.moveLinesDownAction', null)
    }
  } else if (e.ctrlKey && !e.altKey && !e.shiftKey) {
    if (e.key === 'y' || e.key === 'Y') {
      console.debug(monacoStore.monaco.editor.getModels())
      monacoStore.getEditor().trigger('keyboard', 'editor.action.openLink', '/sdk/dtos/SdkResponseDto.restful')
    }
  }
})

// ================ 加载文件 load files ================
const files = ref<Files>({})
const loadFile = (file: utils.rust_api.SdkFileInfoDto, container: Files, level: number) => {
  container[file.file_path] = {
    path: file.file_path + (file.file_type === SdkFileTypeEnum.LOCAL_DIR ? '/' : ''),
    name: file.file_name,
    content: file.content || '',
    readonly: level < 3,
    isFolder: file.file_type === SdkFileTypeEnum.LOCAL_DIR,
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
        resolve()
        setTimeout(() => {
          files.value = container
        })
      } else {
        reject('加载文件树失败，请检查服务端报错信息')
      }
    })
    .catch(() => {
      reject('加载文件树失败，请检查网络是否正常、后端服务是否正常')
    })
}

// ================ 自定义菜单 custom menu ================
const router = useRouter()
const settingsMenu = ref([
  {
    label: '退出',
    handler: () => {
      router.back()
    },
  },
])

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
      reject('保存失败，请检查服务端报错信息')
    }
  })
}
const handleNewFolder = (path: string, resolve: () => void, reject: (msg?: string) => void) => {
  utils.rust_api.createFolder(path).then((res) => {
    if (res) {
      resolve()
    } else {
      reject('保存失败，请检查服务端报错信息')
    }
  })
}
const handleSaveFile = (path: string, value: string, resolve: () => void, reject: (msg?: string) => void) => {
  const currentPath =
    monacoStore.prefix.value + monacoStore.currentPath.value.replaceAll('/', monacoStore.fileSeparator.value)
  utils.rust_api.createOrUpdateFile(path, value).then((res) => {
    if (res) {
      if (path.endsWith('.restful')) {
        api.checkErr(monaco, files.value[path].content!, currentPath === path ? monacoStore.getEditor() : undefined)
      }
      resolve()
    } else {
      reject('保存失败，请检查服务端报错信息')
    }
  })
}
const handleDeleteFile = (path: string, resolve: () => void, reject: (msg?: string) => void) => {
  utils.rust_api.deleteFile(path).then((res) => {
    if (res && res.success) {
      resolve()
    } else {
      reject('删除文件失败，请检查服务端报错信息')
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
const handleDragInEditor = (srcPath: string, targetPath: string, type: 'file' | 'folder') => {
  if (!targetPath.endsWith('.restful')) {
    return
  }
  const editor = monacoStore.getEditor()
  const lineIndex = editor.getPosition()?.lineNumber!
  let str = ''
  if (type === 'file') {
    str += 'import "' + relativePathFrom(srcPath, targetPath) + '"'
  } else {
    str = srcPath.replace(monacoStore.prefix.value, '')
    const tsNamespace = str.replaceAll(monacoStore.fileSeparator.value, '.')
    const javaNamespace: any = str.split(monacoStore.fileSeparator.value)
    javaNamespace.splice(2, 0, 'gen', 'restful')
    str = `namespace java com.github.alphafoxz.oneboot${javaNamespace.join('.')}\n`
    str += `namespace ts gen${tsNamespace}\n`
  }
  editor.executeEdits('drop', [{ range: new monaco.Range(lineIndex, 0, lineIndex, 0), text: str }])
}
//计算相对路径 getRelativePath
const relativePathFrom = (returnPath: string, fromPath: string): string => {
  const prefix = utils.longestCommonPrefix(returnPath, fromPath)
  returnPath = returnPath.replace(prefix, '').replace(/\\/g, '/')
  fromPath = fromPath.replace(prefix, '').replace(/\\/g, '/')
  const fromPathArr = fromPath.split('/')
  let relativePath = ''
  if (fromPathArr.length === 1) {
    relativePath = './'
  } else {
    for (let i = fromPathArr.length - 2; i >= 0; i--) {
      relativePath += '../'
    }
  }
  return (relativePath += returnPath)
}
const sqlTitle = ref<string[]>([])
const sqlIndex = ref(-1)
const sqlContent = ref<string[]>([])
const sqlVisible = ref(false)
const handleContextmenuSelect = async (path: string, item: { label: string; value: string }) => {
  const currentPath =
    monacoStore.prefix.value + monacoStore.currentPath.value.replaceAll('/', monacoStore.fileSeparator.value)
  if (item.value === 'generateTsCode') {
    const ok = await api.checkErr(
      monaco,
      files.value[path].content!,
      currentPath === path ? monacoStore.getEditor() : undefined
    )
    if (ok) {
      utils.rust_api
        .generateTsApi(path)
        .then(() => {
          messageStore.success({
            content: '代码已生成，请稍后重新编译项目并验证',
            timeoutMs: 5000,
            closeable: true,
          })
        })
        .catch(() => {
          messageStore.error({
            content: '保存失败，请检查是否有网络错误',
            closeable: true,
          })
        })
    } else {
      messageStore.error({
        content: '存在语法错误，无法生成',
        closeable: true,
      })
    }
  } else if (item.value === 'generateJavaCode') {
    const ok = await api.checkErr(
      monaco,
      files.value[path].content!,
      currentPath === path ? monacoStore.getEditor() : undefined
    )
    if (ok) {
      utils.rust_api
        .generateJavaApi(path)
        .then(() => {
          messageStore.success({
            content: '代码已生成，请稍后重新编译项目并验证',
            timeoutMs: 5000,
            closeable: true,
          })
        })
        .catch(() => {
          messageStore.error({
            content: '保存失败，请检查是否有网络错误',
            closeable: true,
          })
        })
    } else {
      messageStore.error({
        content: '存在语法错误，无法生成',
        closeable: true,
      })
    }
  } else if (item.value === 'generateDbSql') {
    const ok = await api.checkErr(
      monaco,
      files.value[path].content!,
      currentPath === path ? monacoStore.getEditor() : undefined
    )
    if (ok) {
      utils.rust_api.generateSql(path).then((res) => {
        if (res.success) {
          messageStore.success({
            content: 'SQL已生成',
            timeoutMs: 5000,
            closeable: true,
          })
          if (!res.data) {
            sqlTitle.value = []
            sqlContent.value = []
            sqlIndex.value = -1
            return
          }
          const titles: string[] = []
          const contents: string[] = []
          Object.keys(res.data).forEach((key) => {
            titles.push(key)
            contents.push(res.data![key])
            sqlContent.value = contents
            sqlTitle.value = titles
            sqlIndex.value = 0
          })
          sqlVisible.value = true
        } else {
          messageStore.error({
            content: 'SQL生成失败',
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
  <a-modal v-model:open="sqlVisible" width="70%">
    <a-tabs v-if="sqlTitle.length" v-model:activeKey="sqlIndex">
      <a-tab-pane v-for="(item, index) in sqlTitle" :key="index" :tab="item">
        <pre>{{ sqlContent[index] }}</pre>
      </a-tab-pane>
    </a-tabs>
  </a-modal>
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
      {
        label: 'Generate Db SQL',
        value: 'generateDbSql',
      },
    ]"
    :folder-menu="[]"
    :settings-menu="settingsMenu"
    @contextmenu-select="handleContextmenuSelect"
    @drag-in-editor="handleDragInEditor"
  />
</template>
<!--  -->
