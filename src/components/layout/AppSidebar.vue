<template>
  <div class="app-sidebar" :class="{ collapsed }">
    <button class="collapse-btn" @click="emit('toggle-collapse')" :title="collapsed ? '展开' : '收起'">
      <span class="collapse-icon">☰</span>
    </button>
    <nav class="nav-list">
      <div
        v-for="item in items"
        :key="item.id"
        class="nav-item"
        :class="{ active: item.id === active }"
        @click="emit('update:active', item.id)"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span v-if="!collapsed" class="nav-label">{{ item.label }}</span>
      </div>
    </nav>
    <div class="sidebar-footer">
      <slot name="footer" />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { NavItem } from "./types";

defineProps<{
  items: NavItem[];
  active: string;
  collapsed: boolean;
}>();

const emit = defineEmits<{
  "update:active": [id: string];
  "toggle-collapse": [];
}>();
</script>

<style scoped>
.app-sidebar {
  display: flex;
  flex-direction: column;
  width: 200px;
  background: var(--bg-surface);
  border-right: 1px solid var(--border);
  flex-shrink: 0;
  transition: width var(--t-med) ease;
  overflow: hidden;
}

.app-sidebar.collapsed {
  width: 56px;
}

.collapse-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 40px;
  width: 100%;
  border: none;
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 16px;
  flex-shrink: 0;
}

.collapse-btn:hover {
  background: var(--bg-hover);
  color: var(--text);
}

.nav-list {
  display: flex;
  flex-direction: column;
  padding: 4px 8px;
  gap: 2px;
  flex: 1;
  overflow-y: auto;
}

.nav-item {
  display: flex;
  align-items: center;
  height: 40px;
  padding: 0 12px;
  gap: 10px;
  border-radius: var(--r-md);
  cursor: pointer;
  color: var(--text-muted);
  transition: background var(--t-fast), color var(--t-fast);
  border-left: 3px solid transparent;
  white-space: nowrap;
  overflow: hidden;
}

.nav-item:hover {
  background: var(--bg-hover);
  color: var(--text);
}

.nav-item.active {
  background: var(--bg-hover);
  color: var(--text);
  border-left-color: var(--accent);
}

.nav-icon {
  font-size: 16px;
  flex-shrink: 0;
}

.nav-label {
  font-size: 14px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.sidebar-footer {
  padding: 8px;
  border-top: 1px solid var(--border);
  flex-shrink: 0;
}
</style>
