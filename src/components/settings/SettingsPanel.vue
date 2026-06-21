<template>
  <div class="settings-panel">
    <SettingsTitleBar />
    <div class="settings-content">
      <SettingsSidebar v-model:activeModule="activeModule" />
      <div class="settings-main">
        <Transition name="fade" mode="out-in">
          <JiraModule v-if="activeModule === 'jira'" key="jira" />
          <EmailModule v-else-if="activeModule === 'email'" key="email" />
          <ChatModule v-else-if="activeModule === 'chat'" key="chat" />
          <ModelConfigModule v-else-if="activeModule === 'model'" key="model" />
        </Transition>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import SettingsTitleBar from './SettingsTitleBar.vue';
import SettingsSidebar from './SettingsSidebar.vue';
import JiraModule from './modules/JiraModule.vue';
import EmailModule from './modules/EmailModule.vue';
import ChatModule from './modules/ChatModule.vue';
import ModelConfigModule from './modules/ModelConfigModule.vue';
import type { SettingsModule } from './types';

const activeModule = ref<SettingsModule>('jira');
</script>

<style scoped>
.settings-panel {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: rgba(255, 255, 255, 0.75);
  backdrop-filter: blur(20px);
  border-radius: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);
  overflow: hidden;
}

.settings-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.settings-main {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

/* Fade transition for module switching */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.25s ease-in-out;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* Scrollbar styling */
.settings-main::-webkit-scrollbar {
  width: 8px;
}

.settings-main::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.02);
  border-radius: 4px;
}

.settings-main::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.1);
  border-radius: 4px;
}

.settings-main::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.15);
}
</style>
