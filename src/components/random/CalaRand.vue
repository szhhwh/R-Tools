<script setup lang="ts">
// tauri
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { appConfigDir } from '@tauri-apps/api/path';
import { open } from '@tauri-apps/api/dialog';
// vue
import { ref, onMounted, inject, onUpdated, computed, watch } from 'vue'
// element-plus
import { ElMessage, ElNotification } from 'element-plus'
import { clipboard } from '@tauri-apps/api';
import { Setting } from '@element-plus/icons';
import { Check } from '@element-plus/icons-vue'

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
                name: 'Excel 97-2003 工作簿',
                extensions: ['xls']
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
        get_sheet_names()
        await invoke("reloadlist")
        reset()
    }

// 设置表单
const form = ref({
    cala_path: '',
    cala_list: true,
    cala_animation: false,
    cala_animation_speed: 40,
    antiduble: true,
    lastsheet: ''
})

// 动画速度设置选项
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

let tablename = ref<Sheet[]>([])
interface Sheet {
    value: number,
    label: string
}
async function get_sheet_names() {
    await invoke("return_sheet_names").then(
        (v) => {
            let item = v as string[]
            let newname: Sheet[] = []
            for (let i in item) {
                console.log(i)
                newname.push({
                    value: tablename.value.length + 1,
                    label: item[i]
                })
                tablename.value = newname
            }
        }
    )
}

// init 初始化
onMounted(() => {
    form.value.cala_animation = config.value.cala_animation
    form.value.cala_list = config.value.cala_list
    form.value.cala_animation_speed = config.value.cala_animation_speed
    form.value.cala_path = config.value.cala_path
    form.value.antiduble = config.value.antiduble
    form.value.lastsheet = config.value.lastsheet
    get_sheet_names()
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
    // 读取全局配置
    read_config()
    // 监听器
    list_listen()
    randnum_title_listen()
})
onUpdated(() => {
    read_config()
})

const settingbox = ref(false)
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
            <ElButton :icon="Setting" size="large" @click="settingbox = true">设置</ElButton>
        </el-row>
    </div>

    <ElDrawer v-model="settingbox" title="Cala 随机设置" size="80%">
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
            <ElFormItem label="抽取结果唯一化">
                <el-switch v-model="form.antiduble" active-text="是" inactive-text="否"></el-switch>
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
            <ElFormItem label="选择读取的表">
                <ElSelect v-model="form.lastsheet" :no-data-text="'无可用的表'" :placeholder="'表名称（默认使用第一个表）'">
                    <ElOption v-for="item in tablename" :key="item.value" :label="item.label" :value="item.label">
                    </ElOption>
                </ElSelect>
            </ElFormItem>
            <ElFormItem>
                <ElButton @click="get_sheet_names">重新加载表名称</ElButton>
            </ElFormItem>
        </el-form>
    </ElDrawer>
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
