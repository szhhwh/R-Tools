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

// plashscreen
document.addEventListener('DOMContentLoaded', () => {
  invoke('close_splashscreen')
})

const app = createApp(App)

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

const config = ref()
async function get_config() {
  await invoke('return_config').catch((err) => {
    ElNotification({
      title: '错误',
      type: 'error',
      message: err,
      position: 'bottom-right'
    })
  }).then((v) => {
    config.value = v
  })
}
// 写入配置
async function write_conf(data: JSON, label: String) {
  await invoke('save_config', { data: data, label: label })
  await get_config()
}
get_config()

app.use(ElementPlus)
app.use(router)
app.provide('app_config', { config, write_conf })
app.mount('#app')