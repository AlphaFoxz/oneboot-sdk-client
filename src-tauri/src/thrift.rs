pub mod api;
pub mod client;
pub mod gen;

use async_recursion::async_recursion;

use gen::dtos::sdk_request_dto::SdkStringRequestDto;
use gen::ifaces::sdk_thrift_iface::TSdkThriftIfaceSyncClient;

use crate::core::error::Error;
pub use crate::core::parser;
use crate::core::util;
pub use crate::thrift::gen::dtos::sdk_code_template_dto::SdkCodeTemplateDto;
pub use crate::thrift::gen::dtos::sdk_request_dto::SdkCodeTemplateRequestDto;
pub use crate::thrift::gen::dtos::sdk_response_dto::SdkCodeTemplateResponseDto;

impl SdkCodeTemplateDto {
    #[async_recursion]
    pub async fn try_build_import(
        &mut self,
        task_id: i64,
        template_path: String,
        import_path: String,
    ) -> Result<Option<SdkCodeTemplateDto>, Error> {
        let temp_path_dto = gen::dtos::sdk_request_dto::SdkStringRequestDto {
            id: util::next_snowflake_id(),
            task_id,
            data: template_path.into(),
        };
        let mut template;
        {
            let mut thrift_client = client::try_get_sdk_thrift_client().await?;
            template = thrift_client
                .get_template_content_by_include_path(temp_path_dto, import_path)?
                .data
                .unwrap();
        }
        if self.imports.contains_key(&template.file_path) {
            return Ok(None);
        }
        let value = parser::parse_restful_root_into_json(&template.content)?;
        template.ast = Some(value.to_string());
        for pair in value
            .as_object()
            .unwrap()
            .get("pairs")
            .unwrap()
            .as_array()
            .unwrap()
        {
            let pair = pair.as_object().unwrap();
            if pair.get("rule").unwrap().as_str().unwrap() != "import" {
                continue;
            }
            for pair in pair
                .get("inner")
                .unwrap()
                .as_object()
                .unwrap()
                .get("pairs")
                .unwrap()
                .as_array()
                .unwrap()
            {
                if pair.get("rule").unwrap().as_str().unwrap() != "import_value" {
                    continue;
                }
                let import_path = pair.get("inner").unwrap().as_str().unwrap().to_string();
                let inner = self
                    .try_build_import(task_id, template.file_path.clone(), import_path)
                    .await?;
                if inner.is_some() {
                    let inner = inner.unwrap();
                    let k = &inner.file_path;
                    if self.imports.contains_key(k) {
                        return Ok(None);
                    }
                    self.imports.insert(k.to_string(), Box::new(inner));
                }
            }
        }
        Ok(Some(template))
    }
    pub async fn try_from_file_path(path: String) -> Result<Self, Error> {
        let mut template;
        let task_id = util::next_snowflake_id();
        {
            let mut thrift_client = client::try_get_sdk_thrift_client().await?;
            let path_param = SdkStringRequestDto {
                id: util::next_snowflake_id(),
                task_id: task_id,
                data: path.clone().into(),
            };
            template = thrift_client
                .get_template_content_by_path(path_param)?
                .data
                .unwrap();
        }
        let value = parser::parse_restful_root_into_json(&template.content)?;
        for pair in value
            .as_object()
            .unwrap()
            .get("pairs")
            .unwrap()
            .as_array()
            .unwrap()
        {
            let pair = pair.as_object().unwrap();
            if pair.get("rule").unwrap().as_str().unwrap() != "import" {
                continue;
            }
            for import_pair in pair
                .get("inner")
                .unwrap()
                .as_object()
                .unwrap()
                .get("pairs")
                .unwrap()
                .as_array()
                .unwrap()
            {
                if import_pair.get("rule").unwrap().as_str().unwrap() != "import_value" {
                    continue;
                }
                let import_path = import_pair
                    .get("inner")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string();
                let import_temp = template
                    .try_build_import(task_id, template.file_path.clone(), import_path)
                    .await?;
                if import_temp.is_some() {
                    let import_temp = import_temp.unwrap();
                    template
                        .imports
                        .insert(import_temp.file_path.clone(), Box::new(import_temp));
                }
            }
        }
        template.ast = Some(value.to_string());
        Ok(template)
    }
}

#[cfg(test)]
mod template_test {
    use crate::core::util;
    use crate::thrift::client;
    use crate::thrift::gen::dtos;
    use crate::thrift::gen::ifaces::sdk_gen_code_iface::TSdkGenCodeIfaceSyncClient;
    use crate::thrift::SdkCodeTemplateRequestDto;

    #[tokio::test]
    async fn test_generate_java_api() {
        let file_path = r#"D:\idea-workspace\oneboot\.sdk\gen\restful\app\apis\AppTestApi.restful"#;
        let value =
            dtos::sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into())
                .await
                .unwrap();
        assert!(!value.imports.is_empty());
        let req_dto = SdkCodeTemplateRequestDto {
            id: util::next_snowflake_id(),
            task_id: util::next_snowflake_id(),
            data: value,
        };
        let mut gen_client = client::try_get_sdk_gen_code_client().await.unwrap();
        let result = gen_client.generate_java_api(req_dto);
        assert!(result.is_ok() && result.unwrap().success);
    }

    #[tokio::test]
    async fn test_generate_ts_api() {
        let file_path =
            r#"D:\idea-workspace\oneboot\.sdk\thrift\restful\app\apis\AppTestApi.thrift"#;
        let value =
            dtos::sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into())
                .await
                .unwrap();
        assert!(!value.imports.is_empty());
        let req_dto = SdkCodeTemplateRequestDto {
            id: util::next_snowflake_id(),
            task_id: util::next_snowflake_id(),
            data: value,
        };
        let mut gen_client = client::try_get_sdk_gen_code_client().await.unwrap();
        let result = gen_client.preview_generate_ts_api(
            req_dto,
            String::from(r#"D:\vscode-projects\oneboot_front\src\apis"#),
        );
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
        for (key, value) in result.data.unwrap().iter() {
            let path = std::path::Path::new(key);
            let prefix = path.parent().unwrap();
            std::fs::create_dir_all(prefix).unwrap();
            std::fs::write(path, value).unwrap();
        }
    }

    #[tokio::test]
    async fn test_generate_java_rpc() {
        let mut gen_client = client::try_get_sdk_gen_code_client().await.unwrap();
        let result = gen_client.generate_java_rpc(util::next_snowflake_id());
        assert!(result.is_ok() && result.unwrap().success);
    }
}
