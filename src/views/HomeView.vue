<script setup lang="ts">
import { inject } from 'vue';
import { useRouter } from 'vue-router';

let cards = [
    {
        index: 1,
        name: 'CsvRand',
        label: 'CR',
        descript: '随机输出CSV文件中的内容',
        path: '/random/csvrand'
    },
    {
        index: 2,
        name: 'TimeLapsephoto',
        label: 'TLP',
        descript: '用于延时摄影的计算器',
        path: '/calculators/timeLapsephoto'
    }
]
const router = useRouter()
let { activeIndex, changeactive } = inject<any>('activeIndex')
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
                                @click="() => { router.push(item.path); changeactive(item.path) }">进入工具</el-button>
                        </div>
                    </template>
                    <p>{{ item.descript }}</p>
                </el-card>
            </el-main>
        </el-container>
    </div>
</template>