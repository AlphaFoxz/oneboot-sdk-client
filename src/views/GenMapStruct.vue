<script setup lang="ts">
import LayoutFlex from '@/components/layout/Flex.vue'
import Select from 'primevue/select'
import Button from 'primevue/button'
import Knob from 'primevue/knob'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import Dialog from 'primevue/dialog'
import Checkbox from 'primevue/checkbox'
import { forTimes } from '@/utils/fun'
import { readFolder } from '@/utils/io'
import path from '@/utils/path'
import { settings } from '@/constants'
import { createVoMapperDomain } from '@/domains/vo-mapper'
import { Store } from '@tauri-apps/plugin-store'
import { onMounted, onUnmounted, reactive, ref, shallowRef, watch, watchEffect } from 'vue'
import { FieldMapping, VoMapping } from '@/domains/vo-mapper/define'

const voMapperDomain = shallowRef<ReturnType<typeof createVoMapperDomain>>()
const store = Store.load(settings.FILE_NAME)
const projectRootFolders = shallowRef<string[]>([])
onMounted(async () => {
  const storeInst = await store
  const projectRootPath: string = (await storeInst.get(settings.KEY_PROJECT_ROOT_DIR)) || 'F:\\idea_projects\\oneboot'
  const folders = await readFolder(projectRootPath)
  projectRootFolders.value = forTimes(folders.length).reduce((acc, index) => {
    const folderName = folders[index].path.split(path.sep).pop() || ''
    if (!folderName.startsWith('.') && !folderName.startsWith('_')) {
      acc.push(folderName)
    }
    return acc
  }, [] as string[])
  voMapperDomain.value = createVoMapperDomain(projectRootPath, 'com.github.alphafoxz.oneboot')
  similarityThreshold.value = voMapperDomain.value.states.similarityThreshold.value * 100
})

const domainModuleName = ref<string>()
const outputModuleName = ref<string>()
const similarityThreshold = ref(0)
onUnmounted(
  watch([domainModuleName, outputModuleName], (v) => {
    voMapperDomain.value?.actions.setDomainModuleName(v[0]!)
    voMapperDomain.value?.actions.setOutputModuleName(v[1]!)
  })
)
onUnmounted(
  watch(similarityThreshold, (v) => {
    voMapperDomain.value?.actions.setSimilarityThreshold(v / 100)
  })
)

async function handleParse() {
  if (domainModuleName.value && outputModuleName.value) {
    await voMapperDomain.value?.actions.parse()
  }
}
async function handleAutoMapping() {
  if (domainModuleName.value && outputModuleName.value) {
    await voMapperDomain.value?.actions.autoMapping()
  }
}
function handleGenerate() {
  voMapperDomain.value?.actions.generate()
}

// ============================= 配置表映射 =============================
const currentVoName = ref()
const voMappings = reactive<VoMapping[]>([])
onUnmounted(
  watchEffect(() => {
    if (!voMapperDomain.value) {
      return
    }
    voMappings.splice(0)
    for (const voName in voMapperDomain.value.states.voMappings) {
      const t = voMapperDomain.value.states.voMappings[voName]
      voMappings.push({
        package: t.package,
        aggregation: t.aggregation,
        selfImports: new Set(t.selfImports),
        targetImports: new Set(t.targetImports),
        voName: t.voName,
        ignored: t.ignored,
        isSimple: t.isSimple,
        fields: [],
        selected: t.selected,
        options: [...t.options],
      })
    }
  })
)

function editVoMapping(voName: string, optionIndex: number | undefined) {
  voMapperDomain.value?.actions.editVoMapping(voName, optionIndex)
}

function handleChangeMappingVo(voName: string) {
  const t = voMappings.find((item) => item.voName === voName)
  if (t) {
    const selected = t.selected
    if (selected && typeof selected === 'number') {
      editVoMapping(voName, selected)
    }
  }
  currentVoName.value = undefined
}
function handleCancelMappingVo(voName: string) {
  const t = voMappings.find((item) => item.voName === voName)
  if (t) {
    editVoMapping(voName, undefined)
  }
}

// =========================== 配置字段映射 ===========================
const dialogVisible = ref(false)
const fieldMappings = reactive<FieldMapping[]>([])
onUnmounted(
  watchEffect(() => {
    if (!currentVoName.value || !voMapperDomain.value) {
      return
    }
    fieldMappings.splice(0)
    const voMapping = voMapperDomain.value?.states.voMappings[currentVoName.value]
    if (!voMapping) {
      return
    }
    for (const t of voMapping.fields) {
      fieldMappings.push({
        index: t.index,
        type: t.type,
        fieldName: t.fieldName,
        ignored: t.ignored,
        selected: t.selected,
        options: [...t.options],
      })
    }
  })
)

function handleShowFieldMapping(voName: string) {
  if (voName === undefined) {
    return
  }
  currentVoName.value = voName
  dialogVisible.value = true
}
function editFieldMapping(voFieldName: string, optionIndex: number | undefined) {
  voMapperDomain.value?.actions.editFieldMapping(currentVoName.value, voFieldName, optionIndex)
}
function handleChangeMappingField(voFieldName: string) {
  const t = fieldMappings.find((item) => item.fieldName === voFieldName)
  if (currentVoName.value && t) {
    const selected = t.selected
    if (selected) {
      editFieldMapping(voFieldName, selected)
    }
  }
}
function handleCancelMappingField(voFieldName: string) {
  const fieldMapping = fieldMappings.find((item) => item.fieldName === voFieldName)
  if (currentVoName.value && fieldMapping) {
    editFieldMapping(voFieldName, undefined)
  }
}
</script>

<template>
  <div>
    <Select v-model="domainModuleName" :options="projectRootFolders" placeholder="选择领域模块"></Select>
    <Select v-model="outputModuleName" :options="projectRootFolders" placeholder="选择输出模块"></Select>
    <LayoutFlex wrapped>
      <Knob v-model="similarityThreshold" v-tooltip.top="'相似度阈值'" :value-template="(n) => `${n}%`"></Knob>
      <Button label="解析实体" @click="handleParse"></Button>
      <Button label="自动映射" @click="handleAutoMapping"></Button>
      <Button label="生成代码" @click="handleGenerate"></Button>
    </LayoutFlex>
    <DataTable :value="voMappings" sortField="selected" showGridlines stripedRows>
      <template #empty> 暂无数据 </template>
      <Column header="是否映射">
        <template #body="{ data }">
          <i
            class="pi"
            :class="{ 'pi-check-circle': data.selected !== undefined, 'pi-times-circle': data.selected === undefined }"
          ></i>
        </template>
        <template #filter="{ filterModel, filterCallback }">
          <Checkbox
            v-model="filterModel.value"
            :indeterminate="filterModel.value === null"
            binary
            @change="filterCallback()"
          />
        </template>
      </Column>
      <Column field="voName" header="值对象名称"></Column>
      <Column field="selected" header="实体对象名称">
        <template #body="{ data }">
          <div v-if="data.isSimple">{{ data.selected }}</div>
          <Select
            v-else
            v-model="data.selected"
            :options="data.options"
            option-label="name"
            :option-value="(t) => t.index"
            @change="handleChangeMappingVo(data.voName)"
          ></Select>
          <Button
            v-if="!data.isSimple"
            v-tooltip.top="'取消映射'"
            text
            icon="pi pi-times"
            severity="danger"
            @click="handleCancelMappingVo(data.voName)"
          ></Button>
        </template>
      </Column>

      <Column field="aggregation" header="所属聚合">
        <template #body="{ data }">
          {{ data.aggregation }}
        </template>
      </Column>
      <Column header="操作">
        <template #body="{ data }">
          <Button
            v-if="!data.isSimple && data.selected !== undefined"
            label="字段"
            text
            @click="handleShowFieldMapping(data.voName)"
          ></Button>
        </template>
      </Column>
    </DataTable>
    <Dialog v-model:visible="dialogVisible" header="字段映射情况" maximizable>
      <DataTable :value="fieldMappings" sortField="name">
        <template #empty> 暂无数据 </template>
        <Column header="是否映射">
          <template #body="{ data }">
            <i
              class="pi"
              :class="{
                'pi-check-circle': data.selected !== undefined,
                'pi-times-circle': data.selected === undefined,
              }"
            ></i>
          </template>
          <template #filter="{ filterModel, filterCallback }">
            <Checkbox
              v-model="filterModel.value"
              :indeterminate="filterModel.value === null"
              binary
              @change="filterCallback()"
            />
          </template>
        </Column>
        <Column field="fieldName" header="值对象名称"></Column>
        <Column field="selected" header="实体对象名称">
          <template #body="{ data }">
            <Select
              v-model="data.selected"
              :options="data.options"
              option-label="name"
              :option-value="(t) => t.index"
              @change="handleChangeMappingField(data.fieldName)"
            ></Select>
          </template>
        </Column>
        <Column header="操作">
          <template #body="{ data }">
            <Button
              v-if="!data.isSimple"
              v-tooltip.top="'取消映射'"
              text
              icon="pi pi-times"
              severity="danger"
              @click="handleCancelMappingField(data.fieldName)"
            ></Button>
          </template>
        </Column>
      </DataTable>
    </Dialog>
  </div>
</template>

<style>
.pi-check-circle {
  color: green;
}
.pi-times-circle {
  color: red;
}
</style>
