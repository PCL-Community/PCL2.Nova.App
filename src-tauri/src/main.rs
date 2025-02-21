// Windows 下不显示控制台
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
fn main() {
    // TODO: 内存优化的 CLI 调用?
    
    pcl2_nova_app_lib::run();
}
