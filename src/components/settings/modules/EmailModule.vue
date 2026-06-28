<template>
  <div class="module-container">
    <div class="module-header">
      <h2 class="module-title">邮箱连接</h2>
      <button class="add-btn" @click="handleAdd">+ 添加</button>
    </div>
    <div class="module-content">
      <EmptyState
        v-if="accounts.length === 0"
        title="暂无邮箱连接"
        description="点击添加按钮创建第一个邮箱连接"
        action-label="添加连接"
        @action="handleAdd"
      />
      <ConnectionCard
        v-for="account in accounts"
        :key="account.id"
        :name="account.name"
        :subtitle="account.email"
        :status="account.status"
        :enabled="account.enabled"
        :action-label="getActionLabel(account.status)"
        :menu-items="buildMenuItems(account)"
        @toggle="handleToggle(account.id, $event)"
        @action="handleAction(account)"
      >
        <template #icon>{{ getProviderIcon(account.provider) }}</template>
        <div class="account-details">
          <p>提供商: {{ getProviderName(account.provider) }}</p>
          <p v-if="account.last_sync_at">最后同步: {{ formatDate(account.last_sync_at) }}</p>
          <p v-else>尚未同步</p>
        </div>
      </ConnectionCard>
    </div>

    <ConfirmDialog
      v-model:visible="showDeleteDialog"
      title="删除连接"
      :message="`确定要删除 '${selectedAccount?.name}' 吗？此操作不可恢复。`"
      confirm-text="删除"
      confirm-class="danger"
      @confirm="confirmDelete"
    />

    <RenameModal
      v-model:visible="showRenameModal"
      :initial-value="renameTarget?.name ?? ''"
      title="编辑邮箱名称"
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
import type { EmailAccount } from '../types';

const accounts = ref<EmailAccount[]>([]);
const showDeleteDialog = ref(false);
const selectedAccount = ref<EmailAccount | null>(null);
const showRenameModal = ref(false);
const renameTarget = ref<EmailAccount | null>(null);
const showComingSoon = ref(false);
const comingSoonMessage = ref('');

const loadAccounts = async () => {
  try {
    accounts.value = await invoke('get_email_accounts');
  } catch (error) {
    console.error('Failed to load email accounts:', error);
  }
};

const handleToggle = async (id: string, enabled: boolean) => {
  try {
    await invoke('toggle_email_account', { id, enabled });
    const account = accounts.value.find((a) => a.id === id);
    if (account) {
      account.enabled = enabled;
    }
  } catch (error) {
    console.error('Failed to toggle account:', error);
  }
};

const handleAction = (account: EmailAccount) => {
  if (account.status === 'connected') {
    selectedAccount.value = account;
    showDeleteDialog.value = true;
  } else {
    openComingSoon('重新授权功能开发中');
  }
};

function buildMenuItems(account: EmailAccount): MenuItem[] {
  return [
    {
      id: 'rename',
      label: '编辑名称',
      icon: '✏️',
      onSelect: () => {
        renameTarget.value = account;
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
        selectedAccount.value = account;
        showDeleteDialog.value = true;
      },
    },
  ];
}

async function onRenameSubmit(newName: string) {
  if (!renameTarget.value) return;
  const target = renameTarget.value;
  try {
    await invoke('update_email_account', { id: target.id, name: newName });
    target.name = newName;
  } catch (error) {
    console.error('Failed to update account:', error);
  }
}

const confirmDelete = async () => {
  if (!selectedAccount.value) return;
  try {
    await invoke('delete_email_account', { id: selectedAccount.value.id });
    accounts.value = accounts.value.filter((a) => a.id !== selectedAccount.value!.id);
  } catch (error) {
    console.error('Failed to delete account:', error);
  }
};

const handleAdd = () => {
  openComingSoon('添加邮箱连接功能开发中');
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

const getProviderIcon = (provider: string) => {
  switch (provider) {
    case 'gmail':
      return '📧';
    case 'outlook':
      return '📮';
    case 'imap':
      return '📬';
    default:
      return '📧';
  }
};

const getProviderName = (provider: string) => {
  switch (provider) {
    case 'gmail':
      return 'Gmail';
    case 'outlook':
      return 'Outlook';
    case 'imap':
      return 'IMAP';
    default:
      return provider;
  }
};

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr);
  return date.toLocaleString('zh-CN');
};

onMounted(() => {
  loadAccounts();
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

.account-details {
  font-size: 12px;
  color: var(--text-dim);
  line-height: 1.6;
}

.account-details p {
  margin: 0;
}
</style>
