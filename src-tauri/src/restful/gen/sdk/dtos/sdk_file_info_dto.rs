use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkFileInfoDto {
    pub file_path: String,
    pub parent_dir: String,
    pub file_name: String,
    pub separator: String,
    pub content: Option<String>,
    pub ext: Option<String>,
    pub file_type: i32,
    pub is_read_only: bool,
    pub is_empty: bool,
    pub children: Option<Vec<SdkFileInfoDto>>,
}
impl Into<SdkFileInfoDto> for String {
    fn into(self) -> SdkFileInfoDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
