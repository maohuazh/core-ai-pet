<template>
  <div class="module-container">
    <div class="module-header">
      <h2 class="module-title">快捷键</h2>
    </div>
    <div class="module-content">
      <div class="shortcuts-list">
        <div v-for="shortcut in shortcuts" :key="shortcut.name" class="shortcut-item">
          <span class="shortcut-name">{{ shortcut.name }}</span>
          <span class="shortcut-keys">
            <kbd v-for="(key, i) in shortcut.keys" :key="i">{{ key }}</kbd>
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Shortcut {
  name: string;
  keys: string[];
}

const shortcuts: Shortcut[] = [
  { name: '截图', keys: ['Win', 'F2'] },
  { name: '打开聊天窗口', keys: ['Ctrl', 'Alt', 'N'] },
  { name: '打开主窗口', keys: ['Ctrl', 'Alt', 'L'] },
];
</script>

<style scoped>
.module-container {
  width: 100%;
  height: 100%;
}

.module-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 20px;
}

.module-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text);
  margin: 0;
}

.module-content {
  width: 100%;
}

.shortcuts-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.shortcut-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: var(--r-lg);
}

.shortcut-name {
  font-size: 14px;
  color: var(--text);
}

.shortcut-keys {
  display: flex;
  align-items: center;
  gap: 4px;
}

.shortcut-keys kbd {
  display: inline-block;
  padding: 2px 8px;
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--text-muted);
  background: var(--bg-elevated);
  border: 1px solid var(--border-strong);
  border-radius: var(--r-sm);
  line-height: 1.6;
}
</style>
