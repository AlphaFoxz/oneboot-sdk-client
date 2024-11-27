import { createSingletonAgg } from 'vue-fn/domain'
import { reactive, ref } from 'vue'
import { matchSimilarStrings } from '@/api'
import { recursivelyReadFolderContent, writeCodeFile } from '@/utils/io'
import { java as javaParser } from '@/utils/parser'
import { forTimes } from '@/utils/fun'
import * as StrUtil from '@/utils/str'
import path from '@/utils/path'
import { toInfrastructureMapperPath, toInfrastructurePoPath, toJavaModulePath } from './path'
import {
  isTransientField,
  VoMapperCode,
  type VoMapping,
  createVoMapping,
  type FieldMapping,
  createFieldMapping,
} from './define'

function createAgg(projectRootPath: string, packageName: string) {
  return createSingletonAgg(() => {
    // ========================= 解析Java文件 =========================
    const similarityThreshold = ref(0.3)
    const domainVoPathRegex = ref<RegExp>()
    const domainModuleName = ref<string>()
    const outputModuleName = ref<string>()
    const domainFilesMeta = reactive<javaParser.JavaFileMeta[]>([])
    const poMetaMap = reactive<Record<string, javaParser.JavaFileMeta>>({})
    const voMappings = reactive<{ [voName: string]: VoMapping }>({})
    async function parse() {
      if (!domainModuleName.value || !outputModuleName.value) {
        return
      }
      reset()
      await javaParser.init()
      domainVoPathRegex.value = new RegExp(
        '^' +
          path
            .join(
              toJavaModulePath(projectRootPath, domainModuleName.value, packageName),
              '([a-zA-Z0-9_]+)',
              'vo',
              '.*[.]java'
            )
            .replace(/\\/g, '[\\\\]') +
          '$'
      )
      //遍历实施模块
      await recursivelyReadFolderContent(
        toInfrastructurePoPath(projectRootPath, outputModuleName.value, packageName),
        (fileInfo) => {
          const meta = javaParser.parse(fileInfo.path, fileInfo.content!)
          poMetaMap[meta.record_declaration[0]?.name || meta.class_declaration[0]?.name] = meta
        },
        undefined,
        '.*[.]java'
      )

      //遍历领域模块
      await recursivelyReadFolderContent(
        toJavaModulePath(projectRootPath, domainModuleName.value, packageName),
        (fileInfo) => {
          const meta = javaParser.parse(fileInfo.path, fileInfo.content!)
          domainFilesMeta.push(meta)

          const matches = domainVoPathRegex.value!.exec(fileInfo.path)
          if (!matches) {
            throw Error('匹配失败: ' + domainVoPathRegex.value + fileInfo.path)
          }
          const aggregation = matches[1]

          if (meta.record_declaration.length === 1) {
            const record = meta.record_declaration[0]
            const paras = record.formalParameters
            const mappedFields = forTimes(paras.length).reduce(
              (acc, index) => {
                //TODO: 或许可以检测是否有注解
                let ignored = false
                if (!isTransientField(paras[index].name)) {
                  acc.mappedCount++
                } else {
                  ignored = true
                }
                const fieldMapping = createFieldMapping(index, paras[index].type, paras[index].name)
                fieldMapping.ignored = ignored
                acc.fields.push(fieldMapping)
                return acc
              },
              { mappedCount: 0, fields: [] as FieldMapping[] }
            )
            if (mappedFields.mappedCount === 0) {
              return
            }
            if (mappedFields.mappedCount === 1) {
              const mapping = createVoMapping(aggregation, meta.record_declaration[0].name)
              mapping.isSimple = true
              ;(mapping.selected = `<${mappedFields.fields.find((f) => !f.ignored)!.type}>`),
                (mapping.package = meta.package_declaration?.name! + '.' + record.name)
              mapping.fields = mappedFields.fields
              for (const i of meta.import_declaration) {
                mapping.selfImports.add(i)
              }
              voMappings[mapping.voName] = mapping
            } else {
              const mapping = createVoMapping(aggregation, meta.record_declaration[0].name)
              mapping.package = meta.package_declaration?.name! + '.' + record.name
              const options = Object.keys(poMetaMap).map((name, index) => {
                return { index, name, package: poMetaMap[name].package_declaration?.name || '', score: 0 }
              })
              mapping.options = options
              // mapping.fields = meta.record_declaration[0].formalParameters.map((p, index) => {
              //   const field = createFieldMapping(index, p.type, p.name)
              //   if (isTransientField(field.fieldName)) {
              //     field.ignored = true
              //   }
              //   return field
              // })
              mapping.fields = mappedFields.fields
              voMappings[mapping.voName] = mapping
            }
          }
        },
        undefined,
        domainVoPathRegex.value.toString().substring(1, domainVoPathRegex.value.toString().length - 1)
      )
    }

    // ================================= 映射 ===============================
    async function autoMapping(): Promise<void> {
      if (!domainModuleName.value || !outputModuleName.value) {
        return
      }
      let sourceEntities: string[] = []
      for (const voName in voMappings) {
        const voMapping = voMappings[voName]
        if (voMapping.isSimple || voMapping.ignored) {
          continue
        }
        sourceEntities.push(voName)
      }
      const targetEntities = Object.keys(poMetaMap)
      const matchVoResult = await matchSimilarStrings(sourceEntities, targetEntities)
      for (const matchVo of matchVoResult) {
        if (matchVo.targets.length === 0) {
          continue
        }
        const voMapping = voMappings[matchVo.name]
        if (voMapping.ignored || voMapping.isSimple) {
          continue
        }
        const poOptions = matchVo.targets.map((t, index) => {
          return { index, package: poMetaMap[t.name].package_declaration?.name || '', name: t.name, score: t.score }
        })
        voMapping.options = poOptions
        if (poOptions.length && poOptions[0].score >= similarityThreshold.value) {
          voMapping.selected = 0
        } else {
          continue
        }

        const sourceFields = voMapping.fields.map((f) => f.fieldName)
        const targetFields = (function () {
          const poMeta = poMetaMap[poOptions[voMapping.selected].name]
          const fields = poMeta.record_declaration[0].formalParameters.filter((f) => !isTransientField(f.name))
          return fields.map((f) => f.name)
        })()
        const matchFieldsResult = await matchSimilarStrings(sourceFields, targetFields)
        for (const i in matchFieldsResult) {
          const options = matchFieldsResult[i].targets.map((t, index) => {
            return { index, package: '', name: t.name, score: t.score }
          })
          voMapping.fields[i].options = options
          if (options.length && options[0].score >= similarityThreshold.value) {
            voMapping.fields[i].selected = 0
          }
        }
      }
    }
    async function editVoMapping(voName: string, optionIndex: number | undefined) {
      if (optionIndex === undefined) {
        const voMapping = voMappings[voName]!
        voMapping.selected = undefined
        for (const field of voMapping.fields) {
          field.options = []
        }
        return
      }
      for (const voName in voMappings) {
        const voMapping = voMappings[voName]
        if (voMapping.voName === voName) {
          voMapping.selected = optionIndex
          const voFields = voMapping.fields.map((f) => f.fieldName)
          const poFields = poMetaMap[optionIndex].record_declaration[0].formalParameters.map((f) => f.name)
          const matchFieldsResult = await matchSimilarStrings(voFields, poFields)
          for (const i in matchFieldsResult) {
            voMapping.fields[i].selected = undefined
            voMapping.fields[i].options = matchFieldsResult[i].targets.map((t, index) => {
              return { index, package: '', name: t.name, score: t.score }
            })
          }
        } else if (voMapping.selected === optionIndex) {
          voMapping.selected = undefined
          for (const field of voMapping.fields) {
            field.selected = undefined
            field.options = []
          }
        }
      }
    }
    function editFieldMapping(voName: string, voFieldName: string, optionIndex: number | undefined) {
      const voMapping = voMappings[voName]!
      for (const field of voMapping.fields) {
        if (field.fieldName === voFieldName) {
          field.selected = optionIndex
        } else if (field.selected === optionIndex) {
          field.selected = undefined
        }
      }
    }

    // ================================= 生成 ===============================
    const genResult = ref<FileInfo[]>([])
    async function generate() {
      if (!outputModuleName.value) {
        return
      }
      await javaParser.init()
      const files: FileInfo[] = []
      const groups = (function () {
        const result: { [aggregation: string]: VoMapping[] } = {}
        for (const voName in voMappings) {
          const voMapping = voMappings[voName]
          if (voMapping.selected === undefined) {
            continue
          }
          if (result[voMapping.aggregation] === undefined) {
            result[voMapping.aggregation] = []
          }
          result[voMapping.aggregation].push(voMapping)
        }
        return result
      })()
      for (const group in groups) {
        const mappings = groups[group]
        const voMapperCode = new VoMapperCode(
          `${packageName}.${outputModuleName.value.replace(/-/g, '.')}.gen.mapper`,
          `${StrUtil.snakeToUpperCamel(group)}VoMapper`
        )
        for (const mapping of mappings) {
          if (mapping.isSimple) {
            voMapperCode.addImport(mapping.package ?? '')
            voMapperCode.addSimpleMapping(mapping.voName, mapping.fields)
          } else {
            voMapperCode.addImport('org.mapstruct.Mapping')
            voMapperCode.addImport(mapping.package ?? '')
            const poName = mapping.options[mapping.selected! as number].name!
            voMapperCode.addImport(poMetaMap[poName].package_declaration?.name! + '.' + poName)
            voMapperCode.addPoMapping(mapping.voName, { poMeta: poMetaMap[poName], fields: mapping.fields })
          }
        }

        files.push({
          isFile: true,
          isFolder: false,
          path: path.join(
            toInfrastructureMapperPath(projectRootPath, outputModuleName.value, packageName),
            `${StrUtil.snakeToUpperCamel(group)}VoMapper.java`
          ),
          content: voMapperCode.getCode(),
        })
      }
      genResult.value = files
      writeCodeFile(...genResult.value)
    }

    // ================================== 重置 ===============================
    async function reset() {
      domainVoPathRegex.value = undefined
      domainFilesMeta.splice(0)
      for (const key in poMetaMap) {
        delete poMetaMap[key]
      }
      for (const key in poMetaMap) {
        delete poMetaMap[key]
      }
      genResult.value = []
    }

    return {
      states: {
        genResult,
        reset,
        voMappings,
        similarityThreshold,
      },
      actions: {
        setDomainModuleName(n: string) {
          domainModuleName.value = n
        },
        setOutputModuleName(n: string) {
          outputModuleName.value = n
        },
        setSimilarityThreshold(n: number) {
          similarityThreshold.value = n
        },
        parse,
        autoMapping,
        generate,
        editVoMapping,
        editFieldMapping,
      },
    }
  })
}

export function useVoMapperAgg(projectRootPath: string, packageName: string) {
  return createAgg(projectRootPath, packageName).api
}
