<script setup lang="ts">
import { RouterView } from 'vue-router'
import { ref, provide, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { ElNotification } from 'element-plus';

const activeIndex = ref('/rand/csv')

const config = ref()

async function getconfig() {
  await invoke('return_config').catch((err) => {
    ElNotification({
      title: '错误',
      type: 'error',
      message: err,
      position: 'bottom-right'
    })
  }).then((v) => {
    config.value = v
    console.log(config.value)
  })
}

// 向下级组件透穿config
provide('app_config', config)

onMounted(
  () => {
    getconfig()
  }
)
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