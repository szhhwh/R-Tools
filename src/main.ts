import { createApp, ref } from "vue"
import "./styles.css"
import App from "./App.vue"
import router from "./router"
import { invoke } from "@tauri-apps/api/tauri"
// element-plus
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import { ElNotification } from 'element-plus'
// 注册 Element icons
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

// splashscreen
document.addEventListener('DOMContentLoaded', () => {
  invoke('close_splashscreen')
})

const app = createApp(App)

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

// 配置读取函数
const config = ref()
async function get_config() {
  await invoke('return_config').catch((err) => {
    ElNotification({
      title: '错误',
      type: 'error',
      message: err,
      position: 'bottom-right'
    })
  }).then((conf) => {
    config.value = conf
  })
}

// 写入配置
async function write_conf(data: JSON, label: String) {
  await invoke('save_config', { data: data, label: label })
  await get_config()
}

// 从后端拉取配置
get_config()

// vue_plugins_load
app.use(ElementPlus)
app.use(router)
// 向子级组件提供配置文件变量和相关函数
app.provide('app_config', { config, write_conf })
app.mount('#app')