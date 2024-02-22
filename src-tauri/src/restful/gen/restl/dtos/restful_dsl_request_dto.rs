use super::restful_dsl_code_template_dto;
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslListRequestDto {
    pub id: i64,
    pub task_id: i64,
    pub data: Vec<String>,
}
impl std::convert::Into<RestfulDslListRequestDto> for String {
    fn into(self) -> RestfulDslListRequestDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslMapRequestDto {
    pub id: i64,
    pub task_id: i64,
    pub data: std::collections::HashMap<String, String>,
}
impl std::convert::Into<RestfulDslMapRequestDto> for String {
    fn into(self) -> RestfulDslMapRequestDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslStringRequestDto {
    pub id: i64,
    pub task_id: i64,
    pub data: String,
}
impl std::convert::Into<RestfulDslStringRequestDto> for String {
    fn into(self) -> RestfulDslStringRequestDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslLongRequestDto {
    pub id: i64,
    pub task_id: i64,
    pub data: i64,
}
impl std::convert::Into<RestfulDslLongRequestDto> for String {
    fn into(self) -> RestfulDslLongRequestDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslCodeTemplateRequestDto {
    pub id: i64,
    pub task_id: i64,
    pub data: restful_dsl_code_template_dto::RestfulDslCodeTemplateDto,
}
impl std::convert::Into<RestfulDslCodeTemplateRequestDto> for String {
    fn into(self) -> RestfulDslCodeTemplateRequestDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
