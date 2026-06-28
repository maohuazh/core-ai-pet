<template>
  <div class="settings-sidebar">
    <nav class="sidebar-nav">
      <button
        v-for="item in navItems"
        :key="item.key"
        :class="['nav-item', { active: activeModule === item.key }]"
        @click="activeModule = item.key"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span class="nav-label">{{ item.label }}</span>
        <span v-if="activeModule === item.key" class="nav-indicator"></span>
      </button>
    </nav>
    <div class="sidebar-footer">
      <span class="version-text">CoreAIpet v1.0.0</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { SettingsModule } from './types';

const activeModule = defineModel<SettingsModule>('activeModule', { required: true });

const navItems = [
  { key: 'jira' as SettingsModule, icon: '📋', label: 'Jira' },
  { key: 'email' as SettingsModule, icon: '📧', label: '邮箱' },
  { key: 'chat' as SettingsModule, icon: '💬', label: '聊天' },
  { key: 'model' as SettingsModule, icon: '🐾', label: '宠物' },
  { key: 'llm' as SettingsModule, icon: '🤖', label: 'AI 模型' },
];
</script>

<style scoped>
.settings-sidebar {
  width: 160px;
  display: flex;
  flex-direction: column;
  background: var(--bg-surface);
  border-right: 1px solid var(--border);
}

.sidebar-nav {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 12px 8px;
  gap: 2px;
}

.nav-item {
  position: relative;
  display: flex;
  align-items: center;
  height: 40px;
  padding: 0 12px;
  border: none;
  border-radius: var(--r-md);
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
  font-size: 13px;
  font-family: inherit;
  text-align: left;
  transition: background var(--t-fast), color var(--t-fast);
}

.nav-item:hover {
  background: var(--bg-elevated);
  color: var(--text);
}

.nav-item.active {
  background: var(--bg-hover);
  color: var(--text);
  font-weight: 500;
}

.nav-icon {
  font-size: 16px;
  margin-right: 10px;
  width: 20px;
  text-align: center;
}

.nav-label {
  flex: 1;
}

.nav-indicator {
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 20px;
  background: var(--accent);
  border-radius: 0 3px 3px 0;
}

.sidebar-footer {
  padding: 12px 16px;
  border-top: 1px solid var(--border);
}

.version-text {
  font-size: 11px;
  color: var(--text-dim);
}
</style>
