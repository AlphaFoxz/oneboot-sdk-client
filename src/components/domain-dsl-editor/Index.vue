<script setup lang="ts">
import { useRouter } from 'vue-router'
import { ComputedRef, onMounted, ref, watch } from 'vue'
import { Editor, type Files, useMessage, useMonaco } from 'monaco-tree-editor'
import 'monaco-tree-editor/index.css'
import { useServerApi } from '../server-api'
import * as monaco from 'monaco-editor'
import { registerPuml } from './puml'
import { GenTypeEnum } from './define'
import * as valid from './valid'
import * as api from '@/api'

// ================ 注册语言 ================
const monacoStore = useMonaco(monaco)
watch(monacoStore.isReady, () => {
  monacoStore.getEditor().onDidChangeModelContent(() => {})
})
registerPuml(monaco)

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

const handleContextmenuSelect = async (path: string, item: { label: string | ComputedRef<string>; value: string }) => {
  const currentPath =
    monacoStore.prefix.value + monacoStore.currentPath.value.replaceAll('/', monacoStore.fileSeparator.value)
  const checkFn = async () => {
    const ok = valid.checkErr(
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
  if (item.value === GenTypeEnum.GEN_JAVA_SERVER_CODE) {
    if (await checkFn()) {
      handleGenJavaServerCode(path)
    }
  }
}
const handleGenJavaServerCode = async (path: string) => {
  const msgId = messageStore.info({ content: '正在生成服务端代码...', loading: true, closeable: true })
  api
    .generateJavaServerDomain(path)
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
</script>

<template>
  <Editor
    ref="editorRef"
    @reload="serverApi.handleReloadPuml"
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
  />
</template>
./puml./util
