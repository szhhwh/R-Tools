<script setup lang="ts">
import { inject, onMounted } from 'vue'
import { useRouter } from 'vue-router'

let cards = [
    {
        index: 1,
        name: 'Calarand',
        label: 'CR',
        descript: '随机输出excel文件中的内容',
        path: '/random/calarand',
    },
    {
        index: 2,
        name: 'TimeLapsephoto',
        label: 'TLP',
        descript: '用于延时摄影的计算器',
        path: '/calculators/timeLapsephoto',
    }
]
const router = useRouter()
const { activeIndex, changeactive } = inject<any>('activeIndex')

// 引入全局配置
const { config, write_conf } = inject<any>('app_config')

function writelastview(view: string) {
    // 写入最后打开的页面
    let data_raw = JSON.stringify({
    "lastview": view
  })
    let data = JSON.parse(data_raw)
    write_conf(data, 'main')
}
</script>

<template>
    <div>
        <el-container>
            <el-main>
                <el-card v-for="item in cards" :key="item.label">
                    <template #header>
                        <div style="display: flex;">
                            <span>{{ item.name }}</span>
                            <div class="flex"></div>
                            <el-button text bg
                                @click="() => { router.push(item.path); changeactive(item.path); writelastview(item.path) }">进入工具</el-button>
                        </div>
                    </template>
                    <p>{{ item.descript }}</p>
                </el-card>
            </el-main>
        </el-container>
    </div>
</template>