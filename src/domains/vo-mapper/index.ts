import { createApi } from 'vue-fn/domain'
import { recursivelyReadFolderContent, writeCodeFile } from '@/utils/io'
import path from '@/utils/path'
import { toInfrastructurePoPath, toJavaModulePath } from './path'
import { reactive, ref } from 'vue'
import { java as javaParser } from '@/utils/parser'
import { DomainMetaMap, genCode } from './gen'

export function domain(
  projectRootPath: string,
  domainModuleName: string,
  outputModuleName: string,
  packageName: string
) {
  const domainVoPathRegex = ref<RegExp>()
  const domainFilesMeta = reactive<javaParser.JavaFileMeta[]>([])
  const infrastructureFilesMeta = reactive<javaParser.JavaFileMeta[]>([])

  async function generate() {
    await javaParser.init()
    domainVoPathRegex.value = new RegExp(
      '^' +
        path
          .join(toJavaModulePath(projectRootPath, domainModuleName, packageName), '([a-zA-Z0-9_]+)', 'vo', '.*[.]java')
          .replace(/\\/g, '[\\\\]') +
        '$'
    )
    //遍历领域模块
    await recursivelyReadFolderContent(
      toJavaModulePath(projectRootPath, domainModuleName, packageName),
      (fileInfo) => {
        domainFilesMeta.push(javaParser.parse(fileInfo.path, fileInfo.content!))
      },
      undefined,
      domainVoPathRegex.value.toString().substring(1, domainVoPathRegex.value.toString().length - 1)
    )
    //遍历实施模块
    await recursivelyReadFolderContent(
      toInfrastructurePoPath(projectRootPath, outputModuleName, packageName),
      (fileInfo) => {
        infrastructureFilesMeta.push(javaParser.parse(fileInfo.path, fileInfo.content!))
      },
      undefined,
      '.*[.]java'
    )
    const parseMap = domainFilesMeta.reduce((map, currentValue) => {
      const matches = domainVoPathRegex.value!.exec(currentValue._filePath)
      if (!matches) {
        throw Error('匹配失败: ' + domainVoPathRegex.value! + currentValue._filePath)
      }
      const module = matches[1]
      if (!map[module]) {
        map[module] = []
      }
      map[module]!.push(currentValue)
      return map
    }, {} as DomainMetaMap)
    const genResult = genCode(projectRootPath, packageName, outputModuleName, parseMap)
    writeCodeFile(...genResult)
  }

  function scan() {}
  return createApi({
    actions: {
      generate,
      scan,
    },
  })
}

export function createVoMapperDomain(
  projectRootPath: string,
  domainModuleName: string,
  outputModuleName: string,
  packageName: string
) {
  return domain(projectRootPath, domainModuleName, outputModuleName, packageName)
}
