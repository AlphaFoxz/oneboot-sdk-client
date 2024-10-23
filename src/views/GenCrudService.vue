<script setup lang="ts">
import hljs from 'highlight.js'
import 'highlight.js/styles/github.css'
import { ref, watch, nextTick, onMounted } from 'vue'
import { useToast } from 'primevue/usetoast'
import Button from 'primevue/button'
import Dropdown from 'primevue/dropdown'
import InputText from 'primevue/inputtext'
import InputSwitch from 'primevue/inputswitch'
import { snakeToUpperCamel, camelToUpperSnake, snakeToLowerCamel } from '@/utils/str'
import * as api from '@/api'

const toast = useToast()
const highlightCode = ref('')
const copyRef = ref<HTMLTextAreaElement>()

const templateCachedCodeRef = ref<HTMLInputElement>()
const templateAbacCachedCodeRef = ref<HTMLInputElement>()
const basePackage = ref('com.github.alphafoxz.oneboot')

const moduleOptions = ref([
  { label: 'app', value: 'app' },
  { label: 'preset_sys', value: 'preset_sys' },
  { label: 'sdk', value: 'sdk' },
])
const moduleName = ref(moduleOptions.value[0].value)

const serviceTypeOptions = ref([
  { label: '权限+缓存+增删改查', value: api.SdkCrudServiceTypeEnum.ABAC_CACHED },
  { label: '缓存+增删改查', value: api.SdkCrudServiceTypeEnum.CACHED },
])
const serviceType = ref(api.SdkCrudServiceTypeEnum.ABAC_CACHED)

const poInput = ref('')
const poName = ref('')
const PoName = ref('')
const PO_NAME = ref('')
const isForce = ref(false)
const render = () => {
  const r =
    serviceType.value === api.SdkCrudServiceTypeEnum.ABAC_CACHED
      ? templateAbacCachedCodeRef.value || { innerText: '' }
      : templateCachedCodeRef.value || { innerText: '' }
  highlightCode.value = hljs.highlight(r.innerText, { language: 'java' }).value
  copyRef.value!.value = r.innerText
}
/**
 * 生成当前输入的表
 */
const handleGen = () => {
  if (/^\s*$/.test(PoName.value)) {
    toast.add({ severity: 'warn', summary: '请输入表名', life: 2000 })
    return
  }
  api
    .generateTableCrud(moduleName.value, poName.value, serviceType.value, isForce.value)
    .then(() => {
      toast.add({ severity: 'success', summary: '生成成功，请刷新ide后检查正确性', life: 3000 })
    })
    .catch((e: Error) => {
      toast.add({ severity: 'error', summary: '生成失败，请检查报错信息' + e.message, life: 3000 })
    })
}
/**
 * 生成所有crud
 */
const handleGenAll = () => {
  api
    .generateModuleCrud(moduleName.value, serviceType.value, isForce.value)
    .then(() => {
      toast.add({ severity: 'success', summary: '生成成功，请刷新ide后检查正确性', life: 3000 })
    })
    .catch((e: Error) => {
      toast.add({ severity: 'error', summary: '生成失败，请检查报错信息' + e.message, life: 3000 })
    })
}
watch(poInput, () => {
  if (poInput.value.includes('_')) {
    PO_NAME.value = poInput.value.toUpperCase().trim()
  } else {
    PO_NAME.value = camelToUpperSnake(poInput.value).trim()
  }
  PoName.value = snakeToUpperCamel(PO_NAME.value)
  poName.value = snakeToLowerCamel(PO_NAME.value)
  nextTick(render)
})
watch(moduleName, () => {
  nextTick(render)
})
watch(serviceType, () => {
  nextTick(render)
})
onMounted(render)
</script>

<template>
  <div>
    <div>
      <label>模块:</label>
      <Dropdown
        class="w-1/6"
        show-search
        v-model="moduleName"
        placeholder="选择一个模块名"
        :options="moduleOptions"
        optionLabel="label"
        optionValue="value"
      />
      <label>类型:</label>
      <Dropdown
        class="w-1/4"
        show-search
        v-model="serviceType"
        placeholder="生成crud的类型"
        :options="serviceTypeOptions"
        optionLabel="label"
        optionValue="value"
      />
      <br />
      <label>表名:</label>
      <InputText class="w-1/4" v-model="poInput" placeholder="表名/实体名" />
      <Button label="生成" @click="handleGen"></Button>
      <Button label="生成选定模块的全部表" @click="handleGenAll"></Button>
      <label>强制覆盖已有代码:</label>
      <InputSwitch v-model="isForce" />
    </div>
    <div class="bg-white">
      <textarea ref="copyRef" style="display: none"></textarea>
      <pre
        class="inline-block w-full h-full text-black"
      ><code class="whitespace-pre-wrap overflow-ellipsis overflow-hidden" v-html="highlightCode"></code></pre>
      <pre
        class="inline-block w-full h-full text-black"
      ><code ref="templateCachedCodeRef" class="hidden">package {{ basePackage }}.{{ moduleName }}.service.crud;

import com.github.alphafoxz.oneboot.common.standard.framework.impl.AbstractCachedCrudService;
import {{ basePackage }}.{{ moduleName }}.gen.jooq.tables.{{ PoName }};
import {{ basePackage }}.{{ moduleName }}.gen.jooq.tables.pojos.{{ PoName }}Po;
import {{ basePackage }}.{{ moduleName }}.gen.jooq.tables.records.{{ PoName }}Record;
import jakarta.annotation.Resource;
import lombok.Getter;
import lombok.extern.slf4j.Slf4j;
import org.jooq.DSLContext;
import org.slf4j.Logger;
import org.springframework.cache.CacheManager;
import org.springframework.lang.NonNull;
import org.springframework.stereotype.Service;

import static {{ basePackage }}.{{ moduleName }}.gen.jooq.Tables.{{ PO_NAME }};

/**
 * {{ PoName }}表增删改查service
 */
@Slf4j
@Getter
@Service
public class {{ PoName }}Crud extends AbstractCachedCrudService&lt;{{ PoName }}, {{ PoName }}Po, {{ PoName }}Record> {
    @Resource
    private DSLContext dslContext;
    @Resource
    private CacheManager cacheManager;
    private final {{ PoName }} table = {{ PO_NAME }};
    private final Class&lt;{{ PoName }}Po> poClass = {{ PoName }}Po.class;

    @Override
    @NonNull
    public Logger getLogger() {
        return log;
    }
}
</code>
<code ref="templateAbacCachedCodeRef" class="hidden">
package {{ basePackage }}.{{ moduleName }}.service.crud;

import {{ basePackage }}.{{ moduleName }}.gen.jooq.tables.{{ PoName }};
import {{ basePackage }}.{{ moduleName }}.gen.jooq.tables.pojos.{{ PoName }}Po;
import {{ basePackage }}.{{ moduleName }}.gen.jooq.tables.records.{{ PoName }}Record;
import {{ basePackage }}.common.standard.access_control.AbacPolicy;
import {{ basePackage }}.common.standard.framework.impl.AbstractAbacCachedCrudService;
import {{ basePackage }}.common.toolkit.coding.ArrayUtil;
import {{ basePackage }}.common.toolkit.coding.CollUtil;
import {{ basePackage }}.preset_sys.service.abac.policy.PsysAbacOwnerPolicy;
import jakarta.annotation.Resource;
import lombok.Getter;
import lombok.extern.slf4j.Slf4j;
import org.jooq.Condition;
import org.jooq.DSLContext;
import org.jooq.SortField;
import org.slf4j.Logger;
import org.springframework.cache.CacheManager;
import org.springframework.data.domain.Page;
import org.springframework.lang.NonNull;
import org.springframework.stereotype.Service;

import static {{ basePackage }}.{{ moduleName }}.gen.jooq.Tables.{{ PO_NAME }};

/**
 * {{ PoName }}表增删改查service
 */
@Slf4j
@Getter
@Service
public class {{ PoName }}Crud extends AbstractAbacCachedCrudService&lt;{{ PoName }}, {{ PoName }}Po, {{ PoName }}Record> {
    @Resource
    private DSLContext dslContext;
    @Resource
    private CacheManager cacheManager;
    private final {{ PoName }} table = {{ PO_NAME }};
    private final Class&lt;{{ PoName }}Po> poClass = {{ PoName }}Po.class;

    @Override
    @NonNull
    public Logger getLogger() {
        return log;
    }

    @SuppressWarnings("unchecked")
    Class&lt;? extends AbacPolicy>[] createPolicies = ArrayUtil.toArray(CollUtil.newArrayList(
            PsysAbacOwnerPolicy.class
    ), Class.class);

    @SuppressWarnings("unchecked")
    Class&lt;? extends AbacPolicy>[] readPolicies = ArrayUtil.toArray(CollUtil.newArrayList(
            PsysAbacOwnerPolicy.class
    ), Class.class);

    @SuppressWarnings("unchecked")
    Class&lt;? extends AbacPolicy>[] updatePolicies = ArrayUtil.toArray(CollUtil.newArrayList(
            PsysAbacOwnerPolicy.class
    ), Class.class);

    @SuppressWarnings("unchecked")
    Class&lt;? extends AbacPolicy>[] logicDeletePolicies = ArrayUtil.toArray(CollUtil.newArrayList(
            PsysAbacOwnerPolicy.class
    ), Class.class);

    @SuppressWarnings("unchecked")
    Class&lt;? extends AbacPolicy>[] deletePolicies = ArrayUtil.toArray(CollUtil.newArrayList(
            PsysAbacOwnerPolicy.class
    ), Class.class);

    /**
     * 查询分页不进行权限控制
     */
    @Override
    @NonNull
    public Page&lt;{{ PoName }}Po> selectPage(int pageNum, int pageSize, SortField&lt;?>[] orderBy, Condition... conditions) {
        return super.selectPageWithoutAc(pageNum, pageSize, orderBy, conditions);
    }
}
</code>
</pre>
    </div>
  </div>
</template>
