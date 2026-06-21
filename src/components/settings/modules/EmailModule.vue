<template>
  <div class="module-container">
    <div class="module-header">
      <h2 class="module-title">邮箱连接</h2>
      <button class="add-btn" @click="handleAdd">+ 添加</button>
    </div>
    <div class="module-content">
      <p class="placeholder-text">邮箱连接模块（待实现）</p>
    </div>
  </div>
</template>

<script setup lang="ts">
const handleAdd = () => {
  alert('功能开发中');
};
</script>

<style scoped>
.module-container {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.module-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.module-title {
  font-size: 18px;
  font-weight: 600;
  color: #1f2937;
  margin: 0;
}

.add-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  background: #6366f1;
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.add-btn:hover {
  background: #818cf8;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(99, 102, 241, 0.2);
}

.add-btn:active {
  background: #4f46e5;
  transform: translateY(0);
}

.module-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.placeholder-text {
  color: #9ca3af;
  font-size: 14px;
  text-align: center;
  padding: 40px;
}
</style>
