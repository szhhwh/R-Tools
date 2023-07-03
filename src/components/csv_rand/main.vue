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

// 抽取参数
const max = ref()
const times = ref(1)

// Taggle 切换器
const listshow = ref(true) // 切换list显示状态
const animation = ref(false) // 切换动画使用
const getbutton = ref(false) // 抽取按钮状态
const resetbutton = ref(false) // 重置按钮状态
let animation_lock: boolean = false //动画锁定

// 动画定时器
let motioninterv: any
// 动画速度
const speed = ref(40)

// 抽取按钮
async function getnum() {
    await invoke('init_list').then(
        () => {
            if (animation.value == true && animation_lock === false) {
                animation_lock = true
                resetbutton.value = true
                motioninterv = setInterval(() => {
                    invoke("return_randresult")
                }, speed.value)
            }
            else if (animation.value == true && animation_lock === true) {
                animation_lock = false
                clearInterval(motioninterv)
                invoke("generate_randnum", { times: times.value })
                    .catch((err) => {
                        getbutton.value = true
                        ElNotification({
                            title: '信息',
                            message: err,
                            type: 'info',
                            position: 'bottom-right'
                        })
                    })
                resetbutton.value = false
            }
            else if (animation.value == false && animation_lock === false) {
                invoke("generate_randnum", { times: times.value })
                    .catch((err) => {
                        getbutton.value = true
                        ElNotification({
                            title: '信息',
                            message: err,
                            type: 'info',
                            position: 'bottom-right'
                        })
                    })
            }
        }
    ).catch(
        (err) => {
            ElNotification({
                title: '错误',
                message: err,
                type: 'error',
                position: 'bottom-right'
            })
        }
    )
}

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
            getbutton.value = false
        }
    ).catch(
        (err) => {
            ElNotification({
                title: '错误',
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
                        <p v-if="listshow" id="l-out">{{ randlist }}</p>
                    </Transition>
                </el-col>
            </el-row>
        </el-main>
        <el-footer>
            <el-row justify="center">
                <el-col>
                    <el-text>抽取次数</el-text>
                </el-col>
                <el-col :span="12">
                    <el-slider v-model="times" show-input :min="1" :max="max" />
                </el-col>
            </el-row>
            <el-row justify="center">
                <el-button size="large" @click="getnum()" :disabled="getbutton">抽取</el-button>
                <el-button size="large" @click="reset()" :disabled="resetbutton">重置</el-button>
            </el-row>
            <el-row justify="center">
                <el-col>
                    <el-switch v-model="listshow" active-text="打开列表显示" inactive-text="关闭列表显示"></el-switch>
                </el-col>
                <el-col>
                    <el-switch v-model="animation" active-text="打开动画" inactive-text="关闭动画"></el-switch>
                </el-col>
                <el-col :span="12">
                    <ElText>动画间隔 (单位:ms)</ElText><el-slider v-model="speed" show-input :min="40" :max="100" />
                </el-col>
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
