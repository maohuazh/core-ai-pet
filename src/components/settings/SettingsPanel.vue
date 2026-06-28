<template>
  <div class="settings-panel">
    <SettingsTitleBar />
    <div class="settings-body">
      <AppSidebar
        :items="sidebarItems"
        :active="activeModule"
        :collapsed="false"
        @update:active="activeModule = $event as SettingsModule"
      >
        <template #footer>
          <span class="version-text">CoreAIpet v1.0.0</span>
        </template>
      </AppSidebar>
      <div class="settings-main">
        <Transition name="fade" mode="out-in">
          <JiraModule v-if="activeModule === 'jira'" key="jira" />
          <EmailModule v-else-if="activeModule === 'email'" key="email" />
          <ChatModule v-else-if="activeModule === 'chat'" key="chat" />
          <ModelConfigModule v-else-if="activeModule === 'model'" key="model" />
          <LLMModule v-else-if="activeModule === 'llm'" key="llm" />
          <ShortcutsModule v-else-if="activeModule === 'shortcuts'" key="shortcuts" />
        </Transition>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import SettingsTitleBar from './SettingsTitleBar.vue';
import AppSidebar from '@/components/layout/AppSidebar.vue';
import JiraModule from './modules/JiraModule.vue';
import EmailModule from './modules/EmailModule.vue';
import ChatModule from './modules/ChatModule.vue';
import ModelConfigModule from './modules/ModelConfigModule.vue';
import LLMModule from '@/modules/settings/LLMSettings.vue';
import ShortcutsModule from './modules/ShortcutsModule.vue';
import type { NavItem } from '@/components/layout/types';
import type { SettingsModule } from './types';

const activeModule = ref<SettingsModule>('jira');

const sidebarItems: NavItem[] = [
  { id: 'jira', label: 'Jira', icon: '📋' },
  { id: 'email', label: '邮箱', icon: '📧' },
  { id: 'chat', label: 'IM', icon: '💬' },
  { id: 'model', label: '宠物', icon: '🐾' },
  { id: 'llm', label: 'AI模型', icon: '🤖' },
  { id: 'shortcuts', label: '快捷键', icon: '⌨️' },
];
</script>

<style scoped>
.settings-panel {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-base);
  color: var(--text);
  font-family: var(--font-sans);
  overflow: hidden;
}

.settings-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.settings-main {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity var(--t-med) ease-in-out;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.settings-main::-webkit-scrollbar {
  width: 5px;
}
.settings-main::-webkit-scrollbar-track {
  background: transparent;
}
.settings-main::-webkit-scrollbar-thumb {
  background: var(--border-strong);
  border-radius: 3px;
}

.version-text {
  font-size: 11px;
  color: var(--text-dim);
}
</style>
