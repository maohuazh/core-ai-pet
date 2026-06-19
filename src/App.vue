<template>
  <div class="pet-container" @mouseenter="onMouseEnter" @mouseleave="onMouseLeave">
    <Live2DCanvas ref="canvasRef" />
    <PetHoverMenu v-if="showMenu" :on-action="handleMenuAction" />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import Live2DCanvas from "./components/Live2DCanvas.vue";
import PetHoverMenu from "./components/PetHoverMenu.vue";

const canvasRef = ref<InstanceType<typeof Live2DCanvas> | null>(null);
const showMenu = ref(false);
let hideTimeout: ReturnType<typeof setTimeout> | null = null;

const onMouseEnter = () => {
  if (hideTimeout) {
    clearTimeout(hideTimeout);
    hideTimeout = null;
  }
  showMenu.value = true;
};

const onMouseLeave = () => {
  hideTimeout = setTimeout(() => {
    showMenu.value = false;
  }, 200);
};

const handleMenuAction = async (action: string) => {
  console.log("Menu action:", action);
  const appWindow = getCurrentWindow();

  switch (action) {
    case "chat":
      alert("聊天功能即将推出");
      break;
    case "settings":
      alert("设置功能即将推出");
      break;
    case "menu":
      alert("菜单功能即将推出");
      break;
    case "minimize":
      await appWindow.minimize();
      break;
    case "close":
      await appWindow.close();
      break;
  }
};
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  width: 100%;
  height: 100%;
  overflow: hidden;
  background: transparent;
}

.pet-container {
  width: 100vw;
  height: 100vh;
  position: relative;
  background: transparent;
}
</style>
