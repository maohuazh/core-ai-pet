<template>
  <canvas ref="canvasEl" class="live2d-canvas" @mousedown="onMouseDown"></canvas>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Live2DRenderer } from "../core/renderer/live2d/Live2DRenderer";

const canvasEl = ref<HTMLCanvasElement | null>(null);
const renderer = ref<Live2DRenderer | null>(null);

const onMouseDown = async (event: MouseEvent) => {
  console.log("Mouse down on canvas, button:", event.button);
  // Only trigger drag on left mouse button
  if (event.button !== 0) return;

  try {
    await invoke("start_dragging");
    console.log("Drag started successfully");
  } catch (error) {
    console.error("Failed to start dragging:", error);
  }
};

onMounted(async () => {
  console.log("Live2DCanvas mounted");
  console.log("Window.Live2DCubismCore:", !!window.Live2DCubismCore);
  console.log("Window.Live2D:", !!(window as any).Live2D);

  if (!canvasEl.value) {
    console.error("Canvas element not available");
    return;
  }

  console.log("Canvas element:", canvasEl.value);
  console.log("Canvas size:", canvasEl.value.width, "x", canvasEl.value.height);

  renderer.value = new Live2DRenderer(canvasEl.value);

  try {
    await renderer.value.init();
    console.log("Renderer initialized");
  } catch (error) {
    console.error("Failed to initialize renderer:", error);
    return;
  }

  // Load Haru model (Cubism 4)
  try {
    const modelUrl = "https://cdn.jsdelivr.net/gh/guansss/pixi-live2d-display/test/assets/haru/haru_greeter_t03.model3.json";
    console.log("Loading Haru model from:", modelUrl);
    await renderer.value.loadModel(modelUrl);
    console.log("Haru model loaded successfully");
  } catch (error) {
    console.error("Failed to load Haru model:", error);
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
