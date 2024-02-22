/// 文件类型
#[derive(
    std::fmt::Debug, std::cmp::PartialEq, serde_repr::Deserialize_repr, serde_repr::Serialize_repr,
)]
#[repr(i32)]
pub enum RestfulDslFileTypeEnum {
    /// 本地文件
    LocalFile = 0,
    /// 本地文件夹
    LocalDir = 1,
}
impl Into<i32> for RestfulDslFileTypeEnum {
    fn into(self) -> i32 {
        self as i32
    }
}
/// RestfulDsl服务端语言类型
#[derive(
    std::fmt::Debug, std::cmp::PartialEq, serde_repr::Deserialize_repr, serde_repr::Serialize_repr,
)]
#[repr(i32)]
pub enum RestfulDslServerLanguageTypeEnum {
    /// JAVA
    JAVA = 0,
}
impl Into<i32> for RestfulDslServerLanguageTypeEnum {
    fn into(self) -> i32 {
        self as i32
    }
}
