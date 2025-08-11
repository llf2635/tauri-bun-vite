mod system_tray;
mod tray_menu;

#[cfg(desktop)]
pub use system_tray::create_system_tray;
#[cfg(desktop)]
pub use tray_menu::create_tray_menu;
