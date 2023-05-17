<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap/dist/js/bootstrap.js";
import { ref } from 'vue';

const randnum = ref<number>();
let recordtext = "学号抽取";

let min: number = 1, max: number = 64, tiems: number = 1;
let arry = [];

function resest() {
    recordtext = "学号抽取";
    arry = [];
}

// Call Rust to give random number
async function getnum() {
    randnum.value = await invoke("generate_randnum", {min: min, max: max});
    
}
</script>

<template>
    <div class="main container-fluid">
        <div class="text-center row">
            <p class="display-1" id="text">{{ randnum + recordtext }}</p>
            <p id="record" class="h4">已抽取学号</p>
        </div>
        <div class="num row">
            <div class="col-4">
                <p>最小值</p>
                <input type="number" placeholder="最小数字" class="text-center" id="min" v-model="min" />
            </div>
            <div class="col-4">
                <p>最大值</p>
                <input type="number" placeholder="最大数字" class="text-center" id="max" v-model="max"/>
            </div>
            <div class="col-4">
                <p>抽取次数</p>
                <input type="number" max="56" placeholder="抽取次数" class="text-center" id="frequency" v-model="tiems" />
            </div>
        </div>
        <div>
            <button @click="getnum()">抽取</button>
            <button onclick="reset()">重置</button>
        </div>
    </div>
</template>

<style scoped></style>