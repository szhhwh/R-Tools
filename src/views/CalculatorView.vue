<script lang="ts" setup>
import { ref } from "vue"
import { RouterView } from 'vue-router'

const activeIndex = ref('/calculators/timeLapsephoto')
const iscollapse = ref<boolean>(false)
</script>

<template>
  <div class="container">
    <div class="main">
      <router-view v-slot="{ Component }">
        <keep-alive>
          <component :is="Component"></component>
        </keep-alive>
      </router-view>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
}
</style>
