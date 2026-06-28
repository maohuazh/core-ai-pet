<template>
  <div class="home-page">
    <div class="home-content">
      <div class="home-icon">🐾</div>
      <h1 class="home-title">欢迎使用 CoreAIpet，你的 AI 桌面助手</h1>
    </div>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.home-page {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  width: 100%;
}

.home-content {
  text-align: center;
}

.home-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.home-title {
  font-size: 18px;
  color: var(--text);
  font-weight: 500;
}
</style>
