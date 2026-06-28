<template>
  <aside class="session-panel">
    <div class="session-header">
      <div
        class="search-trigger"
        @click="searchOpen = !searchOpen"
      >
        <input
          class="search-input"
          type="text"
          placeholder="搜索会话..."
          readonly
          @click.stop="searchOpen = !searchOpen"
        />
        <span class="search-chevron" :class="{ open: searchOpen }">▾</span>
      </div>
    </div>
    <div class="session-list">
      <div
        v-for="session in sessions"
        :key="session.id"
        class="session-item"
        :class="{ active: session.id === activeId }"
        @click="emit('select', session.id)"
      >
        <div class="session-top">
          <span class="session-title">{{ session.title }}</span>
          <span class="session-time">{{ session.time }}</span>
        </div>
        <div class="session-preview">{{ session.preview }}</div>
      </div>
    </div>
    <div class="session-footer">
      <button class="clear-btn" @click="emit('clear')">
        <span class="clear-icon">🗑</span>
        <span>清空记录</span>
      </button>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { ref } from "vue";

export interface SessionItem {
  id: string;
  title: string;
  time: string;
  preview: string;
}

defineProps<{
  sessions: SessionItem[];
  activeId?: string;
}>();

const emit = defineEmits<{
  select: [id: string];
  clear: [];
}>();

const searchOpen = ref(false);
</script>

<style scoped>
.session-panel {
  display: flex;
  flex-direction: column;
  width: 280px;
  min-width: 280px;
  background: var(--bg-base);
  border-right: 1px solid var(--border);
  flex-shrink: 0;
  overflow: hidden;
}

.session-header {
  padding: 12px 12px 8px;
  flex-shrink: 0;
}

.search-trigger {
  display: flex;
  align-items: center;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: var(--r-md);
  padding: 0 10px;
  height: 36px;
  cursor: pointer;
  transition: border-color var(--t-fast);
}

.search-trigger:hover {
  border-color: var(--border-strong);
}

.search-input {
  flex: 1;
  background: none;
  border: none;
  outline: none;
  color: var(--text-muted);
  font-size: 13px;
  font-family: var(--font-sans);
  cursor: pointer;
}

.search-chevron {
  font-size: 12px;
  color: var(--text-dim);
  transition: transform var(--t-fast);
}

.search-chevron.open {
  transform: rotate(180deg);
}

.session-list {
  flex: 1;
  overflow-y: auto;
  padding: 4px 8px;
}

.session-list::-webkit-scrollbar {
  width: 4px;
}

.session-list::-webkit-scrollbar-track {
  background: transparent;
}

.session-list::-webkit-scrollbar-thumb {
  background: var(--border-strong);
  border-radius: 2px;
}

.session-item {
  padding: 10px 12px;
  border-radius: var(--r-md);
  cursor: pointer;
  transition: background var(--t-fast);
  margin-bottom: 2px;
}

.session-item:hover {
  background: var(--bg-surface);
}

.session-item.active {
  background: var(--bg-elevated);
}

.session-top {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.session-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.session-time {
  font-size: 11px;
  color: var(--text-dim);
  flex-shrink: 0;
  margin-left: 8px;
}

.session-preview {
  font-size: 12px;
  color: var(--text-dim);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.session-footer {
  padding: 8px 12px;
  border-top: 1px solid var(--border);
  flex-shrink: 0;
}

.clear-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  width: 100%;
  padding: 8px 12px;
  border: none;
  border-radius: var(--r-md);
  background: transparent;
  color: var(--text-dim);
  font-size: 13px;
  font-family: var(--font-sans);
  cursor: pointer;
  transition: background var(--t-fast), color var(--t-fast);
}

.clear-btn:hover {
  background: var(--bg-elevated);
  color: var(--text-muted);
}

.clear-icon {
  font-size: 14px;
}
</style>
