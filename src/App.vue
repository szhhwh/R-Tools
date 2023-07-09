<script setup lang="ts">
import RandView from './views/RandView.vue';
</script>

<template>
  <el-container>
    <el-header>
    </el-header>
    <el-main class="main">
      <RandView></RandView>
    </el-main>
  </el-container>
</template>

<style scoped>
.main {
  height: 100%;
}
</style>