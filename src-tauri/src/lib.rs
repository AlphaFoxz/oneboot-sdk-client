pub mod core;
pub mod thrift;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let mut store =
                tauri_plugin_store::StoreBuilder::new(".settings.dat").build(app.handle().clone());
            let mut lock = crate::core::store::SETTINGS_STORE.lock().unwrap();
            let res = store.load();
            if res.is_err() {
                let _ = store.save();
            }
            *lock = Some(store);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            thrift::api::check_restful_code_err,
            thrift::api::get_restful_template_file_tree,
            thrift::api::get_template_content_by_path,
            thrift::api::delete_file,
            thrift::api::create_or_update_file,
            thrift::api::create_folder,
            thrift::api::rename_file,
            thrift::api::generate_java_api,
            thrift::api::generate_ts_api,
            thrift::api::generate_sql,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
