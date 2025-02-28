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
import { Setting, Check, CopyDocument, RefreshRight, Select } from '@element-plus/icons-vue'

// 引入全局配置
const { config, write_conf } = inject<any>('app_config')

// 抽取显示开始
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
  <div class="cala-rand-container">
    <el-card class="app-card main-card" shadow="hover">
      <template #header>
        <div class="card-header">
          <el-icon class="header-icon"><Select /></el-icon>
          <span>随机抽取</span>
        </div>
      </template>
      
      <div class="result-display">
        <transition name="slide-fade">
          <p id="t-out">{{ title_display }}</p>
        </transition>
        
        <transition name="fade">
          <p v-if="Taggles.cala_list" id="l-out">{{ list_display }}</p>
        </transition>
      </div>
      
      <el-divider />
      
      <div class="control-panel">
        <div class="slider-container">
          <el-text class="slider-label">抽取次数</el-text>
          <el-slider 
            v-model="times" 
            show-input 
            :min="1" 
            :max="max" 
            class="times-slider"
          />
        </div>
        
        <div class="action-buttons">
          <el-button 
            type="primary" 
            size="large" 
            @click="getnum" 
            :disabled="getbutton"
            class="action-button"
          >
            <el-icon><Select /></el-icon>
            抽取
          </el-button>
          
          <el-button 
            type="warning" 
            size="large" 
            @click="confirm_reset" 
            :disabled="resetbutton"
            class="action-button"
          >
            <el-icon><RefreshRight /></el-icon>
            重置
          </el-button>
          
          <el-button 
            size="large" 
            @click="copyresult"
            class="action-button"
          >
            <el-icon><CopyDocument /></el-icon>
            复制结果
          </el-button>
          
          <el-button 
            size="large" 
            @click="() => { settingbox = true, cancel_lock = false }"
            class="action-button"
          >
            <el-icon><Setting /></el-icon>
            设置
          </el-button>
        </div>
      </div>
    </el-card>

    <el-drawer
      v-model="settingbox"
      title="抽取设置"
      size="80%"
      :before-close="handle_close"
      class="settings-drawer"
      destroy-on-close
    >
      <el-card class="settings-card" shadow="never">
        <template #header>
          <div class="card-header">
            <el-icon class="header-icon"><Setting /></el-icon>
            <span>Cala 随机设置</span>
          </div>
        </template>
        
        <el-form :model="form" label-position="top">
          <div class="form-section">
            <h4 class="section-title">Excel 文件设置</h4>
            
            <el-form-item>
              <el-input 
                placeholder="excel file path" 
                v-model="form.cala_path" 
                disabled
                class="path-input"
              >
                <template #prepend>excel 文件路径</template>
              </el-input>
            </el-form-item>
            
            <el-form-item>
              <el-button 
                type="primary" 
                :icon="Check" 
                @click="reload_excel_path"
                class="select-file-btn"
              >
                选择 Excel 工作簿
              </el-button>
            </el-form-item>
            
            <el-form-item label="选择读取的表">
              <div class="sheet-selection">
                <el-select 
                  v-model="form.lastsheet" 
                  :no-data-text="'无可用的表'" 
                  :placeholder="'表名称（默认使用第一个表）'"
                  class="sheet-select"
                >
                  <el-option 
                    v-for="item in tablename" 
                    :key="item.value" 
                    :label="item.label" 
                    :value="item.label"
                  />
                </el-select>
                
                <el-button 
                  @click="get_sheet_names"
                  class="reload-btn"
                >
                  重新加载表名称
                </el-button>
              </div>
            </el-form-item>
          </div>
          
          <div class="form-section">
            <h4 class="section-title">显示设置</h4>
            
            <el-form-item label="列表显示" class="switch-item">
              <el-switch 
                v-model="form.cala_list" 
                active-text="打开" 
                inactive-text="关闭"
              />
            </el-form-item>
            
            <el-form-item label="抽取结果唯一化" class="switch-item">
              <el-switch 
                v-model="form.antiduble" 
                active-text="是" 
                inactive-text="否"
              />
            </el-form-item>
          </div>
          
          <div class="form-section">
            <h4 class="section-title">动画设置</h4>
            
            <el-form-item label="抽取动画" class="switch-item">
              <el-switch 
                v-model="form.cala_animation" 
                active-text="打开" 
                inactive-text="关闭"
              />
            </el-form-item>
            
            <el-form-item label="动画速度">
              <el-select v-model="form.cala_animation_speed" class="speed-select">
                <el-option 
                  v-for="item in speedoption" 
                  :key="item.value" 
                  :label="item.label" 
                  :value="item.value"
                />
              </el-select>
            </el-form-item>
          </div>
        </el-form>
      </el-card>
    </el-drawer>
  </div>
</template>

<style scoped>
.cala-rand-container {
  max-width: 100%;
  margin: 0 auto;
}

.app-card {
  margin-bottom: 20px;
  border-radius: 8px;
  overflow: hidden;
}

.main-card {
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
}

.card-header {
  display: flex;
  align-items: center;
  font-size: 16px;
  font-weight: 600;
}

.header-icon {
  margin-right: 8px;
  font-size: 18px;
  color: var(--el-color-primary);
}

.result-display {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px 10px;
  background-color: var(--el-fill-color-light);
  border-radius: 8px;
  margin-bottom: 20px;
  min-height: 200px;
  transition: all 0.3s;
}

#t-out {
  font-size: min(10vmin, 60px);
  font-weight: 600;
  color: var(--el-color-primary);
  margin-bottom: 16px;
  text-align: center;
}

#l-out {
  font-size: min(2.5vmin, 18px);
  color: var(--el-text-color-primary);
  text-align: center;
  max-width: 100%;
  overflow-wrap: break-word;
  word-break: break-all;
}

.control-panel {
  padding: 10px;
}

.slider-container {
  margin-bottom: 20px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.slider-label {
  font-size: 14px;
  color: var(--el-text-color-regular);
  margin-bottom: 4px;
}

.times-slider {
  margin: 0 auto;
  width: 95%;
}

.action-buttons {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 15px;
  margin-top: 20px;
}

.action-button {
  flex: 1;
  min-width: 120px;
  max-width: 200px;
  padding: 12px 20px;
  transition: all 0.3s ease;
}

.action-button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

/* 设置抽屉样式 */
.settings-drawer :deep(.el-drawer__header) {
  margin-bottom: 0;
  padding: 16px;
  border-bottom: 1px solid var(--el-border-color-light);
}

.settings-drawer :deep(.el-drawer__body) {
  padding: 0;
}

.settings-card {
  border: none;
  box-shadow: none;
}

.form-section {
  margin-bottom: 24px;
  padding: 16px;
  background-color: var(--el-fill-color-light);
  border-radius: 8px;
}

.section-title {
  margin-top: 0;
  margin-bottom: 16px;
  font-size: 16px;
  color: var(--el-text-color-primary);
  font-weight: 500;
}

.path-input {
  width: 100%;
}

.select-file-btn {
  width: 100%;
  margin-bottom: 10px;
  transition: all 0.2s ease;
}

.select-file-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(var(--el-color-primary-rgb), 0.4);
}

.sheet-selection {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.sheet-select {
  flex: 3;
  min-width: 200px;
}

.reload-btn {
  flex: 1;
  min-width: 120px;
}

.switch-item {
  margin-bottom: 16px;
}

.speed-select {
  width: 100%;
}

/* 动画效果 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.slide-fade-enter-active {
  transition: all 0.3s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.3s ease-in;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateY(20px);
  opacity: 0;
}

/* 响应式调整 */
@media (max-width: 768px) {
  .action-buttons {
    flex-direction: column;
  }
  
  .action-button {
    max-width: none;
  }
  
  .sheet-selection {
    flex-direction: column;
  }
  
  .sheet-select,
  .reload-btn {
    width: 100%;
  }
}
</style>
