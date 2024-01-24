pub mod core;
pub mod restful;

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
            restful::restful_dsl_api::check_restful_code_err,
            restful::restful_dsl_api::get_base_package,
            restful::restful_dsl_api::get_restful_template_file_tree,
            restful::restful_dsl_api::get_template_content_by_path,
            restful::restful_dsl_api::delete_file,
            restful::restful_dsl_api::create_or_update_file,
            restful::restful_dsl_api::create_folder,
            restful::restful_dsl_api::rename_file,
            restful::restful_dsl_api::generate_java_server_api,
            restful::restful_dsl_api::generate_ts_client_api,
            restful::restful_dsl_api::generate_rust_client_api,
            restful::restful_dsl_api::generate_sql,
            restful::restful_dsl_api::check_restful_file_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
