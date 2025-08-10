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

### Tauri开发流程

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
   
   
   # 以下内容参考 tauri 官方 “开发” 章节中的 “概述” 
   # https://tauri.app/zh-cn/develop/
   
   # 这条命令会启动一个安卓模拟器，我们的软件会在上面运行，这种方式不会额外打开 Xcode 或 Android Studio 对电脑内存使用更加友好
   bun run tauri [android|ios] dev
   # 使用 Xcode 或 Android Studio 来开发你的应用，会自动打开 Xcode 或 Android Studio
   bun run tauri [android|ios] dev --open
   
   
   # 以下内容参考 tauri 官方 “分发” 章节中的 “概览” 
   # https://tauri.app/zh-cn/distribute/#building
   
   # 这条命令只会打包构建桌面端软件包，不包含 android/ios 移动端软件包
   bun tauri build
   
   
   # 在打包之前必须先签名，否则即使打包成功 apk 也无法安装。安卓签名 https://tauri.app/distribute/sign/android/
   # Android App Bundle 和 APK 必须先签名，然后才能上传以进行分发。
   # 打包后的 apk 安装包存放目录 src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk
   
   keytool -genkey -v -keystore ~/upload-keystore.jks -keyalg RSA -keysize 2048 -validity 10000 -alias upload
   # 为 tauri 应用配置上面的配置签名密钥
   # 创建一个名为 src-tauri/gen/android/keystore.properties 的文件，其中包含对密钥库的引用：
    password=执行时定义的密码 例如 lcqh2635
    keyAlias=upload
    storeFile=密钥存储文件的位置 例如 /home/lcqh/upload-keystore.jks
   
   # 使用脚本创建 keystore.properties 文件，直接在项目根目录下执行即可
    cat > src-tauri/gen/android/keystore.properties << EOF
    password=lcqh2635
    keyAlias=upload
    storeFile=/home/lcqh/upload-keystore.jks
    EOF
   
   # 将 Gradle 配置为使用签名密钥
   # https://tauri.app/distribute/sign/android/#configure-gradle-to-use-the-signing-key
   # 顺带修改一下 Gradle 和 Kotlin 使用的 JDK 版本，默认使用的是 JDK 8 手动改为 JDK 21
   # 打开 src-tauri/gen/android/app/build.gradle.kts 安卓构建配置文件，在配置文件的 android.kotlinOptions 方找到 
       kotlinOptions {
           jvmTarget = "1.8"
       }
   # 将 kotlinOptions 整个块删除并在最外层也就是和 android 块同层下添加如下的的 kotlin 块：
   # 配置参考自 https://kotlinlang.org/docs/gradle-configure-project.html?utm_campaign=gradle-jvm-toolchain&utm_medium=kgp&utm_source=warnings#gradle-java-toolchains-support
   kotlin {
       jvmToolchain(21) // 或 17, 21 等你想要的版本
       // 或者使用更详细的配置：
       // jvmToolchain {
       //     languageVersion.set(JavaLanguageVersion.of(17))
       // }
   }
   
   
   # 这条命令只会打包构建 android 软件包
   bun tauri android build
   # 这条命令只会打包构建 ios 软件包，必须在 macOS 电脑上才能打包，因为需要 xcode 才能打包 ios
   bun tauri ios build
   ```

2. **tauri配置文件**：
   Tauri 2.0的配置主要在`tauri.conf.json`文件中进行，你可以定义前端资源路径、应用元数据等。

   ```bash
   # 以下内容参考 tauri 官方 “开发” 章节中的 “配置文件” 
   # https://tauri.app/zh-cn/develop/configuration-files/
   
   # Tauri 配置的默认格式为 JSON。你可以通过在 Cargo.toml 中的 tauri 和 tauri-build 依赖项中分别添加 config-json5 或 config-toml 功能标志来启用 JSON5 或 TOML 格式。
   [build-dependencies]
   tauri-build = { version = "2", features = [ "config-json5" ] }
   
   [dependencies]
   tauri = { version = "2", features = [  "config-json5" ] }
   # 现在我们就可以将原有的 tauri 配置文件 tauri.conf.json 改为 tauri.conf.json5 这样就可以在配置文件中写注释说明了
   
   
   # 除了默认 tauri.conf.json 配置文件之外，Tauri 还可以从以下位置读取特定于平台的配置：
       tauri.linux.conf.json 或 Tauri.linux.toml （适用于 Linux）
       tauri.windows.conf.json 或 Tauri.windows.toml （适用于 Windows）
       tauri.macos.conf.json 或 Tauri.macos.toml （适用于 macOS）
       tauri.android.conf.json 或 Tauri.android.toml （适用于 Android）
       tauri.ios.conf.json 或 Tauri.ios.toml （适用于 iOS）
   特定于平台的配置文件按照 [JSON Merge Patch（RFC 7396）] 规范与主配置对象合并。
   ```

3. **添加tauri常用插件**：

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

5. **tauri应用程序App图标**：

   Tauri 附带了一个基于其徽标的默认图标集。这不是您在发布应用程序时想要的。为了纠正这种常见情况，Tauri 提供了图标命令，该命令将接受输入文件（默认为“./app-icon.png”）并创建各种平台所需的所有图标。

   ```bash
   # 以下内容参考 tauri 官方 “开发” 章节中的 “App 图标” 
   # https://tauri.app/zh-cn/develop/icons/
   
   # 关于文件类型的说明
   	icon.icns = macOS
       icon.ico = Windows
       *.png = Linux
       Square*Logo.png & StoreLogo.png = 当前未使用，但适用于 AppX/MS Store 目标。
   # 某些图标类型可能用于上面列出的平台以外的平台（尤其是 png）。因此，我们建议包含所有图标，即使您只打算为一部分平台进行构建。
   
   bun tauri icon
   bun tauri icon --help
   ```

### Tauri测试流程

1. Mock Tauri APIs

   ```bash
   # 以下内容参考 tauri 官方 “测试” 章节中的 “Mock Tauri APIs” 
   # https://tauri.app/zh-cn/develop/tests/mocking/
   ```

2. 配置Tauri项目

3. 添加必要的官方插件

4. 使用`tauri dev`命令进行开发

5. 使用`tauri build`命令构建最终应用

### Tauri分发流程

Tauri分发是指将使用Tauri框架开发的应用程序打包并发布到各个平台的过程。Tauri提供了完整的工具链，使开发者能够将应用程序分发到平台应用商店或作为特定于平台的安装程序，覆盖桌面和移动平台的多种分发渠道。

## Tauri分发的主要方式

### 1. 桌面平台分发

**Windows平台:**

- 可以使用WiX Toolset v3生成Microsoft安装程序（.msi文件）
- 也可以使用NSIS生成安装可执行文件（-setup.exe文件）
- 还支持提交到Microsoft Store，但需要创建仅链接到解压后应用的Microsoft Store应用

**macOS平台:**

- 支持生成DMG格式安装包
- 支持macOS Application Bundle格式
- 可以通过Apple App Store分发应用

**Linux平台:**

- 支持AppImage格式
- 支持AUR (Arch User Repository)
- 支持Debian包 (.deb)
- 支持Flathub
- 支持RPM包 (.rpm)

### 2. 移动平台分发

**iOS平台:**

- 可以通过Apple App Store分发Tauri应用
- 需要在App Store Connect中注册应用，Bundle ID必须与tauri.conf.json中的配置匹配
- 需要Apple Developer计划注册

**Android平台:**

- 支持生成APK或AAB文件
- 可以通过Google Play商店分发应用

## Tauri分发的核心工具

Tauri提供了以下命令行工具来支持应用分发：

1. **tauri build**: 生成生产环境构建
2. **tauri bundle**: 创建特定平台的安装包
3. **tauri sign**: 为应用签名（对应用商店分发至关重要）

## 分发流程详解

### 1. 配置分发参数

在`tauri.conf.json`文件的`bundle`部分配置分发相关参数：

```json
{
  "bundle": {
    "active": true,
    "identifier": "com.example.app",
    "icon": [...],
    "resources": [],
    "externalBin": [],
    "copyright": "",
    "category": "Utility",
    "shortDescription": "",
    "longDescription": "",
    "deb": {
      "depends": []
    },
    "osx": {
      "frameworks": [],
      "minimumSystemVersion": "10.13.0"
    },
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": ""
    }
  }
}
```

### 2. 应用签名（重要步骤）

对于应用商店分发，应用签名是必不可少的：

- **macOS/iOS**: 需要Apple开发者证书
- **Windows**: 需要代码签名证书
- **Linux**: 通常不需要签名，但某些分发渠道可能需要

Tauri提供了`tauri sign`命令来简化签名过程。

### 3. App Store分发流程

1. 注册Apple Developer计划
2. 在App Store Connect中注册应用，确保Bundle ID与tauri.conf.json中配置一致
3. 配置正确的签名证书和配置文件
4. 使用Tauri构建应用
5. 通过Transporter工具或Xcode将应用上传到App Store Connect

### 4. Microsoft Store分发特殊说明

由于Tauri目前只生成EXE和MSI安装程序，要分发到Microsoft Store，需要：

- 创建仅链接到解压后应用的Microsoft Store应用
- 确保Microsoft Installer中链接的安装程序处于离线状态
- 处理自动更新机制

## 分发前的必要准备

1. **平台特定配置**：根据目标平台配置tauri.[platform].conf.json文件
2. **资源准备**：准备各平台所需的图标、描述等资源
3. **证书获取**：获取各应用商店所需的开发者证书
4. **测试验证**：在目标平台上测试构建的应用

通过Tauri提供的这些分发工具和流程，开发者可以轻松地将同一代码库构建的应用程序发布到多个平台和应用商店，大大简化了跨平台应用的发布流程。Tauri 2.0进一步增强了分发能力，支持更多平台和分发渠道，使开发者能够更便捷地将应用带给全球用户。
