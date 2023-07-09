<script lang="ts" setup>
// vue
import { inject, onMounted, ref, watch } from 'vue';
// tauri
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
            ElMessage({
                message: "未选择文件",
                type: 'warning'
            })
        } else {
            // user selected a single file
            console.log("Selected File path: ", selected)
            form.value.csv_path = selected.toString()
            let data: JSON = JSON.parse(JSON.stringify(form.value))
            write_conf(data, 'main')
        }
    }

// 设置表单
const form = ref({
    csv_path: "",
    csv_list: true,
    csv_animation: false,
    csv_animation_speed: 40
})

const speedoption = [
    {
        value: 40,
        label: "快 Fast 40ms"
    },
    {
        value: 60,
        label: "适中 Midium 60ms"
    },
    {
        value: 80,
        label: "慢 Slow 80ms"
    }
]

// 注入全局配置
const { config, write_conf } = inject<any>('app_config')

onMounted(() => {
    form.value.csv_animation = config.value.csv_animation
    form.value.csv_list = config.value.csv_list
    form.value.csv_animation_speed = config.value.csv_animation_speed
    form.value.csv_path = config.value.csv_path
    watch(form.value, async () => {
        let data = JSON.parse(JSON.stringify(form.value))
        await write_conf(data, 'main').then(
            ElMessage({
                message: '设置已更新',
                type: 'success',
                grouping: true
            })
        )
    })
})
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
                <ElFormItem>
                    <el-button type="primary" :icon="Check" @click="reload_csv_path">选择CSV文件</el-button>
                </ElFormItem>
                <ElFormItem label="列表显示">
                    <el-switch v-model="form.csv_list" active-text="打开" inactive-text="关闭"></el-switch>
                </ElFormItem>
                <ElFormItem label="抽取动画">
                    <el-switch v-model="form.csv_animation" active-text="打开" inactive-text="关闭"></el-switch>
                </ElFormItem>
                <ElFormItem label="动画速度">
                    <ElSelect v-model="form.csv_animation_speed">
                        <ElOption v-for="item in speedoption" :key="item.value" :label="item.label" :value="item.value">
                        </ElOption>
                    </ElSelect>
                </ElFormItem>
            </el-form>
        </el-main>
    </el-container>
</template>