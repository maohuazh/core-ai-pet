<template>
  <div class="module-container">
    <!-- Model List View -->
    <template v-if="!showMappingPanel">
      <div class="module-header">
        <h2 class="module-title">宠物配置</h2>
        <button class="add-btn" @click="handleImport">+ 导入</button>
      </div>
      <div class="module-content">
        <EmptyState
          v-if="models.length === 0"
          title="暂无宠物"
          description="点击导入按钮添加第一个宠物"
          action-label="导入宠物"
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
            <div class="menu-wrap">
              <button
                :ref="(el) => setMenuBtnRef(model.id, el)"
                class="menu-btn"
                :class="{ active: openMenuId === model.id }"
                @click.stop="toggleMenu(model)"
              >
                <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                  <circle cx="8" cy="3" r="1.5" fill="currentColor" />
                  <circle cx="8" cy="8" r="1.5" fill="currentColor" />
                  <circle cx="8" cy="13" r="1.5" fill="currentColor" />
                </svg>
              </button>
              <AppMenu
                :open="openMenuId === model.id"
                :anchor="menuBtnRefs[model.id] ?? null"
                :items="buildMenuItems(model)"
                placement="bottom-end"
                @update:open="(v) => (openMenuId = v ? model.id : null)"
              />
            </div>
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
              ✓ 当前宠物
            </button>
            <button
              v-else
              class="action-btn primary"
              @click="handleActivate(model)"
            >
              ▶ 使用此宠物
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
    </template>

    <!-- Action Mapping Panel View -->
    <ActionMappingPanel
      v-else
      :model-id="selectedMappingModel!.id"
      :model-name="selectedMappingModel!.name"
      @back="showMappingPanel = false"
    />

    <ConfirmDialog
      v-model:visible="showDeleteDialog"
      title="删除宠物"
      :message="`确定要删除 '${selectedModel?.name}' 吗？此操作不可恢复。`"
      confirm-text="删除"
      confirm-class="danger"
      @confirm="confirmDelete"
    />

    <RenameModal
      v-model:visible="showRenameModal"
      :initial-value="renameTarget?.name ?? ''"
      title="编辑宠物名称"
      label="名称"
      @submit="onRenameSubmit"
    />

    <ComingSoonModal v-model:visible="showComingSoon" :message="comingSoonMessage" />

    <AppModal
      :open="showActiveBlock"
      title="无法删除"
      max-width="360px"
      @update:open="showActiveBlock = $event"
    >
      <p class="block-msg">无法删除当前活跃的宠物，请先切换到其他宠物。</p>
      <template #footer>
        <button class="btn-ok" @click="showActiveBlock = false">好的</button>
      </template>
    </AppModal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import EmptyState from '../shared/EmptyState.vue';
import ConfirmDialog from '../shared/ConfirmDialog.vue';
import RenameModal from '../shared/RenameModal.vue';
import ComingSoonModal from '../shared/ComingSoonModal.vue';
import AppModal from '../../ui/AppModal.vue';
import AppMenu, { type MenuItem } from '../../ui/AppMenu.vue';
import ActionMappingPanel from '../../action-mapping/ActionMappingPanel.vue';
import type { Model } from '../types';

const models = ref<Model[]>([]);
const showDeleteDialog = ref(false);
const selectedModel = ref<Model | null>(null);
const showMappingPanel = ref(false);
const selectedMappingModel = ref<Model | null>(null);

const showRenameModal = ref(false);
const renameTarget = ref<Model | null>(null);
const showComingSoon = ref(false);
const comingSoonMessage = ref('');
const showActiveBlock = ref(false);

const openMenuId = ref<string | null>(null);
const menuBtnRefs = ref<Record<string, HTMLElement | null>>({});

function setMenuBtnRef(id: string, el: any) {
  menuBtnRefs.value[id] = el as HTMLElement | null;
}

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
    models.value.forEach((m) => {
      m.status = m.id === model.id ? 'active' : 'inactive';
    });
  } catch (error) {
    console.error('Failed to activate model:', error);
  }
};

const handleActions = (model: Model) => {
  selectedMappingModel.value = model;
  showMappingPanel.value = true;
};

function toggleMenu(model: Model) {
  openMenuId.value = openMenuId.value === model.id ? null : model.id;
}

function buildMenuItems(model: Model): MenuItem[] {
  const items: MenuItem[] = [
    {
      id: 'rename',
      label: '编辑名称',
      icon: '✏️',
      onSelect: () => {
        renameTarget.value = model;
        showRenameModal.value = true;
      },
    },
  ];
  if (model.source !== 'builtin') {
    items.push({ kind: 'divider' });
    items.push({
      id: 'delete',
      label: '删除',
      icon: '🗑',
      danger: true,
      onSelect: () => handleDelete(model),
    });
  }
  return items;
}

async function onRenameSubmit(newName: string) {
  if (!renameTarget.value) return;
  const target = renameTarget.value;
  try {
    await invoke('update_model', { id: target.id, name: newName });
    target.name = newName;
  } catch (error) {
    console.error('Failed to update model:', error);
  }
}

const handleDelete = (model: Model) => {
  if (model.status === 'active') {
    showActiveBlock.value = true;
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
  comingSoonMessage.value = '宠物导入功能开发中';
  showComingSoon.value = true;
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
  font-size: 16px;
  font-weight: 600;
  color: var(--text);
  margin: 0;
}

.add-btn {
  padding: 7px 14px;
  border: none;
  border-radius: var(--r-lg);
  background: var(--accent);
  color: var(--bg-base);
  font-size: 13px;
  font-weight: 600;
  font-family: inherit;
  cursor: pointer;
  transition: background var(--t-fast);
}

.add-btn:hover {
  background: var(--accent-hover);
}

.module-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.model-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-strong);
  border-radius: 12px;
  padding: 14px 16px;
  transition: border-color var(--t-fast);
}

.model-card:hover {
  border-color: var(--accent);
}

.model-card.active {
  border-left: 3px solid var(--accent);
}

.model-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 10px;
}

.model-icon {
  font-size: 22px;
  width: 38px;
  height: 38px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-base);
  border: 1px solid var(--border-strong);
  border-radius: var(--r-lg);
  flex-shrink: 0;
}

.model-info {
  flex: 1;
  min-width: 0;
}

.model-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text);
  margin: 0 0 3px 0;
}

.model-meta {
  font-size: 12px;
  color: var(--text-dim);
  margin: 0;
}

.menu-wrap {
  position: relative;
}

.menu-btn {
  width: 26px;
  height: 26px;
  border: none;
  border-radius: var(--r-md);
  background: transparent;
  color: var(--text-dim);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background var(--t-fast), color var(--t-fast);
}

.menu-btn:hover,
.menu-btn.active {
  background: var(--bg-hover);
  color: var(--text);
}

.model-description {
  font-size: 12px;
  color: var(--text-muted);
  margin-bottom: 12px;
  line-height: 1.55;
}

.model-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.action-btn {
  padding: 6px 12px;
  border: none;
  border-radius: var(--r-md);
  font-size: 12px;
  font-weight: 500;
  font-family: inherit;
  cursor: pointer;
  transition: background var(--t-fast);
}

.action-btn.active {
  background: rgba(166, 227, 161, 0.12);
  color: var(--success);
  cursor: not-allowed;
}

.action-btn.primary {
  background: var(--accent);
  color: var(--bg-base);
}

.action-btn.primary:hover {
  background: var(--accent-hover);
}

.action-btn.secondary {
  background: var(--bg-hover);
  color: var(--text);
}

.action-btn.secondary:hover {
  background: var(--bg-hover-2);
}

.action-btn.danger {
  background: rgba(243, 139, 168, 0.12);
  color: var(--danger);
}

.action-btn.danger:hover {
  background: rgba(243, 139, 168, 0.2);
}

.block-msg {
  margin: 0;
  font-size: 13px;
  color: var(--text-muted);
  line-height: 1.55;
}

.btn-ok {
  padding: 6px 16px;
  border: none;
  border-radius: var(--r-lg);
  background: var(--accent);
  color: var(--bg-base);
  font-size: 13px;
  font-weight: 600;
  font-family: inherit;
  cursor: pointer;
  transition: background var(--t-fast);
}

.btn-ok:hover {
  background: var(--accent-hover);
}
</style>
