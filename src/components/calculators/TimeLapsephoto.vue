<script lang="ts" setup>
import { ref, computed, inject, onMounted } from 'vue'

// 引入全局配置
const { config, write_conf } = inject<any>('app_config')

onMounted(() => {
    // 写入最后打开的页面
    let data_raw = JSON.stringify({
    "lastview": "/calculators/timeLapsephoto"
  })
  let data = JSON.parse(data_raw)
  write_conf(data, 'main')
})

// 计算锁定
const enum Changed {
    videotime,
    interval,
    realtime,
    None
}
let lastchange: Changed = Changed.None

let fps = ref(25) // 帧率
let videotime = ref() // 视频时间
let interval = ref() // 时间间隔
let realtime = ref() // 拍摄时长
let photosize = ref(20) // 单张图片大小

let totalsize = computed(() => {
    return (photosize.value * number.value)
})

let speed = computed(() => {
    return (realtime.value / videotime.value).toFixed(2)
})

// 照片数量
let number = computed(() => {
    return fps.value * videotime.value
})

let videotime_cp = computed({
    get() {
        if (lastchange !== Changed.videotime && lastchange !== Changed.realtime) {
            let t = realtime.value * 60 / (fps.value * interval.value)
            return t
        }
        else return videotime.value
    },
    set(v) {
        lastchange = Changed.videotime
        videotime.value = v
    }
})
let interval_cp = computed({
    get() {
        if (lastchange != Changed.interval) {
            let t = realtime.value * 60 / (fps.value * videotime.value)
            return t
        }
        else return interval.value
    },
    set(v) {
        lastchange = Changed.interval
        interval.value = v
    }
})
let realtime_cp = computed({
    get() {
        if (lastchange != Changed.realtime) {
            let t = (fps.value * videotime.value * interval.value) / 60
            return t
        }
        else return realtime.value
    },
    set(v) {
        lastchange = Changed.realtime
        realtime.value = v
    }
})

// FPS 选项卡
const fpsoption = [
    {
        value: 10,
        label: '10'
    },
    {
        value: 11.988,
        label: ''
    },
    {
        value: 12,
        label: '12'
    },
    {
        value: 12.5,
        label: '12.5'
    },
    {
        value: 14.985,
        label: '14.985'
    },
    {
        value: 15,
        label: '15'
    },
    {
        value: 23.976,
        label: '23.976'
    },
    {
        value: 24,
        label: '24'
    },
    {
        value: 25,
        label: '25'
    },
    {
        value: 29.97,
        label: '29.97'
    },
    {
        value: 30,
        label: '30'
    },
    {
        value: 50,
        label: '50'
    },
    {
        value: 59.94,
        label: '59.94'
    },
    {
        value: 60,
        label: '60'
    },
    {
        value: 72,
        label: '72'
    },
    {
        value: 90,
        label: '90'
    },
    {
        value: 100,
        label: '100'
    },
    {
        value: 119.88,
        label: '119.88'
    },
    {
        value: 120,
        label: '120'
    },
    {
        value: 144,
        label: '144'
    },
    {
        value: 240,
        label: '240'
    },
    {
        value: 400,
        label: '400'
    },
]
</script>

<template>
    <p>帧率：</p>
    <ElSelect v-model="fps" placeholder="FPS">
        <ElOption v-for="item in fpsoption" :key="item.value" :label="item.label" :value="item.value"></ElOption>
    </ElSelect>
    <p>视频时长（秒）：</p>
    <ElInputNumber v-model="videotime_cp"></ElInputNumber>
    <p>拍摄时长（分）：</p>
    <ElInputNumber v-model="realtime_cp"></ElInputNumber>
    <p>单张图片大小（兆）：</p>
    <ElInputNumber v-model="photosize"></ElInputNumber>
    <p>大约占用的空间（兆）：{{ totalsize }}</p>
    <p>拍摄间隔（秒）：{{ interval_cp }}</p>
    <p>照片数量：{{ number }} 张</p>
    <p>倍率：{{ speed }} 倍</p>
</template>

<style></style>
