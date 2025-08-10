# Tauri 2.0 详细介绍

## 什么是 Tauri 2.0

Tauri 2.0 是一个用于为所有主要桌面平台（macOS、Linux、Windows）和移动平台（iOS、Android）构建小巧快速二进制文件的框架。 它是一个Rust编写的下一代跨平台开发框架，允许开发者使用各平台的Webview技术栈为桌面操作系统构建应用程序。

Tauri 2.0 的核心理念是将你现有的网络技术栈带到Tauri或开始新项目，它支持任何前端框架，因此你不需要改变现有的技术栈。 与Electron等框架相比，Tauri构建的应用程序体积更小、运行速度更快、安全性更高。

## Tauri 2.0 的主要作用

Tauri 2.0 带来了新的跨平台支持、增强的开发者体验、更强大的插件系统以及更安全的权限管理。 其主要优势包括：

1. **轻量级应用**：构建的应用程序体积远小于Electron应用
2. **高性能**：使用Rust作为底层语言，提供更好的性能
3. **跨平台支持**：单一代码库支持Linux、macOS、Windows以及移动平台
4. **安全性**：提供更安全的权限管理系统
5. **技术栈自由**：支持任何前端框架，无需改变现有技术栈

## Tauri 2.0 的官方插件

Tauri 2.0 拥有丰富的官方插件生态系统，所有官方插件可以在GitHub的plugins-workspace仓库中找到，这些插件需要Rust 1.77.2或更高版本。

### 常用官方插件：

1. **tauri-plugin-app-control**：专注于提供全面的Android应用生命周期控制的Tauri 2插件。

2. **菜单插件**：用于配置应用菜单，根据搜索结果，开发者在Tauri v2中配置菜单是一个常见需求。

3. **其他官方插件**：Tauri 2.0的插件系统更为强大，插件开发者能够使用Swift和Kotlin编写代码，以便更紧密地集成到各自平台。

## 配置与使用教程

### 基础配置

Tauri的配置对象从配置文件中读取，你可以在此定义前端资源、配置打包器和定义系统托盘图标。 在Tauri 2.0中，插件配置位置是一个常见问题，根据社区讨论，插件管理通常在lib.rs或main.rs中进行，具体取决于插件类型。

### 基本使用步骤

1. **初始化项目**：

   ```bash
   # 查看 bun create tauri-app 的命令帮助文档
   bun create tauri-app -h
   # 创建 tauri 项目
   bun create tauri-app
   # 项目启动前操作
   cd tauri-app && bun install
   # 如果此处失败了，请在 android-studio 中点击  setting -> 搜索 Android SDK
   bun run tauri android init
   # 启动桌面应用程序
   bun run tauri dev
   # 启动 android 安卓应用程序，在运行该命令之前必须先在 android-studio 中运行内嵌手机，否则执行会卡住没反映
   bun run tauri android dev
   ```

2. **配置文件**：
   Tauri 2.0的配置主要在`tauri.conf.json`文件中进行，你可以定义前端资源路径、应用元数据等。

3. **添加插件**：

   ```bash
   # 在系统启动时自动启动应用程序
   # https://tauri.app/zh-cn/plugin/autostart/
   bun tauri add autostart
   # 允许您的移动应用程序使用相机扫描 QR 码、EAN-13 和其他类型的条形码
   # https://tauri.app/zh-cn/plugin/barcode-scanner/
   bun tauri add barcode-scanner
   # 使用剪贴板插件读取和写入系统剪贴板
   # https://tauri.app/zh-cn/plugin/clipboard/
   bun tauri add clipboard-manager
   # 本机系统对话框，用于打开和保存文件，以及消息对话框
   # https://tauri.app/zh-cn/plugin/dialog/
   bun tauri add dialog
   # 访问文件系统
   # https://tauri.app/zh-cn/plugin/file-system/
   bun tauri add fs
   # 注册全局快捷方式
   # https://tauri.app/zh-cn/plugin/global-shortcut/
   bun tauri add global-shortcut
   # 使用 HTTP 插件发起 HTTP 请求
   # https://tauri.app/zh-cn/plugin/http-client/
   bun tauri add http
   # 为你的 Tauri 应用程序配置日志记录
   # https://tauri.app/zh-cn/plugin/logging/
   bun tauri add log
   # 在 Android 和 iOS 上读取和写入 NFC 标签
   # https://tauri.app/zh-cn/plugin/nfc/
   bun tauri add nfc
   # 使用通知提示插件以向你的用户发送原生通知
   # https://tauri.app/zh-cn/plugin/notification/
   bun tauri add notification
   # 使用操作系统信息插件读取操作系统信息
   # https://tauri.app/zh-cn/plugin/os-info/
   bun tauri add os
   # 使用单实例插件确保 Tauri 应用程序在同一时间只运行单个实例
   # https://tauri.app/zh-cn/plugin/single-instance/
   bun tauri add single-instance
   # 这个插件提供了一个接口，让前端可以通过 sqlx 与 SQL 数据库进行通信。 它支持 SQLite、MySQL 和 PostgreSQL 驱动程序，通过 Cargo 特性来启用。
   # https://tauri.app/zh-cn/plugin/sql/
   bun add @tauri-apps/plugin-sql
   # 简单、持久的键值存储
   # https://tauri.app/zh-cn/plugin/store/
   bun tauri add store
   # 自动使用更新服务器或静态 JSON 更新你的 Tauri 应用程序
   # https://tauri.app/zh-cn/plugin/updater/
   bun tauri add updater
   # 过 HTTP 从磁盘上传文件到远程服务器。从远程 HTTP 服务器下载文件到磁盘
   # https://tauri.app/zh-cn/plugin/upload/
   bun tauri add upload
   # 在 JavaScript 中使用 Rust 客户端打开 WebSocket 连接
   # https://tauri.app/zh-cn/plugin/websocket/
   bun tauri add websocket
   # 保存窗口位置和大小，并在应用程序重新打开时恢复它们
   # https://tauri.app/zh-cn/plugin/window-state/
   bun tauri add window-state
   ```

   然后在Rust代码中导入并注册插件。

4. **JavaScript/TypeScript API使用**：
   以tauri-plugin-app-control为例，你需要导入插件API并在前端代码中使用。

### 开发流程

1. 开发前端应用（使用任何你喜欢的前端框架）
2. 配置Tauri项目
3. 添加必要的官方插件
4. 使用`tauri dev`命令进行开发
5. 使用`tauri build`命令构建最终应用

Tauri 2.0稳定版已正式发布，为开发者提供了更完善的跨平台应用开发体验，特别适合那些希望构建轻量级、高性能桌面和移动应用的团队。 如需更详细的配置指南，建议查阅Tauri官方文档或其中文文档资源。