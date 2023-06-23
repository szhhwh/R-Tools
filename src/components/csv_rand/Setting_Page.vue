<script lang="ts" setup>
// vue
import { ref, onMounted, watch } from 'vue';
// tauri
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';

import { Check } from '@element-plus/icons-vue'

const csvpath = ref()

async function path_listen() {
    const listener = await listen("csv_path", (event: any) => {
        csvpath.value = event.payload
    })
}

await function save_path() {
    
}

</script>

<template>
    <el-container class="container">
        <el-header>
            <h1>CSV 随机设置</h1>
        </el-header>
        <el-main>
            <el-input placeholder="Please input file path" v-model="csvpath"><template #prepend>CSV 文件路径</template></el-input>
            <el-button type="primary" :icon="Check">确认</el-button>
        </el-main>
    </el-container>
</template>

<style scoped>
</style>