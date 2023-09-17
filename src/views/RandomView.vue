<script setup lang="ts">
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

.main {
  width: 100%;
}
</style>