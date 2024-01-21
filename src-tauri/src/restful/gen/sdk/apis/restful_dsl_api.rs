///restful-dsl接口
pub mod restful_dsl_api {
    use super::super::super::dtos::sdk_response_dto;
    use super::super::super::dtos::sdk_request_dto;
    use super::super::super::dtos::sdk_code_template_dto;
    use super::super::super::dtos::sdk_file_info_dto;
    use crate::core::error::Error;
    use super::super::super::super::super::apis_util;

    ///创建Ts client的Api代码
    pub async fn generate_ts_client_api(_template_dto: sdk_request_dto::SdkCodeTemplateRequestDto, _gen_dir: String) -> Result<sdk_response_dto::SdkMapResponseDto, Error> {
        let __param = serde_json::json!({ "templateDto": _template_dto, "genDir": _gen_dir });
        let __res = reqwest::Client::new()
            .post(apis_util::get_server_uri() + "/_restfulDsl/generateTsClientApi")
            .header("Content-Type", "application/json")
            .body(__param.to_string())
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await.unwrap())
    }
    ///创建rust client的Api代码
    pub async fn generate_rust_client_api(_template_dto: sdk_request_dto::SdkCodeTemplateRequestDto, _gen_dir: String) -> Result<sdk_response_dto::SdkMapResponseDto, Error> {
        let __param = serde_json::json!({ "templateDto": _template_dto, "genDir": _gen_dir });
        let __res = reqwest::Client::new()
            .post(apis_util::get_server_uri() + "/_restfulDsl/generateRustClientApi")
            .header("Content-Type", "application/json")
            .body(__param.to_string())
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await.unwrap())
    }
    ///创建java server的Api代码
    pub async fn generate_java_server_api(_template_dto: sdk_request_dto::SdkCodeTemplateRequestDto) -> Result<sdk_response_dto::SdkListResponseDto, Error> {
        let __param = serde_json::json!(_template_dto);
        let __res = reqwest::Client::new()
            .post(apis_util::get_server_uri() + "/_restfulDsl/generateJavaServerApi")
            .header("Content-Type", "application/json")
            .body(__param.to_string())
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await.unwrap())
    }
    ///生成sql语句
    pub async fn generate_sql(_template_dto: sdk_request_dto::SdkCodeTemplateRequestDto) -> Result<sdk_response_dto::SdkMapResponseDto, Error> {
        let __param = serde_json::json!(_template_dto);
        let __res = reqwest::Client::new()
            .post(apis_util::get_server_uri() + "/_restfulDsl/generateSql")
            .header("Content-Type", "application/json")
            .body(__param.to_string())
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await.unwrap())
    }
    ///创建或更新文件
    pub async fn create_or_update_file(_file_path: String, _content: String) -> Result<sdk_response_dto::SdkStringResponseDto, Error> {
        let __param = serde_json::json!({ "filePath": _file_path, "content": _content });
        let __res = reqwest::Client::new()
            .post(apis_util::get_server_uri() + "/_restfulDsl/createOrUpdateFile")
            .header("Content-Type", "application/json")
            .body(__param.to_string())
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await.unwrap())
    }
    ///创建文件夹
    pub async fn create_folder(_folder_path: String) -> Result<sdk_response_dto::SdkStringResponseDto, Error> {
        let __param = serde_json::json!({ "folderPath": _folder_path });
        let __res = reqwest::Client::new()
            .post(apis_util::get_server_uri() + "/_restfulDsl/createFolder")
            .header("Content-Type", "application/json")
            .body(__param.to_string())
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await.unwrap())
    }
    ///重命名文件
    pub async fn rename_file(_file_path: String, _new_path: String) -> Result<sdk_response_dto::SdkStringResponseDto, Error> {
        let __param = serde_json::json!({ "filePath": _file_path, "newPath": _new_path });
        let __res = reqwest::Client::new()
            .post(apis_util::get_server_uri() + "/_restfulDsl/renameFile")
            .header("Content-Type", "application/json")
            .body(__param.to_string())
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await.unwrap())
    }
    ///删除文件
    pub async fn delete_file(_file_path: String) -> Result<sdk_response_dto::SdkListResponseDto, Error> {
        let __param = serde_json::json!({ "filePath": _file_path });
        let __res = reqwest::Client::new()
            .post(apis_util::get_server_uri() + "/_restfulDsl/deleteFile")
            .header("Content-Type", "application/json")
            .body(__param.to_string())
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await.unwrap())
    }
    ///根据路径获取内容
    pub async fn get_template_content_by_path(_file_path: String) -> Result<sdk_response_dto::SdkCodeTemplateResponseDto, Error> {
        let __param = serde_json::json!({ "filePath": _file_path });
        let __res = reqwest::Client::new()
            .post(apis_util::get_server_uri() + "/_restfulDsl/getTemplateContentByPath")
            .header("Content-Type", "application/json")
            .body(__param.to_string())
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await.unwrap())
    }
    ///获取restful模板文件树
    pub async fn get_restful_template_file_tree() -> Result<sdk_response_dto::SdkFileTreeResponseDto, Error> {
        let __res = reqwest::Client::new()
            .get(apis_util::get_server_uri() + "/_restfulDsl/getRestfulTemplateFileTree")
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await.unwrap())
    }
    ///获取引用文件
    pub async fn get_template_content_by_import_path(_temp_path: String, _import_path: String) -> Result<sdk_response_dto::SdkCodeTemplateResponseDto, Error> {
        let __res = reqwest::Client::new()
            .get(apis_util::get_server_uri() + "/_restfulDsl/getTemplateContentByImportPath?temp_path=" + url::form_urlencoded::byte_serialize(_temp_path.as_bytes()).collect::<String>().as_str() + "&import_path=" + url::form_urlencoded::byte_serialize(_import_path.as_bytes()).collect::<String>().as_str() + "")
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await.unwrap())
    }
    ///获取文件包前缀
    pub async fn get_base_package() -> Result<sdk_response_dto::SdkStringResponseDto, Error> {
        let __res = reqwest::Client::new()
            .get(apis_util::get_server_uri() + "/_restfulDsl/getBasePackage")
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await.unwrap())
    }
}
