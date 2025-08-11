//! # Tauri 应用核心库
//!
//! 这是一个 Tauri 桌面应用的核心功能库，提供配置管理、数据存储和系统集成等功能。
//! 在 Tauri 2.x 中，#[cfg(desktop)] 是一个 条件编译属性（conditional compilation attribute），它用于根据目标平台是否为“桌面平台”来决定是否编译某段代码。
//!
//! ## 功能特性
//!
//! - 配置文件管理
//! - 用户偏好设置
//! - 系统信息获取
//! - 文件操作工具
//!
//! ## 使用示例
//!

// --- 模块声明 ---
pub mod commands;
pub mod core;
pub mod models;
pub mod utils;

#[cfg(desktop)]
use crate::core::tray::create_system_tray;
use std::sync::Mutex;
use tauri::async_runtime::spawn;
use tauri::{AppHandle, Manager, State};
use tokio::time::{sleep, Duration};

// 了解有关 Tauri 命令的更多信息，请访问 https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! 你已经收到了 Rust 的问候!", name)
}

// 用于设置设置任务状态的自定义任务
#[tauri::command]
async fn set_complete(
    app: AppHandle,
    state: State<'_, Mutex<SetupState>>,
    task: String,
) -> Result<(), ()> {
    // 在没有写入权限的情况下锁定状态
    let mut state_lock = state.lock().unwrap();
    match task.as_str() {
        "frontend" => state_lock.frontend_task = true,
        "backend" => state_lock.backend_task = true,
        _ => panic!("已完成无效任务!"),
    }
    println!("设置任务 {} 已完成!", task);
    // 检查两项任务是否已完成
    if state_lock.backend_task && state_lock.frontend_task {
        println!("所有设置任务已完成!");
        // 设置完成，我们可以关闭启动画面并取消隐藏主窗口！

        // 使用桌面端条件编译
        #[cfg(desktop)]
        {
            // 获取窗口并处理可能的错误
            let splash_window = app
                .get_webview_window("splashscreen")
                .ok_or("无法获取启动画面窗口".to_string())
                .unwrap();
            let main_window = app
                .get_webview_window("main")
                .ok_or("无法获取主窗口".to_string())
                .unwrap();

            // 关闭启动画面并显示主窗口
            let _ = splash_window.close();
            let _ = main_window.show();
            let _ = main_window.set_focus();
        }
    }
    Ok(())
}

// 执行一些繁重设置任务的异步函数
async fn setup(app: AppHandle) -> Result<(), ()> {
    // 假动作 3 秒
    println!("执行非常繁重的后端设置任务...");
    sleep(Duration::from_secs(3)).await;
    println!("后端设置任务已完成!");
    // 将后端任务设置为已完成
    // 命令可以作为常规函数运行，只要您自己处理输入参数
    set_complete(
        app.clone(),
        app.state::<Mutex<SetupState>>(),
        "backend".to_string(),
    )
    .await?;
    Ok(())
}

// 创建一个结构，我们将用于跟踪设置相关任务的完成情况
struct SetupState {
    frontend_task: bool,
    backend_task: bool,
}

// 我们在版本 2 移动兼容应用程序中的主要入口点
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 不要在 Tauri 启动之前编写代码，而是将其编写在设置钩子中！
    tauri::Builder::default()
        // 注册一个由 Tauri 管理的 “状态”
        // 我们需要对它的写访问权限，因此我们将其包装在 “互斥体” 中
        .manage(Mutex::new(SetupState {
            frontend_task: false,
            backend_task: false,
        }))
        // 添加一个命令，我们可以使用它来检查
        .invoke_handler(tauri::generate_handler![greet, set_complete])
        // 使用设置挂钩执行与设置相关的任务
        // 在主循环之前运行，因此尚未创建任何窗口
        .setup(|app| {
            // 生成设置作为非阻塞任务，以便在执行时可以创建和运行窗口
            spawn(setup(app.handle().clone()));

            #[cfg(desktop)]
            create_system_tray(app);

            // 添加一个单实例插件，用于防止多个实例运行。使用单实例插件确保 Tauri 应用程序在同一时间只运行单个实例
            // 详情请查看 https://tauri.app/zh-cn/plugin/single-instance/
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
                .expect("TODO: panic message");

            // 添加更新插件，允许您检查更新并下载更新。使用更新服务器或静态 JSON 自动更新你的 Tauri 应用程序
            // 详情请查看 https://v2.tauri.org.cn/plugin/updater/
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_updater::Builder::new().build()).expect("TODO: panic message");
            // 钩子期望一个 Ok 结果
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        // 添加对话框插件，允许您使用 Tauri 创建对话框。本机系统对话框，用于打开和保存文件，以及消息对话框。
        // 详情请查看 https://tauri.app/zh-cn/plugin/dialog/
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用程序时出错");
}
