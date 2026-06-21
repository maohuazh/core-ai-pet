<template>
  <div class="settings-titlebar" @mousedown="startDrag">
    <div class="titlebar-drag-region">
      <span class="titlebar-title">⚙️ 设置</span>
    </div>
    <div class="titlebar-buttons">
      <button class="titlebar-btn minimize-btn" @click.stop="minimize" title="最小化">
        <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
          <rect x="2" y="5.5" width="8" height="1" fill="currentColor" />
        </svg>
      </button>
      <button class="titlebar-btn close-btn" @click.stop="close" title="关闭">
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
  background: rgba(255, 255, 255, 0.4);
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  user-select: none;
  cursor: default;
}

.titlebar-drag-region {
  flex: 1;
  display: flex;
  align-items: center;
  height: 100%;
  -webkit-app-region: drag;
}

.titlebar-title {
  font-size: 15px;
  font-weight: 600;
  color: #1f2937;
  margin-left: 4px;
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
  border-radius: 8px;
  background: transparent;
  color: #6b7280;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.titlebar-btn:hover {
  background: rgba(0, 0, 0, 0.05);
  color: #1f2937;
}

.titlebar-btn:active {
  background: rgba(0, 0, 0, 0.08);
  transform: scale(0.95);
}

.close-btn:hover {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.close-btn:hover:active {
  background: rgba(239, 68, 68, 0.15);
}
</style>
