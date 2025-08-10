#### 1、安装 Bun
sudo npm install -g bun
bun create vite bun-vue3-ts-demo --template vue-ts


#### 2、Bun 的常用命令

**启动 Bun 交互式环境：**

* `bun`

**运行 JavaScript 文件：**

* `bun run`

**创建和管理项目：**

* `bun create`：创建一个新的 Bun 项目
* `bun init`：在现有项目中初始化 Bun

**包管理：**

* `bun add`：安装一个 npm 包，可以通过 -d 参数安装为开发依赖项
* `bun install`：安装所有的 npm 包
* `bun remove`：移除一个已安装的 npm 包
* `bun update`：更新已安装的 npm 包

**构建和部署：**

* `bun build`：构建项目
* `bun start`：启动项目
* `bun serve`：启动一个本地开发服务器

**测试：**

* `bun test`：运行项目测试
* `bun coverage`：生成测试覆盖率报告

**调试：**

* `bun debug`：启动调试器
* `bun inspect`：检查正在运行的进程

**打包：**

* `bun bundle`：将项目打包为单个可执行文件

**发布：**

* `bun publish`：将项目发布到 npm

**工具：**

* `bun format`：格式化 JavaScript 代码
* `bun lint`：检查 JavaScript 代码的语法和样式错误

**其他有用命令：**

* `bun repl`：启动一个 REPL（交互式 JavaScript 环境）
* `bun docs`：打开 Bun 文档
* `bun help`：显示所有可用命令的帮助信息
* `bun upgrade`：自行升级 Bun

**命令作用：**

**启动交互式环境：** `bun` 命令启动一个交互式 Bun 环境，你可以在其中输入 JavaScript 代码并立即获得结果。

**运行 JavaScript 文件：** `bun run` 命令用于运行 JavaScript 文件。它接受一个文件名或一个包名作为参数。

**创建和管理项目：** `bun create` 命令创建一个新的 Bun 项目，并初始化它所需的目录和文件。`bun init` 命令在现有项目中初始化 Bun。

**包管理：** `bun add`、`bun remove` 和 `bun update` 命令用于管理项目的 npm 依赖项。

**构建和部署：** `bun build` 命令将项目构建为可部署的格式。`bun start` 命令启动项目，以便在本地运行。`bun serve` 命令启动一个本地开发服务器，用于测试和调试。

**其他有用命令：** `bun repl` 命令启动一个 REPL，便于快速测试 JavaScript 代码或执行交互式任务。`bun docs` 命令打开 Bun 文档，其中包含有关 Bun 的详细信息。`bun help` 命令显示所有可用命令的帮助信息。