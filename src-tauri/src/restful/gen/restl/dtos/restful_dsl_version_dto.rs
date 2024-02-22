/// 版本检查响应
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslVersionCheckResponse {
    pub id: Option<i64>,
    pub task_id: i64,
    pub success: bool,
    pub message: Option<String>,
    pub data: Vec<RestfulDslVersionCheckDto>,
}
impl std::convert::Into<RestfulDslVersionCheckResponse> for String {
    fn into(self) -> RestfulDslVersionCheckResponse {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
/// 版本检查结果
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslVersionCheckDto {
    pub file_path: String,
    pub sha256: String,
    pub same: bool,
    pub message: String,
}
impl std::convert::Into<RestfulDslVersionCheckDto> for String {
    fn into(self) -> RestfulDslVersionCheckDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
