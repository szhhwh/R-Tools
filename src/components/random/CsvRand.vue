<script setup lang="ts">
// tauri
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
// vue
import { ref, onMounted, inject, onUpdated } from 'vue'
// element-plus
import { ElNotification } from 'element-plus'
import { clipboard } from '@tauri-apps/api';

const randnum_title = ref()
const randlist = ref()

// 抽取参数
const max = ref()
const times = ref(1)

// state 状态
const getbutton = ref(false) // 抽取按钮状态
const resetbutton = ref(false) // 重置按钮状态
let animation_lock: boolean = false //动画锁定

const Taggles = ref({
    csv_list: true,
    csv_animation: false,
    csv_animation_speed: 40
})

// 动画定时器
let motioninterv: any

const { config, write_conf } = inject<any>('app_config')

// 抽取按钮
async function getnum() {
    await invoke('init_list').then(
        () => {
            if (Taggles.value.csv_animation == true && animation_lock === false) {
                animation_lock = true
                resetbutton.value = true
                motioninterv = setInterval(() => {
                    invoke("return_randresult")
                }, Taggles.value.csv_animation_speed)
            }
            else if (Taggles.value.csv_animation == true && animation_lock === true) {
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
            else if (Taggles.value.csv_animation == false && animation_lock === false) {
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

function read_config() {
    Taggles.value.csv_animation = config.value.csv_animation
    Taggles.value.csv_list = config.value.csv_list
    Taggles.value.csv_animation_speed = config.value.csv_animation_speed
}

// init 初始化
onMounted(() => {
    // 读取全局配置
    read_config()
    // 重置
    reset()
    // 监听器
    list_listen()
    randnum_title_listen()
})
onUpdated(() => {
    read_config()
})
</script>

<template>
    <div class="main">
        <el-row justify="center">
            <el-col>
                <p id="t-out">{{ randnum_title }}</p>
            </el-col>
            <el-col>
                <Transition>
                    <p v-if="Taggles.csv_list" id="l-out">{{ randlist }}</p>
                </Transition>
            </el-col>
        </el-row>
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
            <ElButton size="large" @click="() => { clipboard.writeText(randlist) }">复制结果</ElButton>
        </el-row>
    </div>
</template>

<style scoped>
.main {
    flex-direction: column;
    justify-content: center;
    text-align: center;
}

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
