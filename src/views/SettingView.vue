<script lang="ts" setup>
import { inject, onMounted, ref, watch } from 'vue'

// 引入全局配置
const { config, write_conf } = inject<any>('app_config')

const settingform = ref({
    lastviewselector: true
})

function loadsettings(): void {
    settingform.value.lastviewselector = config.value.lastviewselector
}

watch(settingform.value, () => {
    let data = JSON.parse(JSON.stringify(settingform.value))
    write_conf(data, 'main')
})

onMounted(() => {
    loadsettings()
})
</script>

<template>
    <div>
        <ElContainer>
            <ElForm :model="settingform">
                <ElFormItem label="记忆最后一次打开的页面">
                    <ElSwitch v-model="settingform.lastviewselector" active-text="开启" inactive-text="关闭"></ElSwitch>
                </ElFormItem>
            </ElForm>
        </ElContainer>
    </div>
</template>

<style scoped></style>