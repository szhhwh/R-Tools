import { createApp } from "vue"
import "./styles.css"
import App from "./App.vue"
import router from "./router"
import { invoke } from "@tauri-apps/api/tauri"

document.addEventListener('DOMContentLoaded', () => {
    invoke('close_splashscreen')
})

const app = createApp(App)

app.use(router)

app.mount('#app')
