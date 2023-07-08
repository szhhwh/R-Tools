<script setup lang="ts">
import { RouterView } from 'vue-router'
import { ref } from 'vue';

const activeIndex = ref('/rand/csv')

</script>

<template>
  <el-container>
    <el-header>
      <!-- 导航菜单 -->
      <el-menu :default-active=activeIndex class="el-menu-demo" mode="horizontal" :ellipsis="false" :router="true">
        <div class="flex-grow" />
        <el-menu-item index="/rand/csv">CSV 随机</el-menu-item>
        <el-menu-item index="/rand/csv/setting">
          <el-icon>
            <Setting />
          </el-icon>
          <span>设置</span>
        </el-menu-item>
      </el-menu>
    </el-header>
    <!-- 组件渲染位置 -->
    <el-main class="main">
      <router-view v-slot="{ Component }">
          <keep-alive>
            <component :is="Component"></component>
          </keep-alive>
      </router-view>
    </el-main>
  </el-container>
</template>

<style scoped>
.main {
  height: 80vh;
}

.flex-grow {
  flex-grow: 1;
}
</style>