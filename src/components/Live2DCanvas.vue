<template>
  <canvas ref="canvasEl" class="live2d-canvas" @mousedown="onMouseDown"></canvas>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Live2DRenderer } from "../core/renderer/live2d/Live2DRenderer";
import { petStore } from "../core/model/PetStore";

const canvasEl = ref<HTMLCanvasElement | null>(null);
const renderer = ref<Live2DRenderer | null>(null);

const onMouseDown = async (event: MouseEvent) => {
  if (event.button !== 0) return;
  try {
    await invoke("start_dragging");
  } catch (error) {
    console.error("Failed to start dragging:", error);
  }
};

// Watch for model changes and reload renderer
watch(
  () => petStore.currentModel.value,
  async (newModel) => {
    if (renderer.value && newModel) {
      console.log(`Switching to model: ${newModel.name}`);
      try {
        await renderer.value.loadModel(newModel.modelUrl);
        console.log(`Model switched to: ${newModel.name}`);
      } catch (error) {
        console.error(`Failed to switch to model ${newModel.name}:`, error);
      }
    }
  }
);

onMounted(async () => {
  console.log("Live2DCanvas mounted");

  if (!canvasEl.value) {
    console.error("Canvas element not available");
    return;
  }

  renderer.value = new Live2DRenderer(canvasEl.value);

  try {
    await renderer.value.init();
    console.log("Renderer initialized");
  } catch (error) {
    console.error("Failed to initialize renderer:", error);
    return;
  }

  // Load the current model from PetStore
  const currentModel = petStore.currentModel.value;
  try {
    console.log(`Loading initial model: ${currentModel.name}`);
    await renderer.value.loadModel(currentModel.modelUrl);
    console.log(`Initial model loaded: ${currentModel.name}`);
  } catch (error) {
    console.error(`Failed to load model ${currentModel.name}:`, error);
  }
});

onBeforeUnmount(() => {
  if (renderer.value) {
    renderer.value.destroy();
  }
});
</script>

<style scoped>
.live2d-canvas {
  width: 100%;
  height: 100%;
  display: block;
  cursor: grab;
}

.live2d-canvas:active {
  cursor: grabbing;
}
</style>
