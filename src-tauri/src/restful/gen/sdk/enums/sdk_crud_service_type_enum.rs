/// CRUD service的枚举类
#[derive(
    std::fmt::Debug, std::cmp::PartialEq, serde_repr::Deserialize_repr, serde_repr::Serialize_repr,
)]
#[repr(i32)]
pub enum SdkCrudServiceTypeEnum {
    /// 有缓存的增删改查
    CACHED = 0,
    /// ABAC数据鉴权 + 缓存的增删改查
    AbacCached = 1,
}
impl Into<i32> for SdkCrudServiceTypeEnum {
    fn into(self) -> i32 {
        self as i32
    }
}
