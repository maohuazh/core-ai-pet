<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import LLMRoleForm from './LLMRoleForm.vue';
import type { LLMConfig } from '@/core/llm/types';

interface LLMSettings {
  roles: Array<{
    name: string;
    displayName: string;
    config: LLMConfig | null;
  }>;
}

const settings = ref<LLMSettings>({
  roles: [
    {
      name: 'chat_assistant',
      displayName: '聊天助手',
      config: null
    },
    {
      name: 'message_assistant',
      displayName: '消息助手',
      config: null
    }
  ]
});

const loading = ref(true);
const error = ref<string | null>(null);

onMounted(async () => {
  await loadConfigs();
});

async function loadConfigs() {
  loading.value = true;
  error.value = null;

  try {
    for (const role of settings.value.roles) {
      try {
        // Backend returns flat LLMConfigPayload, transform to nested LLMConfig
        const flat = await invoke<any>('llm_load_config', {
          role: role.name
        });
        role.config = {
          provider: flat.provider,
          model: flat.model,
          base_url: flat.base_url || '',
          secret_ref: flat.secret_ref,
          role: flat.role,
          params: {
            temperature: flat.temperature,
            max_tokens: flat.max_tokens
          }
        };
      } catch (e: any) {
        // Config not found, leave as null
        role.config = null;
      }
    }
  } catch (e: any) {
    error.value = `加载配置失败: ${e}`;
  } finally {
    loading.value = false;
  }
}

async function handleSave(roleName: string, config: LLMConfig) {
  // Update local state — the actual invoke is done in LLMRoleForm.vue
  const role = settings.value.roles.find(r => r.name === roleName);
  if (role) {
    role.config = config;
  }
}
</script>

<template>
  <div class="llm-settings">
    <h2>🤖 AI 模型配置</h2>

    <div v-if="loading" class="loading">
      加载中...
    </div>

    <div v-else-if="error" class="error">
      {{ error }}
    </div>

    <div v-else class="roles">
      <div v-for="role in settings.roles" :key="role.name" class="role-section">
        <h3>{{ role.displayName }}</h3>
        <LLMRoleForm
          :role="role.name"
          :config="role.config"
          @save="(config) => handleSave(role.name, config)"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.llm-settings {
  padding: 20px;
}

h2 {
  margin-bottom: 20px;
  font-size: 20px;
  font-weight: 600;
}

.loading {
  color: #666;
  padding: 20px;
  text-align: center;
}

.error {
  color: #d32f2f;
  padding: 12px;
  background-color: #ffebee;
  border-radius: 4px;
  margin-bottom: 16px;
}

.roles {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.role-section {
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  padding: 16px;
}

.role-section h3 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
}
</style>
