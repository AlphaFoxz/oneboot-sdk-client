use super::super::enums::restful_dsl_file_type_enum;
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslFileInfoDto {
    pub file_path: String,
    pub parent_dir: String,
    pub file_name: String,
    pub separator: String,
    pub content: Option<String>,
    pub ext: Option<String>,
    pub file_type: restful_dsl_file_type_enum::RestfulDslFileTypeEnum,
    pub is_read_only: bool,
    pub is_empty: bool,
    pub children: Option<Vec<RestfulDslFileInfoDto>>,
}
impl std::convert::Into<RestfulDslFileInfoDto> for String {
    fn into(self) -> RestfulDslFileInfoDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
