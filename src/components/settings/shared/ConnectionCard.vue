<template>
  <div :class="['connection-card', { disabled: !enabled }]">
    <div class="card-header">
      <div class="card-icon">
        <slot name="icon">🔗</slot>
      </div>
      <div class="card-info">
        <h3 class="card-title">{{ name }}</h3>
        <p class="card-subtitle">{{ subtitle }}</p>
      </div>
      <div class="card-actions">
        <button class="menu-btn" @click.stop="$emit('menu')">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <circle cx="8" cy="3" r="1.5" fill="currentColor" />
            <circle cx="8" cy="8" r="1.5" fill="currentColor" />
            <circle cx="8" cy="13" r="1.5" fill="currentColor" />
          </svg>
        </button>
      </div>
    </div>
    <div class="card-body">
      <slot></slot>
    </div>
    <div class="card-footer">
      <ToggleSwitch :enabled="enabled" @update:enabled="$emit('toggle', $event)" />
      <button :class="['action-btn', statusClass]" @click.stop="$emit('action')">
        {{ actionLabel }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import ToggleSwitch from './ToggleSwitch.vue';

const props = defineProps<{
  name: string;
  subtitle: string;
  status: 'connected' | 'expired' | 'error' | 'disconnected';
  enabled: boolean;
  actionLabel: string;
}>();

defineEmits<{
  toggle: [enabled: boolean];
  action: [];
  menu: [];
}>();

const statusClass = computed(() => {
  switch (props.status) {
    case 'connected':
      return 'success';
    case 'expired':
    case 'error':
      return 'danger';
    case 'disconnected':
      return 'warning';
    default:
      return '';
  }
});
</script>

<style scoped>
.connection-card {
  background: rgba(255, 255, 255, 0.6);
  border: 1px solid rgba(0, 0, 0, 0.04);
  border-radius: 12px;
  padding: 16px;
  transition: all 0.2s ease;
}

.connection-card:hover {
  background: rgba(255, 255, 255, 0.8);
  border-color: rgba(99, 102, 241, 0.15);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.06);
}

.connection-card.disabled {
  opacity: 0.6;
}

.card-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 12px;
}

.card-icon {
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

.card-info {
  flex: 1;
  min-width: 0;
}

.card-title {
  font-size: 14px;
  font-weight: 500;
  color: #1f2937;
  margin: 0 0 4px 0;
}

.card-subtitle {
  font-size: 12px;
  color: #6b7280;
  margin: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.card-actions {
  display: flex;
  gap: 4px;
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

.card-body {
  margin-bottom: 12px;
  font-size: 13px;
  color: #6b7280;
}

.card-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
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

.action-btn.success {
  background: rgba(16, 185, 129, 0.1);
  color: #10b981;
}

.action-btn.success:hover {
  background: rgba(16, 185, 129, 0.15);
}

.action-btn.danger {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.action-btn.danger:hover {
  background: rgba(239, 68, 68, 0.15);
}

.action-btn.warning {
  background: rgba(245, 158, 11, 0.1);
  color: #f59e0b;
}

.action-btn.warning:hover {
  background: rgba(245, 158, 11, 0.15);
}
</style>
