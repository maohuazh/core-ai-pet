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
  background: rgba(255, 255, 255, 0.4);
  border-right: 1px solid rgba(0, 0, 0, 0.06);
}

.sidebar-nav {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 12px 8px;
  gap: 4px;
}

.nav-item {
  position: relative;
  display: flex;
  align-items: center;
  height: 44px;
  padding: 0 12px;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: #6b7280;
  cursor: pointer;
  font-size: 14px;
  text-align: left;
  transition: all 0.2s ease;
}

.nav-item:hover {
  background: rgba(99, 102, 241, 0.08);
  color: #1f2937;
}

.nav-item.active {
  background: rgba(99, 102, 241, 0.12);
  color: #6366f1;
  font-weight: 500;
}

.nav-icon {
  font-size: 18px;
  margin-right: 10px;
  width: 24px;
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
  height: 24px;
  background: #6366f1;
  border-radius: 0 3px 3px 0;
}

.sidebar-footer {
  padding: 12px 16px;
  border-top: 1px solid rgba(0, 0, 0, 0.06);
}

.version-text {
  font-size: 12px;
  color: #9ca3af;
}
</style>
