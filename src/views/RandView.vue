<script setup lang="ts">
import { Fold } from "@element-plus/icons";
import { Expand } from "@element-plus/icons";
import { ref } from "vue"
import { RouterView } from 'vue-router'

const activeIndex = ref('/rand/csv')
const iscollapse = ref<boolean>(false)
</script>

<template>
  <div class="container">
    <div class="aside">
      <!-- 导航菜单 -->
      <ElButton :icon="iscollapse ? Expand : Fold" @click="iscollapse = !iscollapse"></ElButton>
      <el-menu :default-active=activeIndex class="el-menu-demo" mode="vertical" :collapse="iscollapse" :router="true">
        <el-menu-item index="/rand/csv">
          <ElIcon>
            <House></House>
          </ElIcon><span>CSV 随机</span>
        </el-menu-item>
        <el-menu-item index="/rand/csv/setting">
          <el-icon>
            <Setting />
          </el-icon>
          <span>设置</span>
        </el-menu-item>
      </el-menu>
    </div>
    <div class="main">
      <router-view v-slot="{ Component }">
        <keep-alive>
          <component :is="Component"></component>
        </keep-alive>
      </router-view>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
}

.main {
  width: 100%;
}
</style>