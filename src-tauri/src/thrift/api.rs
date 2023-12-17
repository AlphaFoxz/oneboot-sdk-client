use std::collections::BTreeMap;

use crate::core::error::Error;
use crate::core::parser;
use crate::core::util;
use crate::thrift::client;
use crate::thrift::gen::dtos::{sdk_request_dto, sdk_response_dto};
use crate::thrift::gen::{dtos, ifaces};

use ifaces::sdk_gen_code_iface::TSdkGenCodeIfaceSyncClient;
use ifaces::sdk_info_iface::TSdkInfoIfaceSyncClient;
use ifaces::sdk_thrift_iface::TSdkThriftIfaceSyncClient;

/// Check thrift code
#[tauri::command]
pub fn check_restful_code_err(code: &str) -> parser::CheckResult {
    parser::restful::check_restful_code_err(code)
}

/// 获取restful模板文件树
#[tauri::command]
pub async fn get_restful_template_file_tree(
) -> Result<sdk_response_dto::SdkFileTreeResponseDto, Error> {
    let c = client::try_get_sdk_thrift_client().await;
    if c.is_err() {
        return Err("请求失败".into());
    }
    let result = c.unwrap().get_restful_template_file_tree();
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
    let mut c = client::try_get_sdk_thrift_client().await?;
    let param = sdk_request_dto::SdkStringRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: file_path.to_string(),
    };
    let result = c.get_template_content_by_path(param);
    if result.is_err() {
        return Err("请求失败".into());
    }
    return Ok(result.unwrap());
}

/// 删除文件
#[tauri::command]
pub async fn delete_file(file_path: &str) -> Result<sdk_response_dto::SdkListResponseDto, Error> {
    let c = client::try_get_sdk_info_client().await;
    if c.is_err() {
        return Err("请求失败".into());
    }
    let mut c = c.unwrap();
    let param = sdk_request_dto::SdkStringRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: file_path.to_string(),
    };
    let result = c.delete_file(param);
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
    use ifaces::sdk_info_iface::TSdkInfoIfaceSyncClient;
    let c = client::try_get_sdk_info_client().await;
    if c.is_err() {
        eprint!("error {:?}", c.err());
        return Err("请求失败，请检查服务器状态".into());
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
        return Err("请求失败，请检查服务器状态".into());
    }
    return Ok(result.unwrap());
}

/// 创建文件夹
#[tauri::command]
pub async fn create_folder(
    folder_path: &str,
) -> Result<sdk_response_dto::SdkStringResponseDto, Error> {
    use ifaces::sdk_info_iface::TSdkInfoIfaceSyncClient;
    let c = client::try_get_sdk_info_client().await;
    if c.is_err() {
        eprint!("error {:?}", c.err());
        return Err("请求失败，请检查服务器状态".into());
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
    let c = client::try_get_sdk_info_client().await;
    if c.is_err() {
        eprint!("error {:?}", c.err());
        return Err("请求失败，请检查服务器状态".into());
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
        return Err("请求失败，请检查服务器状态".into());
    }
    return Ok(result.unwrap());
}

/// 根据文件路径生成java api
#[tauri::command]
pub async fn generate_java_api(file_path: &str) -> Result<(), Error> {
    let value =
        dtos::sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into()).await;
    if value.is_err() {
        return Err("请求失败".into());
    }
    let req_dto = sdk_request_dto::SdkCodeTemplateRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: value.unwrap(),
    };
    let gen_client = client::try_get_sdk_gen_code_client().await;
    if gen_client.is_err() {
        return Err("请求失败".into());
    }
    let result = gen_client.unwrap().generate_java_api(req_dto);
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
pub async fn generate_ts_api(file_path: &str) -> Result<(), Error> {
    use crate::core::store;
    let gen_dir;
    let res = store::get_settings_value(store::TS_GEN_DIR.clone()).await;
    if res.is_none() {
        return Err("请求失败".into());
    } else {
        gen_dir = res.unwrap().as_str().unwrap().to_string();
    }
    let value =
        dtos::sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into()).await;
    if value.is_err() {
        return Err("请求失败".into());
    }
    let req_dto = sdk_request_dto::SdkCodeTemplateRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: value.unwrap(),
    };
    let gen_client = client::try_get_sdk_gen_code_client().await;
    if gen_client.is_err() {
        return Err("请求失败".into());
    }
    let result = gen_client
        .unwrap()
        .preview_generate_ts_api(req_dto, gen_dir);
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
        dtos::sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into()).await;
    if value.is_err() {
        return Ok(result);
    }
    let req_dto = sdk_request_dto::SdkCodeTemplateRequestDto {
        id: util::next_snowflake_id(),
        task_id: util::next_snowflake_id(),
        data: value.unwrap(),
    };
    let gen_client = client::try_get_sdk_gen_code_client().await;
    if gen_client.is_err() {
        return Ok(result);
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
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::thrift::gen::ifaces::sdk_info_iface::TSdkInfoIfaceSyncClient;

    use crate::core::util;
    use crate::thrift::gen::dtos::sdk_request_dto;

    #[tokio::test]
    async fn it_works() {
        let mut c = crate::thrift::client::try_get_sdk_info_client()
            .await
            .unwrap();
        let param = sdk_request_dto::SdkStringRequestDto {
            id: util::next_snowflake_id(),
            task_id: util::next_snowflake_id(),
            data: "".to_string(),
        };
        let result = c.create_or_update_file(param, "".to_string());
        eprintln!("first: {:?} ", result);
        assert!(super::create_or_update_file("", "").await.is_ok());
    }
}
