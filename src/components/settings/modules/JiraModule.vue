<template>
  <div class="module-container">
    <div class="module-header">
      <h2 class="module-title">Jira 连接</h2>
      <button class="add-btn" @click="handleAdd">+ 添加</button>
    </div>
    <div class="module-content">
      <EmptyState
        v-if="connections.length === 0"
        title="暂无 Jira 连接"
        description="点击添加按钮创建第一个 Jira 连接"
        action-label="添加连接"
        @action="handleAdd"
      />
      <ConnectionCard
        v-for="conn in connections"
        :key="conn.id"
        :name="conn.name"
        :subtitle="conn.url"
        :status="conn.status"
        :enabled="conn.enabled"
        :action-label="getActionLabel(conn.status)"
        :menu-items="buildMenuItems(conn)"
        @toggle="handleToggle(conn.id, $event)"
        @action="handleAction(conn)"
      >
        <template #icon>🔗</template>
        <div class="conn-details">
          <p>邮箱: {{ conn.email }}</p>
          <p v-if="conn.last_sync_at">最后同步: {{ formatDate(conn.last_sync_at) }}</p>
          <p v-else>尚未同步</p>
        </div>
      </ConnectionCard>
    </div>

    <ConfirmDialog
      v-model:visible="showDeleteDialog"
      title="删除连接"
      :message="`确定要删除 '${selectedConnection?.name}' 吗？此操作不可恢复。`"
      confirm-text="删除"
      confirm-class="danger"
      @confirm="confirmDelete"
    />

    <RenameModal
      v-model:visible="showRenameModal"
      :initial-value="renameTarget?.name ?? ''"
      title="编辑连接名称"
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
import type { JiraConnection } from '../types';

const connections = ref<JiraConnection[]>([]);
const showDeleteDialog = ref(false);
const selectedConnection = ref<JiraConnection | null>(null);
const showRenameModal = ref(false);
const renameTarget = ref<JiraConnection | null>(null);
const showComingSoon = ref(false);
const comingSoonMessage = ref('');

const loadConnections = async () => {
  try {
    connections.value = await invoke('get_jira_connections');
  } catch (error) {
    console.error('Failed to load Jira connections:', error);
  }
};

const handleToggle = async (id: string, enabled: boolean) => {
  try {
    await invoke('toggle_jira_connection', { id, enabled });
    const conn = connections.value.find((c) => c.id === id);
    if (conn) {
      conn.enabled = enabled;
    }
  } catch (error) {
    console.error('Failed to toggle connection:', error);
  }
};

const handleAction = (conn: JiraConnection) => {
  if (conn.status === 'connected') {
    selectedConnection.value = conn;
    showDeleteDialog.value = true;
  } else {
    openComingSoon('重新授权功能开发中');
  }
};

function buildMenuItems(conn: JiraConnection): MenuItem[] {
  return [
    {
      id: 'rename',
      label: '编辑名称',
      icon: '✏️',
      onSelect: () => {
        renameTarget.value = conn;
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
        selectedConnection.value = conn;
        showDeleteDialog.value = true;
      },
    },
  ];
}

async function onRenameSubmit(newName: string) {
  if (!renameTarget.value) return;
  const target = renameTarget.value;
  try {
    await invoke('update_jira_connection', { id: target.id, name: newName });
    target.name = newName;
  } catch (error) {
    console.error('Failed to update connection:', error);
  }
}

const confirmDelete = async () => {
  if (!selectedConnection.value) return;
  try {
    await invoke('delete_jira_connection', { id: selectedConnection.value.id });
    connections.value = connections.value.filter((c) => c.id !== selectedConnection.value!.id);
  } catch (error) {
    console.error('Failed to delete connection:', error);
  }
};

const handleAdd = () => {
  openComingSoon('添加 Jira 连接功能开发中');
};

function openComingSoon(msg: string) {
  comingSoonMessage.value = msg;
  showComingSoon.value = true;
}

const getActionLabel = (status: string) => {
  switch (status) {
    case 'connected':
      return '撤销授权';
    case 'expired':
      return '重新授权';
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
  loadConnections();
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

.conn-details {
  font-size: 12px;
  color: var(--text-dim);
  line-height: 1.6;
}

.conn-details p {
  margin: 0;
}
</style>
