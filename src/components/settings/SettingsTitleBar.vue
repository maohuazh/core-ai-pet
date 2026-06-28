<template>
  <div class="settings-titlebar">
    <div class="titlebar-drag-region" @mousedown="startDrag">
      <span class="titlebar-title">⚙️ 设置</span>
    </div>
    <div class="titlebar-buttons">
      <button class="titlebar-btn minimize-btn" @click="minimize" title="最小化">
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
          <rect x="2" y="5.5" width="8" height="1" fill="currentColor" />
        </svg>
      </button>
      <button class="titlebar-btn close-btn" @click="close" title="关闭">
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
          <path d="M2.5 2.5L9.5 9.5M9.5 2.5L2.5 9.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
        </svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';

const startDrag = async (event: MouseEvent) => {
  if (event.button !== 0) return;
  const appWindow = getCurrentWindow();
  await appWindow.startDragging();
};

const minimize = async () => {
  const appWindow = getCurrentWindow();
  await appWindow.minimize();
};

const close = async () => {
  const appWindow = getCurrentWindow();
  await appWindow.hide();
};
</script>

<style scoped>
.settings-titlebar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 48px;
  padding: 0 16px;
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border);
  user-select: none;
  cursor: default;
}

.titlebar-drag-region {
  flex: 1;
  display: flex;
  align-items: center;
  height: 100%;
  cursor: move;
}

.titlebar-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text);
  margin-left: 4px;
  pointer-events: none;
}

.titlebar-buttons {
  display: flex;
  gap: 8px;
  -webkit-app-region: no-drag;
}

.titlebar-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: var(--r-lg);
  background: transparent;
  color: var(--text-dim);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background var(--t-fast), color var(--t-fast);
}

.titlebar-btn:hover {
  background: var(--bg-elevated);
  color: var(--text);
}

.titlebar-btn:active {
  background: var(--bg-hover);
}

.close-btn:hover {
  background: rgba(243, 139, 168, 0.12);
  color: var(--danger);
}
</style>
