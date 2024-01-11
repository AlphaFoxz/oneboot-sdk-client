<script setup lang="ts">
import { useRouter } from 'vue-router'
import Button from 'primevue/button'

const router = useRouter()
</script>

<template>
  <div style="min-height: 100%">
    <div>
      <Button label="返回" class="text-white" @click="router.push({ name: 'Home' })"></Button>
      <label class="text-white">Restful语法说明</label>
    </div>
    <div class="bg-white text-black">
      <h1 class="text-3xl">注释</h1>
      <h2>普通注释</h2>
      <p>
        <b>// 这是一段普通注释</b
        ><br />注释中的内容将会原样生成到所有前、后端代码中，推荐写得足够详细，比如需求变更之类的
      </p>
      <h2>文档注释</h2>
      <p>
        <b>/* 这是一段文档注释 */</b
        ><br />注释中的内容将会用于文档化，每个字段或结构只支持一段文档注释，而且暂不支持多行
      </p>
      <h1 class="text-3xl">关键字</h1>
      <h2>预置数据类型</h2>
      <p><b>boolean</b>：布尔类型<br />用法：<code>/*是否为文件*/ boolean isFile</code></p>
      <p><b>byte</b>：字节（不推荐）</p>
      <p>
        <b>i16</b>或<b>short</b>：短整数<br />用法：<br /><code>/*服务端口号*/<br />i16 port</code>
      </p>
      <p>
        <b>i32</b>或<b>int</b>：整数<br />用法：<br /><code>/*文件数量*/<br />i32 fileCount</code>
      </p>
      <p>
        <b>i64</b>或<b>long</b>：长整数<br />用法：<br /><code>/*文件大小（单位：字节）*/<br />i64 fileSize</code>
      </p>
      <p>
        <b>double</b>：浮点数<br />用法：<br /><code>/*坐标经度*/<br />double lon</code>
      </p>
      <p><b>binary</b>：二进制（不推荐）</p>
      <p>
        <b>string</b>：字符串<br />用法：<br /><code>/*文件名*/<br />string fileName</code>
      </p>
      <p>
        <b>map</b>：键值对<br />用法：<br /><code>/*文件列表*/<br />map&lt;string, SdkFileDto> children</code>
      </p>
      <p>
        <b>list</b>：列表<br />用法：<br /><code>/*文件列表*/<br />list&lt;SdkFileDto> fileList</code>
      </p>
      <p>
        <b>set</b>：不重复的集合<br />用法：<br /><code>/*权限集合*/<br />set&lt;string> roleList</code>
      </p>
      <p>
        <b>enum</b
        >：当作为字段类型时，表示对枚举的引用（文档级别），但实际上所有enum类型的字段在代码中都是i32类型。但是因为以这种方式进行了引用，所以在代码会生成对应的枚举供开发使用，未来的文档生成也会更详细。<br />用法：<br /><code
          >/*声明一个性别字段*/<br />enum&lt;GenderEnum> gender</code
        >
      </p>
      <p>
        <b>void</b>：无类型（暂定），用在接口返回值，表示不关心返回值，只要请求成功就行了<br />用法：<br /><code
          >/*埋点，推送用户操作数据*/<br />void userHistory(list&lt;string> data)</code
        >
      </p>
      <h2>修饰语</h2>
      <p>
        <b>required</b
        >：必填。所有的字段如果不加修饰语，默认就是必填，所以通常情况下推荐省略，但如果同一个结构体中存在其他optional，那么推荐写上required以强调必填，方便维护<br />用法：<br /><code
          >/*查询id*/<br />required i64 id</code
        >
      </p>
      <p>
        <b>optional</b>：可选。<br />用法：<br /><code>/*筛选标题*/<br />optional string title</code>
      </p>
      <h2>声明结构</h2>
      <p><b>interface</b>：声明一个接口，这个接口会作为前后端交互的标准。详见下面的示例</p>
      <p><b>class</b>：声明一个类，这个类会作为复杂对象的建模，被interface所引用。详见下面的示例</p>
      <p><b>enum</b>：声明一个枚举对象。详见下面的示例</p>
      <h2>引用文件</h2>
      <p><b>import</b>：引用另一个文件（相对路径），可以使用其中的class或enum</p>
      <h2>命名空间</h2>
      <p><b>namespace</b>：声明指定语言的命名空间，用于决定代码生成的结构、包名</p>

      <h1 class="text-3xl">注解</h1>
      <p><b>@uri</b>：指定interface的uri前缀<br />用法：<br /><code>@uri(/api/file)</code></p>
      <p>
        <b>@getUri</b
        >：指定具体接口的get请求uri，支持通配符，通配符中的变量将会以请求头的方式传递（拼接为最终的url）<br />用法:<br /><code
          >@postUri(/query/{id})</code
        >
      </p>
      <p>
        <b>@postUri</b>：指定具体接口的post请求uri，支持通配符，但post通常不会这么使用<br />用法:<br /><code
          >@postUri(/edit)</code
        >
      </p>
      <p>
        <b>@putUri</b>：指定具体接口的put请求uri（不推荐使用），支持通配符<br />用法:<br /><code>@put(/create)</code>
      </p>
      <p>
        <b>@patchUri</b>：指定具体接口的patch请求uri（不推荐使用），支持通配符<br />用法:<br /><code
          >@patchUri(/update)</code
        >
      </p>
      <p>
        <b>@deleteUri</b>：指定具体接口的delete请求uri（不推荐使用），支持通配符<br />用法:<br /><code
          >@deleteUri(/del/{id})</code
        >
      </p>
      <p><b>@page</b>：指定当前接口返回值为分页数据，生成的代码返回值会包装在分页对象中</p>

      <h1 class="text-3xl">示例</h1>
      <h2>接口</h2>
      <pre><code>
namespace java com.github.alphafoxz.oneboot.sdk.gen.restful.apis
namespace ts gen.sdk.apis

import "../dtos/SdkDto.restful"

/* SDK模块文件接口 */
@uri(/sdk/file)
interface SdkFileApi {
  // 这个接口是分页数据
  // 斜杠开头的单行注释内容将原样输出至代码源文件的注释中
  // 在实际请求后，本接口的最终url会是这样的格式 http://127.0.0.1:8080/sdk/file/queryPage/1/10?prefix=xxx
  // TODO: 注意！此需求已经多次变更了，涉及到另外一个XXX模块，需要再找业务确认 2023-12-11 王
  /* 分页查询 */
  @getUri(/queryPage/{pageNum}/{pageSize})
  @page
  list&lt;SdkDto.SdkFileDto> queryPage(i32 pageNum, i32 pageSize, /*文件名前缀*/ optional string prefix)

  // 通常我们需要一个只改单个字段的接口，因为这个字段往往非常重要，比如逻辑删除
  // 可实际中要做这样的设计是很麻烦的，因为post传参需要一个对象，而我们又不希望创建一个只有2个字段的实体（id和status）
  // 在restful模板语言中，零散的几个字段将被前端生成的代码直接放进map&lt;string, xxx>中，后端生成的代码也会自动读，无需人力对接
  // 把这里的注释写好就是我们唯一需要做的事情
  // 此外，如果你的服务端要对接第三方系统，返回值这里要提前沟通，大概率还是需要封装一个dto的，否则就可能导致第三方系统需要做大量修改
  /* 修改状态 */
  @postUri(/editStatus)
  boolean updateStatus(/*数据id*/i64 id,/*修改状态 1:正常 0:删除*/ string status)
}
      </code></pre>
      <h2>类</h2>
      <pre><code>
namespace java com.github.alphafoxz.oneboot.sdk.gen.restful.dtos
namespace ts gen.sdk.dtos

import "../enums/SdkEnum.restful"

// 根据实际情况，我们约定，能通用的实体都叫作dto，当业务复杂之后，只需要对dto进行平级拆分/嵌套即可，返回值的粒度也能够大体控制好
// 重构是免不了的，但由于所有的代码都是带类型的，所以风险会更小
/* SDK模块文件对象 */
class SdkFileDto {
  /* 数据id */
  required i64 id
  /* 文件名 */
  required string fileName
  // 这里
  /* 文件类型 */
  required enum&lt;SdkEnum.SdkFileTypeEnum> fileType
  /* 拥有者 */
  optional string owner
}

// 根据实际情况，我们约定，只让前端提供必要传参的实体叫做param
/* SDK模块文件修改请求参数 */
class SdkFileEditParam {
  /* 数据id */
  i64 id
  /* 修改状态 1:正常 0:删除 */
  string status
}
      </code></pre>
      <h2>枚举</h2>
      <pre><code>
namespace java com.github.alphafoxz.oneboot.sdk.gen.restful.enums
namespace ts gen.sdk.enums

// 每一个枚举值都是一个唯一的i32值，在同一个枚举类中，不能重复
/* 文件类型枚举 */
enum SdkFileTypeEnum {
  /* 本地文件 */
  LOCAL_FILE = 0
  /* 本地文件夹 */
  LOCAL_DIR = 1
}
      </code></pre>
    </div>
  </div>
</template>

<style scoped>
p {
  font-size: 1rem;
  padding: 0.7rem;
}
h1 {
  font-weight: bold;
  padding: 1.5rem 0;
}
h2 {
  font-size: 1.2rem;
  font-weight: bold;
  text-align: center;
}
</style>
