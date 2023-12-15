<script setup lang="ts">
import hljs from 'highlight.js'
import 'highlight.js/styles/github.css'
import { ref, watch, nextTick, onMounted } from 'vue'
import * as utils from '@/utils'
import { useRouter } from 'vue-router'
const router = useRouter()
const highlightCode = ref('')
const copyRef = ref<HTMLTextAreaElement>()

const templateCachedCodeRef = ref<HTMLInputElement>()
const templateAbacCachedCodeRef = ref<HTMLInputElement>()
const packagePrefix = ref('com.github.alphafoxz.oneboot')

const moduleOptions = ref([
  { value: 'app', label: 'app' },
  { value: 'preset_sys', label: 'preset_sys' },
  { value: 'sdk', label: 'sdk' },
])
const moduleName = ref(moduleOptions.value[0].value)

const serviceTypeOptions = ref([
  { value: 'abacCachedCrud', label: '权限+缓存+增删改查' },
  { value: 'cachedCrud', label: '缓存+增删改查' },
])
const serviceType = ref(serviceTypeOptions.value[0].value)

const poInput = ref('')
const PoName = ref('')
const PO_NAME = ref('')
const render = () => {
  if (/^\s*$/.test(poInput.value)) {
    return
  }
  const r =
    serviceType.value === 'abacCachedCrud'
      ? templateAbacCachedCodeRef.value || { innerText: '' }
      : templateCachedCodeRef.value || { innerText: '' }
  highlightCode.value = hljs.highlight(r.innerText, { language: 'java' }).value
  copyRef.value!.value = r.innerText
}
const handleCopy = () => {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(copyRef.value!.value)
    utils.global_message.api.success('复制成功')
  } else {
    copyRef.value!.select()
    document.execCommand('copy')
    utils.global_message.api.success('复制成功')
  }
}
watch(poInput, () => {
  if (poInput.value.includes('_')) {
    PO_NAME.value = poInput.value.toUpperCase().trim()
  } else {
    PO_NAME.value = utils.camelToUpperSnake(poInput.value).trim()
  }
  PoName.value = utils.snakeToUpperCamel(PO_NAME.value)
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
  <a-layout>
    <a-layout-header>
      <a-button class="text-white" @click="router.push({ name: 'Home' })">返回</a-button>
      <label class="text-white">模块:</label>
      <a-select class="w-1/6" show-search v-model:value="moduleName" :options="moduleOptions"></a-select>
      <label class="text-white">实体:</label>
      <a-input class="w-1/4" v-model:value="poInput" placeholder="表名/实体名"></a-input>
      <label class="text-white">类型:</label>
      <a-select class="w-1/4" show-search v-model:value="serviceType" :options="serviceTypeOptions"></a-select>
      <a-button class="text-white" @click="handleCopy">复制</a-button>
    </a-layout-header>
    <a-layout-content>
      <textarea ref="copyRef" style="display: none"></textarea>
      <pre
        class="inline-block w-full h-full text-black"
      ><code class="whitespace-pre-wrap overflow-ellipsis overflow-hidden" v-html="highlightCode"></code></pre>
      <pre
        class="inline-block w-full h-full text-black"
      ><code ref="templateCachedCodeRef" class="hidden">package {{ packagePrefix }}.{{ moduleName }}.service.crud;

import com.github.alphafoxz.oneboot.common.interfaces.framework.impl.AbstractCachedCrudService;
import {{ packagePrefix }}.{{ moduleName }}.gen.jooq.tables.{{ PoName }};
import {{ packagePrefix }}.{{ moduleName }}.gen.jooq.tables.pojos.{{ PoName }}Po;
import {{ packagePrefix }}.{{ moduleName }}.gen.jooq.tables.records.{{ PoName }}Record;
import jakarta.annotation.Resource;
import lombok.Getter;
import lombok.extern.slf4j.Slf4j;
import org.jooq.DSLContext;
import org.slf4j.Logger;
import org.springframework.cache.CacheManager;
import org.springframework.lang.NonNull;
import org.springframework.stereotype.Service;

import static {{ packagePrefix }}.{{ moduleName }}.gen.jooq.Tables.{{ PO_NAME }};

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
package {{ packagePrefix }}.{{ moduleName }}.service.crud;

import {{ packagePrefix }}.{{ moduleName }}.gen.jooq.tables.{{ PoName }};
import {{ packagePrefix }}.{{ moduleName }}.gen.jooq.tables.pojos.{{ PoName }}Po;
import {{ packagePrefix }}.{{ moduleName }}.gen.jooq.tables.records.{{ PoName }}Record;
import {{ packagePrefix }}.common.interfaces.access_control.AbacPolicy;
import {{ packagePrefix }}.common.interfaces.framework.impl.AbstractAbacCachedCrudService;
import {{ packagePrefix }}.common.toolkit.coding.ArrayUtil;
import {{ packagePrefix }}.common.toolkit.coding.CollUtil;
import {{ packagePrefix }}.preset_sys.service.abac.policy.PsysAbacOwnerPolicy;
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

import static {{ packagePrefix }}.{{ moduleName }}.gen.jooq.Tables.{{ PO_NAME }};

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
    </a-layout-content>
  </a-layout>
</template>
