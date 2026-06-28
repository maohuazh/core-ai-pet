<template>
  <nav class="icon-rail">
    <div class="rail-items">
      <button
        v-for="item in items"
        :key="item.id"
        class="rail-btn"
        :class="{ active: item.id === active }"
        :title="item.label"
        @click="emit('update:active', item.id)"
      >
        <span class="rail-icon">{{ item.icon }}</span>
      </button>
    </div>
    <div class="rail-footer">
      <slot name="footer" />
    </div>
  </nav>
</template>

<script setup lang="ts">
import type { NavItem } from "./types";

defineProps<{
  items: NavItem[];
  active: string;
}>();

const emit = defineEmits<{
  "update:active": [id: string];
}>();
</script>

<style scoped>
.icon-rail {
  display: flex;
  flex-direction: column;
  width: 56px;
  min-width: 56px;
  background: var(--bg-surface);
  border-right: 1px solid var(--border);
  flex-shrink: 0;
}

.rail-items {
  display: flex;
  flex-direction: column;
  padding: 8px 0;
  gap: 2px;
  flex: 1;
  overflow-y: auto;
}

.rail-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 40px;
  border: none;
  background: transparent;
  color: var(--text-dim);
  cursor: pointer;
  border-left: 3px solid transparent;
  transition: background var(--t-fast), color var(--t-fast), border-color var(--t-fast);
}

.rail-btn:hover {
  background: var(--bg-elevated);
  color: var(--text);
}

.rail-btn.active {
  color: var(--accent);
  border-left-color: var(--accent);
  background: var(--bg-elevated);
}

.rail-icon {
  font-size: 18px;
  line-height: 1;
}

.rail-footer {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 8px 0;
  border-top: 1px solid var(--border);
  flex-shrink: 0;
}
</style>
