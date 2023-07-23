<script lang="ts" setup>
// vue
import { inject, onMounted, ref, watch } from 'vue';
// tauri
import { open } from '@tauri-apps/api/dialog';
import { appConfigDir } from '@tauri-apps/api/path';
// element-plus
import { Check } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

// 重新加载cala文件
const reload_cala_path =
    async () => {
        let selected = await open({
            directory: false,
            multiple: false,
            filters: [{
                name: 'Excel 工作簿',
                extensions: ['xlsx']
            },
            {
                name: 'Excel 启用宏的工作簿',
                extensions: ['xlsm']
            },
            {
                name: 'Excel 加载宏',
                extensions: ['xlam']
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
            form.value.cala_path = selected.toString()
            let data: JSON = JSON.parse(JSON.stringify(form.value))
            write_conf(data, 'main')
        }
    }

// 设置表单
const form = ref({
    cala_path: "",
    cala_list: true,
    cala_animation: false,
    cala_animation_speed: 40
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
    form.value.cala_animation = config.value.cala_animation
    form.value.cala_list = config.value.cala_list
    form.value.cala_animation_speed = config.value.cala_animation_speed
    form.value.cala_path = config.value.cala_path
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
            <h1>Cala 随机设置</h1>
        </el-header>
        <el-main>
            <el-form :model="form">
                <el-form-item>
                    <el-input placeholder="excel file path" v-model="form.cala_path" disabled>
                        <template #prepend>excel 文件路径</template>
                    </el-input>
                </el-form-item>
                <ElFormItem>
                    <el-button type="primary" :icon="Check" @click="reload_cala_path">选择 Excel 工作簿</el-button>
                </ElFormItem>
                <ElFormItem label="列表显示">
                    <el-switch v-model="form.cala_list" active-text="打开" inactive-text="关闭"></el-switch>
                </ElFormItem>
                <ElFormItem label="抽取动画">
                    <el-switch v-model="form.cala_animation" active-text="打开" inactive-text="关闭"></el-switch>
                </ElFormItem>
                <ElFormItem label="动画速度">
                    <ElSelect v-model="form.cala_animation_speed">
                        <ElOption v-for="item in speedoption" :key="item.value" :label="item.label" :value="item.value">
                        </ElOption>
                    </ElSelect>
                </ElFormItem>
            </el-form>
        </el-main>
    </el-container>
</template>