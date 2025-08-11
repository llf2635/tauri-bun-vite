import { createApp } from "vue";
import App from "./App.vue";
import pinia from "@/stores";
import router from "@/router";

// import '@/styles/main.scss'
// UnoCSS 样式
import 'virtual:uno.css'
// import '@unocss/reset/tailwind.css'
import {createI18n} from "vue-i18n";
import {invoke} from "@tauri-apps/api/core";


const i18n = createI18n({
    // something vue-i18n options here ...
})

// 创建vue实例
const app = createApp(App);

// 注册i18n
app.use(i18n);
// 注册pinia
app.use(pinia)
// 注册路由
app.use(router)

// 等待路由准备就绪后再挂载应用
router.isReady().then(() => {
    // 路由初始化完成后挂载Vue根实例
    app.mount('#app')
})

// 应用级错误处理 可以用来向追踪服务报告错误
app.config.errorHandler = (error) => {
    /* 处理错误 */
    console.log("the error is: " + error);
};


// 你现在应该会看到一个启动画面窗口弹出，前端和后端将各自执行耗时 3 秒的初始化任务，完成后启动画面会消失，并显示主窗口
// 参考 https://v2.tauri.org.cn/learn/splashscreen/

// 在 TypeScript 中实现的一个 sleep 函数
function sleep(seconds: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, seconds * 1000));
}

// 设置函数
async function setup() {
    // 模拟执行一个很重的前端设置任务
    console.log('执行非常繁重的前端设置任务...')
    await sleep(3);
    console.log('前端设置任务完成!')
    // 设置前端任务为完成
    invoke('set_complete', {task: 'frontend'})
}

// 实际上的 JavaScript main 函数
window.addEventListener("DOMContentLoaded", () => {
    setup()
});