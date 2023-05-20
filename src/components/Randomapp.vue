<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
//bootstrap
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap/dist/js/bootstrap.js";
import { reactive, ref, watch } from 'vue';
//Gsap Animation
import { gsap } from 'gsap';

const randnum = ref();
const output = ref();

let min: number = 1, max: number = 64, times: number = 1;
let record: number[] = [];

//Reset Func
function reset() {
    randnum.value = "Rand";
    record = [];
}

//Give Randnumber
async function getnum() {
    if (record.length < (max - min)) {
        for (var i = 0; i < times; i++) {
            do {
                randnum.value = await invoke("generate_randnum", { min: min, max: max });
            } while (record.includes(randnum.value as number))
            record.push(randnum.value as number)
        }
    }
}

watch(randnum, () => {
    if (record.length != 0) {
        output.value = record.toString();
    }
    else {
        output.value = "Hello Rand"
    }
})

//Animation
var show = ref(true);
const tweened = reactive({
    number: 0
})

watch(randnum, (n) => {
    gsap.to(tweened, { duration: 0.5, number: Number(n) || 0 })
})

//init
reset()
</script>

<template>
    <div class="main container-fluid">
        <div class="text-center row">
            <p class="display-1" id="text">{{ tweened.number.toFixed(0) }}</p>
            <Transition>
                <p v-if="show" class="h4">{{ output }}</p>
            </Transition>
            <!-- <p id="record" class="h4">{{ output }}</p> -->
        </div>
        <div class="num row">
            <div class="col-4">
                <p>最小值</p>
                <input type="number" placeholder="最小数字" class="text-center" id="min" v-model="min" />
            </div>
            <div class="col-4">
                <p>最大值</p>
                <input type="number" placeholder="最大数字" class="text-center" id="max" v-model="max" />
            </div>
            <div class="col-4">
                <p>抽取次数</p>
                <input type="number" max="56" placeholder="抽取次数" class="text-center" id="frequency" v-model="times" />
            </div>
        </div>
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
    transition: opacity 0.5s ease;
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

.num {
    margin: 20px 0;
    display: flex;
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
</style>