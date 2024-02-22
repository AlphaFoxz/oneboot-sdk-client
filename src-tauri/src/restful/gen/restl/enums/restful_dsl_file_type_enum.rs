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
