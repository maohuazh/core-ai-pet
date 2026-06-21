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
        @toggle="handleToggle(account.id, $event)"
        @action="handleAction(account)"
        @menu="handleMenu(account)"
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import ConnectionCard from '../shared/ConnectionCard.vue';
import EmptyState from '../shared/EmptyState.vue';
import ConfirmDialog from '../shared/ConfirmDialog.vue';
import type { EmailAccount } from '../types';

const accounts = ref<EmailAccount[]>([]);
const showDeleteDialog = ref(false);
const selectedAccount = ref<EmailAccount | null>(null);

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
    // Revoke authorization
    selectedAccount.value = account;
    showDeleteDialog.value = true;
  } else {
    // Re-authorize
    alert('重新授权功能开发中');
  }
};

const handleMenu = (account: EmailAccount) => {
  const action = prompt(`操作: ${account.name}\n1. 编辑名称\n2. 删除\n请输入选项 (1/2):`);
  if (action === '1') {
    const newName = prompt('请输入新名称:', account.name);
    if (newName && newName !== account.name) {
      invoke('update_email_account', { id: account.id, name: newName })
        .then(() => {
          account.name = newName;
        })
        .catch((error) => {
          console.error('Failed to update account:', error);
        });
    }
  } else if (action === '2') {
    selectedAccount.value = account;
    showDeleteDialog.value = true;
  }
};

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
  alert('添加邮箱连接功能开发中');
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

.account-details {
  font-size: 12px;
  color: #6b7280;
  line-height: 1.6;
}

.account-details p {
  margin: 0;
}
</style>
