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
        @toggle="handleToggle(conn.id, $event)"
        @action="handleAction(conn)"
        @menu="handleMenu(conn)"
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import ConnectionCard from '../shared/ConnectionCard.vue';
import EmptyState from '../shared/EmptyState.vue';
import ConfirmDialog from '../shared/ConfirmDialog.vue';
import type { JiraConnection } from '../types';

const connections = ref<JiraConnection[]>([]);
const showDeleteDialog = ref(false);
const selectedConnection = ref<JiraConnection | null>(null);

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
    // Revoke authorization
    selectedConnection.value = conn;
    showDeleteDialog.value = true;
  } else {
    // Re-authorize
    alert('重新授权功能开发中');
  }
};

const handleMenu = (conn: JiraConnection) => {
  const action = prompt(`操作: ${conn.name}\n1. 编辑名称\n2. 删除\n请输入选项 (1/2):`);
  if (action === '1') {
    const newName = prompt('请输入新名称:', conn.name);
    if (newName && newName !== conn.name) {
      invoke('update_jira_connection', { id: conn.id, name: newName })
        .then(() => {
          conn.name = newName;
        })
        .catch((error) => {
          console.error('Failed to update connection:', error);
        });
    }
  } else if (action === '2') {
    selectedConnection.value = conn;
    showDeleteDialog.value = true;
  }
};

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
  alert('添加 Jira 连接功能开发中');
};

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

.conn-details {
  font-size: 12px;
  color: #6b7280;
  line-height: 1.6;
}

.conn-details p {
  margin: 0;
}
</style>
