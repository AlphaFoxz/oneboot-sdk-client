<script setup lang="ts">
import { useRouter } from 'vue-router'
const router = useRouter()
</script>

<template>
    <a-layout>
        <a-layout-header>
            <a-button class="text-white" @click="router.push({ name: 'Home' })">返回</a-button>
            <a-space class="text-white">数据库设计准则</a-space>
        </a-layout-header>
        <a-layout-content class="text-black">
            <h1 class=" text-3xl">约定</h1>
            <p>1、乐观锁支持版本号和时间戳，版本号字段名统一使用rec_version，时间戳字段名统一使用rec_time。</p>
            <p>2、乐观锁支持版本号和时间戳，版本号字段名统一使用rec_version，时间戳字段名统一使用rec_time。</p>

        </a-layout-content>
    </a-layout>
</template>