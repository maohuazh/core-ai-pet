<template>
  <div class="module-container">
    <div class="module-header">
      <h2 class="module-title">聊天工具连接</h2>
      <button class="add-btn" @click="handleAdd">+ 添加</button>
    </div>
    <div class="module-content">
      <EmptyState
        v-if="platforms.length === 0"
        title="暂无聊天工具连接"
        description="点击添加按钮创建第一个聊天工具连接"
        action-label="添加连接"
        @action="handleAdd"
      />
      <ConnectionCard
        v-for="platform in platforms"
        :key="platform.id"
        :name="platform.name"
        :subtitle="platform.account_name || '未连接'"
        :status="platform.status"
        :enabled="platform.enabled"
        :action-label="getActionLabel(platform.status)"
        :menu-items="buildMenuItems(platform)"
        @toggle="handleToggle(platform.id, $event)"
        @action="handleAction(platform)"
      >
        <template #icon>{{ platform.icon || '💬' }}</template>
        <div class="platform-details">
          <p v-if="platform.connected_at">连接时间: {{ formatDate(platform.connected_at) }}</p>
          <p v-else>尚未连接</p>
        </div>
      </ConnectionCard>
    </div>

    <ConfirmDialog
      v-model:visible="showDeleteDialog"
      title="删除连接"
      :message="`确定要删除 '${selectedPlatform?.name}' 吗？此操作不可恢复。`"
      confirm-text="删除"
      confirm-class="danger"
      @confirm="confirmDelete"
    />

    <RenameModal
      v-model:visible="showRenameModal"
      :initial-value="renameTarget?.name ?? ''"
      title="编辑名称"
      label="名称"
      @submit="onRenameSubmit"
    />

    <ComingSoonModal v-model:visible="showComingSoon" :message="comingSoonMessage" />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import ConnectionCard from '../shared/ConnectionCard.vue';
import EmptyState from '../shared/EmptyState.vue';
import ConfirmDialog from '../shared/ConfirmDialog.vue';
import RenameModal from '../shared/RenameModal.vue';
import ComingSoonModal from '../shared/ComingSoonModal.vue';
import type { MenuItem } from '../../ui/AppMenu.vue';
import type { ChatPlatform } from '../types';

const platforms = ref<ChatPlatform[]>([]);
const showDeleteDialog = ref(false);
const selectedPlatform = ref<ChatPlatform | null>(null);
const showRenameModal = ref(false);
const renameTarget = ref<ChatPlatform | null>(null);
const showComingSoon = ref(false);
const comingSoonMessage = ref('');

const loadPlatforms = async () => {
  try {
    platforms.value = await invoke('get_chat_platforms');
  } catch (error) {
    console.error('Failed to load chat platforms:', error);
  }
};

const handleToggle = async (id: string, enabled: boolean) => {
  try {
    await invoke('toggle_chat_platform', { id, enabled });
    const platform = platforms.value.find((p) => p.id === id);
    if (platform) {
      platform.enabled = enabled;
    }
  } catch (error) {
    console.error('Failed to toggle platform:', error);
  }
};

const handleAction = (platform: ChatPlatform) => {
  if (platform.status === 'connected') {
    selectedPlatform.value = platform;
    showDeleteDialog.value = true;
  } else {
    openComingSoon('连接功能开发中');
  }
};

function buildMenuItems(platform: ChatPlatform): MenuItem[] {
  return [
    {
      id: 'rename',
      label: '编辑名称',
      icon: '✏️',
      onSelect: () => {
        renameTarget.value = platform;
        showRenameModal.value = true;
      },
    },
    { kind: 'divider' },
    {
      id: 'delete',
      label: '删除',
      icon: '🗑',
      danger: true,
      onSelect: () => {
        selectedPlatform.value = platform;
        showDeleteDialog.value = true;
      },
    },
  ];
}

function onRenameSubmit(newName: string) {
  if (!renameTarget.value) return;
  // 没有后端 update 命令，仅本地更新（保持原逻辑）
  renameTarget.value.name = newName;
}

const confirmDelete = async () => {
  if (!selectedPlatform.value) return;
  try {
    if (selectedPlatform.value.status === 'connected') {
      await invoke('disconnect_chat_platform', { id: selectedPlatform.value.id });
      selectedPlatform.value.status = 'disconnected';
      selectedPlatform.value.enabled = false;
      selectedPlatform.value.account_name = null;
      selectedPlatform.value.connected_at = null;
    } else {
      await invoke('delete_chat_platform', { id: selectedPlatform.value.id });
      platforms.value = platforms.value.filter((p) => p.id !== selectedPlatform.value!.id);
    }
  } catch (error) {
    console.error('Failed to delete platform:', error);
  }
};

const handleAdd = () => {
  openComingSoon('添加聊天工具连接功能开发中');
};

function openComingSoon(msg: string) {
  comingSoonMessage.value = msg;
  showComingSoon.value = true;
}

const getActionLabel = (status: string) => {
  switch (status) {
    case 'connected':
      return '断开连接';
    case 'disconnected':
      return '连接';
    case 'error':
      return '重试';
    default:
      return '操作';
  }
};

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr);
  return date.toLocaleString('zh-CN');
};

onMounted(() => {
  loadPlatforms();
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

.platform-details {
  font-size: 12px;
  color: var(--text-dim);
  line-height: 1.6;
}

.platform-details p {
  margin: 0;
}
</style>
