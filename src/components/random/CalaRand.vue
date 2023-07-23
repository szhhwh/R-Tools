<script setup lang="ts">
// tauri
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
// vue
import { ref, onMounted, inject, onUpdated, computed } from 'vue'
// element-plus
import { ElMessage, ElNotification } from 'element-plus'
import { clipboard } from '@tauri-apps/api';

const randnum_title = ref()
const randlist = ref()

let title_display = computed(() => {
    if (randnum_title.value == undefined) {
        return "Rand"
    }
    else {
        return randnum_title.value
    }
})

let list_display = computed(() => {
    if (randlist.value == undefined) {
        return "Hello Rand"
    }
    else {
        return randlist.value
    }
})

// 抽取参数
const max = ref()
const times = ref(1)

// state 状态
const getbutton = ref(false) // 抽取按钮状态
const resetbutton = ref(false) // 重置按钮状态
let animation_lock: boolean = false //动画锁定

const Taggles = ref({
    cala_list: true,
    cala_animation: false,
    cala_animation_speed: 40
})

// 动画定时器
let motioninterv: any

const { config, write_conf } = inject<any>('app_config')

// 抽取按钮
async function getnum() {
    await invoke('init_list').then(
        () => {
            if (Taggles.value.cala_animation == true && animation_lock === false) {
                animation_lock = true
                resetbutton.value = true
                motioninterv = setInterval(() => {
                    invoke("return_randresult")
                }, Taggles.value.cala_animation_speed)
            }
            else if (Taggles.value.cala_animation == true && animation_lock === true) {
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
            else if (Taggles.value.cala_animation == false && animation_lock === false) {
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
            randnum_title.value = undefined
            randlist.value = undefined
            getbutton.value = false
            ElMessage({
                message: '已重置',
                type: 'success'
            })
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
    Taggles.value.cala_animation = config.value.cala_animation
    Taggles.value.cala_list = config.value.cala_list
    Taggles.value.cala_animation_speed = config.value.cala_animation_speed
}

function copyresult() {
    clipboard.writeText(randlist.value).then(() => {
        ElMessage({
            message: '结果已复制',
            type: 'info',
            grouping: true
        })
    })
}

// init 初始化
onMounted(() => {
    // 读取全局配置
    read_config()
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
                <p id="t-out">{{ title_display }}</p>
            </el-col>
            <el-col>
                <Transition>
                    <p v-if="Taggles.cala_list" id="l-out">{{ list_display }}</p>
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
            <el-button size="large" @click="getnum" :disabled="getbutton">抽取</el-button>
            <el-popconfirm confirm-button-text="是" cancel-button-text="否" title="是否重置" @confirm="reset">
                <template #reference>
                    <el-button size="large">重置</el-button>
                </template>
            </el-popconfirm>
            <ElButton size="large" @click="copyresult">复制结果</ElButton>
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
