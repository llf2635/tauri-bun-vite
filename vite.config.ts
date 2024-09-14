import {defineConfig, loadEnv} from 'vite'
import vue from '@vitejs/plugin-vue'
import AutoImport from "unplugin-auto-import/vite"
import Components from "unplugin-vue-components/vite"
import {ElementPlusResolver} from "unplugin-vue-components/resolvers"
import { createSvgIconsPlugin } from 'vite-plugin-svg-icons'
import path from 'path'

// 更多 vite 配置详细细节可以参考官网: https://cn.vitejs.dev/config/
// 在 Vite 的 API 中，开发环境下 command 的值为 serve（在 CLI 中， vite dev 和 vite serve 是 vite 的别名），而在生产环境下为 build（vite build）。
// mode 参数的值为 development（开发环境）或 production（生产环境）。具体的传递值通过 --mode 选项指定，例如 vite --mode dev 那么 mode 的值为 dev。这意味着读取 .env.development 文件中的环境变量。
/** @type {import('vite').UserConfig} */
export default defineConfig(({ command, mode })=> {
  // 根据当前工作目录中的 `mode` 加载 .env 文件
  // 设置第三个参数为 '' 来加载所有环境变量，而不管是否有 VITE_ 前缀。
  // 如果是两个参数则读取我们配置在 .env.mode的环境变量；加第三个参数 ”“ 则当前主机的全量环境变量配置
  const env = loadEnv(mode, process.cwd())
  const { VITE_VERSION, VITE_PORT, VITE_BASE_URL, VITE_API_URL } = env
  console.log(`🚀 API_URL = ${VITE_API_URL}`)
  console.log(`🚀 VERSION = ${VITE_VERSION}`)


  console.log("项目根目录： ", process.cwd())
  console.log("读取.env中配置的环境变量", env);
  // 读取运行模式，如果是dev运行 "dev": "vite --mode development", 则mode为development
  console.log("读取mode变量", mode);
  console.log("读取当前环境变量： ", env.VITE_APP_BASE_URL)
  // process.cwd() 可以获取当前工作目录，例如 D:\Git\Git-Repository\vue3-template-web
  console.log("Command: " + command)
  console.log("Mode: " + mode)

  // vite 配置
  return {
    // 插件配置，需要用到的插件数组
    plugins: [
      // 官方插件，用于支持 Vue 单文件组件
      vue(),
      // 自动导入插件，自动引入全局组件、插件、指令等
      AutoImport({
        // 全局导入注册，配置的插件中的函数、对象能够自动引入，不用显示声明
        imports: ['vue', 'vue-router', 'vue-i18n', 'pinia'],
        // 生成相应的.d.ts文件的文件路径。
        // 当“typescript”本地安装时，默认为“./auto-imports.d.ts”。
        dts: 'src/types/auto-import.d.ts',
        // 自定义 resolvers, 需要配合 unplugin-vue-components 插件一起使用
        resolvers: [ElementPlusResolver()],
      }),
      // 全局组件注册插件，自动扫描并注册全局组件
      Components({
        // 如果使用了本插件，则会自动扫描并注册全局组件，就不需要自己在 main.ts 中手动逐个注册，或者额外写一个插件来扫描并注册全局组件。
        // 用于搜索组件的目录的相对路径。自定义封装的组件目录，dirs 为本项目的组件目录，而 resolver 用于引入第三方组件
        dirs: ['src/components'],
        // 生成`components.d.ts`全局声明，还接受自定义文件名的路径
        dts: 'src/types/components.d.ts',
        // 自定义组件的解析器，为特定的插件提供按需导入的功能
        resolvers: [
          ElementPlusResolver()
        ],
      }),
      // 用于将 SVG 图标文件转换为可用的组件，并在组件中使用。
      createSvgIconsPlugin({
        // 配置图标文件夹路径（必须）
        iconDirs: [path.resolve(process.cwd(), 'src/assets/icons')],
        // 配置SymbolId格式
        symbolId: "icon-[dir]-[name]",
      }),
    ],

    // 开发服务器配置，绝对不能配置 https 选项，否则导致整个文件报错
    server: {
      // 指定服务器应该监听哪个 IP 地址。 如果将此设置为 0.0.0.0 或者 true 将监听所有地址，包括局域网和公网地址。
      host: true,
      // 指定开发服务器端口。注意：如果端口已经被使用，Vite 会自动尝试下一个可用的端口
      port: 3002,
      // 固定端口，设为 true 时若端口已占用则会直接退出，而不是尝试下一个可用端口。
      strictPort: false,
      // 为开发服务器配置 CORS,默认启用并允许任何源
      cors: true,
      // 开发服务器启动时，自动在浏览器中打开应用程序
      open: true,
      // 热更新
      hmr: true,
      // 为开发服务器配置自定义代理规则，代理所有从vite发出的url中带/api的请求
      proxy: {
        '/api': {
          // 匹配上则转发到target 目标Host
          target: env.VITE_APP_BASE_URL,
          // 是否跨域
          changeOrigin: true,
          // 路径重写，剔除/api，然后将剩余的path拼接到target后，组成最终发出去请求
          // path 参数代表的是端口后的路径，例如http://localhost:6666/api/userInfo ，则path代表/api/userInfo
          rewrite: (path) => path.replace(/^\/api/, '')
        },
        '/api/gen': {
          //单体架构下特殊处理代码生成模块代理
          target: env.VITE_IS_MICRO === 'true' ? env.VITE_ADMIN_PROXY_PATH : env.VITE_GEN_PROXY_PATH,
          changeOrigin: true,
          rewrite: (path) => path.replace(/^\/api/, ''),
        },
        // 正则表达式写法：http://localhost:5173/fallback/ -> http://jsonplaceholder.typicode.com/
        '^/fallback/.*': {
          target: 'http://jsonplaceholder.typicode.com',
          changeOrigin: true,
          rewrite: (path) => path.replace(/^\/fallback/, ''),
        },
      }
    },

    // 配置 CSS 相关的选项，包括内联的 PostCSS 配置、CSS 预处理器配置等
    css: {
      // 指定传递给 CSS 预处理器的选项
      preprocessorOptions: {
        scss: {
          // 启用 scss 语法
          javascriptEnabled: true,
          // 引入全局变量文件
          additionalData: `@import "src/styles/variables.scss";`
        }
      }
    },

    // 配置src的别名@
    // 当使用文件系统路径的别名时，请始终使用绝对路径。相对路径的别名值会原封不动地被使用，因此无法被正常解析。
    resolve: {
      alias: {
        "@": path.resolve(__dirname, "./src"),
      },
    },

    // 构建配置
    build: {
      // 构建输出目录
      outDir: 'dist',
      // 静态资源目录
      assetsDir: 'assets',
      // 构建时清空目标目录
      emptyOutDir: true,
      // 构建后是否生成 source map 文件
      sourcemap: false,
      // 构建时的根目录
      base: '/',
      // 构建时的目标环境
      target: 'esnext',
      // 构建时的模式
      mode: 'production',
    },

    // 日志级别
    logLevel: 'info',
    // 设为 false 可以避免 Vite 清屏而错过在终端中打印某些关键信息
    clearScreen: false,
    // 以 envPrefix 内元素开头的环境变量会通过 import.meta.env 暴露在你的客户端源码中。默认只对外暴露以 VITE_ 开头的环境变量。
    envPrefix: ["VITE_", "BUN_", "MODE_"],
  }
})