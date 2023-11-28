pub mod core;
pub mod thrift;

use std::collections::BTreeMap;

use crate::thrift::gen::ifaces::sdk_gen_code_iface::TSdkGenCodeIfaceSyncClient;
use crate::thrift::gen::ifaces::sdk_info_iface::TSdkInfoIfaceSyncClient;

use crate::core::{parser, util};
use crate::thrift::client::ThriftDefaultClient;
use crate::thrift::gen::dtos;
use crate::thrift::gen::ifaces::sdk_thrift_iface::TSdkThriftIfaceSyncClient;
use dtos::{sdk_request_dto, sdk_response_dto};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

/// Check thrift code
#[tauri::command]
fn check_restful_code_err(code: &str) -> parser::CheckResult {
    parser::restful::check_restful_code_err(code)
}

/// 获取restful模板文件树
#[tauri::command]
fn get_restful_template_file_tree() -> Option<sdk_response_dto::SdkFileTreeResponseDto> {
    let c = thrift::client::SdkThriftIfaceSyncClient::default_client();
    if c.is_err() {
        return None;
    }
    let result = c.unwrap().get_restful_template_file_tree();
    if result.is_err() {
        return None;
    }
    Some(result.unwrap())
}

/// 根据路径获取模板内容
#[tauri::command]
fn get_template_content_by_path(
    file_path: &str,
) -> Option<sdk_response_dto::SdkCodeTemplateResponseDto> {
    let c = thrift::client::SdkThriftIfaceSyncClient::default_client();
    if c.is_err() {
        return None;
    }
    let mut c = c.unwrap();
    let param = sdk_request_dto::SdkStringRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: file_path.to_string(),
    };
    let result = c.get_template_content_by_path(param);
    if result.is_err() {
        return None;
    }
    return Some(result.unwrap());
}

/// 删除文件
#[tauri::command]
fn delete_file(file_path: &str) -> Option<sdk_response_dto::SdkListResponseDto> {
    let c = thrift::client::SdkInfoIfaceSyncClient::default_client();
    if c.is_err() {
        return None;
    }
    let mut c = c.unwrap();
    let param = sdk_request_dto::SdkStringRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: file_path.to_string(),
    };
    let result = c.delete_file(param);
    if result.is_err() {
        return None;
    }
    return Some(result.unwrap());
}

/// 创建或更新文件
#[tauri::command]
fn create_or_update_file(
    file_path: &str,
    content: &str,
) -> Option<sdk_response_dto::SdkStringResponseDto> {
    let c = thrift::client::SdkInfoIfaceSyncClient::default_client();
    if c.is_err() {
        eprint!("error {:?}", c.err());
        return None;
    }
    let mut c = c.unwrap();
    let param = sdk_request_dto::SdkStringRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: file_path.to_string(),
    };
    let result = c.create_or_update_file(param, content.to_string());
    if result.is_err() {
        eprint!("error {:?}", result.err().unwrap());
        return None;
    }
    return Some(result.unwrap());
}

/// 创建文件夹
#[tauri::command]
fn create_folder(folder_path: &str) -> Option<sdk_response_dto::SdkStringResponseDto> {
    let c = thrift::client::SdkInfoIfaceSyncClient::default_client();
    if c.is_err() {
        eprint!("error {:?}", c.err());
        return None;
    }
    let mut c = c.unwrap();
    let param = sdk_request_dto::SdkStringRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: folder_path.to_string(),
    };
    let result = c.create_folder(param);
    if result.is_err() {
        eprint!("error {:?}", result.err().unwrap());
        return None;
    }
    return Some(result.unwrap());
}

/// 重命名文件
#[tauri::command]
fn rename_file(file_path: &str, new_path: &str) -> Option<sdk_response_dto::SdkStringResponseDto> {
    let c = thrift::client::SdkInfoIfaceSyncClient::default_client();
    if c.is_err() {
        eprint!("error {:?}", c.err());
        return None;
    }
    let mut c = c.unwrap();
    let param = sdk_request_dto::SdkStringRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: file_path.to_string(),
    };
    let result = c.rename_file(param, new_path.to_string());
    if result.is_err() {
        eprint!("error {:?}", result.err().unwrap());
        return None;
    }
    return Some(result.unwrap());
}

/// 根据文件路径生成java api
#[tauri::command]
fn generate_java_api(file_path: &str) -> bool {
    let value =
        dtos::sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into());
    if value.is_err() {
        return false;
    }
    let req_dto = sdk_request_dto::SdkCodeTemplateRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: value.unwrap(),
    };
    let gen_client = thrift::client::SdkGenCodeIfaceSyncClient::default_client();
    if gen_client.is_err() {
        return false;
    }
    let result = gen_client.unwrap().generate_java_api(req_dto);
    result.is_ok() && result.unwrap().success
}

/// 根据文件路径生成ts api
#[tauri::command]
fn generate_ts_api(file_path: &str) -> bool {
    use crate::core::db;
    let gen_dir = db::get_option_tree_value(db::OPTION_TS_GEN_DIR_KEY).unwrap();
    let value =
        dtos::sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into());
    if value.is_err() {
        return false;
    }
    let req_dto = sdk_request_dto::SdkCodeTemplateRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: value.unwrap(),
    };
    let gen_client = thrift::client::SdkGenCodeIfaceSyncClient::default_client();
    if gen_client.is_err() {
        return false;
    }
    let result = gen_client
        .unwrap()
        .preview_generate_ts_api(req_dto, gen_dir);
    if result.is_err() {
        return false;
    }
    let result = result.unwrap();
    if !result.success {
        return false;
    }
    for (key, value) in result.data.unwrap().iter() {
        let path = std::path::Path::new(key);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        std::fs::write(path, value).unwrap();
    }
    true
}

#[tauri::command]
fn generate_sql(file_path: &str) -> BTreeMap<String, serde_json::Value> {
    let mut result = BTreeMap::<String, serde_json::Value>::new();
    result.insert("success".into(), false.into());
    let value =
        dtos::sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into());
    if value.is_err() {
        return result;
    }
    let req_dto = sdk_request_dto::SdkCodeTemplateRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: value.unwrap(),
    };
    let gen_client = thrift::client::SdkGenCodeIfaceSyncClient::default_client();
    if gen_client.is_err() {
        return result;
    }
    let response = gen_client.unwrap().preview_generate_sql(req_dto);
    if response.is_ok() {
        let mut codes = serde_json::Map::new();
        let response = response.unwrap();
        if response.success && response.data.is_some() {
            for (k, v) in response.data.unwrap().iter() {
                codes.insert(k.into(), v.to_string().into());
            }
        }
        result.insert("data".into(), codes.into());
        result.insert("success".into(), true.into());
    }
    result
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            check_restful_code_err,
            get_restful_template_file_tree,
            get_template_content_by_path,
            delete_file,
            create_or_update_file,
            create_folder,
            rename_file,
            generate_java_api,
            generate_ts_api,
            generate_sql,
            crate::core::db::get_option_tree_value_command,
            crate::core::db::set_option_tree_value_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use crate::thrift::gen::ifaces::sdk_info_iface::TSdkInfoIfaceSyncClient;

    use crate::core::util;
    use crate::thrift::client::ThriftDefaultClient;
    use crate::thrift::gen::dtos::sdk_request_dto;

    #[test]
    fn it_works() {
        let mut c = crate::thrift::client::SdkInfoIfaceSyncClient::default_client().unwrap();
        let param = sdk_request_dto::SdkStringRequestDto {
            id: util::next_snowflake_id(),
            task_id: util::next_snowflake_id(),
            data: "".to_string(),
        };
        let result = c.create_or_update_file(param, "".to_string());
        eprintln!("first: {:?} ", result);
        super::create_or_update_file("", "");
    }
}
