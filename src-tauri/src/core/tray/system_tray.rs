// //! # Tauri 应用核心库
// //!
// //! 这是一个 Tauri 桌面应用的核心功能库，提供配置管理、数据存储和系统集成等功能。
// //! 在 Tauri 2.x 中，#[cfg(desktop)] 是一个 条件编译属性（conditional compilation attribute），它用于根据目标平台是否为“桌面平台”来决定是否编译某段代码。
// //!
// //! ## 功能特性
// //!
// //! - 配置文件管理
// //! - 用户偏好设置
// //! - 系统信息获取
// //! - 文件操作工具
// //!
// //! ## 使用示例
// //!
// 
// use std::sync::Mutex;
// use tauri::{App, AppHandle, Emitter, Manager, State, Wry};
// #[cfg(desktop)]
// use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
// use tokio::time::{sleep, Duration};
// use crate::core::tray::tray_menu::create_tray_menu;
// 
// // 用于设置设置任务状态的自定义任务
// #[tauri::command]
// async fn set_complete(
//     app: AppHandle,
//     state: State<'_, Mutex<SetupState>>,
//     task: String,
// ) -> Result<(), ()> {
//     // 在没有写入权限的情况下锁定状态
//     let mut state_lock = state.lock().unwrap();
//     match task.as_str() {
//         "frontend" => state_lock.frontend_task = true,
//         "backend" => state_lock.backend_task = true,
//         _ => panic!("已完成无效任务!"),
//     }
//     println!("设置任务 {} 已完成!", task);
//     // 检查两项任务是否已完成
//     if state_lock.backend_task && state_lock.frontend_task {
//         println!("所有设置任务已完成!");
//         // 设置完成，我们可以关闭启动画面并取消隐藏主窗口！
//         let splash_window = app.get_webview_window("splashscreen").unwrap();
//         let main_window = app.get_webview_window("main").unwrap();
//         // 当所有初始化任务都完成时，关闭启动画面并显示主窗口
//         splash_window.close().unwrap();
//         main_window.show().unwrap();
//     }
//     Ok(())
// }
// 
// // 执行一些繁重设置任务的异步函数
// async fn setup(app: AppHandle) -> Result<(), ()> {
//     // 假动作 3 秒
//     println!("执行非常繁重的后端设置任务...");
//     sleep(Duration::from_secs(3)).await;
//     println!("后端设置任务已完成!");
//     // 将后端任务设置为已完成
//     // 命令可以作为常规函数运行，只要您自己处理输入参数
//     set_complete(
//         app.clone(),
//         app.state::<Mutex<SetupState>>(),
//         "backend".to_string(),
//     )
//         .await?;
//     Ok(())
// }
// 
// // 创建一个结构，我们将用于跟踪设置相关任务的完成情况
// struct SetupState {
//     frontend_task: bool,
//     backend_task: bool,
// }
// 
// /// 创建系统托盘
// /// 想要创建一个系统托盘，请阅读 https://v2.tauri.org.cn/learn/system-tray/
// #[cfg(desktop)]
// pub fn create_system_tray(app: &mut App) {
//     // 创建托盘菜单，调用下面的方法
//     let menu = create_tray_menu(app);
// 
//     let _tray = TrayIconBuilder::with_id("tray")
//         .icon(app.default_window_icon().unwrap().clone()) // 默认的图片
//         // .icon(Image::from_bytes(include_bytes!("../icons/light@2x.png")).expect("REASON")) // 自定义的图片，需要给 tauri 添加 image-png 特性
//         // tooltip 为此托盘图标设置工具提示。但 linux 不支持使用此功能。
//         .tooltip("Tauri App")
//         .menu(&menu)
//         // 监听菜单事件，每一个配置的前缀为上面的 MenuItem 中的 id
//         .on_menu_event(|app, event| match event.id.as_ref() {
//             "open" => {
//                 // 打开事件
//                 println!("open menu item was clicked");
//                 // handle_open_coco(app);
//                 let window = app.get_webview_window("main").unwrap();
//                 let _ = window.show();
//             }
//             "hide" => {
//                 // 隐藏事件
//                 println!("hide menu item was clicked");
//                 // handle_hide_coco(app);
//                 let window = app.get_webview_window("main").unwrap();
//                 let _ = window.hide();
//             }
//             "about" => {
//                 // 将同步事件发出到所有 Web 视图
//                 let _ = app.emit("open_settings", "about");
//             }
//             "settings" => {
//                 // Windows 无法打开第二个窗口，问题: https://github.com/tauri-apps/tauri/issues/11144 https://github.com/tauri-apps/tauri/issues/8196
//                 //#[cfg(windows)]
//                 let _ = app.emit("open_settings", "");
// 
//                 // #[cfg(not(windows))]
//                 // open_settings(&app);
//             }
//             "quit" => {
//                 println!("quit menu item was clicked");
//                 app.exit(0);
//             }
//             _ => {
//                 println!("menu item {:?} not handled", event.id);
//             }
//         })
//         // 监听托盘事件
//         .on_tray_icon_event(|tray, event| match event {
//             TrayIconEvent::Click {
//                 button: MouseButton::Left,
//                 button_state: MouseButtonState::Up,
//                 ..
//             } => {
//                 println!("left click pressed and released");
//                 // 在这个例子中，当点击托盘图标时，将展示并聚焦于主窗口
//                 let app = tray.app_handle();
//                 if let Some(window) = app.get_webview_window("main") {
//                     let _ = window.unminimize();
//                     let _ = window.show();
//                     let _ = window.set_focus();
//                 }
//             }
//             _ => {
//                 println!("unhandled event {event:?}");
//             }
//         })
//         .build(app)
//         .unwrap();
// }
// 
// 
// 
// // fn handle_open_coco(app: &AppHandle) {
// //     println!("open menu item was clicked");
// //     if let Some(window) = app.get_webview_window("main") {
// //         let _ = window.unminimize();
// //         let _ = window.show();
// //         let _ = window.set_focus();
// //     }
// // }
// //
// // fn handle_hide_coco(app: &AppHandle) {
// //     println!("hide menu item was clicked");
// //     if let Some(window) = app.get_webview_window("main") {
// //         let _ = window.hide();
// //     }
// // }
