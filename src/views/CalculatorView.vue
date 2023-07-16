<script lang="ts" setup>
import { Fold } from "@element-plus/icons";
import { Expand } from "@element-plus/icons";
import { ref } from "vue"
import { RouterView } from 'vue-router'

const activeIndex = ref('/calculators/timeLapsephoto')
const iscollapse = ref<boolean>(false)
</script>

<template>
  <div class="container">
    <div class="aside">
      <!-- 导航菜单 -->
      <ElButton :icon="iscollapse ? Expand : Fold" @click="iscollapse = !iscollapse"></ElButton>
      <el-menu :default-active=activeIndex mode="vertical" :collapse="iscollapse" :router="true">
        <el-menu-item index="/calculators/timeLapsephoto">
          <ElIcon>
            <House></House>
          </ElIcon><span>延时摄影小工具</span>
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
</style>