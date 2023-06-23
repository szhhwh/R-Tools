import { createApp } from "vue"
import "./styles.css"
import App from "./App.vue"
import router from "./router"
import { invoke } from "@tauri-apps/api/tauri"

import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

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

app.use(ElementPlus)

app.use(router)

app.mount('#app')
