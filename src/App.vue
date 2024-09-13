<script setup lang="ts">
import { House } from '@element-plus/icons-vue'
// Vue
import { inject, provide, ref, watch } from 'vue'
import { useRouter } from 'vue-router'

let activeIndex = ref('/')
function changeactive(newindex: string) {
  console.log(newindex)
  activeIndex.value = newindex
}

provide('activeIndex', { activeIndex, changeactive })

const router = useRouter()
// 通过监控当前router，显示返回首页按钮
watch(router.currentRoute, () => {
  switch (router.currentRoute.value.fullPath) {
    case '/':
    case '/app/setting_center':
    case '/app/update_dialog':
      backhome.value = false
      break
    default:
      backhome.value = true
  }
})

let backhome = ref<boolean>(false)

// 关闭右键菜单
document.addEventListener('contextmenu', e => e.preventDefault())

// 引入全局配置
const { config, write_conf } = inject<any>('app_config')

function backtohome() {
  // 写入最后打开的页面
  let data_raw = JSON.stringify({
    "lastview": "/"
  })
  let data = JSON.parse(data_raw)
  write_conf(data, 'main')
}
</script>

<template>
  <el-container>
    <el-button v-show="backhome" @click="() => { backtohome(); router.push('/') }" :icon="House"
      id="backhome">返回首页</el-button>
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
