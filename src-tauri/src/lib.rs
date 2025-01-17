pub mod core;
pub mod restful;

use tauri_plugin_store::StoreExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let store = app.store(".settings.json").expect("初始化设置失败");
            store.save().expect("保存设置失败");
            let mut lock = crate::core::store::SETTINGS_STORE.lock().unwrap();
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
            restful::restful_dsl_api::generate_java_server_mock_service,
            restful::restful_dsl_api::generate_ts_client_api,
            restful::restful_dsl_api::generate_rust_client_api,
            restful::restful_dsl_api::generate_sql,
            restful::restful_dsl_api::check_restful_file_version,
            restful::restful_dsl_api::get_server_language_type,
            restful::sdk_api::generate_table_crud,
            restful::sdk_api::generate_module_crud,
            restful::sdk_api::generate_word_api,
            core::io::read_folder_content,
            core::io::write_code_files,
            core::strsim::match_similar_strings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
