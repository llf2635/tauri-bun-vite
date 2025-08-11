在 **Tauri 2.x** 中，`updater` 插件（即 `@tauri-apps/plugin-updater`）用于实现桌面应用的**自动更新功能**。它支持从远程服务器下载更新包（如 `.tar.gz` 或 `.zip`），并静默或提示用户安装更新。

你提到的命令：

```bash
bun tauri add updater

# 使用 Tauri 更新器插件，使用更新服务器或静态 JSON 自动更新你的 Tauri 应用程序。
# 详情参考 https://v2.tauri.org.cn/plugin/updater/

# Tauri 的更新器需要签名来验证更新是否来自可信来源。这无法禁用。
bunx tauri signer generate -w ~/.tauri/tauri-bun-vite.key

# 在构建你的更新工件时，你需要将上面生成的私钥放在你的环境变量中。.env 文件不起作用！
# 私钥的路径或私钥字符串
export TAURI_SIGNING_PRIVATE_KEY="/home/lcqh/.tauri/tauri-bun-vite.key"
# 还可以添加密码（可选）
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD="lcqh2635"

# Private: /home/lcqh/.tauri/tauri-bun-vite.key (Keep it secret!)
# Public: /home/lcqh/.tauri/tauri-bun-vite.key.pub


{
  "bundle": {
    "createUpdaterArtifacts": true
  },
  # Tauri 插件配置，https://v2.tauri.org.cn/reference/config/#plugins
  "plugins": {
    # 应用更新插件配置，https://v2.tauri.org.cn/plugin/updater/
    "updater": {
      # cat /home/lcqh/.tauri/tauri-bun-vite.key.pub
      "pubkey": "来自公钥的内容。PEM的",
      "endpoints": [
        # {{target}}：操作系统名称（linux、windows 或 darwin 之一）。
        # {{arch}}：机器架构（x86_64、i686、aarch64 或 armv7 之一）。
        # {{current_version}}：请求更新的应用程序版本。
        "https://releases.myapp.com/{{target}}/{{arch}}/{{current_version}}",
        # 或静态 github json 文件
        "https://github.com/user/repo/releases/latest/download/latest.json"
      ]
    }
  }
}

# Tauri Action 为你生成一个静态 JSON 文件，以便在 GitHub Releases 等 CDN 上使用。
# https://github.com/tauri-apps/tauri-action
```

是 **使用 Bun 作为包管理器时** 安装 Tauri 更新插件的正确方式。

---

## 🧰 一、安装插件（`bun tauri add updater`）

### ✅ 前提条件

- 使用 Tauri 2.x（`@tauri-apps/cli@^2`）
- 项目已通过 `bun create tauri-app` 或手动配置
- 使用 **Bun** 作为包管理器（`bun.lockb`）

### 🔧 安装命令

```bash
bun tauri add updater
```

这会自动执行以下操作：

1. 安装 npm 包：`@tauri-apps/plugin-updater`
2. 在 `src-tauri/Cargo.toml` 中添加原生依赖：
   ```toml
   [dependencies]
   tauri-plugin-updater = { version = "2" }
   ```
3. 创建或更新 `tauri.conf.json` 配置（如果需要）

> ✅ 该命令是 Tauri CLI 提供的高级封装，等价于手动安装 npm 包 + 修改 `Cargo.toml`

---

## ⚙️ 二、配置 `updater` 插件

### 1. 在 `main.rs` 中注册插件（Rust 端）

编辑 `src-tauri/src/main.rs`：

```rust
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build()) // 注册 updater 插件
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let win = app.get_window("main").unwrap();
                win.open_devtools(); // 调试用
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. 配置 `tauri.conf.json`（更新源）

编辑 `src-tauri/tauri.conf.json`，添加 `updater` 配置：

```json
{
  "package": {
    "version": "1.0.0"
  },
  "tauri": {
    "bundle": {
      "identifier": "com.example.myapp"
    },
    "updater": {
      "endpoints": [
        "https://your-server.com/releases.json"  // 更新信息地址
      ],
      "pubkey": "your-public-key-here"           // 可选：用于签名验证
    }
  }
}
```

---

## 📦 三、更新包格式与服务器要求

### 1. `releases.json` 文件结构

服务器必须提供一个 `releases.json` 文件，格式如下：

```json
{
  "version": "1.1.0",
  "notes": "修复了若干 bug，新增自动保存功能。",
  "pub_date": "2025-04-05T12:00:00Z",
  "platforms": {
    "x86_64-apple-darwin": {
      "url": "https://your-server.com/myapp-x86_64-apple-darwin.tar.gz",
      "signature": "...",
      "size": 12345678
    },
    "x86_64-pc-windows-msvc": {
      "url": "https://your-server.com/myapp-x86_64-pc-windows-msvc.zip",
      "signature": "...",
      "size": 12345678
    },
    "aarch64-apple-darwin": {
      "url": "https://your-server.com/myapp-aarch64-apple-darwin.tar.gz"
    }
  }
}
```

> 🔍 Tauri updater 会根据当前系统自动选择对应平台的包。

### 2. 打包格式支持

| 平台 | 支持格式 |
|------|----------|
| Windows | `.zip` |
| macOS | `.tar.gz` |
| Linux | `.tar.gz` |

> 包内必须包含可执行文件（如 `myapp.exe` 或 `myapp`），结构与打包输出一致。

---

## 📞 四、前端调用更新功能（JavaScript/TypeScript）

### 1. 安装 npm 包（已由 `bun tauri add` 完成）

```bash
# 已自动安装
bun add @tauri-apps/plugin-updater
```

### 2. 在前端代码中使用

```ts
import { check, InstallError } from '@tauri-apps/plugin-updater';

async function checkForUpdate() {
  try {
    const update = await check();

    if (update?.available) {
      console.log(`发现新版本：${update.version}`);
      console.log(`更新日志：${update.note}`);

      // 显示更新对话框
      const confirmed = confirm(`发现新版本 ${update.version}，是否更新？\n\n${update.note}`);
      if (confirmed) {
        await update.downloadAndInstall(); // 下载并安装
        alert('更新完成，应用将重启。');
      }
    } else {
      alert('当前已是最新版本');
    }
  } catch (error) {
    if (error instanceof InstallError) {
      console.error('安装失败:', error);
      alert('更新失败：安装出错');
    } else {
      console.error('检查更新失败:', error);
      alert('网络错误或无法连接更新服务器');
    }
  }
}
```

---

## 🔐 五、可选：签名验证（安全更新）

为了防止中间人攻击，你可以对更新包进行签名。

### 1. 生成密钥对

```bash
tauri signkey
```

输出：

```
Public key: your-public-key-here
Private key: your-private-key-here
```

### 2. 在 `tauri.conf.json` 中配置公钥

```json
"updater": {
  "endpoints": ["https://your-server.com/releases.json"],
  "pubkey": "your-public-key-here"
}
```

### 3. 签名 `releases.json`

使用私钥签名：

```bash
tauri sign --content releases.json --private-key your-private-key-here
```

输出签名值，填入 `releases.json` 的 `signature` 字段：

```json
{
  "version": "1.1.0",
  "signature": "MEUCIQD...",
  "platforms": { ... }
}
```

> ✅ updater 插件会自动验证签名，防止篡改。

---

## 🧪 六、测试更新流程

### 1. 本地测试建议

- 使用 `http-server` 启动本地服务：
  ```bash
  bunx http-server -p 8080
  ```
- 修改 `tauri.conf.json` 指向 `http://localhost:8080/releases.json`
- 手动创建一个 `releases.json`，版本高于当前应用

### 2. 模拟更新

运行应用 → 调用 `check()` → 应提示更新

---

## 📌 七、注意事项与限制

| 项目 | 说明 |
|------|------|
| ❌ 不支持移动端 | `updater` 仅适用于桌面平台（Windows/macOS/Linux） |
| ✅ 支持增量更新？ | ❌ 否，目前是全量更新 |
| 🔄 更新后行为 | 安装完成后自动重启应用 |
| 🔐 安全性 | 建议启用 `pubkey` 签名验证 |
| 🌐 网络要求 | 用户需能访问你的更新服务器 |
| 🧼 清理旧版本 | 插件会自动清理临时下载文件 |

---

## 🧩 八、常见问题

### Q1: `bun tauri add updater` 报错？

- 确保你在项目根目录运行
- 确保 `bun` 和 `@tauri-apps/cli` 已安装
- 尝试手动安装：
  ```bash
  bun add @tauri-apps/plugin-updater
  ```
  并手动在 `Cargo.toml` 添加：
  ```toml
  tauri-plugin-updater = { version = "2" }
  ```

### Q2: 更新不触发？

- 检查 `releases.json` 版本是否高于当前 `package.version`
- 检查网络是否能访问 `endpoints`
- 打开 DevTools 查看控制台错误

---

## ✅ 总结：完整流程

| 步骤 | 操作 |
|------|------|
| 1️⃣ | `bun tauri add updater` 安装插件 |
| 2️⃣ | 在 `main.rs` 注册插件 |
| 3️⃣ | 配置 `tauri.conf.json` 的 `updater.endpoints` |
| 4️⃣ | 部署 `releases.json` 和更新包到服务器 |
| 5️⃣ | 前端调用 `check()` 和 `downloadAndInstall()` |
| 6️⃣ | （可选）启用签名验证提升安全性 |

---

如果你希望，我可以为你生成一个：
- 完整的 `releases.json` 示例
- 自动化打包并发布更新的 CI/CD 脚本（如 GitHub Actions）
- 本地测试用的 `MockUpdater` 模拟环境

欢迎继续提问！