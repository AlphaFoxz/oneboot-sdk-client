// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    //初始化数据库
    let init_db = oneboot_sdk_client_lib::core::db::init_values();
    if init_db.is_err() {
        eprintln!("{:?}", init_db.err().unwrap());
    }
    #[cfg(desktop)]
    oneboot_sdk_client_lib::run();
}
