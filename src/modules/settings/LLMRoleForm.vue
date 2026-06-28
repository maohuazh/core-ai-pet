<script setup lang="ts">
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import AppSelect, { type SelectOption } from '@/components/ui/AppSelect.vue';
import type { LLMConfig, LLMProvider } from '@/core/llm/types';

const props = defineProps<{
  role: string;
  config: LLMConfig | null;
}>();

const emit = defineEmits<{
  save: [config: LLMConfig];
  saved: [config: LLMConfig];
}>();

// Form state
const provider = ref<LLMProvider>('anthropic');
const model = ref('claude-3-5-sonnet-20241022');
const baseUrl = ref('https://api.anthropic.com');
const apiKey = ref(''); // Actual API key input (not secret_ref)
const temperature = ref(0.7);
const maxTokens = ref(4096);
const existingSecretRef = ref(''); // Track existing secret_ref from loaded config

const providerOptions: SelectOption[] = [
  { value: 'anthropic', label: 'Anthropic (兼容协议)' },
  { value: 'openai', label: 'OpenAI (兼容协议)' },
  { value: 'mock', label: 'Mock (调试用)', disabled: true },
];

// UI state
const testing = ref(false);
const testResult = ref<'success' | 'error' | null>(null);
const testError = ref<string>('');
const saving = ref(false);
const saveResult = ref<'success' | 'error' | null>(null);
const saveError = ref<string>('');
const validationErrors = ref<Record<string, string>>({});

// Initialize form from config
watch(() => props.config, (newConfig) => {
  if (newConfig) {
    provider.value = newConfig.provider;
    model.value = newConfig.model;
    baseUrl.value = newConfig.base_url;
    existingSecretRef.value = newConfig.secret_ref;
    temperature.value = newConfig.params.temperature;
    maxTokens.value = newConfig.params.max_tokens;
    // Don't load existing secret - user must enter new API key to change it
    apiKey.value = '';
  }
}, { immediate: true });

// When provider changes, update default model and base_url
watch(provider, (newProvider) => {
  switch (newProvider) {
    case 'anthropic':
      model.value = 'claude-3-5-sonnet-20241022';
      baseUrl.value = 'https://api.anthropic.com';
      break;
    case 'openai':
      model.value = 'gpt-4o';
      baseUrl.value = 'https://api.openai.com';
      break;
    case 'mock':
      model.value = 'mock-model';
      baseUrl.value = 'http://localhost:11434';
      break;
  }
});

// Validation
function validate(): boolean {
  const errors: Record<string, string> = {};

  if (!model.value.trim()) {
    errors.model = '模型名称不能为空';
  }

  if (!baseUrl.value.trim()) {
    errors.baseUrl = 'API 地址不能为空';
  } else {
    try {
      new URL(baseUrl.value);
    } catch {
      errors.baseUrl = '请输入有效的 URL';
    }
  }

  // API key is required only if no existing secret_ref
  if (!apiKey.value.trim() && !existingSecretRef.value) {
    errors.apiKey = 'API 密钥不能为空';
  }

  if (temperature.value < 0 || temperature.value > 2) {
    errors.temperature = '温度必须在 0-2 之间';
  }

  if (maxTokens.value < 1 || maxTokens.value > 1000000) {
    errors.maxTokens = '最大 token 数必须在 1-1000000 之间';
  }

  validationErrors.value = errors;
  return Object.keys(errors).length === 0;
}

// Test connection
async function testConnection() {
  if (!validate()) {
    return;
  }

  testing.value = true;
  testResult.value = null;
  testError.value = '';

  try {
    // Build flat config payload (matching LLMConfigPayload structure)
    const tempConfig = {
      role: props.role,
      provider: provider.value,
      model: model.value,
      base_url: baseUrl.value,
      secret_ref: existingSecretRef.value || 'temp',
      temperature: temperature.value,
      max_tokens: maxTokens.value
    };

    // Call test connection with config and optional API key
    const result = await invoke<{ ok: boolean; reason?: string }>('llm_test_connection', {
      role: props.role,
      config: tempConfig,
      apiKey: apiKey.value || undefined // Pass API key if provided
    });

    if (result.ok) {
      testResult.value = 'success';
    } else {
      testResult.value = 'error';
      testError.value = result.reason || '连接失败';
    }
  } catch (e: any) {
    testResult.value = 'error';
    testError.value = e.toString();
  } finally {
    testing.value = false;
  }
}

// Save configuration
async function save() {
  if (!validate()) {
    return;
  }

  saving.value = true;
  saveResult.value = null;
  saveError.value = '';

  try {
    // If API key is provided, save it to keyring first
    let secretRef = existingSecretRef.value;
    if (apiKey.value.trim()) {
      const result = await invoke<{ secret_ref: string }>('llm_save_secret', {
        role: props.role,
        plaintext: apiKey.value
      });
      secretRef = result.secret_ref;
      existingSecretRef.value = secretRef;
      apiKey.value = '';
    }

    // Build flat config for backend (matching LLMConfigPayload)
    const flatConfig = {
      role: props.role,
      provider: provider.value,
      model: model.value,
      base_url: baseUrl.value,
      secret_ref: secretRef,
      temperature: temperature.value,
      max_tokens: maxTokens.value
    };

    // Directly invoke backend — properly awaits and catches errors
    console.log('[LLM] Saving config for role:', props.role, 'flatConfig:', flatConfig);
    await invoke('llm_save_config', {
      role: props.role,
      cfg: flatConfig
    });
    console.log('[LLM] Save invoke succeeded');

    // Build nested config for parent state
    const config: LLMConfig = {
      provider: provider.value,
      model: model.value,
      base_url: baseUrl.value,
      secret_ref: secretRef,
      role: props.role,
      params: {
        temperature: temperature.value,
        max_tokens: maxTokens.value
      }
    };

    emit('save', config);
    emit('saved', config);
    saveResult.value = 'success';

    // Auto-hide success message after 3 seconds
    setTimeout(() => {
      saveResult.value = null;
    }, 3000);
  } catch (e: any) {
    console.error('[LLM] Save failed:', e);
    saveResult.value = 'error';
    saveError.value = e.toString();
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <div class="llm-role-form">
    <div class="form-group">
      <label>Provider</label>
      <AppSelect
        v-model="provider"
        :options="providerOptions"
        placeholder="选择 Provider"
      />
    </div>

    <div class="form-group">
      <label>模型名称</label>
      <input
        v-model="model"
        type="text"
        placeholder="例如: claude-3-5-sonnet-20241022"
        :class="{ error: validationErrors.model }"
      />
      <span v-if="validationErrors.model" class="error-text">
        {{ validationErrors.model }}
      </span>
    </div>

    <div class="form-group">
      <label>API 地址</label>
      <input
        v-model="baseUrl"
        type="text"
        placeholder="https://api.anthropic.com"
        :class="{ error: validationErrors.baseUrl }"
      />
      <span v-if="validationErrors.baseUrl" class="error-text">
        {{ validationErrors.baseUrl }}
      </span>
    </div>

    <div class="form-group">
      <label>API 密钥</label>
      <input
        v-model="apiKey"
        type="password"
        :placeholder="existingSecretRef ? '已配置，输入新密钥以更新' : 'sk-ant-...'"
        :class="{ error: validationErrors.apiKey }"
      />
      <span v-if="validationErrors.apiKey" class="error-text">
        {{ validationErrors.apiKey }}
      </span>
      <small>API 密钥通过系统密钥链安全存储</small>
    </div>

    <div class="form-group">
      <label>温度 (Temperature)</label>
      <input
        v-model.number="temperature"
        type="number"
        step="0.1"
        min="0"
        max="2"
        :class="{ error: validationErrors.temperature }"
      />
      <span v-if="validationErrors.temperature" class="error-text">
        {{ validationErrors.temperature }}
      </span>
      <small>控制输出的随机性 (0-2)</small>
    </div>

    <div class="form-group">
      <label>最大 Token 数</label>
      <input
        v-model.number="maxTokens"
        type="number"
        min="1"
        max="1000000"
        :class="{ error: validationErrors.maxTokens }"
      />
      <span v-if="validationErrors.maxTokens" class="error-text">
        {{ validationErrors.maxTokens }}
      </span>
    </div>

    <div class="actions">
      <button
        @click="testConnection"
        :disabled="testing || saving"
        class="test-btn"
      >
        <span v-if="testing">测试中...</span>
        <span v-else>🔗 测试连接</span>
      </button>

      <button
        @click="save"
        :disabled="saving || testing"
        class="save-btn"
      >
        <span v-if="saving">保存中...</span>
        <span v-else>💾 保存配置</span>
      </button>
    </div>

    <div v-if="testResult === 'success'" class="test-success">
      ✓ 连接成功
    </div>

    <div v-if="testResult === 'error'" class="test-error">
      ✗ 连接失败: {{ testError }}
    </div>

    <div v-if="saveResult === 'success'" class="save-success">
      ✓ 配置已保存
    </div>

    <div v-if="saveResult === 'error'" class="save-error">
      ✗ 保存失败: {{ saveError }}
    </div>
  </div>
</template>

<style scoped>
.llm-role-form {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

label {
  font-weight: 500;
  font-size: 12px;
  color: var(--text-muted);
}

input, select {
  padding: 7px 10px;
  border: 1px solid var(--border-strong);
  background: var(--bg-base);
  color: var(--text);
  border-radius: var(--r-lg);
  font-size: 13px;
  font-family: inherit;
  transition: border-color var(--t-fast);
}

input::placeholder {
  color: var(--text-dim);
}

input:focus, select:focus {
  outline: none;
  border-color: var(--accent);
}

input.error {
  border-color: var(--danger);
}

.error-text {
  color: var(--danger);
  font-size: 11px;
}

small {
  color: var(--text-dim);
  font-size: 11px;
}

.actions {
  display: flex;
  gap: 8px;
  margin-top: 6px;
}

button {
  padding: 7px 16px;
  border: none;
  border-radius: var(--r-lg);
  font-size: 13px;
  font-weight: 500;
  font-family: inherit;
  cursor: pointer;
  transition: background var(--t-fast), color var(--t-fast);
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.test-btn {
  background: var(--bg-hover);
  color: var(--text);
}

.test-btn:hover:not(:disabled) {
  background: var(--bg-hover-2);
}

.save-btn {
  background: var(--accent);
  color: var(--bg-base);
  font-weight: 600;
}

.save-btn:hover:not(:disabled) {
  background: var(--accent-hover);
}

.test-success,
.save-success {
  padding: 8px 12px;
  background: rgba(166, 227, 161, 0.1);
  color: var(--success);
  border: 1px solid rgba(166, 227, 161, 0.3);
  border-radius: var(--r-lg);
  font-size: 12px;
}

.save-success {
  font-weight: 500;
}

.test-error,
.save-error {
  padding: 8px 12px;
  background: rgba(243, 139, 168, 0.1);
  color: var(--danger);
  border: 1px solid rgba(243, 139, 168, 0.3);
  border-radius: var(--r-lg);
  font-size: 12px;
}
</style>
