<template>
  <AppModal
    :open="visible"
    :title="title"
    :close-on-overlay="closeOnOverlay"
    :max-width="'420px'"
    @update:open="onOpenChange"
  >
    <p class="dialog-message">{{ message }}</p>
    <template #footer>
      <button class="dialog-btn cancel-btn" @click="cancel">{{ cancelText }}</button>
      <button :class="['dialog-btn', 'confirm-btn', confirmClass]" @click="confirm">
        {{ confirmText }}
      </button>
    </template>
  </AppModal>
</template>

<script setup lang="ts">
/**
 * 兼容包装：保留原 ConfirmDialog 的 props/emits，内部委托给 AppModal。
 * 现有所有 `<ConfirmDialog v-model:visible @confirm @cancel>` 调用方零改动。
 */
import AppModal from '../../ui/AppModal.vue';

const props = withDefaults(
  defineProps<{
    visible: boolean;
    title?: string;
    message: string;
    confirmText?: string;
    cancelText?: string;
    confirmClass?: 'danger' | 'warning' | 'primary';
    closeOnOverlay?: boolean;
  }>(),
  {
    title: '确认',
    confirmText: '确定',
    cancelText: '取消',
    confirmClass: 'primary',
    closeOnOverlay: true,
  }
);

const emit = defineEmits<{
  'update:visible': [visible: boolean];
  confirm: [];
  cancel: [];
}>();

function onOpenChange(v: boolean) {
  // AppModal 通过 ESC / 遮罩 / 右上角 X 关闭时，等同于 cancel
  if (!v && props.visible) emit('cancel');
  emit('update:visible', v);
}

function confirm() {
  emit('confirm');
  emit('update:visible', false);
}
function cancel() {
  emit('cancel');
  emit('update:visible', false);
}
</script>

<style scoped>
.dialog-message {
  font-size: 13px;
  color: var(--text-muted);
  margin: 0;
  line-height: 1.55;
}

.dialog-btn {
  padding: 7px 16px;
  border: none;
  border-radius: var(--r-lg);
  font-size: 13px;
  font-weight: 500;
  font-family: inherit;
  cursor: pointer;
  transition: background var(--t-fast), color var(--t-fast);
}

.cancel-btn {
  background: transparent;
  color: var(--text-muted);
}
.cancel-btn:hover {
  background: var(--bg-elevated);
  color: var(--text);
}

.confirm-btn.primary {
  background: var(--accent);
  color: var(--bg-base);
}
.confirm-btn.primary:hover {
  background: var(--accent-hover);
}

.confirm-btn.danger {
  background: var(--danger);
  color: var(--bg-base);
}
.confirm-btn.danger:hover {
  background: #f5a3b8;
}

.confirm-btn.warning {
  background: var(--warning);
  color: var(--bg-base);
}
.confirm-btn.warning:hover {
  background: #fbe9bb;
}
</style>
