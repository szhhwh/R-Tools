<script lang="ts" setup>
import MkRender from '../components/app/MkRender.vue';
import { invoke } from '@tauri-apps/api/tauri';
import { ElNotification } from 'element-plus';
import { onMounted, ref } from 'vue';

const update_dialog: string = `
## v0.5.0
- 新增“更新日志”页面

## v0.4.6
- 修复工作簿中第一个表的名称不为“Sheet1”时闪退的情况
- 增加反重复开关
- 添加log输出至.rtools目录
- 增加excel工作簿中的表选择功能
- 将calarand设置界面移动回主界面
- 为延时摄影计算器增加照片容量计算
- 使用新的导航
- 优化calarand交互
`
const version = ref()

async function get_version() {
    await invoke("return_version").then((v) => {
        version.value = v
    }).catch((err) => {
        ElNotification({
            title: '错误',
            message: err,
            type: 'error',
            position: 'bottom-right'
        })
    })
}

onMounted(() => {
    get_version()
})
</script>

<template>
    <h1 style="display: flex;justify-content: center;">R-tools 更新日志</h1>
    <h4>本应用程序版本: v{{ version }}</h4>
    <MkRender :text="update_dialog"></MkRender>
</template>

<style scoped></style>
