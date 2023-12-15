// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    //初始化数据库
    #[cfg(desktop)]
    oneboot_sdk_client_lib::run();
}
