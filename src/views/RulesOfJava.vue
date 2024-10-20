<script setup lang="ts"></script>

<template>
  <div style="min-height: 100%">
    <div>
      <label>Java编写原则（设计期间拟定）</label>
    </div>
    <div class="bg-white text-black">
      <h1 class="text-3xl">框架推荐</h1>
      <p>1. for循环的局部变量，泛型复杂的变量推荐使用var进行声明</p>
      <p>2. 当一个方法返回少量值时，避免增加新的实体，应该使用框架封装的元组（Tuple）类</p>
      <p>3. 不对swagger文档等自动生成的文件进行手动修改</p>
      <p>4. 同一个java文件内，ide提示的warning数不应该超过10个，推荐不超过5个</p>
      <p>5. 积极使用spring.framework包中的@Nullable和@NotNull注解接口与service方法，可以避免大部分空指针异常</p>
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
</style>
