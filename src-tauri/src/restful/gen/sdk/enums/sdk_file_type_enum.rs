use serde::{Deserialize, Serialize};
/// 文件类型
#[derive(Debug, Deserialize, Serialize)]
pub enum SdkFileTypeEnum {
    /// 本地文件
    LocalFile = 0,
    /// 本地文件夹
    LocalDir = 1,
}
impl From<SdkFileTypeEnum> for i32 {
    fn from(_e: SdkFileTypeEnum) -> i32 {
        _e.into()
    }
}
