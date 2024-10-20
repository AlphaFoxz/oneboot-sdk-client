<script setup lang="ts"></script>

<template>
  <div style="min-height: 100%">
    <div>
      <label>数据库设计原则（设计期间拟定）</label>
    </div>
    <div class="bg-white text-black">
      <h1 class="text-3xl">设计规范</h1>
      <p><a href="https://developer.aliyun.com/article/60899">阿里云相关规范</a></p>
      <p>
        1. 字段允许适当冗余，以提高查询性能，但必须考虑数据一致。冗余字段应遵循: (1) 不是频繁修改的字段。(2) 不是
        varchar 超长字段，更不能是 text 字段。
      </p>
      <p>
        2.单表行数超过500万行或者单表容量超过2GB，才推荐进行分库分表。
        说明:如果预计2年后的数据量根本达不到这个级别，请不要在创建表时就分库分表。
      </p>
      <p>3. id必须是主键，每个表必须有主键，且保持增长趋势的，id统一使用雪花算法snowflakeId</p>
      <p>4. 每个字段和表必须提供清晰的注释</p>
      <p>5. 时间统一格式: yyyy-MM-dd HH:mm:ss</p>
      <p>6. 每个字段和表必须提供清晰的注释</p>
      <p>7. 更新数据表记录时，必须同时更新记录对应的 【更新时间】 字段值为当前时间,</p>
      <p>
        8. 任何业务字段禁止设置默认值。对于业务上唯一的字段，采用非空限制+代码常量池声明的方式定义
        备注：当需求变更导致常量变更的时候，修改已有常量的变量名（推荐后面+时间戳），然后检查系统中所有报错代码的上下文，看是否会因为常量值的改变造成事故。
        最后，定义一个新常量，名称与原常量名相同。运行受影响代码的单元测试/集成测试。
      </p>
      <p>
        9. postgreSQL有布尔类型字段，但是要慎用，否则难免会出现 true/false/"
        未知"的情况。对于业务字段尽量使用char(1)的枚举方式
      </p>
      <p>10. 多个表的相同字段，必须保证列名一致，数据类型一致。</p>
      <p>
        11.
        可以使用外键的场景：相对更方便在子表建立查询索引，同时主表存在更新、删除外键字段的业务（非高频、非高并发）。其他情况尽量避免设计外键
      </p>
      <p>
        12. 对于频繁更新的表，建议建表时指定表的fillfactor=85，每页预留15%的空间给HOT更新使用。例：create table
        test123(id int, info text) with(fillfactor=85);
      </p>
      <p>
        13. 为了全球化的需求，所有的字符存储与表示，均以UTF-8编码，那么字符计数方法注意：select length('阿里巴巴'); 输出
        4; select octet_length('阿里巴巴'); 输出 12
      </p>
      <p>
        14. 对于值与堆表的存储顺序线性相关的数据，如果通常的查询为范围查询，建议使用BRIN索引。
        例如流式数据，时间字段或自增字段，可以使用BRIN索引，减少索引的大小，加快数据插入速度。 例如 create index idx on
        tbl using brin(id);
      </p>
      <p>
        15. 设计时应尽可能选择合适的数据类型，能用数字的坚决不用字符串，能用树类型的，坚决不用字符串。
        使用好的数据类型，可以使用数据库的索引，操作符，函数，提高数据的查询效率。
      </p>
      <p>16. 避免使用数据库触发器，这会使得数据处理逻辑复杂，不便于调试。</p>
      <p>
        17.
        【JOOQ插件，可选字段】乐观锁支持版本号和时间戳，所以版本号字段名统一使用rec_version，时间戳字段名统一使用rec_time。
      </p>
      <p>
        18.
        btree索引字段不建议超过2000字节，如果有超过2000字节的字段需要建索引，建议使用函数索引（例如哈希值索引），或者使用分词索引。
      </p>
      <h1 class="text-3xl">命名规范</h1>
      <p>
        1.
        表达是与否概念的字段，不需要加is前缀，因为代码中相关bean为强类型。同时能避免一些插件生成isIsEnable这种语义重复的方法
      </p>
      <p>
        2. 表名、字段名必须使用小写字母或数字，禁止出现数字开头，禁止两个下划线中间只
        出现数字。数据库字段名的修改代价很大，因为无法进行预发布，所以字段名称需要慎重考虑。
        推荐采用3部分组成，大模块名_小模块名_业务名。 正确例子: psys_auth_user, psys_auth_token, psys_auth_level3_detail
        错误例子:psysAuthUser, PSYS_AUTH_TOKEN, psys_auth_level_3_detail
      </p>
      <p>3. 表名不使用复数名词。 说明:表名应该仅仅表示表里面的实体内容，不应该表示实体数量</p>
      <p>
        4. 表名不能以pg开头，避免使用保留字。
        <a href="https://www.postgresql.org/docs/14/sql-keywords-appendix.html" target="_blank">参考文档</a>
      </p>
      <p>5. 库名应与应用名称保持一致</p>
      <p>
        6. 不建议使用public schema(不同业务共享的对象可以使用public
        schema)，应该为每个应用分配对应的schema，schema_name最好与user name一致。
      </p>
      <p>7. query中的别名不能使用 "小写字母，下划线，数字" 以外的字符，例如中文。</p>
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
  margin: 1.5rem 0;
}
</style>
