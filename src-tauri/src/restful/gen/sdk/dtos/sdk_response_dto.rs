use super::sdk_code_template_dto;
use super::sdk_file_info_dto;
use serde::{Deserialize, Serialize};
// 响应体的data字段是list
/// 字符串列表响应实体
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkListResponseDto {
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
impl Into<SdkListResponseDto> for String {
    fn into(self) -> SdkListResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是map
/// 字符串map响应实体
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkMapResponseDto {
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
impl Into<SdkMapResponseDto> for String {
    fn into(self) -> SdkMapResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是long
/// 长整型响应实体
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkLongResponseDto {
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
impl Into<SdkLongResponseDto> for String {
    fn into(self) -> SdkLongResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是double
/// 双精度小数响应实体
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkDoubleResponseDto {
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
impl Into<SdkDoubleResponseDto> for String {
    fn into(self) -> SdkDoubleResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是binary
/// 二进制响应实体
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkBinaryResponseDto {
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
impl Into<SdkBinaryResponseDto> for String {
    fn into(self) -> SdkBinaryResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是string
/// 字符串响应实体
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkStringResponseDto {
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
impl Into<SdkStringResponseDto> for String {
    fn into(self) -> SdkStringResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是SdkCodeTemplateDto
/// 代码模板响应实体
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkCodeTemplateResponseDto {
    /// 主键
    pub id: i64,
    /// 任务id
    pub task_id: i64,
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: Option<String>,
    /// 数据内容
    pub data: Option<sdk_code_template_dto::SdkCodeTemplateDto>,
}
impl Into<SdkCodeTemplateResponseDto> for String {
    fn into(self) -> SdkCodeTemplateResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
// 响应体的data字段是SdkFileInfoDto
/// 文件树响应实体
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SdkFileTreeResponseDto {
    /// 主键
    pub id: i64,
    /// 任务id
    pub task_id: i64,
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: Option<String>,
    /// 数据内容
    pub data: Option<sdk_file_info_dto::SdkFileInfoDto>,
}
impl Into<SdkFileTreeResponseDto> for String {
    fn into(self) -> SdkFileTreeResponseDto {
        serde_json::from_str(self.as_str()).unwrap()
    }
}
