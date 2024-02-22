use super::restful_dsl_code_template_dto;
use super::restful_dsl_file_info_dto;
// 响应体的data字段是list
/// 字符串列表响应实体
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslListResponseDto {
    /// 主键
    pub id: i64,
    /// 任务id
    pub task_id: i64,
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: Option<String>,
    /// 数据内容
    pub data: Option<Vec<String>>,
}
impl std::convert::Into<RestfulDslListResponseDto> for String {
    fn into(self) -> RestfulDslListResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是map
/// 字符串map响应实体
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslMapResponseDto {
    /// 主键
    pub id: i64,
    /// 任务id
    pub task_id: i64,
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: Option<String>,
    /// 数据内容
    pub data: Option<std::collections::HashMap<String, String>>,
}
impl std::convert::Into<RestfulDslMapResponseDto> for String {
    fn into(self) -> RestfulDslMapResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是long
/// 长整型响应实体
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslLongResponseDto {
    /// 主键
    pub id: i64,
    /// 任务id
    pub task_id: i64,
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: Option<String>,
    /// 数据内容
    pub data: Option<i64>,
}
impl std::convert::Into<RestfulDslLongResponseDto> for String {
    fn into(self) -> RestfulDslLongResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是double
/// 双精度小数响应实体
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslDoubleResponseDto {
    /// 主键
    pub id: i64,
    /// 任务id
    pub task_id: i64,
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: Option<String>,
    /// 数据内容
    pub data: Option<f64>,
}
impl std::convert::Into<RestfulDslDoubleResponseDto> for String {
    fn into(self) -> RestfulDslDoubleResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是binary
/// 二进制响应实体
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslBinaryResponseDto {
    /// 主键
    pub id: i64,
    /// 任务id
    pub task_id: i64,
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: Option<String>,
    /// 数据内容
    pub data: Option<String>,
}
impl std::convert::Into<RestfulDslBinaryResponseDto> for String {
    fn into(self) -> RestfulDslBinaryResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是string
/// 字符串响应实体
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslStringResponseDto {
    /// 主键
    pub id: i64,
    /// 任务id
    pub task_id: i64,
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: Option<String>,
    /// 数据内容
    pub data: Option<String>,
}
impl std::convert::Into<RestfulDslStringResponseDto> for String {
    fn into(self) -> RestfulDslStringResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是RestfulDslCodeTemplateDto
/// 代码模板响应实体
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslCodeTemplateResponseDto {
    /// 主键
    pub id: i64,
    /// 任务id
    pub task_id: i64,
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: Option<String>,
    /// 数据内容
    pub data: Option<restful_dsl_code_template_dto::RestfulDslCodeTemplateDto>,
}
impl std::convert::Into<RestfulDslCodeTemplateResponseDto> for String {
    fn into(self) -> RestfulDslCodeTemplateResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是RestfulDslFileInfoDto
/// 文件树响应实体
#[derive(std::fmt::Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestfulDslFileTreeResponseDto {
    /// 主键
    pub id: i64,
    /// 任务id
    pub task_id: i64,
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: Option<String>,
    /// 数据内容
    pub data: Option<restful_dsl_file_info_dto::RestfulDslFileInfoDto>,
}
impl std::convert::Into<RestfulDslFileTreeResponseDto> for String {
    fn into(self) -> RestfulDslFileTreeResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
