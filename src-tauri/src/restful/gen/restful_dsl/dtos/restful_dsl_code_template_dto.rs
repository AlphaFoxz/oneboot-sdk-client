/// 代码模板实体
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslCodeTemplateDto {
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
    pub imports: std::collections::HashMap<String, RestfulDslCodeTemplateDto>,
}
impl std::convert::Into<RestfulDslCodeTemplateDto> for String {
    fn into(self) -> RestfulDslCodeTemplateDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
