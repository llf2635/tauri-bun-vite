# Tauri 应用 src-tauri/src 目录结构推荐

在 Tauri 应用中，`src-tauri/src` 目录是 Rust 后端代码的主要存放位置。以下是经过实践验证的推荐目录结构和详细说明：

## 推荐目录结构

```
src-tauri/
├── src/
│   ├── main.rs                  # 主入口文件
│   ├── lib.rs                   # 库入口文件
│   ├── commands/                # Tauri 命令模块
│   │   ├── mod.rs               # 命令模块导出
│   │   ├── file_operations.rs   # 文件操作命令
│   │   ├── system_info.rs       # 系统信息命令
│   │   └── user_preferences.rs  # 用户偏好设置命令
│   ├── core/                    # 核心业务逻辑
│   │   ├── mod.rs
│   │   ├── config/              # 配置管理
│   │   │   ├── mod.rs
│   │   │   ├── config_manager.rs
│   │   │   └── settings.rs
│   │   ├── database/            # 数据库操作
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs
│   │   │   ├── migrations.rs
│   │   │   └── models.rs
│   │   ├── auth/                # 认证授权
│   │   │   ├── mod.rs
│   │   │   ├── jwt.rs
│   │   │   └── session.rs
│   │   └── services/            # 业务服务
│   │       ├── mod.rs
│   │       ├── user_service.rs
│   │       ├── data_service.rs
│   │       └── notification_service.rs
│   ├── utils/                   # 工具函数
│   │   ├── mod.rs
│   │   ├── logger.rs
│   │   ├── file_utils.rs
│   │   ├── crypto_utils.rs
│   │   └── system_utils.rs
│   ├── models/                  # 数据模型
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── file.rs
│   │   └── preference.rs
│   ├── events/                  # 事件系统
│   │   ├── mod.rs
│   │   ├── emitter.rs
│   │   └── types.rs
│   ├── plugins/                 # 自定义插件
│   │   ├── mod.rs
│   │   ├── custom_filesystem.rs
│   │   └── custom_dialog.rs
│   ├── windows/                 # 窗口管理
│   │   ├── mod.rs
│   │   ├── window_manager.rs
│   │   └── window_types.rs
│   ├── updater/                 # 应用更新
│   │   ├── mod.rs
│   │   └── updater.rs
│   └── state/                   # 全局状态管理
│       ├── mod.rs
│       └── app_state.rs
├── build.rs                     # 构建脚本
├── Cargo.toml                   # 依赖管理
└── tauri.conf.json              # Tauri 配置
```

## 详细说明

### 1. `main.rs` - 主入口文件

这是应用的启动入口，负责初始化 Tauri 应用和注册各种组件。

```rust
// src-tauri/src/main.rs
use tauri::{Manager, WindowBuilder, WindowUrl};

mod commands;
mod core;
mod utils;
mod state;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 初始化全局状态
            app.manage(state::AppState::default());
            
            // 创建主窗口
            let main_window = WindowBuilder::new(
                app,
                "main",
                WindowUrl::App("index.html".into()),
            )
            .build()?;
            
            // 初始化核心服务
            core::init(app)?;
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::file_operations::read_file,
            commands::file_operations::write_file,
            commands::system_info::get_system_info,
            commands::user_preferences::get_preference,
            commands::user_preferences::set_preference,
        ])
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. `commands/` 目录 - Tauri 命令

存放所有通过 `@tauri-apps/api/tauri.invoke()` 调用的命令。

**文件结构**：
- 按功能模块划分文件
- 每个文件包含相关的命令函数
- 使用 `#[tauri::command]` 宏标记

```rust
// src-tauri/src/commands/file_operations.rs
use tauri::{AppHandle, State};
use std::path::Path;

#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    if !Path::new(&path).exists() {
        return Err("File not found".to_string());
    }
    
    match std::fs::read_to_string(&path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn write_file(path: String, content: String) -> Result<(), String> {
    match std::fs::write(&path, content) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
```

### 3. `core/` 目录 - 核心业务逻辑

存放应用的核心业务逻辑，与 Tauri 特定功能解耦。

**子目录说明**：

#### `core/config/` - 配置管理
```rust
// 配置管理器，处理应用配置的读写
pub struct ConfigManager {
    config_path: String,
    settings: Settings,
}

pub struct Settings {
    theme: String,
    language: String,
    auto_update: bool,
}
```

#### `core/database/` - 数据库操作
```rust
// 数据库连接和操作
pub struct DatabaseConnection {
    pool: sqlx::SqlitePool,
}

pub struct User {
    id: i64,
    name: String,
    email: String,
}
```

#### `core/services/` - 业务服务
```rust
// 业务逻辑服务
pub struct UserService {
    db: DatabaseConnection,
}

pub struct DataService {
    cache: HashMap<String, Vec<u8>>,
}
```

### 4. `utils/` 目录 - 工具函数

存放通用的工具函数，避免重复代码。

```rust
// src-tauri/src/utils/file_utils.rs
pub fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> Result<(), std::io::Error> {
    if !path.as_ref().exists() {
        std::fs::create_dir_all(path)
    } else {
        Ok(())
    }
}

pub fn get_file_size<P: AsRef<Path>>(path: P) -> Result<u64, std::io::Error> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len())
}
```

### 5. `models/` 目录 - 数据模型

定义应用中使用的数据结构。

```rust
// src-tauri/src/models/user.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: u32,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub preferences: UserPreferences,
}
```

### 6. `events/` 目录 - 事件系统

处理应用内部事件和前后端通信。

```rust
// src-tauri/src/events/types.rs
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum AppEvent {
    UserLoggedIn { user_id: u32 },
    FileSaved { path: String },
    ErrorOccurred { message: String },
    ProgressUpdate { progress: f32, message: String },
}

// src-tauri/src/events/emitter.rs
pub struct EventManager {
    app_handle: AppHandle,
}

impl EventManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }
    
    pub fn emit(&self, event: AppEvent) -> Result<(), tauri::Error> {
        self.app_handle.emit_all("app-event", event)
    }
}
```

### 7. `plugins/` 目录 - 自定义插件

当需要扩展 Tauri 功能时使用。

```rust
// src-tauri/src/plugins/custom_filesystem.rs
use tauri::{plugin::{Builder, TauriPlugin}, Runtime};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("custom-filesystem")
        .invoke_handler(tauri::generate_handler![
            read_secure_file,
            write_secure_file,
        ])
        .build()
}
```

### 8. `windows/` 目录 - 窗口管理

管理多窗口应用的窗口操作。

```rust
// src-tauri/src/windows/window_manager.rs
pub struct WindowManager {
    windows: HashMap<String, Window>,
}

impl WindowManager {
    pub fn create_window(&mut self, label: &str, url: WindowUrl) -> Result<(), String> {
        // 创建新窗口的逻辑
    }
    
    pub fn close_window(&mut self, label: &str) -> Result<(), String> {
        // 关闭窗口的逻辑
    }
}
```

### 9. `state/` 目录 - 全局状态

管理应用的全局状态。

```rust
// src-tauri/src/state/app_state.rs
use std::sync::Mutex;
use tauri::State;

#[derive(Default, Clone)]
pub struct AppState {
    pub user: Mutex<Option<User>>,
    pub theme: Mutex<String>,
    pub is_initialized: Mutex<bool>,
}

impl AppState {
    pub fn set_user(&self, user: User) {
        *self.user.lock().unwrap() = Some(user);
    }
    
    pub fn get_user(&self) -> Option<User> {
        self.user.lock().unwrap().clone()
    }
}
```

## 最佳实践建议

### 1. 模块化设计
- 使用 `mod.rs` 文件组织模块
- 遵循单一职责原则
- 保持模块之间的低耦合

### 2. 错误处理
```rust
// 统一的错误类型
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Custom error: {0}")]
    Custom(String),
}
```

### 3. 异步处理
```rust
// 使用 async/await 处理耗时操作
#[tauri::command]
pub async fn process_large_file(path: String) -> Result<ProcessResult, String> {
    let content = tokio::fs::read_to_string(&path).await
        .map_err(|e| e.to_string())?;
    
    // 处理大文件
    let result = process_content(&content).await;
    
    Ok(result)
}
```

### 4. 测试组织
```rust
// 在相应模块中添加测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_file() {
        // 测试代码
    }
}
```

### 5. 文档注释
```rust
/// 读取文件内容
/// 
/// # Arguments
/// 
/// * `path` - 文件路径
/// 
/// # Returns
/// 
/// 返回文件内容的字符串，或错误信息
/// 
/// # Errors
/// 
/// 如果文件不存在或读取失败，返回错误
#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    // 实现
}
```

这个目录结构提供了良好的可维护性和扩展性，适合中大型 Tauri 应用。根据项目规模，可以适当调整目录深度和模块划分。