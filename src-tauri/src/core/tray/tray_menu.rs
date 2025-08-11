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

#[cfg(desktop)]
use tauri::{App, Wry};
#[cfg(desktop)]
use tauri::image::Image;
#[cfg(desktop)]
use tauri::menu::{CheckMenuItem, CheckMenuItemBuilder, IconMenuItem, Menu, SubmenuBuilder};

/// 创建系统托盘菜单
/// 想要定义和操作系统托盘中的菜单，请阅读 https://v2.tauri.org.cn/learn/window-menu/
#[cfg(desktop)]
pub fn create_tray_menu(app: &mut App) -> Menu<Wry> {
    use tauri::menu::{MenuBuilder, MenuItem};

    // 定义具体菜单项

    // 退出按钮
    let quit_i = MenuItem::with_id(app, "quit", "退出 Coco", true, None::<&str>).unwrap();
    // 设置按钮
    let settings_i = MenuItem::with_id(app, "settings", "设置", true, None::<&str>).unwrap();
    // 打开按钮
    let open_i = MenuItem::with_id(app, "open", "打开 Coco", true, None::<&str>).unwrap();
    // 关于按钮
    let about_i = MenuItem::with_id(app, "about", "关于 Coco", true, None::<&str>).unwrap();
    // 隐藏按钮
    let hide_i = MenuItem::with_id(app, "hide", "隐藏 Coco", true, None::<&str>).unwrap();
    // ......
    let dashboard_i = MenuItem::with_id(app, "dashboard", "仪表盘", true, None::<&str>).unwrap();

    // 从路径加载图标
    let icon_image = Image::from_bytes(include_bytes!("../../../icons/icon.png")).unwrap();

    // 创建具有子菜单的菜单项，参考 https://v2.tauri.org.cn/learn/window-menu/
    let dashboard_submenu = SubmenuBuilder::new(app, "Dashboard")
        .item(&dashboard_i)
        .item(&settings_i)
        .items(&[
            &CheckMenuItem::new(app, "CheckMenuItem 1", true, true, None::<&str>).unwrap(),
            &IconMenuItem::new(app, "IconMenuItem 2", true, Some(icon_image), None::<&str>)
                .unwrap(),
        ])
        .build()
        .expect("TODO: panic message");

    // let icon_item = IconMenuItemBuilder::new("icon")
    //     .icon(icon_image)
    //     .build(app).unwrap();

    let lang_str = app.config().identifier.clone();
    let check_sub_item_1 = CheckMenuItemBuilder::new("English")
        .id("en")
        .checked(lang_str == "en")
        .build(app)
        .unwrap();
    let check_sub_item_2 = CheckMenuItemBuilder::new("简体中文")
        .id("en")
        .checked(lang_str == "en")
        .enabled(false)
        .build(app)
        .unwrap();
    let other_item = SubmenuBuilder::new(app, "语言切换")
        .item(&check_sub_item_1)
        .item(&check_sub_item_2)
        .build()
        .unwrap();

    // 按照一定顺序 把菜单项 MenuItem 放到菜单 Menu 里
    let menu = MenuBuilder::new(app)
        .item(&open_i)
        .separator() // 分割线
        .item(&hide_i)
        .item(&about_i)
        .item(&dashboard_submenu)
        .item(&other_item)
        .item(&settings_i)
        .separator() // 分割线
        .item(&quit_i)
        .build()
        .unwrap();

    menu
}