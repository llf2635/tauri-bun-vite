这是一个非常关键的问题！在使用 **Tauri 2** 开发跨平台应用（尤其是未来可能支持移动端）时，像 **系统托盘（tray icon）、多窗口、文件对话框、全局快捷键** 等功能，**仅在桌面平台（Windows/macOS/Linux）有效**，在移动端（Android/iOS）上要么不支持，要么调用会崩溃或静默失败。

为了确保你的应用在所有平台上都能稳定运行，你需要 **合理地隔离和管理这些“仅桌面可用”的功能**。以下是最佳实践和推荐方式：

---

## ✅ 核心原则

> **“编译时排除”比“运行时报错”更安全、更高效。**

你应该使用 Rust 的 **条件编译（`#[cfg(desktop)]`）** 来确保移动端根本不会编译这些不支持的功能代码。

---

## 🧱 一、使用 `#[cfg(desktop)]` 隔离桌面专属功能

### ✅ 示例 1：系统托盘（Tray Icon）

```rust
// main.rs 或 lib.rs

#[cfg(desktop)]
fn create_tray(app: &tauri::AppHandle) {
    use tauri::SystemTray;
    use tauri::SystemTrayMenu;
    use tauri::SystemTrayMenuItem;

    let tray_menu = SystemTrayMenu::new()
        .add_item(SystemTrayMenuItem::Action("Open".to_string(), "open"))
        .add_item(SystemTrayMenuItem::Separator)
        .add_item(SystemTrayMenuItem::Action("Quit".to_string(), "quit"));

    let tray = SystemTray::new().with_menu(tray_menu);

    app.plugin(tauri_plugin_system_tray::init(tray))
        .expect("Failed to initialize system tray plugin");
}

#[cfg(not(desktop))]
fn create_tray(_app: &tauri::AppHandle) {
    // 移动端空实现，或打印日志
    println!("System tray not supported on mobile");
}
```

然后在 `main` 中调用：

```rust
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            create_tray(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

> ✅ 这样移动端不会编译 `system-tray` 相关代码，避免依赖和运行时错误。

---

### ✅ 示例 2：Tauri Command 命令（防止移动端调用）

```rust
#[cfg(desktop)]
#[tauri::command]
async fn show_tray_notification(title: String, body: String) {
    use tauri::api::notification::Notification;
    Notification::new("your-app-id")
        .title(&title)
        .body(&body)
        .show()
        .unwrap();
}

#[cfg(not(desktop))]
#[tauri::command]
async fn show_tray_notification(_title: String, _body: String) {
    // 移动端使用 Android Notification 或前端 Toast
    println!("Native tray notification not available on mobile");
}
```

前端调用时无需关心平台差异：

```js
await invoke('show_tray_notification', { title: 'Hello', body: 'World' });
```

> ✅ 自动调用对应平台的实现。

---

## 📦 二、条件编译依赖（Cargo.toml）

有些 crate **只应在桌面平台引入**，例如：

- `tauri-plugin-system-tray`
- `rfd`（文件对话框）
- `tauri-plugin-global-shortcut`

### 使用 `cfg` 控制依赖（可选）

你可以在 `Cargo.toml` 中使用 **feature + cfg** 来更精细控制：

```toml
[features]
default = ["desktop"]
desktop = []

[target.'cfg(desktop)'.dependencies]
tauri-plugin-system-tray = { version = "2", optional = true }
```

然后在代码中：

```rust
#[cfg(feature = "desktop")]
use tauri_plugin_system_tray;
```

但更简单的方式是直接使用 `#[cfg(desktop)]`，因为 Tauri 已自动定义 `desktop`。

---

## 🧩 三、封装平台抽象层（Platform Abstraction）

为了更好的可维护性，建议将平台差异封装成统一接口。

### 示例：通知系统抽象

```rust
// platform/mod.rs

pub trait Notifier {
    fn send(&self, title: &str, body: &str);
}

#[cfg(desktop)]
pub struct DesktopNotifier;

#[cfg(desktop)]
impl Notifier for DesktopNotifier {
    fn send(&self, title: &str, body: &str) {
        tauri::api::notification::Notification::new("app-id")
            .title(title)
            .body(body)
            .show()
            .ok();
    }
}

#[cfg(not(desktop))]
pub struct MobileNotifier;

#[cfg(not(desktop))]
impl Notifier for MobileNotifier {
    fn send(&self, title: &str, body: &str) {
        // 可通过事件发送到前端，由前端显示 Toast
        println!("Notify (mobile): {} - {}", title, body);
        // 或使用 tauri::EventTarget 发送事件
    }
}

pub fn get_notifier() -> impl Notifier {
    #[cfg(desktop)]
    return DesktopNotifier;
    #[cfg(not(desktop))]
    return MobileNotifier;
}
```

使用：

```rust
let notifier = get_notifier();
notifier.send("提醒", "任务完成！");
```

---

## 🚫 四、避免在移动端调用桌面 API 的错误方式

### ❌ 错误：不加保护直接调用

```rust
// 危险！移动端可能 panic 或链接失败
#[tauri::command]
async fn create_tray_icon() {
    let tray = SystemTray::new(); // ❌ 移动端没有 SystemTray 类型
    // ...
}
```

### ❌ 错误：运行时判断平台（不彻底）

```rust
#[tauri::command]
async fn create_tray() {
    if cfg!(not(target_os = "android")) && cfg!(not(target_os = "ios")) {
        // 仍然会编译 SystemTray 相关代码，可能导致链接失败
    }
}
```

> ⚠️ `cfg!()` 是运行时求值，但类型系统仍需编译通过。如果 `SystemTray` 在移动端不存在，**编译都无法通过**。

---

## ✅ 五、完整项目结构建议

```
src/
├── main.rs
├── tray.rs          # #[cfg(desktop)] mod
├── commands.rs      # 命令函数，按平台分隔
├── platform/        # 平台抽象层
│   ├── mod.rs
│   ├── desktop.rs
│   └── mobile.rs
└── mobile_ui/       # 移动端专用 UI 逻辑（可选）
```

在 `tray.rs` 中：

```rust
#[cfg(desktop)]
pub fn init_tray(app: &tauri::AppHandle) { ... }
#[cfg(not(desktop))]
pub fn init_tray(_app: &tauri::AppHandle) { }
```

---

## ✅ 六、前端如何避免调用无效功能？

你可以通过 Tauri 提供的 **平台检测 API** 来控制 UI 显示：

```js
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';

// 获取平台信息
const isMobile = !await invoke('is_desktop'); // 自定义命令

// 自定义命令：判断是否为桌面
#[tauri::command]
fn is_desktop() -> bool {
    cfg!(desktop)
}
```

前端：

```js
if (await isDesktop()) {
  document.getElementById('tray-btn').style.display = 'block';
}
```

---

## ✅ 七、总结：最佳实践清单

| 做法 | 说明 |
|------|------|
| ✅ 使用 `#[cfg(desktop)]` | 编译时排除不支持的代码 |
| ✅ 为移动端提供空实现 | 避免接口断裂 |
| ✅ 封装平台抽象层 | 提高可维护性 |
| ✅ 条件导出 Tauri 命令 | 防止前端误调 |
| ✅ 前端检测平台 | 隐藏不支持的功能按钮 |
| ❌ 避免运行时才判断类型 | 编译都过不了 |

---

如果你希望，我可以为你提供一个 **完整的模板项目结构**，包含：
- 桌面托盘 + 移动端 fallback
- 跨平台命令
- 前端平台检测
- Cargo.toml 配置

欢迎继续提问！