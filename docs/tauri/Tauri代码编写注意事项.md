在 Tauri 2 中处理跨平台功能差异，需要使用条件编译来区分桌面端和移动端代码。以下是几种有效的管理方式：

## 1. 使用条件编译属性

### Rust 代码中的条件编译

```rust
// src/core/tray/system_tray.rs
use tauri::AppHandle;

#[cfg(desktop)]
pub fn create_system_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    use tauri_plugin_system_tray::{SystemTray, SystemTrayEvent};
    
    let tray = SystemTray::new()
        .with_id("main-tray")
        .with_tooltip("我的应用");
    
    app.handle().plugin(
        tauri_plugin_system_tray::Builder::new()
            .with_tray(tray)
            .on_event(|app, event| match event {
                SystemTrayEvent::LeftClick { .. } => {
                    // 处理左键点击
                }
                SystemTrayEvent::RightClick { .. } => {
                    // 处理右键点击
                }
                _ => {}
            })
            .build()
    )?;
    
    Ok(())
}

// 移动端提供空实现
#[cfg(mobile)]
pub fn create_system_tray(_app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 移动端无需实现系统托盘
    println!("系统托盘功能在移动端不可用");
    Ok(())
}
```


### 在主程序中调用

```rust
// src/lib.rs
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
use tauri::async_runtime::spawn;
use tokio::time::{sleep, Duration};

// 条件导入模块
#[cfg(desktop)]
use crate::core::tray::create_system_tray;

// ... 其他代码 ...

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(SetupState {
            frontend_task: false,
            backend_task: false,
        }))
        .invoke_handler(tauri::generate_handler![greet, set_complete])
        .setup(|app| {
            spawn(setup(app.handle().clone()));

            // 仅在桌面端创建系统托盘
            #[cfg(desktop)]
            {
                if let Err(e) = create_system_tray(app.handle()) {
                    eprintln!("创建系统托盘失败: {}", e);
                }
            }

            // 仅在桌面端添加单实例插件
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
                .expect("单实例插件初始化失败");

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用程序时出错");
}
```


## 2. 创建平台抽象层

### 定义平台无关的 trait

```rust
// src/core/platform/mod.rs
pub mod tray;
```
```rust
// src/core/platform/tray.rs
use tauri::AppHandle;

pub trait TrayManager {
    fn create_tray(&self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>>;
    fn destroy_tray(&self) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(desktop)]
pub struct DesktopTrayManager;

#[cfg(desktop)]
impl TrayManager for DesktopTrayManager {
    fn create_tray(&self, app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        // 桌面端实现
        println!("创建桌面系统托盘");
        Ok(())
    }
    
    fn destroy_tray(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 桌面端销毁实现
        println!("销毁桌面系统托盘");
        Ok(())
    }
}

#[cfg(mobile)]
pub struct MobileTrayManager;

#[cfg(mobile)]
impl TrayManager for MobileTrayManager {
    fn create_tray(&self, _app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        // 移动端无操作
        println!("移动端不支持系统托盘");
        Ok(())
    }
    
    fn destroy_tray(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 移动端无操作
        Ok(())
    }
}
```


## 3. 使用功能标志（Features）

在 [Cargo.toml](file:///home/lcqh/项目/Tauri/tauri-bun-vite/src-tauri/Cargo.toml) 中定义功能：

```toml
[features]
default = ["desktop"]
desktop = ["tauri-plugin-system-tray"]
mobile = []

[dependencies]
tauri = { version = "2.0", features = [] }
tauri-plugin-system-tray = { version = "2.0", optional = true }
```


然后在代码中使用功能标志：

```rust
// src/core/tray/system_tray.rs
use tauri::AppHandle;

#[cfg(feature = "desktop")]
pub fn create_system_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 仅在启用 desktop 功能时编译
    println!("创建系统托盘（桌面端）");
    Ok(())
}

#[cfg(not(feature = "desktop"))]
pub fn create_system_tray(_app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 其他平台的空实现
    println!("当前平台不支持系统托盘");
    Ok(())
}
```


## 4. 前端代码的平台适配

### 在前端使用条件判断

```typescript
// src/utils/platform.ts
export const isDesktop = () => {
  // @ts-ignore
  return window.__TAURI_INTERNALS__?.metadata?.target !== undefined && 
         // @ts-ignore
         !window.__TAURI_INTERNALS__?.metadata?.target?.includes('android') &&
         // @ts-ignore
         !window.__TAURI_INTERNALS__?.metadata?.target?.includes('ios');
};

export const isMobile = () => {
  // @ts-ignore
  return window.__TAURI_INTERNALS__?.metadata?.target !== undefined && 
         (// @ts-ignore
          window.__TAURI_INTERNALS__?.metadata?.target?.includes('android') ||
          // @ts-ignore
          window.__TAURI_INTERNALS__?.metadata?.target?.includes('ios'));
};
```


### 条件调用 Tauri 命令

```typescript
// src/composables/useSystemTray.ts
import { invoke } from '@tauri-apps/api/core';
import { isDesktop } from '@/utils/platform';

export const useSystemTray = () => {
  const updateTrayMenu = async (menuItems: any[]) => {
    if (isDesktop()) {
      try {
        await invoke('update_tray_menu', { menuItems });
      } catch (error) {
        console.error('更新系统托盘菜单失败:', error);
      }
    } else {
      console.log('当前平台不支持系统托盘');
    }
  };

  return {
    updateTrayMenu
  };
};
```


## 5. 统一的平台管理模块

创建一个统一的平台管理模块：

```rust
// src/core/platform/mod.rs
pub mod tray;
pub mod window;
pub mod notification;

use tauri::AppHandle;

pub struct PlatformManager;

impl PlatformManager {
    #[cfg(desktop)]
    pub fn initialize(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        crate::core::tray::create_system_tray(app)?;
        println!("桌面平台初始化完成");
        Ok(())
    }
    
    #[cfg(mobile)]
    pub fn initialize(_app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        println!("移动平台初始化完成");
        Ok(())
    }
}
```


在主程序中使用：

```rust
// src/lib.rs
use crate::core::platform::PlatformManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // ... 其他配置 ...
        .setup(|app| {
            spawn(setup(app.handle().clone()));

            // 统一平台初始化
            if let Err(e) = PlatformManager::initialize(app.handle()) {
                eprintln!("平台初始化失败: {}", e);
            }

            Ok(())
        })
        // ... 其他配置 ...
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用程序时出错");
}
```


## 总结

推荐的最佳实践：

1. **优先使用 `#[cfg(desktop)]` 和 `#[cfg(mobile)]`** 进行条件编译
2. **为移动端提供空实现** 或使用 `unimplemented!()` 宏
3. **在前端代码中进行平台检测**，避免调用不支持的 API
4. **创建统一的平台抽象层**，简化平台差异处理
5. **合理组织模块结构**，将平台相关代码分离到独立模块

这样可以确保代码在不同平台间正确编译和运行，避免因平台差异导致的编译错误或运行时错误。