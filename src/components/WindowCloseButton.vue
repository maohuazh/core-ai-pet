<template>
  <button class="close-button" @click="handleClose" title="退出">
    ✕
  </button>
</template>

<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";

const handleClose = () => {
  getCurrentWindow().close();
};
</script>

<style scoped>
.close-button {
  position: absolute;
  top: 8px;
  right: 8px;
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 50%;
  background: transparent;
  color: rgba(255, 255, 255, 0.8);
  font-size: 14px;
  line-height: 28px;
  text-align: center;
  cursor: pointer;
  opacity: 0.3;
  transition: opacity 0.2s ease, background 0.2s ease;
  z-index: 20;
  padding: 0;
}

.close-button:hover {
  opacity: 1;
  background: rgba(220, 50, 50, 0.8);
}
</style>
