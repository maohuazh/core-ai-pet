<template>
  <div class="module-container">
    <div class="module-header">
      <h2 class="module-title">模型配置</h2>
      <button class="add-btn" @click="handleImport">+ 导入</button>
    </div>
    <div class="module-content">
      <EmptyState
        v-if="models.length === 0"
        title="暂无模型"
        description="点击导入按钮添加第一个模型"
        action-label="导入模型"
        @action="handleImport"
      />
      <div
        v-for="model in models"
        :key="model.id"
        :class="['model-card', { active: model.status === 'active' }]"
      >
        <div class="model-header">
          <div class="model-icon">
            {{ model.model_type === 'live2d' ? '🎭' : '🖼️' }}
          </div>
          <div class="model-info">
            <h3 class="model-name">{{ model.name }}</h3>
            <p class="model-meta">
              {{ model.model_type === 'live2d' ? 'Live2D' : 'Sprite' }}
              <span v-if="model.author"> · {{ model.author }}</span>
              <span v-if="model.version"> · v{{ model.version }}</span>
            </p>
          </div>
          <button class="menu-btn" @click.stop="handleMenu(model)">
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
              <circle cx="8" cy="3" r="1.5" fill="currentColor" />
              <circle cx="8" cy="8" r="1.5" fill="currentColor" />
              <circle cx="8" cy="13" r="1.5" fill="currentColor" />
            </svg>
          </button>
        </div>
        <div v-if="model.description" class="model-description">
          {{ model.description }}
        </div>
        <div class="model-actions">
          <button
            v-if="model.status === 'active'"
            class="action-btn active"
            disabled
          >
            ✓ 当前模型
          </button>
          <button
            v-else
            class="action-btn primary"
            @click="handleActivate(model)"
          >
            ▶ 使用此模型
          </button>
          <button class="action-btn secondary" @click="handleActions(model)">
            ⚙ 动作映射
          </button>
          <button
            v-if="model.source !== 'builtin'"
            class="action-btn danger"
            @click="handleDelete(model)"
          >
            🗑 删除
          </button>
        </div>
      </div>
    </div>

    <ConfirmDialog
      v-model:visible="showDeleteDialog"
      title="删除模型"
      :message="`确定要删除 '${selectedModel?.name}' 吗？此操作不可恢复。`"
      confirm-text="删除"
      confirm-class="danger"
      @confirm="confirmDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import EmptyState from '../shared/EmptyState.vue';
import ConfirmDialog from '../shared/ConfirmDialog.vue';
import type { Model } from '../types';

const models = ref<Model[]>([]);
const showDeleteDialog = ref(false);
const selectedModel = ref<Model | null>(null);

const loadModels = async () => {
  try {
    models.value = await invoke('get_models');
  } catch (error) {
    console.error('Failed to load models:', error);
  }
};

const handleActivate = async (model: Model) => {
  try {
    await invoke('set_active_model', { id: model.id });
    // Update local state
    models.value.forEach((m) => {
      m.status = m.id === model.id ? 'active' : 'inactive';
    });
  } catch (error) {
    console.error('Failed to activate model:', error);
  }
};

const handleActions = (model: Model) => {
  alert(`动作映射功能开发中\n模型: ${model.name}`);
};

const handleMenu = (model: Model) => {
  const action = prompt(`操作: ${model.name}\n1. 编辑名称\n2. 删除\n请输入选项 (1/2):`);
  if (action === '1') {
    const newName = prompt('请输入新名称:', model.name);
    if (newName && newName !== model.name) {
      invoke('update_model', { id: model.id, name: newName })
        .then(() => {
          model.name = newName;
        })
        .catch((error) => {
          console.error('Failed to update model:', error);
        });
    }
  } else if (action === '2') {
    handleDelete(model);
  }
};

const handleDelete = (model: Model) => {
  if (model.status === 'active') {
    alert('无法删除当前活跃的模型，请先切换到其他模型');
    return;
  }
  selectedModel.value = model;
  showDeleteDialog.value = true;
};

const confirmDelete = async () => {
  if (!selectedModel.value) return;
  try {
    await invoke('delete_model', { id: selectedModel.value.id });
    models.value = models.value.filter((m) => m.id !== selectedModel.value!.id);
  } catch (error) {
    console.error('Failed to delete model:', error);
  }
};

const handleImport = () => {
  alert('模型导入功能开发中');
};

onMounted(() => {
  loadModels();
});
</script>

<style scoped>
.module-container {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.module-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.module-title {
  font-size: 18px;
  font-weight: 600;
  color: #1f2937;
  margin: 0;
}

.add-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  background: #6366f1;
  color: white;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.add-btn:hover {
  background: #818cf8;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(99, 102, 241, 0.2);
}

.add-btn:active {
  background: #4f46e5;
  transform: translateY(0);
}

.module-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.model-card {
  background: rgba(255, 255, 255, 0.6);
  border: 1px solid rgba(0, 0, 0, 0.04);
  border-radius: 12px;
  padding: 16px;
  transition: all 0.2s ease;
}

.model-card:hover {
  background: rgba(255, 255, 255, 0.8);
  border-color: rgba(99, 102, 241, 0.15);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.06);
}

.model-card.active {
  border-left: 3px solid #6366f1;
}

.model-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 12px;
}

.model-icon {
  font-size: 24px;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(99, 102, 241, 0.1);
  border-radius: 8px;
  flex-shrink: 0;
}

.model-info {
  flex: 1;
  min-width: 0;
}

.model-name {
  font-size: 14px;
  font-weight: 500;
  color: #1f2937;
  margin: 0 0 4px 0;
}

.model-meta {
  font-size: 12px;
  color: #6b7280;
  margin: 0;
}

.menu-btn {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: #9ca3af;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.menu-btn:hover {
  background: rgba(0, 0, 0, 0.05);
  color: #6b7280;
}

.model-description {
  font-size: 13px;
  color: #6b7280;
  margin-bottom: 12px;
  line-height: 1.5;
}

.model-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.action-btn {
  padding: 6px 12px;
  border: none;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.action-btn.active {
  background: rgba(16, 185, 129, 0.1);
  color: #10b981;
  cursor: not-allowed;
}

.action-btn.primary {
  background: #6366f1;
  color: white;
}

.action-btn.primary:hover {
  background: #818cf8;
}

.action-btn.secondary {
  background: rgba(0, 0, 0, 0.05);
  color: #6b7280;
}

.action-btn.secondary:hover {
  background: rgba(0, 0, 0, 0.08);
  color: #1f2937;
}

.action-btn.danger {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.action-btn.danger:hover {
  background: rgba(239, 68, 68, 0.15);
}
</style>
