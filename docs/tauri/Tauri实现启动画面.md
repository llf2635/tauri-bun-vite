# Tauri 应用启动流程实现方案

在 Tauri 应用中实现启动画面和异步初始化，可以通过创建一个专门的"启动窗口"来实现。以下是完整的解决方案：

## 1. 创建启动窗口配置

首先在 `tauri.conf.json` 中配置两个窗口：

```json
{
  "tauri": {
    "windows": [
      {
        "label": "splash",
        "title": "启动中...",
        "width": 400,
        "height": 300,
        "resizable": false,
        "decorations": false,
        "alwaysOnTop": true,
        "center": true,
        "visible": false
      },
      {
        "label": "main",
        "title": "主应用",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "center": true,
        "visible": false
      }
    ]
  }
}
```

## 2. 创建启动画面 HTML

创建 `src/splash.html` 文件：

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>启动中...</title>
  <style>
    body {
      margin: 0;
      padding: 0;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      height: 100vh;
      color: white;
    }
    
    .container {
      text-align: center;
      animation: fadeIn 1s ease-in;
    }
    
    .logo {
      width: 80px;
      height: 80px;
      margin-bottom: 20px;
      animation: spin 2s linear infinite;
    }
    
    .title {
      font-size: 24px;
      font-weight: 600;
      margin-bottom: 10px;
    }
    
    .subtitle {
      font-size: 16px;
      opacity: 0.8;
      margin-bottom: 30px;
    }
    
    .progress-container {
      width: 200px;
      height: 6px;
      background: rgba(255, 255, 255, 0.2);
      border-radius: 3px;
      overflow: hidden;
    }
    
    .progress-bar {
      height: 100%;
      width: 0%;
      background: white;
      border-radius: 3px;
      transition: width 0.3s ease;
    }
    
    .status {
      margin-top: 15px;
      font-size: 14px;
      opacity: 0.7;
    }
    
    @keyframes spin {
      0% { transform: rotate(0deg); }
      100% { transform: rotate(360deg); }
    }
    
    @keyframes fadeIn {
      from { opacity: 0; transform: translateY(20px); }
      to { opacity: 1; transform: translateY(0); }
    }
  </style>
</head>
<body>
  <div class="container">
    <div class="logo">⚡</div>
    <div class="title">应用名称</div>
    <div class="subtitle">正在初始化...</div>
    <div class="progress-container">
      <div class="progress-bar" id="progressBar"></div>
    </div>
    <div class="status" id="statusText">准备启动</div>
  </div>

  <script>
    // 接收来自 Rust 后端的进度更新
    window.addEventListener('DOMContentLoaded', () => {
      // 监听初始化进度事件
      window.__TAURI__.event.listen('init-progress', (event) => {
        const data = event.payload;
        document.getElementById('progressBar').style.width = data.progress + '%';
        document.getElementById('statusText').textContent = data.message;
      });
      
      // 监听初始化完成事件
      window.__TAURI__.event.listen('init-complete', () => {
        // 这里可以添加淡出动画
        document.body.style.opacity = '0';
        setTimeout(() => {
          // 通知后端关闭启动窗口
          window.__TAURI__.event.emit('close-splash');
        }, 300);
      });
    });
  </script>
</body>
</html>
```

## 3. 后端 Rust 代码实现

在 `src/main.rs` 中实现启动逻辑：

```rust
use tauri::{
    async_runtime, App, AppHandle, Manager, State, Window, WindowBuilder, WindowUrl,
};
use tauri_plugin_fs::FsExt;
use std::sync::Mutex;
use std::time::Duration;

// 初始化状态
struct InitState {
    progress: u8,
    message: String,
    is_complete: bool,
}

impl Default for InitState {
    fn default() -> Self {
        Self {
            progress: 0,
            message: "初始化开始".to_string(),
            is_complete: false,
        }
    }
}

// 更新初始化进度
async fn update_progress(
    app_handle: &AppHandle,
    progress: u8,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = serde_json::json!({
        "progress": progress,
        "message": message
    });
    
    app_handle.emit_all("init-progress", payload)?;
    Ok(())
}

// 模拟耗时的后端初始化任务
async fn backend_initialization(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 1. 初始化数据库
    update_progress(app_handle, 10, "初始化数据库...").await?;
    async_runtime::sleep(Duration::from_millis(500)).await;
    
    // 2. 加载配置文件
    update_progress(app_handle, 30, "加载配置文件...").await?;
    async_runtime::sleep(Duration::from_millis(300)).await;
    
    // 3. 建立网络连接
    update_progress(app_handle, 50, "建立网络连接...").await?;
    async_runtime::sleep(Duration::from_millis(400)).await;
    
    // 4. 加载缓存数据
    update_progress(app_handle, 70, "加载缓存数据...").await?;
    async_runtime::sleep(Duration::from_millis(300)).await;
    
    // 5. 准备服务
    update_progress(app_handle, 90, "准备服务...").await?;
    async_runtime::sleep(Duration::from_millis(200)).await;
    
    Ok(())
}

// 前端初始化完成的回调
#[tauri::command]
async fn frontend_ready(app_handle: AppHandle) -> Result<(), String> {
    println!("前端已准备就绪");
    
    // 发送前端准备完成的事件
    app_handle.emit_all("frontend-ready", true).map_err(|e| e.to_string())?;
    Ok(())
}

// 启动主应用
async fn start_main_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();
    
    // 显示启动窗口
    let splash_window = WindowBuilder::new(
        app,
        "splash",
        WindowUrl::App("splash.html".into()),
    )
    .build()?;
    
    // 确保启动窗口可见
    splash_window.show()?;
    splash_window.center()?;
    
    // 并行执行前后端初始化
    let app_handle_clone = app_handle.clone();
    let main_window_label = "main".to_string();
    
    async_runtime::spawn(async move {
        // 创建一个通道来协调前后端初始化
        let (tx, rx) = async_channel::unbounded::<bool>();
        
        // 启动后端初始化任务
        let backend_handle = async_runtime::spawn(async move {
            if let Err(e) = backend_initialization(&app_handle_clone).await {
                eprintln!("后端初始化失败: {}", e);
                return;
            }
            
            // 后端初始化完成
            let _ = tx.send(true).await;
        });
        
        // 等待前端准备就绪
        let frontend_ready = async_runtime::spawn(async move {
            // 这里可以监听前端事件或设置超时
            async_runtime::sleep(Duration::from_secs(10)).await; // 超时保护
            false
        });
        
        // 等待后端完成和前端准备
        tokio::select! {
            result = rx.recv() => {
                match result {
                    Ok(_) => {
                        // 后端初始化完成，检查前端状态
                        // 这里可以添加逻辑等待前端准备
                        async_runtime::sleep(Duration::from_millis(200)).await;
                        
                        // 发送初始化完成事件
                        if let Err(e) = app_handle_clone.emit_all("init-complete", ()) {
                            eprintln!("发送初始化完成事件失败: {}", e);
                        }
                        
                        // 关闭启动窗口并显示主窗口
                        if let Some(splash) = app_handle_clone.get_window("splash") {
                            let _ = splash.close();
                        }
                        
                        if let Some(main) = app_handle_clone.get_window(&main_window_label) {
                            let _ = main.show();
                            let _ = main.center();
                        }
                    }
                    Err(e) => eprintln!("接收后端初始化结果失败: {}", e),
                }
            }
            _ = frontend_ready => {
                eprintln!("前端初始化超时");
            }
        }
        
        // 确保后台任务完成
        let _ = backend_handle.await;
    });
    
    Ok(())
}

// 应用启动前的钩子
fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // 启动主应用流程
    async_runtime::spawn(async move {
        if let Err(e) = start_main_app(&mut *app).await {
            eprintln!("启动应用失败: {}", e);
        }
    });
    
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![frontend_ready])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## 4. 前端初始化代码

在主前端应用中添加初始化逻辑：

```javascript
// src/main.js 或主组件中
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

class AppInitializer {
  constructor() {
    this.initialized = false;
  }

  async initialize() {
    if (this.initialized) return;
    
    try {
      // 显示加载状态
      this.updateStatus('前端初始化中...');
      
      // 1. 初始化前端服务
      await this.initServices();
      
      // 2. 加载用户数据
      await this.loadUserData();
      
      // 3. 初始化UI组件
      await this.initUI();
      
      // 4. 通知后端前端已准备就绪
      await invoke('frontend_ready');
      
      this.initialized = true;
      console.log('前端初始化完成');
      
    } catch (error) {
      console.error('前端初始化失败:', error);
      // 可以显示错误信息或重试
    }
  }

  async initServices() {
    // 模拟耗时的前端初始化
    return new Promise((resolve) => {
      setTimeout(() => {
        this.updateStatus('初始化服务...');
        resolve();
      }, 300);
    });
  }

  async loadUserData() {
    // 模拟加载用户数据
    return new Promise((resolve) => {
      setTimeout(() => {
        this.updateStatus('加载用户数据...');
        resolve();
      }, 500);
    });
  }

  async initUI() {
    // 模拟UI初始化
    return new Promise((resolve) => {
      setTimeout(() => {
        this.updateStatus('初始化界面...');
        resolve();
      }, 400);
    });
  }

  updateStatus(message) {
    console.log('状态:', message);
    // 可以更新页面上的状态显示
  }
}

// 应用启动时执行
document.addEventListener('DOMContentLoaded', async () => {
  const initializer = new AppInitializer();
  
  // 开始前端初始化
  await initializer.initialize();
});
```

## 5. 优化版本（带超时和错误处理）

```rust
async fn start_main_app_with_timeout(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let app_handle = app.handle();
    
    // 显示启动窗口
    let splash_window = WindowBuilder::new(
        app,
        "splash",
        WindowUrl::App("splash.html".into()),
    )
    .build()?;
    
    splash_window.show()?;
    splash_window.center()?;
    
    let app_handle_clone = app_handle.clone();
    
    async_runtime::spawn(async move {
        let timeout_duration = Duration::from_secs(30); // 30秒超时
        
        let init_future = async {
            // 并行执行前后端初始化
            let backend_result = backend_initialization(&app_handle_clone).await;
            let frontend_ready = wait_for_frontend_ready(&app_handle_clone).await;
            
            (backend_result, frontend_ready)
        };
        
        match tokio::time::timeout(timeout_duration, init_future).await {
            Ok((Ok(_), _)) => {
                // 成功完成初始化
                async_runtime::sleep(Duration::from_millis(150)).await; // 短暂延迟确保动画完成
                
                app_handle_clone.emit_all("init-complete", ()).ok();
                
                // 关闭启动窗口，显示主窗口
                if let Some(splash) = app_handle_clone.get_window("splash") {
                    let _ = splash.close();
                }
                
                if let Some(main) = app_handle_clone.get_window("main") {
                    let _ = main.show();
                    let _ = main.center();
                }
            }
            Ok((Err(e), _)) => {
                eprintln!("后端初始化失败: {}", e);
                show_error_message(&app_handle_clone, "后端初始化失败").await;
            }
            Err(_) => {
                eprintln!("初始化超时");
                show_error_message(&app_handle_clone, "初始化超时，请重试").await;
            }
        }
    });
    
    Ok(())
}

async fn wait_for_frontend_ready(app_handle: &AppHandle) -> bool {
    // 监听前端准备就绪事件
    let mut rx = match app_handle.listen("frontend-ready") {
        Ok(rx) => rx,
        Err(_) => return false,
    };
    
    // 设置超时
    let timeout = tokio::time::sleep(Duration::from_secs(15));
    tokio::pin!(timeout);
    
    loop {
        tokio::select! {
            _ = &mut timeout => {
                return false; // 超时
            }
            event = rx.recv() => {
                match event {
                    Ok(_) => return true,
                    Err(_) => return false,
                }
            }
        }
    }
}
```

## 6. 使用建议

### 优点：
- **用户体验好**：用户知道应用正在启动，不会觉得卡顿
- **并行处理**：前后端可以同时进行初始化
- **进度可视化**：可以显示详细的初始化进度
- **错误处理**：可以优雅地处理初始化失败的情况

### 注意事项：
1. **不要阻塞主线程**：所有耗时操作都应该在异步任务中执行
2. **设置超时**：避免无限等待前端准备
3. **错误处理**：提供友好的错误提示
4. **资源管理**：确保启动窗口正确关闭
5. **跨平台兼容**：测试在不同平台上的表现

这个方案提供了一个完整的启动流程，可以根据具体需求调整初始化任务和UI设计。