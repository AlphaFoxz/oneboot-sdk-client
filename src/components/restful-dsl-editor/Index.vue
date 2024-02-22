<script setup lang="ts">
import { useRouter } from 'vue-router'
import { Modal as AModal, Tabs as ATabs, TabPane as ATabPane } from 'ant-design-vue'
import { ComputedRef, onMounted, ref, watch } from 'vue'
import { Editor, type Files, useMessage, useMonaco, useHotkey } from 'monaco-tree-editor'
import 'monaco-tree-editor/index.css'
import * as utils from '@/utils'
import * as api from './api'
import * as rustApi from '@/api/rust_api'
import { SdkFileInfoDto } from '@/api/gen/sdk/dtos/SdkFileInfoDto'
import { SdkFileTypeEnum } from '@/api/gen/sdk/enums/SdkFileTypeEnum'
import { registerRestl } from './restl'
import * as monaco from 'monaco-editor'
import { GenTypeEnum } from './define'
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'

window.MonacoEnvironment = {
  getWorker: function (_moduleId, _label: string) {
    return new editorWorker()
  },
  globalAPI: true,
}

// ================ 注册语言 ================
const monacoStore = useMonaco(monaco)
watch(monacoStore.isReady, () => {
  monacoStore.getEditor().onDidChangeModelContent(() => {})
})
registerRestl(monaco)

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
      monacoStore.getEditor().trigger('keyboard', 'editor.action.openLink', '/sdk/dtos/SdkResponseDto.restl')
    }
  }
})

// ================ 加载文件 load files ================
const files = ref<Files>({})
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
const handleReload = (resolve: () => void, reject: (msg?: string) => void) => {
  rustApi
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

// ================ 自定义菜单 custom menu ================
const router = useRouter()
const settingsMenu = ref([
  {
    label: '退出',
    handler: () => {
      router.push({ name: 'Home' })
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
  rustApi.createOrUpdateFile(path, '').then((res) => {
    if (res) {
      resolve()
    } else {
      reject('保存失败，请检查服务端报错信息')
    }
  })
}
const handleNewFolder = (path: string, resolve: () => void, reject: (msg?: string) => void) => {
  rustApi.createFolder(path).then((res) => {
    if (res) {
      resolve()
    } else {
      reject('保存失败，请检查服务端报错信息')
    }
  })
}
const handleSaveFile = async (path: string, value: string, resolve: () => void, reject: (msg?: string) => void) => {
  const currentPath =
    monacoStore.prefix.value + monacoStore.currentPath.value.replaceAll('/', monacoStore.fileSeparator.value)
  if (path.endsWith('.restl')) {
    api.checkErr(monaco, value, currentPath === path ? monacoStore.getEditor() : undefined)
  }

  rustApi.createOrUpdateFile(path, value).then((res) => {
    if (res) {
      resolve()
    } else {
      reject('保存失败，请检查服务端报错信息')
    }
  })
}
const handleDeleteFile = (path: string, resolve: () => void, reject: (msg?: string) => void) => {
  rustApi.deleteFile(path).then((res) => {
    if (res && res.success) {
      resolve()
    } else {
      reject('删除文件失败，请检查服务端报错信息')
    }
  })
}
const handleRename = (path: string, newPath: string, resolve: () => void, reject: (msg?: string) => void) => {
  rustApi.renameFile(path, newPath).then((res) => {
    if (res && res.success) {
      resolve()
    } else {
      reject('重命名文件失败，请检查服务端保存信息')
    }
  })
}
const basePackage = ref('')
onMounted(() => {
  rustApi.getBasePackage().then((res) => {
    basePackage.value = res.data!
  })
})
const handleDragInEditor = (srcPath: string, targetPath: string, type: 'file' | 'folder') => {
  if (!targetPath.endsWith('.restl')) {
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
    const javaNamespace: any = str.replaceAll('-', '_').split(monacoStore.fileSeparator.value)
    javaNamespace.splice(2, 0, 'gen', 'restl')
    str = `namespace java ${basePackage.value.replaceAll('-', '_')}${javaNamespace.join('.')}\n`
    str += `namespace ts gen${tsNamespace.replaceAll('-', '_')}\n`
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
const handleContextmenuSelect = async (path: string, item: { label: string | ComputedRef<string>; value: string }) => {
  const currentPath =
    monacoStore.prefix.value + monacoStore.currentPath.value.replaceAll('/', monacoStore.fileSeparator.value)
  const checkFn = async () => {
    const ok = api.checkErr(
      monaco,
      files.value[path].content!,
      currentPath === path ? monacoStore.getEditor() : undefined
    )
    if (!ok) {
      messageStore.error({
        content: '存在语法错误，无法生成',
        closeable: true,
      })
    }
    return ok
  }
  if (item.value === GenTypeEnum.GEN_TS_CLIENT_CODE) {
    if (await checkFn()) {
      handleGenTsClientApi(path)
    }
  } else if (item.value === GenTypeEnum.GEN_RUST_CLIENT_CODE) {
    handleGenRustClientApi(path)
  } else if (item.value === GenTypeEnum.GEN_JAVA_SERVER_CODE) {
    if (await checkFn()) {
      handleGenJavaServerApi(path)
    }
  } else if (item.value === GenTypeEnum.GEN_JAVA_SERVER_MOCK_SERVICE) {
    if (await checkFn()) {
      handleGenJavaMockService(path)
    }
  } else if (item.value === GenTypeEnum.GEN_DB_SQL) {
    if (await checkFn()) {
      handleGenDbSql(path)
    }
  }
}
const handleGenTsClientApi = (path: string) => {
  const msgId = messageStore.info({ content: '正在生成ts客户端代码...', loading: true, closeable: true })
  rustApi
    .generateTsClientApi(path)
    .then(() => {
      messageStore.close(msgId)
      messageStore.success({
        content: '代码已生成，请稍后重新编译项目并验证',
        timeoutMs: 5000,
        closeable: true,
      })
    })
    .catch(() => {
      messageStore.close(msgId)
      messageStore.error({
        content: '保存失败，请检查是否有网络错误',
        closeable: true,
      })
    })
}
const handleGenRustClientApi = (path: string) => {
  const msgId = messageStore.info({ content: '正在生成Rust客户端代码...', loading: true, closeable: true })
  rustApi
    .generateRustClientApi(path)
    .then(() => {
      messageStore.close(msgId)
      messageStore.success({
        content: '代码已生成，请稍后重新编译项目并验证',
        timeoutMs: 5000,
        closeable: true,
      })
    })
    .catch(() => {
      messageStore.close(msgId)
      messageStore.error({
        content: '保存失败，请检查是否有网络错误',
        closeable: true,
      })
    })
}
const handleGenJavaMockService = (path: string) => {
  const msgId = messageStore.info({ content: '正在生成Java服务端mock service...', loading: true, closeable: true })
  rustApi
    .generateJavaServerMockService(path)
    .then(() => {
      messageStore.close(msgId)
      messageStore.success({
        content: '代码已生成，请稍后重新编译项目并验证',
        timeoutMs: 5000,
        closeable: true,
      })
    })
    .catch(() => {
      messageStore.close(msgId)
      messageStore.error({
        content: '保存失败，请检查是否有网络错误',
        closeable: true,
      })
    })
}
const handleGenJavaServerApi = (path: string) => {
  const msgId = messageStore.info({ content: '正在生成Java服务端代码...', loading: true, closeable: true })
  rustApi
    .generateJavaServerApi(path)
    .then(() => {
      messageStore.close(msgId)
      messageStore.success({
        content: '代码已生成，请稍后重新编译项目并验证',
        timeoutMs: 5000,
        closeable: true,
      })
    })
    .catch(() => {
      messageStore.close(msgId)
      messageStore.error({
        content: '保存失败，请检查是否有网络错误',
        closeable: true,
      })
    })
}
const handleGenDbSql = (path: string) => {
  rustApi.generateSql(path).then((res) => {
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
    language="zh-CN"
    @new-file="handleNewFile"
    @new-folder="handleNewFolder"
    @save-file="handleSaveFile"
    @rename-file="handleRename"
    @rename-folder="handleRename"
    @delete-file="handleDeleteFile"
    @delete-folder="handleDeleteFile"
    :file-menu="[
      { label: '生成JAVA服务端代码', value: GenTypeEnum.GEN_JAVA_SERVER_CODE },
      { label: '生成JAVA服务端模拟代码', value: GenTypeEnum.GEN_JAVA_SERVER_MOCK_SERVICE },
      {
        label: '生成ts客户端代码',
        value: GenTypeEnum.GEN_TS_CLIENT_CODE,
      },
      {
        label: '生成Rust客户端代码',
        value: GenTypeEnum.GEN_RUST_CLIENT_CODE,
      },
      {
        label: '预览SQL',
        value: GenTypeEnum.GEN_DB_SQL,
      },
    ]"
    :folder-menu="[]"
    :settings-menu="settingsMenu"
    @contextmenu-select="handleContextmenuSelect"
    @drag-in-editor="handleDragInEditor"
  />
</template>
./restl
