<script setup lang="ts">
import { RouterView } from 'vue-router'
import { ref } from 'vue';

const activeIndex = ref('/rand/csv')
</script>

<template>
  <el-container>
    <!-- 导航菜单 -->
    <el-header>
      <el-menu :default-active=activeIndex class="el-menu-demo" mode="horizontal" :ellipsis="false" :router="true">
        <div class="flex-grow" />
        <el-menu-item index="/rand/csv">csv_rand</el-menu-item>
        <!-- <el-menu-item index="/">nor_rand</el-menu-item> -->
      </el-menu>
    </el-header>
    <!-- 组件渲染位置 -->
    <el-main class="main">
      <RouterView />
    </el-main>
  </el-container>
</template>

<style>
.main {
  height: 80vh;
}

.flex-grow {
  flex-grow: 1;
}
</style>