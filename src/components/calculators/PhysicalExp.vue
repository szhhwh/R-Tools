<script setup lang="ts">
import { onMounted, reactive, ref } from "vue"
import { Search, Plus, Delete, DataAnalysis } from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api";

// 定义实验数据项接口
interface ExpDataItem {
  id: number;
  value: string;
  description: string;
}

interface ExpDataForm {
  expItems: ExpDataItem[];
}

const dataform = reactive<ExpDataForm>({
  expItems: [
    { id: 1, value: "", description: "" }
  ] // 初始化一行数据
})

// 添加新的实验数据行
function addDataRow() {
  dataform.expItems.push(
    { id: dataform.expItems.length + 1, value: "", description: "" }
  );
}

// 删除实验数据行
function removeDataRow(index: number) {
  if (dataform.expItems.length > 1) {
    dataform.expItems.splice(index, 1);
  }
}

const result = reactive({
  mean: 0,
  stdDev: 0,
  count: 0,
  ua: 0,
  ub: 0,
  uc: 0,
  stdErr: 0,
  error: ''
})

const loading = ref(false)
const instrument_error = ref<number>(0.5)

function compute() {
  result.error = ''
  
  // 验证输入
  if (dataform.expItems.some(item => item.value.trim() === '')) {
    result.error = '请填写所有数值'
    return
  }

  loading.value = true;
  
  invoke("compute", { data: dataform.expItems, instrument_error: instrument_error.value })
    .then((res) => {
      const response = res as {
        mean: number,
        std_dev: number,
        count: number
        ua: number,
        ub: number,
        uc: number
        std_error: number
      };
      result.mean = response.mean;
      result.stdDev = response.std_dev;
      result.count = response.count;
      result.ua = response.ua;
      result.ub = response.ub;
      result.uc = response.uc;
      result.stdErr = response.std_error;
    })
    .catch(err => {
      result.error = err.toString()
    })
    .finally(() => {
      loading.value = false;
    })
}

onMounted(() => {
  console.log("PhysicalExp component mounted");
})
</script>

<template>
  <div class="physical-exp-container">
    <el-card class="app-card" shadow="hover">
      <template #header>
        <div class="card-header">
          <el-icon class="header-icon"><DataAnalysis /></el-icon>
          <span>物理实验数据计算器</span>
        </div>
      </template>

      <ElForm :model="dataform">
        <div class="header-actions">
          <ElButton 
            size="small" 
            type="primary" 
            @click="addDataRow"
            class="add-button"
          >
            <el-icon><Plus /></el-icon>
            添加数据行
          </ElButton>
        </div>
        
        <TransitionGroup name="data-list" tag="div" class="data-rows-container">
          <div v-for="(item, index) in dataform.expItems" :key="index" class="data-row">
            <div class="data-row-badge">{{ index + 1 }}</div>
            <ElRow :gutter="20">
              <ElCol :span="8">
                <ElFormItem
                  label="数值"
                  :prop="`expItems.${index}.value`"
                  :rules="[{ required: true, message: '请输入实验值', trigger: 'blur' }]"
                >
                  <ElInput v-model="item.value" placeholder="请输入实验值"></ElInput>
                </ElFormItem>
              </ElCol>
              <ElCol :span="8">
                <ElFormItem
                  label="描述"
                  :prop="`expItems.${index}.description`"
                >
                  <ElInput v-model="item.description" placeholder="备注说明"></ElInput>
                </ElFormItem>
              </ElCol>
              <ElCol :span="2" class="delete-action">
                <ElButton
                  type="danger"
                  circle
                  size="small"
                  @click="removeDataRow(index)"
                  :disabled="dataform.expItems.length <= 1"
                  class="delete-button"
                >
                  <el-icon><Delete /></el-icon>
                </ElButton>
              </ElCol>
            </ElRow>
          </div>
        </TransitionGroup>
        
        <ElFormItem class="compute-action">
          <ElButton
            type="primary"
            @click="compute"
            :loading="loading"
            class="compute-button"
          >
            <el-icon><Search /></el-icon>
            计算
          </ElButton>
        </ElFormItem>
        <ElFormItem>
          <ElInputNumber v-model="instrument_error" controls-position="right" :min="0" :max="100" :step="0.1" :precision="1" placeholder="仪器误差" />
        </ElFormItem>
      </ElForm>
    </el-card>

    <Transition name="fade">
      <div v-if="result.error" class="result-error">
        <ElAlert :title="result.error" type="error" show-icon />
      </div>
    </Transition>

    <Transition name="slide-fade">
      <el-card v-if="result.count > 0" class="result-container" shadow="hover">
        <template #header>
          <div class="card-header">
            <span>计算结果</span>
          </div>
        </template>
        
        <div class="results-grid">
          <div class="result-item">
            <div class="result-label">数据量</div>
            <div class="result-value">{{ result.count }}</div>
          </div>
          <div class="result-item">
            <div class="result-label">平均值</div>
            <div class="result-value">{{ result.mean }}</div>
          </div>
          <div class="result-item">
            <div class="result-label">标准偏差</div>
            <div class="result-value">{{ result.stdDev }}</div>
          </div>
          <div class="result-item">
            <div class="result-label">A类不确定度</div>
            <div class="result-value">{{ result.ua }}</div>
          </div>
          <div class="result-item">
            <div class="result-label">B类不确定度</div>
            <div class="result-value">{{ result.ub }}</div>
          </div>
          <div class="result-item">
            <div class="result-label">C类不确定度</div>
            <div class="result-value">{{ result.uc }}</div>
          </div>
          <div class="result-item">
            <div class="result-label">标准误差</div>
            <div class="result-value">{{ result.stdErr * 100 + '%'}}</div>
          </div>
        </div>
      </el-card>
    </Transition>
  </div>
</template>

<style scoped>
.physical-exp-container {
  max-width: 100%;
  margin: 0 auto;
}

.app-card {
  margin-bottom: 20px;
  border-radius: 8px;
  overflow: hidden;
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

.header-actions {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 20px;
}

.add-button {
  transition: transform 0.2s ease;
}

.add-button:hover {
  transform: translateY(-2px);
}

.data-rows-container {
  margin-bottom: 20px;
}

.data-row {
  padding: 16px;
  margin-bottom: 16px;
  border-radius: 8px;
  background-color: var(--el-fill-color-light);
  position: relative;
  transition: all 0.3s;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.05);
}

.data-row:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  background-color: var(--el-fill-color);
}

.data-row-badge {
  position: absolute;
  top: -10px;
  left: -10px;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background-color: var(--el-color-primary);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: bold;
}

.delete-action {
  display: flex;
  align-items: center;
  justify-content: center;
}

.delete-button {
  transition: all 0.2s ease;
}

.delete-button:hover:not(:disabled) {
  transform: scale(1.1);
  background-color: var(--el-color-danger);
  color: white;
}

.compute-action {
  display: flex;
  justify-content: center;
  margin-top: 20px;
}

.compute-button {
  padding: 12px 30px;
  font-weight: 500;
  transition: all 0.3s ease;
}

.compute-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(var(--el-color-primary-rgb), 0.4);
}

.result-container {
  margin-top: 24px;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
}

.results-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
  padding: 10px;
}

.result-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px;
  background-color: var(--el-fill-color-lighter);
  border-radius: 6px;
  transition: transform 0.3s ease;
}

.result-item:hover {
  transform: translateY(-5px);
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.08);
}

.result-label {
  font-size: 14px;
  color: var(--el-text-color-secondary);
  margin-bottom: 8px;
}

.result-value {
  font-size: 24px;
  font-weight: 600;
  color: var(--el-color-primary);
}

.result-error {
  margin-top: 20px;
}

/* 动画效果 */
.data-list-enter-active,
.data-list-leave-active {
  transition: all 0.5s ease;
}

.data-list-enter-from,
.data-list-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

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
  .results-grid {
    grid-template-columns: 1fr;
  }
  
  .data-row {
    padding: 12px 8px;
  }
}
</style>