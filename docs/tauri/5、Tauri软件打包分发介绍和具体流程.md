# Tauri分发详解

## 什么是Tauri分发

Tauri分发是指将使用Tauri框架开发的应用程序打包并发布到各个平台的过程。Tauri提供了完整的工具链，使开发者能够将应用程序分发到平台应用商店或作为特定于平台的安装程序，覆盖桌面和移动平台的多种分发渠道。

## Tauri分发的主要方式

### 1. 桌面平台分发

**Windows平台:	https://tauri.app/zh-cn/distribute/#windows**

- 可以使用WiX Toolset v3生成Microsoft安装程序（.msi文件）
- 也可以使用NSIS生成安装可执行文件（-setup.exe文件）
- 还支持提交到Microsoft Store，但需要创建仅链接到解压后应用的Microsoft Store应用

**macOS平台:	https://tauri.app/zh-cn/distribute/#macos**

- 支持生成DMG格式安装包
- 支持macOS Application Bundle格式
- 可以通过Apple App Store分发应用

**Linux平台:	https://tauri.app/zh-cn/distribute/#linux**

- 支持AppImage格式
- 支持AUR (Arch User Repository)
- 支持Debian包 (.deb)
- 支持Flathub
- 支持RPM包 (.rpm)

### 2. 移动平台分发

**iOS平台:	https://tauri.app/zh-cn/distribute/#ios**

- 可以通过Apple App Store分发Tauri应用
- 需要在App Store Connect中注册应用，Bundle ID必须与tauri.conf.json中的配置匹配
- 需要Apple Developer计划注册

**Android平台:	https://tauri.app/zh-cn/distribute/#android**

- 支持生成APK或AAB文件
- 可以通过Google Play商店分发应用

## Tauri分发的核心流程

Tauri提供了以下命令行工具来支持应用分发：

1. **tauri build**: 生成生产环境构建

   ```bash
   # 以下内容参考 tauri 官方 “分发” 章节中的 “概览” 
   # https://tauri.app/zh-cn/distribute/
   
   # Tauri 通过 build、android build 和 ios build 命令直接从其 CLI 构建您的应用程序。
   bun tauri build
   # 大多数平台都需要代码签名。有关签名部分详细信息，请参阅
   # https://tauri.app/zh-cn/distribute/#signing
   ```

2. **tauri bundle**: 创建特定平台的安装包

   默认情况下，build 命令会自动将应用程序捆绑为配置的格式。

   如果您需要进一步自定义平台捆绑包的生成方式，可以拆分构建和捆绑包步骤：

   ```bash
   bun tauri build --no-bundle
   # 用于在 macOS App Store 之外分发的捆绑包
   bun tauri bundle --bundles app,dmg
   # 用于 App Store 分发的捆绑包
   bun tauri bundle --bundles app --config src-tauri/tauri.appstore.conf.json
   ```

3. **tauri应用软件版本控制**:

   可以在tauri.conf.json >版本配置选项中定义应用程序版本，这是管理应用程序版本的推荐方法。如果未设置该配置值，Tauri 将改用 src-tauri/Cargo.toml 文件中的包>版本值。

   某些平台对版本字符串有一些限制和特殊情况。有关详细信息，请参阅各个分发文档页面。

4. **tauri sign**: 为应用签名（对应用商店分发至关重要）

   代码签名通过将数字签名应用于应用程序的可执行文件和捆绑包来增强应用程序的安全性，从而验证应用程序提供商的身份。

   大多数平台上都需要签名。有关详细信息，请参阅每个平台的文档。https://tauri.app/zh-cn/distribute/#signing

   ```bash
   # macOS 代码签名
   # https://tauri.app/zh-cn/distribute/sign/macos/
   
   # Windows 代码签名
   # https://tauri.app/zh-cn/distribute/sign/windows/
   
   # Linux 代码签名
   # https://tauri.app/zh-cn/distribute/sign/linux/
   
   # iOS 代码签名
   # https://tauri.app/zh-cn/distribute/sign/ios/
   
   # 安卓代码签名
   # https://tauri.app/zh-cn/distribute/sign/android/
   ```

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

### 4. Github分发管道	https://tauri.app/zh-cn/distribute/pipelines/github/

本指南将向您展示如何在 GitHub Actions 中使用 tauri-action 轻松构建和上传您的应用程序，以及如何使 Tauri 的更新程序查询新创建的 GitHub 版本以获取更新。

最后，它还将展示如何为 Linux Arm AppImage 设置更复杂的构建管道。

- 创建仅链接到解压后应用的Microsoft Store应用
- 确保Microsoft Installer中链接的安装程序处于离线状态
- 处理自动更新机制

## 分发前的必要准备

1. **平台特定配置**：根据目标平台配置tauri.[platform].conf.json文件
2. **资源准备**：准备各平台所需的图标、描述等资源
3. **证书获取**：获取各应用商店所需的开发者证书
4. **测试验证**：在目标平台上测试构建的应用

通过Tauri提供的这些分发工具和流程，开发者可以轻松地将同一代码库构建的应用程序发布到多个平台和应用商店，大大简化了跨平台应用的发布流程。Tauri 2.0进一步增强了分发能力，支持更多平台和分发渠道，使开发者能够更便捷地将应用带给全球用户。