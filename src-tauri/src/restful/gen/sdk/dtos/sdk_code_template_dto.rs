use serde::{Deserialize, Serialize};
/// 代码模板实体
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkCodeTemplateDto {
    /// 文件路径
    pub file_path: String,
    /// 系统分隔符
    pub file_separator: String,
    /// 命名空间
    pub namespace: std::collections::HashMap<String, String>,
    /// 抽象语法树
    pub ast: Option<String>,
    /// 文件内容
    pub content: String,
    /// 包含其他模板
    pub imports: std::collections::HashMap<String, SdkCodeTemplateDto>,
}
impl Into<SdkCodeTemplateDto> for String {
    fn into(self) -> SdkCodeTemplateDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
