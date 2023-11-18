pub mod client;
pub mod gen;

use crate::core::error::*;
pub use crate::core::parser;
use crate::core::util;

use self::gen::dtos::sdk_request_dto::SdkStringRequestDto;
use self::{client::ThriftDefaultClient, gen::ifaces::sdk_thrift_iface::TSdkThriftIfaceSyncClient};

pub use crate::thrift::gen::dtos::sdk_code_template_dto::SdkCodeTemplateDto;
pub use crate::thrift::gen::dtos::sdk_request_dto::SdkCodeTemplateRequestDto;
pub use crate::thrift::gen::dtos::sdk_response_dto::SdkCodeTemplateResponseDto;

impl SdkCodeTemplateDto {
    fn build_import(
        &mut self,
        task_id: i64,
        template_path: String,
        import_path: String,
    ) -> Result<Option<SdkCodeTemplateDto>> {
        let temp_path_dto = gen::dtos::sdk_request_dto::SdkStringRequestDto {
            id: util::next_snowflake_id(),
            task_id,
            data: template_path.into(),
        };
        let mut template;
        {
            let mut thrift_client = client::SdkThriftIfaceSyncClient::default_client()?;
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
            if pair.get("rule").unwrap().as_str().unwrap() == "import" {
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
                    if pair.get("rule").unwrap().as_str().unwrap() == "import_value" {
                        let import_path = pair.get("inner").unwrap().as_str().unwrap().to_string();
                        let inner =
                            self.build_import(task_id, template.file_path.clone(), import_path)?;
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
            }
        }
        Ok(Some(template))
    }
    pub fn try_from_file_path(path: String) -> Result<Self> {
        let mut template;
        let task_id = util::next_snowflake_id();
        {
            let mut thrift_client = client::SdkThriftIfaceSyncClient::default_client()?;
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
            if pair.get("rule").unwrap().as_str().unwrap() == "import" {
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
                    if import_pair.get("rule").unwrap().as_str().unwrap() == "import_value" {
                        let import_path = import_pair
                            .get("inner")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string();
                        let import_temp = template.build_import(
                            task_id,
                            template.file_path.clone(),
                            import_path,
                        )?;
                        if import_temp.is_some() {
                            let import_temp = import_temp.unwrap();
                            template
                                .imports
                                .insert(import_temp.file_path.clone(), Box::new(import_temp));
                        }
                    }
                }
            }
        }
        template.ast = Some(value.to_string());
        Ok(template)
    }
}

#[cfg(test)]
mod template_test {
    use crate::core::error::*;
    use crate::core::util;
    use crate::thrift::client::{self, ThriftDefaultClient};
    use crate::thrift::gen::dtos;
    use crate::thrift::gen::ifaces::sdk_gen_code_iface::TSdkGenCodeIfaceSyncClient;
    use crate::thrift::SdkCodeTemplateRequestDto;

    #[test]
    fn test_generate_java_api() -> Result<()> {
        let file_path = r#"D:\idea-workspace\oneboot\.sdk\gen\restful\app\apis\AppTestApi.restful"#;
        let value =
            dtos::sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into())?;
        assert!(!value.imports.is_empty());
        let req_dto = SdkCodeTemplateRequestDto {
            id: util::next_snowflake_id(),
            task_id: util::next_snowflake_id(),
            data: value,
        };
        let mut gen_client = client::SdkGenCodeIfaceSyncClient::default_client()?;
        let result = gen_client.generate_java_api(req_dto);
        assert!(result.is_ok() && result.unwrap().success);
        Ok(())
    }

    #[test]
    fn test_generate_ts_api() -> Result<()> {
        let file_path =
            r#"D:\idea-workspace\oneboot\.sdk\thrift\restful\app\apis\AppTestApi.thrift"#;
        let value =
            dtos::sdk_code_template_dto::SdkCodeTemplateDto::try_from_file_path(file_path.into())?;
        assert!(!value.imports.is_empty());
        let req_dto = SdkCodeTemplateRequestDto {
            id: util::next_snowflake_id(),
            task_id: util::next_snowflake_id(),
            data: value,
        };
        let mut gen_client = client::SdkGenCodeIfaceSyncClient::default_client()?;
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
        Ok(())
    }

    #[test]
    fn test_generate_java_rpc() -> Result<()> {
        let mut gen_client = client::SdkGenCodeIfaceSyncClient::default_client()?;
        let result = gen_client.generate_java_rpc(util::next_snowflake_id());
        assert!(result.is_ok() && result.unwrap().success);
        Ok(())
    }
}
