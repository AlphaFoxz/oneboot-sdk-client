/**
 * 代码模板实体
 */
export type SdkCodeTemplateDto = {
  /**
   * 文件路径
   */
  filePath: string
  /**
   * 系统分隔符
   */
  fileSeparator: string
  /**
   * 命名空间
   */
  namespace: Record<string, string>
  /**
   * 抽象语法树
   */
  ast: string | undefined
  /**
   * 文件内容
   */
  content: string
  /**
   * 包含其他模板
   */
  imports: Record<string, SdkCodeTemplateDto>
}
