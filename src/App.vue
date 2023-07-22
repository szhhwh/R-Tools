<script setup lang="ts">
import { House } from '@element-plus/icons'
import { provide, ref } from 'vue';

let activeIndex = ref('/')
function changeactive(newindex: string) {
  console.log(newindex)
  activeIndex.value = newindex
}

provide('activeIndex', { activeIndex, changeactive })
</script>

<template>
  <el-container>
    <el-header>
      <el-menu :default-active=activeIndex :router="true" mode="horizontal">
        <div class="flex"></div>
        <el-menu-item index="/">
          <ElIcon>
            <House></House>
          </ElIcon>
          <span>主页</span>
        </el-menu-item>
        <el-menu-item index="/random/calarand">
          <span>Cala 随机数</span>
        </el-menu-item>
        <el-menu-item index="/calculators/timeLapsephoto">
          <span>延时摄影小工具</span>
        </el-menu-item>
      </el-menu>
    </el-header>
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
  height: 100%;
}
</style>

<style>
.flex {
  flex: 1;
}
</style>