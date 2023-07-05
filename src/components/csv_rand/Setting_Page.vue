<script lang="ts" setup>
// vue
import { inject, onMounted, reactive, ref } from 'vue';
// tauri
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { appConfigDir } from '@tauri-apps/api/path';
// element-plus
import { Check } from '@element-plus/icons-vue'
import { ElMessage, ElNotification } from 'element-plus'

// 重新加载CSV文件
const reload_csv_path =
    async () => {
        let selected = await open({
            directory: false,
            multiple: false,
            filters: [{
                name: 'CSV File',
                extensions: ['csv', 'CSV']
            }],
            defaultPath: await appConfigDir(),
        });

        if (selected === null) {
            // user cancelled the selection
            ElNotification({
                title: '错误',
                message: '未选择文件',
                type: 'error',
                position: 'bottom-right'
            })
        } else {
            // user selected a single file
            console.log("选择的文件路径", selected)
            form.value = { csv_path: selected.toString() }
            ElMessage({
                message: '成功获取CSV文件路径',
                type: 'success'
            })
            let data = JSON.parse(JSON.stringify(form))
            console.log(data)
            await invoke('save_config', { data: data, label: 'main' })
            getconfig()
        }
    }

const form = ref({
    csv_path: ""
})

function getconfig() {
    let config: any = inject('app_config')
    form.value = { csv_path: config.value.csv_path }
}

onMounted(
    () => {
        getconfig()
    }
)
</script>

<template>
    <el-container class="container">
        <el-header>
            <h1>CSV 随机设置</h1>
        </el-header>
        <el-main>
            <el-form :model="form">
                <el-form-item>
                    <el-input placeholder="CSV file path" v-model="form.csv_path" disabled>
                        <template #prepend>CSV 文件路径</template>
                    </el-input>
                </el-form-item>
            </el-form>
            <el-button type="primary" :icon="Check" @click="reload_csv_path">选择CSV文件</el-button>
        </el-main>
    </el-container>
</template>
