import { Button, Popover, } from 'ant-design-vue';
import { VNode, h } from 'vue';

export const rclickContext = (opt: { title: string, data: any, buttons: { title: string, name: string, callback?: Function }[] }): VNode => {
    const button = (name: string, text: string, callback: Function) => h(Button, { type: 'text', style: { display: 'block', width: '80px' }, onClick: (data: any) => callback(name, data) }, () => text)
    const hbuttons: VNode[] = []
    opt.buttons.forEach(element => {
        hbuttons.push(button(element.name, element.title, element.callback || (() => { })))
    });
    const content = () => h('div', hbuttons)
    const popover = h(Popover, {
        open: false,
        style: {
            marginLeft: '20px'
        },
        title: opt.title
    }, { content, default: () => opt.title })
    return popover
}