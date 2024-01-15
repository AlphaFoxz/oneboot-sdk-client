/**
 * CRUD service的枚举类
 */
export enum SdkCrudServiceTypeEnum {
  /**
   * 有缓存的增删改查
   */
  CACHED = 0,
  /**
   * ABAC数据鉴权 + 缓存的增删改查
   */
  ABAC_CACHED = 1,
}
