import { defineStore } from "pinia"

declare type ConfigState = {
  debug: boolean,
  componentSize: 'large' | 'middle' | 'small',
  theme: {
    token?: {
      colorPrimary: string
    }
  },
  space: {
    align: 'start' | 'end'| 'center' | 'baseline',
    direction: 'vertical' | 'horizontal',
    size: 'small' | 'middle' | 'large' | number,
  }
}

const initData : ConfigState = {
  debug: true,
  componentSize: 'middle',
  theme: {
  },
  space: {
    align: 'start',
    direction: 'horizontal',
    size: 'middle',
  }
}

const configStore = defineStore('config', {
  state: () => (initData),
  getters: {},
  actions: {
    async setComponentSize(size: number) {
      switch (size) {
        case 1: this.componentSize = 'small'; break
        case 2: this.componentSize = 'middle'; break
        case 3: this.componentSize = 'large'; break
        default:
      }
    }
  },
})

export {
  configStore
}