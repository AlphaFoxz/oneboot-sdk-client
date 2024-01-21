use super::sdk_code_template_dto;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkListRequestDto {
    pub id: i64,
    pub task_id: i64,
    pub data: Vec<String>,
}
impl Into<SdkListRequestDto> for String {
    fn into(self) -> SdkListRequestDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkMapRequestDto {
    pub id: i64,
    pub task_id: i64,
    pub data: std::collections::HashMap<String, String>,
}
impl Into<SdkMapRequestDto> for String {
    fn into(self) -> SdkMapRequestDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkStringRequestDto {
    pub id: i64,
    pub task_id: i64,
    pub data: String,
}
impl Into<SdkStringRequestDto> for String {
    fn into(self) -> SdkStringRequestDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkLongRequestDto {
    pub id: i64,
    pub task_id: i64,
    pub data: i64,
}
impl Into<SdkLongRequestDto> for String {
    fn into(self) -> SdkLongRequestDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkCodeTemplateRequestDto {
    pub id: i64,
    pub task_id: i64,
    pub data: sdk_code_template_dto::SdkCodeTemplateDto,
}
impl Into<SdkCodeTemplateRequestDto> for String {
    fn into(self) -> SdkCodeTemplateRequestDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
