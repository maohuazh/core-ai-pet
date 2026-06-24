<template>
  <SettingsPanel v-if="isSettingsRoute" />
  <div v-else class="pet-container">
    <Live2DCanvas ref="canvasRef" />
    <WindowCloseButton v-if="showMenu" />
    <PetHoverMenu v-if="showMenu" :on-action="handleMenuAction" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import Live2DCanvas from "./components/Live2DCanvas.vue";
import WindowCloseButton from "./components/WindowCloseButton.vue";
import PetHoverMenu from "./components/PetHoverMenu.vue";
import SettingsPanel from "./components/settings/SettingsPanel.vue";
import { invoke } from "@tauri-apps/api/core";
import { modelRegistry } from "./core/model/ModelRegistry";
import { petStore } from "./core/model/PetStore";

// Check if current route is /settings
const isSettingsRoute = computed(() => {
  return window.location.pathname === "/settings";
});

const canvasRef = ref<InstanceType<typeof Live2DCanvas> | null>(null);
const showMenu = ref(false);

let unlistenStart: UnlistenFn | null = null;
let unlistenEnd: UnlistenFn | null = null;
let unlistenModelChanged: UnlistenFn | null = null;
let hideTimeout: ReturnType<typeof setTimeout> | null = null;

onMounted(async () => {
  // Only set up pet window listeners if not in settings route
  if (!isSettingsRoute.value) {
    // Listen for model change events from settings window
    unlistenModelChanged = await listen<{ modelId: string }>("pet-model-changed", (event) => {
      const model = modelRegistry.getById(event.payload.modelId);
      if (model) {
        petStore.setCurrentModel(model);
      }
    });

    // Sync active model from DB on startup
    try {
      const activeId = await invoke<string | null>("get_active_model_id");
      if (activeId) {
        const model = modelRegistry.getById(activeId);
        if (model) {
          petStore.setCurrentModel(model);
        }
      }
    } catch (e) {
      console.warn("Failed to sync active model from DB:", e);
    }

    // Listen for cursor monitor events from Rust backend
    unlistenStart = await listen("pet-hover-start", () => {
      if (hideTimeout) {
        clearTimeout(hideTimeout);
        hideTimeout = null;
      }
      showMenu.value = true;
    });

    unlistenEnd = await listen("pet-hover-end", () => {
      hideTimeout = setTimeout(() => {
        showMenu.value = false;
      }, 200);
    });
  }
});

onUnmounted(() => {
  unlistenStart?.();
  unlistenEnd?.();
  unlistenModelChanged?.();
  if (hideTimeout) clearTimeout(hideTimeout);
});

const actionLabels: Record<string, string> = {
  task: "任务",
  message: "消息",
  jira: "Jira",
  email: "邮件",
  agent: "Agent",
  settings: "设置",
};

const handleMenuAction = async (action: string) => {
  console.log("Menu action:", action);

  if (action === "settings") {
    // Open settings window
    await invoke("open_settings_window");
    return;
  }

  const label = actionLabels[action] ?? action;
  alert(`${label}功能即将推出`);
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
