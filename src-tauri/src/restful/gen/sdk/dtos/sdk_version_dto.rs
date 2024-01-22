use serde::{Deserialize, Serialize};
/// 版本检查响应
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkVersionCheckResponse {
    pub id: Option<i64>,
    pub task_id: i64,
    pub success: bool,
    pub message: Option<String>,
    pub data: Vec<SdkVersionCheckDto>,
}
impl Into<SdkVersionCheckResponse> for String {
    fn into(self) -> SdkVersionCheckResponse {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
/// 版本检查结果
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkVersionCheckDto {
    pub file_path: String,
    pub sha256: String,
    pub same: bool,
    pub message: String,
}
impl Into<SdkVersionCheckDto> for String {
    fn into(self) -> SdkVersionCheckDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
