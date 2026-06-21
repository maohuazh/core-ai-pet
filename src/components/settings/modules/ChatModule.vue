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
        @toggle="handleToggle(platform.id, $event)"
        @action="handleAction(platform)"
        @menu="handleMenu(platform)"
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import ConnectionCard from '../shared/ConnectionCard.vue';
import EmptyState from '../shared/EmptyState.vue';
import ConfirmDialog from '../shared/ConfirmDialog.vue';
import type { ChatPlatform } from '../types';

const platforms = ref<ChatPlatform[]>([]);
const showDeleteDialog = ref(false);
const selectedPlatform = ref<ChatPlatform | null>(null);

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
    // Disconnect
    selectedPlatform.value = platform;
    showDeleteDialog.value = true;
  } else {
    // Connect
    alert('连接功能开发中');
  }
};

const handleMenu = (platform: ChatPlatform) => {
  const action = prompt(`操作: ${platform.name}\n1. 编辑名称\n2. 删除\n请输入选项 (1/2):`);
  if (action === '1') {
    const newName = prompt('请输入新名称:', platform.name);
    if (newName && newName !== platform.name) {
      // For now, just update locally since we don't have an update command
      platform.name = newName;
      alert('名称已更新（重启后生效）');
    }
  } else if (action === '2') {
    selectedPlatform.value = platform;
    showDeleteDialog.value = true;
  }
};

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
  alert('添加聊天工具连接功能开发中');
};

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

.platform-details {
  font-size: 12px;
  color: #6b7280;
  line-height: 1.6;
}

.platform-details p {
  margin: 0;
}
</style>
