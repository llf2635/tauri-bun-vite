// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use redis::{Client, Commands};
use sea_orm::DatabaseConnection;
use tauri::{Manager, State};
use tokio::sync::Mutex;

use tauri_bun_vite::{init_system, init_system_tray, init_window_menu, window_menu_event_handler};
use tauri_bun_vite::xxl_job_user::Model;

/*
Tauri 是一个框架，用于为所有主要的桌面平台构建微小、极快的二进制文件。
开发人员可以集成任何编译为 HTML、JS 和 CSS 的前端框架来构建他们的用户界面。
应用程序的后端是一个源自 rust 的二进制文件，其中包含前端可以与之交互的 API。
*/
// tauri 配置参考： https://docs.rs/tauri/1.7.1/tauri/index.html
// tauri 也可以像 axum 一样配置： https://docs.rs/tauri/1.7.1/tauri/trait.Manager.html#method.manage
fn main() {
    // 初始化屏幕，也就是主窗口在准备好之前，屏幕上会显示一个加载的窗口。参考 https://tauri.app/zh-cn/v1/guides/features/splashscreen


    // 构建并启动 Tauri
    // default() 方法用于创建默认的 Tauri 构建器，将 Wry 设为 Builder 的默认运行时
    // Wry 是 Rust 中的跨平台 WebView 渲染库，支持所有主要的桌面平台，如 Windows、macOS 和 Linux。
    // app 是当前正在运行的应用程序的实例。此类型（ App ）实现（ Rust 中的实现类似于 Java 中的继承 ）了 Manager，它允许操作全局应用程序项。管理正在运行的应用程序。
    let app = tauri::Builder::default()
        // setup 方法允许你在应用启动之前执行一些初始化操作。这对于设置应用的状态、注册全局事件监听器、
        // 或者执行任何需要在应用启动前完成的任务非常有用。
        .setup(|app| {
            // 在 `setup` 方法中，你可以访问 `AppHandle` 对象，这是一个包含了应用状态和方法的结构体。
            // 你可以使用它来注册事件监听器、设置应用的环境变量、初始化数据库连接等。
            let handle = app.handle();

            // 异步初始化系统，包括日志、mysql、redis等
            // Tauri 的 async_runtime 使用 tokio Runtime 初始化代码  https://docs.rs/tauri/1.7.1/tauri/async_runtime/index.html
            tauri::async_runtime::spawn(async move {
                init_system().await.expect("TODO: panic message");
            });

            // 异步检查更新，需要启用 tauri 依赖包的 updater 特性功能
            // 参考文档： https://docs.rs/tauri/1.7.1/tauri/updater/struct.UpdateBuilder.html
            // tauri::async_runtime::spawn(async move {
            //     // 使用平台的目标三元组来检查更新，检查当前平台的更新
            //     let builder = tauri::updater::builder(handle).target(tauri::utils::platform::target_triple().unwrap());
            //     match builder.check().await {
            //         Ok(update) => {
            //             if update.is_update_available() {
            //                 update.download_and_install().await.unwrap();
            //             }
            //         }
            //         Err(e) => {
            //             println!("failed to get update: {}", e);
            //         }
            //     }
            // });

            // 测试系统弹窗，需要启用 tauri 依赖包的 dialog 特性功能
            // let main_window = app.get_window("main").unwrap();
            // tauri::api::dialog::blocking::message(Some(&main_window), "Hello", "Welcome back!");

            Ok(())
        })
        // 应用菜单栏
        .menu(init_window_menu())
        // 自定义菜单栏事件监听
        .on_menu_event(|event| {
            // 自定义菜单栏事件处理器
            window_menu_event_handler(event)
        })
        // 应用系统托盘
        .system_tray(init_system_tray())
        // 管理共享状态，这通常用于在应用的不同部分之间共享数据
        // 配置参考文档： https://docs.rs/tauri/1.7.1/tauri/all.html
        // https://docs.rs/tauri/1.7.1/tauri/struct.Builder.html#method.manage
        .manage(AppState::default())
        // 应用指令注册。接受命令、函数的列表。创建一个处理程序，允许使用 invoke() 从 JS 调用命令。
        .invoke_handler(tauri::generate_handler![
            greet,
            close_splashscreen
        ])
        // 注意，此处采用 build 方法而不是默认的 run 方法，因为我们需要拿到 AppHandle 对象，并在下面的监听更新事件中使用它
        // 在编译时读取配置文件，并根据其内容生成 Context
        // 默认的配置文件路径是正在构建的 crate 的 Cargo 清单目录中的一个tauri.conf.json文件。 参考： https://docs.rs/tauri/1.7.1/tauri/macro.generate_context.html
        .build(tauri::generate_context!())
        .expect("error while running tauri application");


    // 获取所有被管理的窗口，并对它们进行一些操作，如管理窗口。
    // 参看： https://docs.rs/tauri/1.7.1/tauri/trait.Manager.html#method.windows
    // app.windows().iter().for_each(|window| {
    //      // 管理窗口，如设置标题、大小、位置、关闭事件等。
    // });


    // 监听更新事件，需要启用 tauri 依赖包的 updater 特性功能  更新的静态 JSON 文件存放  https://gist.github.com/mike2635
    // 需要生成密钥并在 tauri.conf.json 中配置 updater 部分   https://tauri.app/zh-cn/v1/guides/distribution/updater
    // 配置参考文档： https://docs.rs/tauri/1.7.1/tauri/struct.App.html
    // https://docs.rs/tauri/1.7.1/tauri/updater/index.html
    app.run(|_app_handle, event| match event {
        tauri::RunEvent::Updater(updater_event) => {
            match updater_event {
                // 当有可用更新时触发
                tauri::UpdaterEvent::UpdateAvailable { body, date, version } => {
                    println!("update available {} {:?} {}", body, date, version);
                    // tauri::api::dialog::blocking::message(Some(&main_window), "Hello", "Welcome back!");
                }
                // 在即将开始下载时触发
                tauri::UpdaterEvent::Pending => {
                    println!("update is pending!");
                }
                tauri::UpdaterEvent::DownloadProgress { chunk_length, content_length } => {
                    println!("downloaded {} of {:?}", chunk_length, content_length);
                }
                // 在下载完成且即将安装更新时触发
                tauri::UpdaterEvent::Downloaded => {
                    println!("update has been downloaded!");
                }
                // 在安装更新时触发。然后，您可以要求重新启动应用程序
                tauri::UpdaterEvent::Updated => {
                    println!("app has been updated");
                }
                // 当应用程序已安装最新版本且不需要更新时触发
                tauri::UpdaterEvent::AlreadyUpToDate => {
                    println!("app is already up to date");
                }
                // 当更新程序出现错误时触发。我们建议即使启用了默认对话框，也要侦听此事件。
                tauri::UpdaterEvent::Error(error) => {
                    println!("failed to update: {}", error);
                }
            }
        }
        _ => {}
    });

}


// 定义一个简单的状态结构体，用于管理应用共享状态。共享状态需要考虑并发问题，参考： https://rustwiki.org/zh-CN/book/ch16-03-shared-state.html
#[derive(Default)]
struct AppState {
    // 关于 Mutex 互斥锁的使用，参考： https://rustwiki.org/zh-CN/book/ch16-03-shared-state.html
    db: Mutex<Option<DatabaseConnection>>,
    store: Mutex<HashMap<u64, String>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 提取共享状态 AppState
// // 任何 tauri指令处理程序都可以通过 State Guard 检索托管状态。参考： https://docs.rs/tauri/1.7.1/tauri/trait.Manager.html#method.manage
// #[tauri::command]
// async fn connect(app_state: State<AppState>) {
//     // 初始化连接，使用内部可变性改变状态
//     let _ = app_state.db.lock().await;
//     app_state.store.lock().insert(2024, String::from("Hello, World!"));
// }



// 创建命令：
// 该命令必须是异步的，以便它不会在主线程上运行。
#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // 关闭初始屏幕
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // 显示主窗口
    window.get_window("main").unwrap().show().unwrap();
}



// 处理函数，使用 AppState 中的 Redis 客户端
async fn store_user_to_redis(client: Client) {
    // 获取连接
    // let mut conn = state.redis.get_connection().unwrap();
    let mut conn = client.get_connection().unwrap();

    let _: () = conn.set("test", "test_data").unwrap();
    let data: String = conn.get("test").unwrap();
    println!("Redis get test: {}", data);

    // 定义一个用户
    let user = Model {
        id: Default::default(),
        username: Default::default(),
        password: Default::default(),
        role: Default::default(),
        permission: Default::default(),
    };
    // 设置值
    let json_user = serde_json::to_string(&user).unwrap();
    println!("json_user: {:#?}", json_user);

    let _: () = conn.set("user", json_user).unwrap();


    let json_user: String = conn.get("user").unwrap();
    let user: Model = serde_json::from_str(&json_user).unwrap();
    println!("Redis get user: {:#?}", user);
}

