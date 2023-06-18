<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
//bootstrap
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap/dist/js/bootstrap.js";
import { reactive, ref, watch } from 'vue';

const randnum_title = ref();
const randlist = ref();
const max_times = ref();

let times: number = 1;

// Reset function
async function reset() {
    max_times.value = await invoke("return_list_number");
    invoke("reset")
    randnum_title.value = "Rand";
    randlist.value = "Hello Rand"
}

// 抽取按钮
async function getnum() {
    randlist.value = await invoke("generate_randnum", { times: times });
    randnum_title.value = await invoke("return_last_result");
}

// Animation
var show = ref(true);
const tweened = reactive({
    number: 0
})

//init
reset()
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
        <input type="number" min="1" :max="max_times" placeholder="抽取次数" class="text-center" id="frequency"
            v-model="times" />
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
