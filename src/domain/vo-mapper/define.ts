import * as StrUtil from '@/utils/str'
import { java as javaParser } from '@/utils/parser'

type SimpleVoMappers = { [voName: string]: FieldMapping[] }
type PoMappings = { [voName: string]: PoMappingInfo }
type PoMappingInfo = { poMeta: javaParser.JavaFileMeta; fields: FieldMapping[] }

export function isTransientField(fieldName: string) {
  return fieldName.startsWith('_')
}

export class VoMapperCode {
  private packageName: string
  private ClassName: string
  private importPackages: Set<string> = new Set()
  private simpleMappings: SimpleVoMappers = {}
  private poMappings: PoMappings = {}

  constructor(packageName: string, ClassName: string) {
    this.packageName = packageName
    this.ClassName = ClassName
  }
  addImport(packageName: string) {
    if (packageName) {
      this.importPackages.add(packageName)
    }
  }
  addImports(packageNames: string[] | Set<string>) {
    this.importPackages = new Set([...this.importPackages, ...packageNames])
  }
  private getImportsCode(): string {
    let importsCode = ''
    this.importPackages.forEach((i) => {
      importsCode += `import ${i};\n`
    })
    return importsCode
  }
  addSimpleMapping(voName: string, mapping: FieldMapping[]) {
    this.simpleMappings[voName] = mapping
  }
  addPoMapping(voName: string, info: PoMappingInfo) {
    this.poMappings[voName] = info
  }
  private getMappingsCode(): string {
    let mappingsCode = ''
    mappingsCode += '    // =============================== 单一值 映射 start ===============================\n'
    Object.keys(this.simpleMappings).forEach((voName) => {
      const { type: fieldType, fieldName } = this.simpleMappings[voName].find((m) => !m.ignored)!
      const args = this.simpleMappings[voName]
        .map((m) => {
          if (!m.ignored) {
            return `source`
          } else {
            return `null`
          }
        })
        .join(', ')
      mappingsCode += `    default ${fieldType} ${StrUtil.lowerFirst(fieldType)}To${voName}(${voName} source) {
        return source.${fieldName}();
    }
    default ${voName} ${StrUtil.lowerFirst(voName)}To${fieldType}(${fieldType} source) {
        return new ${voName}(${args});
    }
`
    })
    mappingsCode += '    // =============================== 单一值 映射 finish ===============================\n\n'
    mappingsCode += '    // =============================== 复合值 映射 start ===============================\n'
    Object.keys(this.poMappings).forEach((voName) => {
      const poInfo = this.poMappings[voName]
      const annoStr = (function () {
        let code: string[] = []
        for (const field of poInfo.fields) {
          if (field.ignored || field.selected === undefined) {
            continue
          }
          code.push(`    @Mapping(source = "${field.fieldName}", target = "${field.options[field.selected].name}")`)
        }
        return code.join('\n')
      })()
      const poName = poInfo.poMeta.record_declaration[0].name
      mappingsCode += `${annoStr}
    ${poName} ${StrUtil.lowerFirst(voName)}To${poName}(${voName} source);
    ${voName} ${StrUtil.lowerFirst(poName)}To${voName}(${poName} source);

`
    })
    mappingsCode += '    // =============================== 复合值 映射 finish ==============================='
    return mappingsCode
  }
  getCode(): string {
    return `package ${this.packageName};

${this.getImportsCode()}
public interface ${this.ClassName} {
${this.getMappingsCode()}
}
`
  }
}

export type VoMapping = {
  package?: string
  aggregation: string
  selfImports: Set<string>
  targetImports: Set<string>
  voName: string
  ignored: boolean
  isSimple: boolean
  selected?: number | string
  options: {
    index: number
    package: string
    name: string
    score: number
  }[]
  fields: FieldMapping[]
}

export function createVoMapping(aggregation: string, voName: string): VoMapping {
  return {
    aggregation,
    selfImports: new Set(),
    targetImports: new Set(),
    voName,
    ignored: false,
    isSimple: false,
    options: [],
    fields: [],
  }
}

export type FieldMapping = {
  index: number
  type: string
  fieldName: string
  ignored: boolean
  selected?: number
  options: {
    index: number
    package: string
    name: string
    score: number
  }[]
}

export function createFieldMapping(index: number, type: string, fieldName: string): FieldMapping {
  return {
    index,
    type,
    fieldName,
    options: [],
    ignored: false,
  }
}
