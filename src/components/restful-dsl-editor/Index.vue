<script setup lang="ts">
import { useRouter } from 'vue-router'
import { Modal as AModal, Tabs as ATabs, TabPane as ATabPane } from 'ant-design-vue'
import { ComputedRef, onMounted, ref, watch } from 'vue'
import { Editor, type Files, useMessage, useMonaco, useHotkey } from 'monaco-tree-editor'
import 'monaco-tree-editor/index.css'
import * as utils from '@/utils'
import * as valid from './valid'
import * as api from '@/api'
import { useServerApi } from '../server-api'
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
watch(monacoStore.state.isReady, (n) => {
  if (!n) return
  monacoStore.action.getEditor().onDidChangeModelContent(() => {})
})
registerRestl(monaco)

// ================ 快捷键 ================
const hotkeyStore = useHotkey()
hotkeyStore.listen('editor', (e) => {
  if (e.ctrlKey && e.altKey && !e.shiftKey) {
    if (e.key === 'ArrowLeft') {
      monacoStore.action.getEditor().trigger('keyboard', 'cursorUndo', null)
    } else if (e.key === 'ArrowRight') {
      monacoStore.action.getEditor().trigger('keyboard', 'cursorRedo', null)
    }
  } else if (e.ctrlKey && !e.altKey && e.shiftKey) {
    if (e.key === '?' || e.key === '/') {
      monacoStore.action.getEditor().trigger('keyboard', 'editor.action.blockComment', null)
    } else if (e.key === 'ArrowUp') {
      monacoStore.action.getEditor().trigger('keyboard', 'editor.action.moveLinesUpAction', null)
    } else if (e.key === 'ArrowDown') {
      monacoStore.action.getEditor().trigger('keyboard', 'editor.action.moveLinesDownAction', null)
    }
  } else if (e.ctrlKey && !e.altKey && !e.shiftKey) {
    if (e.key === 'y' || e.key === 'Y') {
      console.debug(monacoStore.state.monaco.editor.getModels())
      monacoStore.action.getEditor().trigger('keyboard', 'editor.action.openLink', '/sdk/dtos/SdkResponseDto.restl')
    }
  }
})

// ================ 加载文件 load files ================
const files = ref<Files>({})
const serverApi = useServerApi(files, valid)

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
const basePackage = ref('')
onMounted(() => {
  api.getBasePackage().then((res) => {
    basePackage.value = res.data!
  })
})
const handleDragInEditor = (srcPath: string, targetPath: string, type: 'file' | 'folder') => {
  if (!targetPath.endsWith('.restl')) {
    return
  }
  const editor = monacoStore.action.getEditor()
  const lineIndex = editor.getPosition()?.lineNumber!
  let str = ''
  if (type === 'file') {
    str += 'import "' + relativePathFrom(srcPath, targetPath) + '"'
  } else {
    str = srcPath.replace(monacoStore._state.prefix.value, '')
    const tsNamespace = str.replaceAll(monacoStore._state.fileSeparator.value, '.')
    const javaNamespace: any = str.replaceAll('-', '_').split(monacoStore._state.fileSeparator.value)
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
    monacoStore._state.prefix.value +
    monacoStore.state.currentPath.value.replaceAll('/', monacoStore._state.fileSeparator.value)
  const checkFn = async () => {
    const ok = valid.checkErr(
      monaco,
      files.value[path].content!,
      currentPath === path ? monacoStore.action.getEditor() : undefined
    )
    if (!ok) {
      messageStore.action.error({
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
  const msgId = messageStore.action.info({ content: '正在生成ts客户端代码...', loading: true, closeable: true })
  api
    .generateTsClientApi(path)
    .then(() => {
      messageStore.action.close(msgId)
      messageStore.action.success({
        content: '代码已生成，请稍后重新编译项目并验证',
        timeoutMs: 5000,
        closeable: true,
      })
    })
    .catch(() => {
      messageStore.action.close(msgId)
      messageStore.action.error({
        content: '保存失败，请检查是否有网络错误',
        closeable: true,
      })
    })
}
const handleGenRustClientApi = (path: string) => {
  const msgId = messageStore.action.info({ content: '正在生成Rust客户端代码...', loading: true, closeable: true })
  api
    .generateRustClientApi(path)
    .then(() => {
      messageStore.action.close(msgId)
      messageStore.action.success({
        content: '代码已生成，请稍后重新编译项目并验证',
        timeoutMs: 5000,
        closeable: true,
      })
    })
    .catch(() => {
      messageStore.action.close(msgId)
      messageStore.action.error({
        content: '保存失败，请检查是否有网络错误',
        closeable: true,
      })
    })
}
const handleGenJavaMockService = (path: string) => {
  const msgId = messageStore.action.info({
    content: '正在生成Java服务端mock service...',
    loading: true,
    closeable: true,
  })
  api
    .generateJavaServerMockService(path)
    .then(() => {
      messageStore.action.close(msgId)
      messageStore.action.success({
        content: '代码已生成，请稍后重新编译项目并验证',
        timeoutMs: 5000,
        closeable: true,
      })
    })
    .catch(() => {
      messageStore.action.close(msgId)
      messageStore.action.error({
        content: '保存失败，请检查是否有网络错误',
        closeable: true,
      })
    })
}
const handleGenJavaServerApi = (path: string) => {
  const msgId = messageStore.action.info({ content: '正在生成Java服务端代码...', loading: true, closeable: true })
  api
    .generateJavaServerApi(path)
    .then(() => {
      messageStore.action.close(msgId)
      messageStore.action.success({
        content: '代码已生成，请稍后重新编译项目并验证',
        timeoutMs: 5000,
        closeable: true,
      })
    })
    .catch(() => {
      messageStore.action.close(msgId)
      messageStore.action.error({
        content: '保存失败，请检查是否有网络错误',
        closeable: true,
      })
    })
}
const handleGenDbSql = (path: string) => {
  api.generateSql(path).then((res) => {
    if (res.success) {
      messageStore.action.success({
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
      messageStore.action.error({
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
    @reload="serverApi.handleReloadRestfulDsl"
    :files="files"
    language="zh-CN"
    @new-file="serverApi.handleNewFile"
    @new-folder="serverApi.handleNewFolder"
    @save-file="serverApi.handleSaveFile"
    @rename-file="serverApi.handleRename"
    @rename-folder="serverApi.handleRename"
    @delete-file="serverApi.handleDeleteFile"
    @delete-folder="serverApi.handleDeleteFile"
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
