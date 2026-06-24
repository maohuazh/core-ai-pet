<template>
  <div ref="containerEl" class="live2d-container" @mousedown="onMouseDown"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Live2DRenderer } from "../core/renderer/live2d/Live2DRenderer";
import { SpriteSheetRenderer } from "../core/renderer/sprite/SpriteSheetRenderer";
import { petStore } from "../core/model/PetStore";
import { createAvatar, type Avatar } from "../core/avatar";
import type { PetModelConfig } from "../core/model/ModelRegistry";

const containerEl = ref<HTMLDivElement | null>(null);
const renderer = ref<Live2DRenderer | SpriteSheetRenderer | null>(null);
const avatar = ref<Avatar | null>(null);

/** Expose avatar for parent components to use */
defineExpose({ avatar });

const onMouseDown = async (event: MouseEvent) => {
  if (event.button !== 0) return;
  try {
    await invoke("start_dragging");
    // Save position after drag ends
    const pos = await invoke<[number, number]>("get_window_position");
    await invoke("storage_set", { key: "window_position_x", value: String(pos[0]) });
    await invoke("storage_set", { key: "window_position_y", value: String(pos[1]) });
  } catch (error) {
    console.error("Failed to start dragging:", error);
  }
};

/** Create the appropriate renderer based on model type */
function createRenderer(model: PetModelConfig, container: HTMLElement, width: number, height: number): Live2DRenderer | SpriteSheetRenderer {
  if (model.type === "sprite") {
    return new SpriteSheetRenderer(container, width, height);
  }
  return new Live2DRenderer(container, width, height);
}

// Watch for model changes and reload renderer
watch(
  () => petStore.currentModel.value,
  async (newModel) => {
    if (!newModel || !containerEl.value) return;

    console.log(`Switching to model: ${newModel.name} (${newModel.type})`);
    try {
      // Destroy old renderer and avatar
      if (avatar.value) {
        avatar.value.destroy();
        avatar.value = null;
      }
      if (renderer.value) {
        renderer.value.destroy();
        renderer.value = null;
      }

      const width = containerEl.value.clientWidth || 200;
      const height = containerEl.value.clientHeight || 200;

      // Create new renderer based on model type
      renderer.value = createRenderer(newModel, containerEl.value, width, height);
      await renderer.value.init();
      await renderer.value.loadModel(newModel.modelUrl);

      // Create avatar with matching type
      avatar.value = createAvatar(newModel.type === "sprite" ? "sprite" : "live2d", renderer.value as any);
      console.log(`Model switched to: ${newModel.name}`);
    } catch (error) {
      console.error(`Failed to switch to model ${newModel.name}:`, error);
    }
  }
);

onMounted(async () => {
  console.log("Live2DCanvas mounted");

  // Restore window position if saved
  try {
    const savedX = await invoke<string | null>("storage_get", { key: "window_position_x" });
    const savedY = await invoke<string | null>("storage_get", { key: "window_position_y" });
    if (savedX !== null && savedY !== null) {
      const x = parseInt(savedX, 10);
      const y = parseInt(savedY, 10);
      if (!isNaN(x) && !isNaN(y)) {
        await invoke("set_window_position", { x, y });
        console.log(`Restored window position: (${x}, ${y})`);
      }
    }
  } catch (error) {
    console.warn("Failed to restore window position:", error);
  }

  if (!containerEl.value) {
    console.error("Container element not available");
    return;
  }

  // Read container dimensions
  const width = containerEl.value.clientWidth || 200;
  const height = containerEl.value.clientHeight || 200;

  // Load the current model from PetStore
  const currentModel = petStore.currentModel.value;

  // Create renderer based on model type
  renderer.value = createRenderer(currentModel, containerEl.value, width, height);

  try {
    await renderer.value.init();
    console.log(`Renderer initialized (${currentModel.type}), size:`, width, "x", height);
  } catch (error) {
    console.error("Failed to initialize renderer:", error);
    return;
  }

  try {
    console.log(`Loading initial model: ${currentModel.name} (${currentModel.type})`);
    await renderer.value.loadModel(currentModel.modelUrl);
    console.log(`Initial model loaded: ${currentModel.name}`);

    // Create Avatar abstraction with matching type
    avatar.value = createAvatar(currentModel.type === "sprite" ? "sprite" : "live2d", renderer.value as any);
    console.log("Avatar created");
  } catch (error) {
    console.error(`Failed to load model ${currentModel.name}:`, error);
  }
});

onBeforeUnmount(() => {
  if (avatar.value) {
    avatar.value.destroy();
  }
  if (renderer.value) {
    renderer.value.destroy();
  }
});
</script>

<style scoped>
.live2d-container {
  width: 100%;
  height: 100%;
  cursor: grab;
  overflow: hidden;
}

.live2d-container:active {
  cursor: grabbing;
}

.live2d-container canvas {
  display: block;
  width: 100% !important;
  height: 100% !important;
}
</style>
