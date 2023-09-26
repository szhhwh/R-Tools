<script setup lang="ts">
// tauri
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { appConfigDir } from '@tauri-apps/api/path';
import { open } from '@tauri-apps/api/dialog';
// vue
import { ref, onMounted, inject, onUpdated, computed, watch } from 'vue'
// element-plus
import { Action, ElMessage, ElMessageBox, ElNotification } from 'element-plus'
import { clipboard } from '@tauri-apps/api';
import { Setting } from '@element-plus/icons';
import { Check } from '@element-plus/icons-vue'

// 引入全局配置
const { config, write_conf } = inject<any>('app_config')

// 抽取显示开始
// 
const randnum_title = ref() // 抽取结果
const randlist = ref() // 抽取结果列表

// 抽取结果显示
let title_display = computed(() => {
    if (randnum_title.value == undefined) {
        return "Rand"
    }
    else {
        return randnum_title.value
    }
})

// 抽取结果列表
let list_display = computed(() => {
    if (randlist.value == undefined) {
        return "Hello Rand"
    }
    else {
        return randlist.value
    }
})

// 标题监听器
async function randnum_title_listen() {
    const listener = await listen("titleoutput", (event: any) => {
        randnum_title.value = event.payload
    })
}

// 抽取列表监听器
async function list_listen() {
    const listener = await listen("listoutput", (event: any) => {
        randlist.value = event.payload
    })
}
// 
// 抽取显示结束

// 抽取参数
const max = ref()
const times = ref(1)

// 按钮状态控制
const getbutton = ref(false) // 抽取按钮状态，false为开启抽取按钮
const resetbutton = ref(true) // 重置按钮状态，false为开启重置按钮

const Taggles = ref({
    cala_list: true,
    cala_animation: false,
    cala_animation_speed: 40
})

// 动画定时器
let motioninterv: any
// 动画锁定，false为关闭锁定
let animation_lock: boolean = false

// 抽取按钮
async function getnum() {
    await invoke('cala_is_exist').then(
        () => {
            if (Taggles.value.cala_animation == true && animation_lock === false) {
                // 开启动画时进入该判断支
                animation_lock = true // 打开动画锁
                resetbutton.value = true // 关闭重置按钮
                motioninterv = setInterval(() => {
                    invoke("return_randresult")
                }, Taggles.value.cala_animation_speed)
            }
            else if (Taggles.value.cala_animation == true && animation_lock === true) {
                // 开启动画并点击了一次抽取按钮后进入该判断支
                animation_lock = false // 关闭动画锁
                clearInterval(motioninterv) // 停止动画
                // 调用生成函数
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
                resetbutton.value = false // 开启重置按钮
            }
            else if (Taggles.value.cala_animation == false && animation_lock === false) {
                // 未开启动画时进入该判断支
                invoke("generate_randnum", { times: times.value })
                    .then(() => {
                        resetbutton.value = false // 开启重置按钮
                    })
                    .catch((err) => {
                        getbutton.value = true // 关闭抽取按钮
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

// 重置按钮开始
// 
// 先判断excel文件路径是否有效，如文件路径无效则报错并重置界面显示
async function reset() {
    await invoke('cala_is_exist').then(
        () => {
            // 重新读取元素个数
            invoke("return_list_number").then((len) => {
                max.value = len
            })
            // 重置rust计数器
            invoke("reset_counter")
            // 重置界面显示
            randnum_title.value = undefined
            randlist.value = undefined
            ElMessage({
                message: '已重置抽取器',
                type: 'success'
            })
        }
    ).catch(
        (err) => {
            // 重置界面显示
            randnum_title.value = undefined
            randlist.value = undefined
            // 重置按钮的错误处理
            ElNotification({
                title: '错误',
                message: '因 ' + err + ' 仅重置界面显示',
                type: 'error',
                position: 'bottom-right'
            })
        }
    )
    // 关闭重置按钮
    resetbutton.value = true
    // 打开抽取按钮
    getbutton.value = false
}

// 确认重置界面框
const confirm_reset = () => {
    ElMessageBox.confirm(
        '抽取结果将会被永久删除，确认吗？',
        '重置确认',
        {
            confirmButtonText: '确认',
            cancelButtonText: '取消',
            type: 'warning',
        }
    ).then(() => {
        reset()
    })
}
// 
// 重置按钮结束

// 复制抽取结果至剪切板
function copyresult() {
    clipboard.writeText(randlist.value).then(() => {
        ElMessage({
            message: '结果已复制',
            type: 'info',
            grouping: true
        })
    })
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

// 表选择相关
let tablename = ref<Sheet[]>([]) // 包含的表数组
interface Sheet {
    value: number,
    label: string
}
async function get_sheet_names(): Promise<void> {
    await invoke("return_sheet_names").then(
        (v) => {
            let item = v as string[]
            let newname: Sheet[] = []
            for (let i in item) {
                newname.push({
                    value: tablename.value.length + 1,
                    label: item[i]
                })
                tablename.value = newname
            }
        }
    )
}

// 重新加载excel文件
const reload_excel_path =
    async () => {
        // 选择器
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
            // 用户取消选择
            ElMessage({
                message: "未选择文件",
                type: 'warning'
            })
        } else {
            // 用户选择单个文件
            console.log("Selected File path: ", selected)
            form.value.cala_path = selected.toString()
        }
        get_sheet_names()
    }

// 从全局配置加载设置
function loadsettings(): void {
    form.value.cala_animation = config.value.cala_animation
    form.value.cala_list = config.value.cala_list
    form.value.cala_animation_speed = config.value.cala_animation_speed
    form.value.cala_path = config.value.cala_path
    form.value.antiduble = config.value.antiduble
    form.value.lastsheet = config.value.lastsheet
    get_sheet_names()
}

function read_config() {
    Taggles.value.cala_animation = config.value.cala_animation
    Taggles.value.cala_list = config.value.cala_list
    Taggles.value.cala_animation_speed = config.value.cala_animation_speed
}

// 保存设置
async function save_setting() {
    // 将设置内容解析成为json对象
    let data = JSON.parse(JSON.stringify(form.value))
    await write_conf(data, 'main').then(
        ElMessage({
            message: '设置已更新',
            type: 'success',
            grouping: true
        })
    )
    await invoke("reloadlist")
    reset()
}

// 设置抽屉开启状态，true为开启抽屉
const settingbox = ref(false)

// 是否修改设置，false为未修改
const setting_change = ref(false)
// 不保存设置锁定，防止侦听器修改setting_change状态
const cancel_lock = ref(false)

// 处理设置抽屉关闭事件
const handle_close = () => {
    // 判断是否对设置进行了修改
    if (setting_change.value === true) {
        ElMessageBox.confirm(
            '是否保存设置',
            '退出设置',
            {
                confirmButtonText: '保存',
                cancelButtonText: '不保存',
                distinguishCancelAndClose: true
            }
        ).then(() => {
            save_setting()
            settingbox.value = false // 关闭设置页面
            setting_change.value = false // 重置设置修改状态
        }).catch((action: Action) => {
            if (action === 'cancel') {
                cancel_lock.value = true // 打开取消锁定
                loadsettings()
                read_config()
                settingbox.value = false // 关闭设置页面
                console.log(setting_change.value)
                setting_change.value = false // 重置设置修改状态
                ElMessage({
                    message: '设置未保存',
                    type: 'info',
                })
            }
        })

    }
    else {
        // 如未修改设置可直接退出设置界面
        settingbox.value = false
    }
}

// 初始化
onMounted(() => {
    // 读取全局配置
    loadsettings()
    read_config()
    // 挂载返回值监听器
    list_listen()
    randnum_title_listen()
    // 监听设置修改
    watch(form.value, () => {
        if (cancel_lock.value === false) {
            setting_change.value = true
        }
    })
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
            <el-button size="large" @click="confirm_reset" :disabled="resetbutton">重置</el-button>
            <ElButton size="large" @click="copyresult">复制结果</ElButton>
            <ElButton :icon="Setting" size="large" @click="() => { settingbox = true, cancel_lock = false }">设置</ElButton>
        </el-row>
    </div>

    <ElDrawer v-model="settingbox" title="Cala 随机设置" size="80%" :before-close="handle_close">
        <el-form :model="form">
            <el-form-item>
                <el-input placeholder="excel file path" v-model="form.cala_path" disabled>
                    <template #prepend>excel 文件路径</template>
                </el-input>
            </el-form-item>
            <ElFormItem>
                <el-button type="primary" :icon="Check" @click="reload_excel_path">选择 Excel 工作簿</el-button>
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
