use super::gen::sdk::apis::restful_dsl_api::restful_dsl_api;
use super::gen::sdk::dtos::{sdk_code_template_dto, sdk_request_dto, sdk_response_dto};
use crate::core::error::Error;
use crate::core::parser;
use crate::core::util;
use std::collections::BTreeMap;

/// Check thrift code
#[tauri::command]
pub fn check_restful_code_err(code: &str) -> parser::CheckResult {
    parser::restful::check_restful_code_err(code)
}

/// Check thrift code
#[tauri::command]
pub async fn get_base_package() -> Result<sdk_response_dto::SdkStringResponseDto, Error> {
    Ok(restful_dsl_api::get_base_package().await?)
}

///获取restful模板文件树
#[tauri::command]
pub async fn get_restful_template_file_tree(
) -> Result<sdk_response_dto::SdkFileTreeResponseDto, Error> {
    let result = restful_dsl_api::get_restful_template_file_tree().await;
    if result.is_err() {
        return Err("请求失败".into());
    }
    Ok(result.unwrap())
}

/// 根据路径获取模板内容
#[tauri::command]
pub async fn get_template_content_by_path(
    file_path: &str,
) -> Result<sdk_response_dto::SdkCodeTemplateResponseDto, Error> {
    let result = restful_dsl_api::get_template_content_by_path(file_path.to_string()).await;
    if result.is_err() {
        return Err("请求失败".into());
    }
    return Ok(result.unwrap());
}

// 删除文件
#[tauri::command]
pub async fn delete_file(file_path: &str) -> Result<sdk_response_dto::SdkListResponseDto, Error> {
    let result = restful_dsl_api::delete_file(file_path.to_string()).await;
    if result.is_err() {
        return Err("请求失败".into());
    }
    return Ok(result.unwrap());
}

/// 创建或更新文件
#[tauri::command]
pub async fn create_or_update_file(
    file_path: &str,
    content: &str,
) -> Result<sdk_response_dto::SdkStringResponseDto, Error> {
    let result =
        restful_dsl_api::create_or_update_file(file_path.to_string(), content.to_string()).await;
    if result.is_err() {
        eprint!("error {:?}", result.err().unwrap());
        return Err("请求失败，请检查服务器状态".into());
    }
    return Ok(result.unwrap());
}

/// 创建文件夹
#[tauri::command]
pub async fn create_folder(
    folder_path: &str,
) -> Result<sdk_response_dto::SdkStringResponseDto, Error> {
    let result = restful_dsl_api::create_folder(folder_path.to_string()).await;
    if result.is_err() {
        eprint!("error {:?}", result.err().unwrap());
        return Err("请求失败，请检查服务器状态".into());
    }
    return Ok(result.unwrap());
}

/// 重命名文件
#[tauri::command]
pub async fn rename_file(
    file_path: &str,
    new_path: &str,
) -> Result<sdk_response_dto::SdkStringResponseDto, Error> {
    let result = restful_dsl_api::rename_file(file_path.to_string(), new_path.to_string()).await;
    if result.is_err() {
        eprint!("error {:?}", result.err().unwrap());
        return Err("请求失败，请检查服务器状态".into());
    }
    return Ok(result.unwrap());
}

/// 根据文件路径生成java api
#[tauri::command]
pub async fn generate_java_server_api(file_path: &str) -> Result<(), Error> {
    let value =
        sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into()).await;
    if value.is_err() {
        return Err("请求失败".into());
    }
    let req_dto = sdk_request_dto::SdkCodeTemplateRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: value.unwrap(),
    };
    let result = restful_dsl_api::generate_java_server_api(req_dto).await;
    if result.is_ok() {
        let result = result.unwrap();
        if result.success {
            return Ok(());
        }
    }
    Err("请求失败".into())
}

/// 根据文件路径生成ts api
#[tauri::command]
pub async fn generate_ts_client_api(file_path: &str) -> Result<(), Error> {
    use crate::core::store;
    let gen_dir;
    let res = store::get_settings_value(store::KEY_TS_CLIENT_GEN_DIR.clone());
    if res.is_none() {
        return Err("获取TS客户端生成目录失败".into());
    } else {
        gen_dir = res.unwrap().as_str().unwrap().to_string();
    }
    let value =
        sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into()).await;
    if value.is_err() {
        return Err("请求失败".into());
    }
    let req_dto = sdk_request_dto::SdkCodeTemplateRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: value.unwrap(),
    };
    let result = restful_dsl_api::generate_ts_client_api(req_dto, gen_dir).await;
    if result.is_err() {
        return Err("请求失败".into());
    }
    let result = result.unwrap();
    if !result.success {
        return Err("请求失败".into());
    }
    for (key, value) in result.data.unwrap().iter() {
        let path = std::path::Path::new(key);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        std::fs::write(path, value).unwrap();
    }
    Ok(())
}

/// 根据文件路径生成ts api
#[tauri::command]
pub async fn generate_rust_client_api(file_path: &str) -> Result<(), Error> {
    use crate::core::store;
    let gen_dir;
    let res = store::get_settings_value(store::KEY_RUST_CLIENT_GEN_DIR.clone());
    if res.is_none() {
        return Err("获取Rust客户端生成目录失败".into());
    } else {
        gen_dir = res.unwrap().as_str().unwrap().to_string();
    }
    let value =
        sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into()).await;
    if value.is_err() {
        return Err("请求失败".into());
    }
    let req_dto = sdk_request_dto::SdkCodeTemplateRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: value.unwrap(),
    };
    let result = restful_dsl_api::generate_rust_client_api(req_dto, gen_dir).await;
    if result.is_err() {
        return Err("请求失败".into());
    }
    let result = result.unwrap();
    if !result.success {
        return Err("请求失败".into());
    }
    for (key, value) in result.data.unwrap().iter() {
        let path = std::path::Path::new(key);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        std::fs::write(path, value).unwrap();
    }
    Ok(())
}

#[tauri::command]
pub async fn generate_sql(file_path: &str) -> Result<BTreeMap<String, serde_json::Value>, Error> {
    let mut result = BTreeMap::<String, serde_json::Value>::new();
    result.insert("success".into(), false.into());
    let value =
        sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into()).await;
    if value.is_err() {
        return Ok(result);
    }
    let req_dto = sdk_request_dto::SdkCodeTemplateRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: value.unwrap(),
    };
    let response = restful_dsl_api::generate_sql(req_dto).await;
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
    Ok(result)
}
