<script setup lang="ts">
// tauri
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
// bootstrap
import "bootstrap/dist/css/bootstrap.css"
import "bootstrap/dist/js/bootstrap.js"
// vue
import { ref, onMounted } from 'vue'

const randnum_title = ref()
const randlist = ref()
const max_times = ref()
let times: number = 1

// 抽取按钮
async function getnum() {
    await invoke("generate_randnum", { times: times })
}

// Animation 动画监听
let show = ref(true) // 切换list显示动画

// Reset function 重置按钮
async function reset() {
    max_times.value = await invoke("return_list_number")
    invoke("reset") // 调用rust重置函数
    randnum_title.value = "Rand"
    randlist.value = "Hello Rand"
}

// 下方抽取列表监听器
async function list_listen() {
    const listener = await listen("listoutput", (event: any) => {
        randlist.value = event.payload
    })
}

// 标题监听器
async function randnum_title_listen() {
    const listener = await listen("titleoutput", (event: any) => {
        randnum_title.value = event.payload
    })
}

// init 初始化
onMounted(() => {
    reset()
    list_listen()
    randnum_title_listen()
})
</script>

<template>
    <div class="main container-fluid">
        <div class="text-center row">
            <p class="display-1">{{ randnum_title }}</p>
            <Transition>
                <p v-if="show" class="h3" id="output">{{ randlist }}</p>
            </Transition>
        </div>
        <p>抽取次数</p>
        <input type="number" min="1" :max="max_times" placeholder="抽取次数" class="text-center" v-model="times" />
        <div>
            <button @click="getnum()">抽取</button>
            <button @click="reset()">重置</button>
            <button @click="show = !show">显示已抽取</button>
        </div>
    </div>
</template>

<style scoped>
/* Animation */
.v-enter-active,
.v-leave-active {
    transition: opacity 0.3s ease;
}

.v-enter-from,
.v-leave-to {
    opacity: 0;
}

/* Main css Start */
.main {
    height: 70vh;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
}

input {
    border-radius: 2px;
}

button {
    width: 140px;
    height: 50px;
    border-radius: 5px;
    background-color: white;
    transition: background-color 0.3s;
}

button:hover {
    background-color: rgba(138, 138, 138, 0.233);

    transition: background-color 0.3s;
}

p {
    word-wrap: break-word;
    word-break: break-all;
}
</style>
