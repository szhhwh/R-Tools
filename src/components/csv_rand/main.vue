<script setup lang="ts">
// tauri
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
// vue
import { ref, onMounted } from 'vue'
// element-plus
import { ElNotification } from 'element-plus'

const randnum_title = ref()
const randlist = ref()
const max = ref()
let times = ref(1)
let genbutton = ref(false)

// 抽取按钮
async function getnum() {
    await invoke('init_list').then(
        () => {
            invoke("generate_randnum", { times: times.value })
                .catch((err) => {
                    genbutton.value = true
                    ElNotification({
                        title: 'Info',
                        message: err,
                        type: 'info',
                        position: 'bottom-right'
                    })
                })
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

// Animation 动画切换
let show = ref(true) // 切换list显示动画

// Reset function 重置按钮
async function reset() {
    await invoke('init_list').then(
        () => {
            invoke("return_list_number").then((len) => {
                max.value = len
            })
            invoke("reset") // 调用rust重置函数
            randnum_title.value = "Rand"
            randlist.value = "Hello Rand"
            genbutton.value = false
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

// 下方抽取列表监听器
async function list_listen() {
    const listener = await listen("listoutput", (event: any) => {
        randlist.value = event.payload
    })
}

// 标题监听器
async function randnum_title_listen() {
    const listener = await listen("titleoutput", (event: any) => {
        randnum_title.value = event.payload
    })
}

// init 初始化
onMounted(() => {
    reset()
    list_listen()
    randnum_title_listen()
})
</script>

<template>
    <el-container>
        <el-main>
            <el-row justify="center">
                <el-col>
                    <p id="t-out">{{ randnum_title }}</p>
                </el-col>
                <el-col>
                    <Transition>
                        <p v-if="show" id="l-out">{{ randlist }}</p>
                    </Transition>
                </el-col>
            </el-row>
        </el-main>
        <el-footer>
            <el-row justify="center">
                <el-text>抽取次数</el-text>
                <el-input-number v-model="times" :min="1" :max="max" />
            </el-row>
            <el-row justify="center">
                <el-button size="large" @click="getnum()" :disabled="genbutton">抽取</el-button>
                <el-button size="large" @click="reset()">重置</el-button>
            </el-row>
            <el-row justify="center">
                <el-switch v-model="show" active-text="打开列表显示" inactive-text="关闭列表显示"></el-switch>
            </el-row>
        </el-footer>
    </el-container>
</template>

<style scoped>
#t-out {
    font-size: calc(10vmin);
}

#l-out {
    font-size: calc(2.5vmin);
}

/* Animation Start */
.v-enter-active,
.v-leave-active {
    transition: opacity 0.3s ease;
}

.v-enter-from,
.v-leave-to {
    opacity: 0;
}

/* Animation End */

/* 段落自动换行 */
p {
    word-wrap: break-word;
    word-break: break-all;
}
</style>
