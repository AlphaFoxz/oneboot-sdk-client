import { defineStore } from "pinia"

export interface HotkeyState {
    dom: HTMLElement | undefined
    currentEvent?: KeyboardEvent
}

const initData = {
    dom: undefined,
    currentEvent: undefined
} as HotkeyState

export const hotkeyStore = defineStore('hotkey', {
    state: () => (initData),
    getters: {},
    actions: {
        async mounted(dom: HTMLElement) {
            dom.addEventListener('keydown', getKeydownHandler(this))
        },
        async unmount(dom: HTMLElement) {
            dom.removeEventListener('keydown', getKeydownHandler(this), { capture: false })
        },
        async setCurrentEvent(event: KeyboardEvent) {
            this.currentEvent = event
        }
    },
})

//快捷键部分
function getKeydownHandler(store: any) {
    if (!store) {
        return () => { }
    }
    return (event: KeyboardEvent) => {
        if (event.ctrlKey) {
            if (['r', 's'].includes(event.key)) {
                event.preventDefault()
            }
        }
        store.setCurrentEvent(event)
    }
}
