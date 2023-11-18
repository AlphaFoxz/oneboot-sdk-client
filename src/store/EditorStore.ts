import { defineStore } from 'pinia'

declare interface EditorState {
    content: string
    path: string
}

const initData = {
    content: '',
    path: ''
} as EditorState

const editorStore = defineStore('editor', {
    state: () => (initData),
    getters: {},
    actions: {
        async openFile(info: { path: string, content: string, [k: string]: any }) {
            this.content = info.content
            this.path = info.path
        }
    },
})

export {
    editorStore
}