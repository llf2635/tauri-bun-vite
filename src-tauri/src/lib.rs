//! # Tauri 应用核心库
//!
//! 这是一个 Tauri 桌面应用的核心功能库，提供配置管理、数据存储和系统集成等功能。
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
//! ```rust
//! ```

// --- 模块声明 ---
pub mod commands;
pub mod core;
pub mod utils;
pub mod models;

use tauri::Manager;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

// 了解有关 Tauri 命令的更多信息，请访问 https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 配置系统托盘 https://tauri.app/zh-cn/learn/system-tray/
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let tray = TrayIconBuilder::new()
                // 创建托盘时，您可以使用应用程序图标作为托盘图标
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Tauri App")
                .menu(&menu)
                .show_menu_on_left_click(true)
                // 监听菜单事件
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                // 监听托盘事件
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        println!("left click pressed and released");
                        // 在这个例子中，当点击托盘图标时，将展示并聚焦于主窗口
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {
                        println!("unhandled event {event:?}");
                    }
                })
                .build(app)?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


/// 创建系统托盘菜单
pub fn create_system_tray()  {

}