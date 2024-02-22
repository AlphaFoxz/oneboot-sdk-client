pub mod restl;

use crate::core::error::Error;
use crate::core::parser;
use async_recursion::async_recursion;
use restl::apis::restful_dsl_api::restful_dsl_api;
use restl::dtos::restful_dsl_code_template_dto::RestfulDslCodeTemplateDto;

impl RestfulDslCodeTemplateDto {
    #[async_recursion]
    pub async fn try_build_import(
        &mut self,
        template_path: String,
        import_path: String,
    ) -> Result<Option<RestfulDslCodeTemplateDto>, Error> {
        let mut template;
        template = restful_dsl_api::get_template_content_by_import_path(
            template_path.clone(),
            import_path,
        )
        .await?
        .data
        .unwrap();
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
                    .try_build_import(template.file_path.clone(), import_path)
                    .await?;
                if inner.is_some() {
                    let inner = inner.unwrap();
                    let k = &inner.file_path;
                    if self.imports.contains_key(k) {
                        return Ok(None);
                    }
                    self.imports.insert(k.to_string(), inner);
                }
            }
        }
        Ok(Some(template))
    }
    pub async fn try_from_file_path(path: String) -> Result<Self, Error> {
        let mut template;
        template = restful_dsl_api::get_template_content_by_path(path)
            .await?
            .data
            .unwrap();
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
                    .try_build_import(template.file_path.clone(), import_path)
                    .await?;
                if import_temp.is_some() {
                    let import_temp = import_temp.unwrap();
                    template
                        .imports
                        .insert(import_temp.file_path.clone(), import_temp);
                }
            }
        }
        template.ast = Some(value.to_string());
        Ok(template)
    }
}
