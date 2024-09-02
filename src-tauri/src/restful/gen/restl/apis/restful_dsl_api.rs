///restful-dsl接口
pub mod restful_dsl_api {
    use super::super::super::dtos::restful_dsl_response_dto;
    use super::super::super::dtos::restful_dsl_request_dto;
    use super::super::super::dtos::restful_dsl_version_dto;
    use super::super::super::enums::restful_dsl_enum;
    use crate::core::error::Error;
    use super::super::super::super::super::apis_util;

    ///创建Ts client的Api代码
    pub async fn generate_ts_client_api(_template_dto: restful_dsl_request_dto::RestfulDslCodeTemplateRequestDto, _gen_dir: String) -> Result<restful_dsl_response_dto::RestfulDslMapResponseDto, Error> {
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
        Ok(__res.unwrap().json().await?)
    }
    ///创建rust client的Api代码
    pub async fn generate_rust_client_api(_template_dto: restful_dsl_request_dto::RestfulDslCodeTemplateRequestDto, _gen_dir: String) -> Result<restful_dsl_response_dto::RestfulDslMapResponseDto, Error> {
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
        Ok(__res.unwrap().json().await?)
    }
    ///创建java server的Api代码
    pub async fn generate_java_server_api(_template_dto: restful_dsl_request_dto::RestfulDslCodeTemplateRequestDto) -> Result<restful_dsl_response_dto::RestfulDslListResponseDto, Error> {
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
        Ok(__res.unwrap().json().await?)
    }
    ///创建java server的mock service代码
    pub async fn generate_java_server_mock_service(_template_dto: restful_dsl_request_dto::RestfulDslCodeTemplateRequestDto) -> Result<restful_dsl_response_dto::RestfulDslListResponseDto, Error> {
        let __param = serde_json::json!(_template_dto);
        let __res = reqwest::Client::new()
            .post(apis_util::get_server_uri() + "/_restfulDsl/generateJavaServerMockService")
            .header("Content-Type", "application/json")
            .body(__param.to_string())
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await?)
    }
    ///生成sql语句
    pub async fn generate_sql(_template_dto: restful_dsl_request_dto::RestfulDslCodeTemplateRequestDto) -> Result<restful_dsl_response_dto::RestfulDslMapResponseDto, Error> {
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
        Ok(__res.unwrap().json().await?)
    }
    ///创建或更新文件
    pub async fn create_or_update_file(_file_path: String, _content: String) -> Result<restful_dsl_response_dto::RestfulDslStringResponseDto, Error> {
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
        Ok(__res.unwrap().json().await?)
    }
    ///创建文件夹
    pub async fn create_folder(_folder_path: String) -> Result<restful_dsl_response_dto::RestfulDslStringResponseDto, Error> {
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
        Ok(__res.unwrap().json().await?)
    }
    ///重命名文件
    pub async fn rename_file(_file_path: String, _new_path: String) -> Result<restful_dsl_response_dto::RestfulDslStringResponseDto, Error> {
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
        Ok(__res.unwrap().json().await?)
    }
    ///删除文件
    pub async fn delete_file(_file_path: String) -> Result<restful_dsl_response_dto::RestfulDslListResponseDto, Error> {
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
        Ok(__res.unwrap().json().await?)
    }
    ///根据路径获取内容
    pub async fn get_template_content_by_path(_file_path: String) -> Result<restful_dsl_response_dto::RestfulDslCodeTemplateResponseDto, Error> {
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
        Ok(__res.unwrap().json().await?)
    }
    ///获取restful模板文件树
    pub async fn get_restful_template_file_tree() -> Result<restful_dsl_response_dto::RestfulDslFileTreeResponseDto, Error> {
        let __res = reqwest::Client::new()
            .get(apis_util::get_server_uri() + "/_restfulDsl/getRestfulTemplateFileTree")
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await?)
    }
    ///获取引用文件
    pub async fn get_template_content_by_import_path(_temp_path: String, _import_path: String) -> Result<restful_dsl_response_dto::RestfulDslCodeTemplateResponseDto, Error> {
        let __res = reqwest::Client::new()
            .get(apis_util::get_server_uri() + "/_restfulDsl/getTemplateContentByImportPath?temp_path=" + url::form_urlencoded::byte_serialize(_temp_path.as_bytes()).collect::<String>().as_str() + "&import_path=" + url::form_urlencoded::byte_serialize(_import_path.as_bytes()).collect::<String>().as_str() + "")
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await?)
    }
    ///获取包前缀
    pub async fn get_base_package() -> Result<restful_dsl_response_dto::RestfulDslStringResponseDto, Error> {
        let __res = reqwest::Client::new()
            .get(apis_util::get_server_uri() + "/_restfulDsl/getBasePackage")
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await?)
    }
    ///检查restful文件版本与生成情况
    pub async fn check_restful_file_version() -> Result<restful_dsl_version_dto::RestfulDslVersionCheckResponse, Error> {
        let __res = reqwest::Client::new()
            .get(apis_util::get_server_uri() + "/_restfulDsl/checkRestfulFileVersion")
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().json().await?)
    }
    ///获取当前服务端语言类型
    pub async fn get_server_language_type() -> Result<restful_dsl_enum::RestfulDslServerLanguageTypeEnum, Error> {
        let __res = reqwest::Client::new()
            .get(apis_util::get_server_uri() + "/_restfulDsl/getServerLanguageType")
            .send()
            .await;
        if __res.is_err() {
            return Err("请求失败".into());
        }
        Ok(__res.unwrap().text().await?.parse::<i32>()?.try_into()?)
    }
}
