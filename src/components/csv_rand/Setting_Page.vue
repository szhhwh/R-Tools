<script lang="ts" setup>
// vue
import { ref, onMounted, watch } from 'vue';
// tauri
import { invoke } from '@tauri-apps/api/tauri';
// element-plus
import { Check } from '@element-plus/icons-vue'
import { ElMessage, ElNotification } from 'element-plus'

const csvpath = ref()
async function get_csv_path() {
    await invoke("return_csv_path").then(
        (path) => {
            csvpath.value = path
        }
    ).catch(
        (err) => {
            ElNotification({
                title: 'Error',
                message: err,
                type: 'error',
                position: 'bottom-right'
            })
        }
    )
}

// 重新加载CSV文件
async function reload_csv_path() {
    await invoke("reload_csv_path").then(
        (path) => {
            ElMessage({
                message: '当前csv路径: ' + path,
                type: 'success',
            })
            get_csv_path()
        }
    ).catch(
        (err) => {
            ElNotification({
                title: 'Error',
                message: err,
                type: 'error',
                position: 'bottom-right'
            })
        }
    )
}

onMounted(
    () => {
        get_csv_path()
    }
)
</script>

<template>
    <el-container class="container">
        <el-header>
            <h1>CSV 随机设置</h1>
        </el-header>
        <el-main>
            <el-row :gutter="5">
                <el-col :span="20">
                    <el-input placeholder="csv file path" v-model="csvpath" disabled>
                        <template #prepend>CSV 文件路径</template>
                    </el-input>
                </el-col>
                <el-col :span="4">
                    <el-button type="primary" :icon="Check" @click="reload_csv_path()">重新选择CSV文件</el-button>
                </el-col>
            </el-row>
        </el-main>
    </el-container>
</template>

<style scoped></style>