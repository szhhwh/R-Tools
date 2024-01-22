<script setup lang="ts">
import { House } from '@element-plus/icons-vue'
import { provide, ref, watch } from 'vue';
import { useRouter } from 'vue-router';

let activeIndex = ref('/')
function changeactive(newindex: string) {
  console.log(newindex)
  activeIndex.value = newindex
}

provide('activeIndex', { activeIndex, changeactive })

const router = useRouter()
watch(router.currentRoute, () => {
  if (router.currentRoute.value.path !== '/' && router.currentRoute.value.fullPath !== '/app/update_dialog') {
    backhome.value = true
  }
  else {
    console.log(router.currentRoute.value.fullPath)
    backhome.value = false
  }
})

let backhome = ref<boolean>(false)

// 关闭右键菜单
document.addEventListener('contextmenu', e => e.preventDefault())
</script>

<template>
  <el-container>
    <el-button v-show="backhome" @click="() => router.push('/')" :icon="House" id="backhome">返回首页</el-button>
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

#backhome {
  position: fixed;
  right: 2%;
  top: 2%;
  z-index: 1;
}
</style>

<style>
.flex {
  flex: 1;
}
</style>
