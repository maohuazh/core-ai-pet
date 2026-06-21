<template>
  <Teleport to="body">
    <Transition name="fade">
      <div v-if="visible" class="dialog-overlay" @click="handleOverlayClick">
        <div class="dialog-container" @click.stop>
          <div class="dialog-header">
            <h3 class="dialog-title">{{ title }}</h3>
          </div>
          <div class="dialog-body">
            <p class="dialog-message">{{ message }}</p>
          </div>
          <div class="dialog-footer">
            <button class="dialog-btn cancel-btn" @click="cancel">{{ cancelText }}</button>
            <button :class="['dialog-btn', 'confirm-btn', confirmClass]" @click="confirm">
              {{ confirmText }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
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

const handleOverlayClick = () => {
  if (props.closeOnOverlay) {
    emit('update:visible', false);
  }
};

const confirm = () => {
  emit('confirm');
  emit('update:visible', false);
};

const cancel = () => {
  emit('cancel');
  emit('update:visible', false);
};
</script>

<style scoped>
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.dialog-container {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border-radius: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  min-width: 320px;
  max-width: 480px;
  overflow: hidden;
}

.dialog-header {
  padding: 20px 24px 0;
}

.dialog-title {
  font-size: 16px;
  font-weight: 600;
  color: #1f2937;
  margin: 0;
}

.dialog-body {
  padding: 16px 24px;
}

.dialog-message {
  font-size: 14px;
  color: #6b7280;
  margin: 0;
  line-height: 1.5;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 0 24px 20px;
}

.dialog-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.cancel-btn {
  background: rgba(0, 0, 0, 0.05);
  color: #6b7280;
}

.cancel-btn:hover {
  background: rgba(0, 0, 0, 0.08);
  color: #1f2937;
}

.confirm-btn.primary {
  background: #6366f1;
  color: white;
}

.confirm-btn.primary:hover {
  background: #818cf8;
}

.confirm-btn.danger {
  background: #ef4444;
  color: white;
}

.confirm-btn.danger:hover {
  background: #f87171;
}

.confirm-btn.warning {
  background: #f59e0b;
  color: white;
}

.confirm-btn.warning:hover {
  background: #fbbf24;
}

/* Fade transition */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
