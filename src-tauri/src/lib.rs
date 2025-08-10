// 了解有关 Tauri 命令的更多信息，请访问 https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


// 导入我们需要使用的模块
use std::sync::Mutex;
use tauri::async_runtime::spawn;
use tauri::{AppHandle, Manager, State};
use tokio::time::{sleep, Duration};

// 创建一个结构，用于跟踪前端任务完成情况
// 设置相关任务
struct SetupState {
    frontend_task: bool,
    backend_task: bool,
}

// 在 v2 移动兼容应用中我们的主要入口点
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 不要在 Tauri 启动之前写代码，而是写在 setup 钩子中
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // 注册一个由 Tauri 管理的 `State`
        // 我们需要对它拥有写访问权限，因此我们将其包裹在 `Mutex` 中
        .manage(Mutex::new(SetupState {
            frontend_task: false,
            backend_task: false,
        }))
        // 添加我们用于检查的命令
        .invoke_handler(tauri::generate_handler![greet, set_complete])
        // 使用 setup 钩子来执行设置相关任务
        // 在主循环之前运行，因此尚未创建窗口
        .setup(|app| {
            // Spawn 操作设置为一个非阻塞任务，以便在它执行的同时可以创建并运行窗口。
            spawn(setup(app.handle().clone()));
            // 钩子期望返回一个 Ok 的结果
            Ok(())
        })
        // 启动应用
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 一个用于设置初始化任务状态的自定义任务
#[tauri::command]
async fn set_complete(
    app: AppHandle,
    state: State<'_, Mutex<SetupState>>,
    task: String,
) -> Result<(), ()> {
    // 以只读方式锁定 `State`
    let mut state_lock = state.lock().unwrap();
    match task.as_str() {
        "frontend" => state_lock.frontend_task = true,
        "backend" => state_lock.backend_task = true,
        _ => panic!("invalid task completed!"),
    }
    // 检查两个任务是否都已完成
    if state_lock.backend_task && state_lock.frontend_task {
        // 设置都已完成，我们可以关闭启动画面并且显示 main 窗口了
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        splash_window.close().unwrap();
        main_window.show().unwrap();
    }
    Ok(())
}

// 一个异步函数，用于执行一些耗时的设置任务
async fn setup(app: AppHandle) -> Result<(), ()> {
    // 模拟执行一些耗时的设置任务，3秒后完成
    println!("Performing really heavy backend setup task...");
    sleep(Duration::from_secs(3)).await;
    println!("Backend setup task completed!");
    // 设置后端任务为已完成
    // 可以像普通函数一样运行命令，但需要自己处理输入参数
    set_complete(
        app.clone(),
        app.state::<Mutex<SetupState>>(),
        "backend".to_string(),
    )
        .await?;
    Ok(())
}