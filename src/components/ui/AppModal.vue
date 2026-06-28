<template>
  <Teleport to="body">
    <Transition name="modal">
      <div
        v-if="open"
        class="modal-overlay"
        @mousedown.self="onOverlay"
      >
        <div
          class="modal-container"
          role="dialog"
          aria-modal="true"
          :style="{ maxWidth: maxWidth }"
        >
          <header v-if="title || $slots.title" class="modal-header">
            <h3 class="modal-title">
              <slot name="title">{{ title }}</slot>
            </h3>
            <button v-if="closable" class="modal-close" @click="close" aria-label="关闭">
              <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
                <path
                  d="M2.5 2.5L9.5 9.5M9.5 2.5L2.5 9.5"
                  stroke="currentColor"
                  stroke-width="1.5"
                  stroke-linecap="round"
                />
              </svg>
            </button>
          </header>

          <div class="modal-body">
            <slot />
          </div>

          <footer v-if="$slots.footer" class="modal-footer">
            <slot name="footer" :close="close" />
          </footer>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { watch, onUnmounted } from "vue";

const props = withDefaults(
  defineProps<{
    open: boolean;
    title?: string;
    /** 点击遮罩关闭，默认 true */
    closeOnOverlay?: boolean;
    /** ESC 关闭，默认 true */
    closeOnEscape?: boolean;
    /** 右上角关闭按钮，默认 true */
    closable?: boolean;
    maxWidth?: string;
  }>(),
  {
    closeOnOverlay: true,
    closeOnEscape: true,
    closable: true,
    maxWidth: "480px",
  }
);

const emit = defineEmits<{
  "update:open": [value: boolean];
}>();

function close() {
  emit("update:open", false);
}
function onOverlay() {
  if (props.closeOnOverlay) close();
}
function onKey(e: KeyboardEvent) {
  if (!props.open || !props.closeOnEscape) return;
  if (e.key === "Escape") {
    e.stopPropagation();
    close();
  }
}

watch(
  () => props.open,
  (v) => {
    if (v) document.addEventListener("keydown", onKey);
    else document.removeEventListener("keydown", onKey);
  }
);

onUnmounted(() => document.removeEventListener("keydown", onKey));
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: var(--z-modal);
  font-family: var(--font-sans);
}

.modal-container {
  width: calc(100% - 48px);
  background: var(--bg-surface);
  color: var(--text);
  border: 1px solid var(--border-strong);
  border-radius: var(--r-2xl);
  box-shadow: var(--shadow-modal);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 18px 10px;
}
.modal-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text);
  margin: 0;
}
.modal-close {
  width: 26px;
  height: 26px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  border-radius: var(--r-md);
  background: transparent;
  color: var(--text-dim);
  cursor: pointer;
  transition: background var(--t-fast), color var(--t-fast);
}
.modal-close:hover {
  background: rgba(243, 139, 168, 0.12);
  color: var(--danger);
}

.modal-body {
  padding: 4px 18px 14px;
  font-size: 13px;
  line-height: 1.55;
  color: var(--text-muted);
  max-height: 60vh;
  overflow-y: auto;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 10px 18px 16px;
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity var(--t-med) ease;
}
.modal-enter-active .modal-container,
.modal-leave-active .modal-container {
  transition: transform var(--t-med) ease, opacity var(--t-med) ease;
}
.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
  transform: translateY(6px) scale(0.98);
  opacity: 0;
}

/* Scrollbar */
.modal-body::-webkit-scrollbar {
  width: 5px;
}
.modal-body::-webkit-scrollbar-track {
  background: transparent;
}
.modal-body::-webkit-scrollbar-thumb {
  background: var(--border-strong);
  border-radius: 3px;
}
</style>
