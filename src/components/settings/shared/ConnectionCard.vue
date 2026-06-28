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
        <button
          ref="menuBtnEl"
          class="menu-btn"
          :class="{ active: menuOpen }"
          @click.stop="onMenuClick"
        >
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <circle cx="8" cy="3" r="1.5" fill="currentColor" />
            <circle cx="8" cy="8" r="1.5" fill="currentColor" />
            <circle cx="8" cy="13" r="1.5" fill="currentColor" />
          </svg>
        </button>
        <AppMenu
          v-if="menuItems && menuItems.length"
          :open="menuOpen"
          :anchor="menuBtnEl"
          :items="menuItems"
          placement="bottom-end"
          @update:open="menuOpen = $event"
        />
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
import { computed, ref } from 'vue';
import ToggleSwitch from './ToggleSwitch.vue';
import AppMenu, { type MenuItem } from '../../ui/AppMenu.vue';

const props = defineProps<{
  name: string;
  subtitle: string;
  status: 'connected' | 'expired' | 'error' | 'disconnected';
  enabled: boolean;
  actionLabel: string;
  /** 提供则启用内置依附菜单；不提供则回落到 @menu 事件 */
  menuItems?: MenuItem[];
}>();

const emit = defineEmits<{
  toggle: [enabled: boolean];
  action: [];
  menu: [];
}>();

const menuBtnEl = ref<HTMLButtonElement | null>(null);
const menuOpen = ref(false);

function onMenuClick() {
  if (props.menuItems && props.menuItems.length) {
    menuOpen.value = !menuOpen.value;
  } else {
    emit('menu');
  }
}

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
  background: var(--bg-elevated);
  border: 1px solid var(--border-strong);
  border-radius: 12px;
  padding: 14px 16px;
  transition: border-color var(--t-fast);
}

.connection-card:hover {
  border-color: var(--accent);
}

.connection-card.disabled {
  opacity: 0.55;
}

.card-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 10px;
}

.card-icon {
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

.card-info {
  flex: 1;
  min-width: 0;
}

.card-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text);
  margin: 0 0 3px 0;
}

.card-subtitle {
  font-size: 12px;
  color: var(--text-dim);
  margin: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.card-actions {
  display: flex;
  gap: 4px;
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

.card-body {
  margin-bottom: 12px;
  font-size: 12px;
  color: var(--text-muted);
}

.card-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.action-btn {
  padding: 6px 14px;
  border: none;
  border-radius: var(--r-md);
  font-size: 12px;
  font-weight: 500;
  font-family: inherit;
  cursor: pointer;
  transition: background var(--t-fast);
}

.action-btn.success {
  background: rgba(166, 227, 161, 0.12);
  color: var(--success);
}
.action-btn.success:hover {
  background: rgba(166, 227, 161, 0.2);
}

.action-btn.danger {
  background: rgba(243, 139, 168, 0.12);
  color: var(--danger);
}
.action-btn.danger:hover {
  background: rgba(243, 139, 168, 0.2);
}

.action-btn.warning {
  background: rgba(249, 226, 175, 0.12);
  color: var(--warning);
}
.action-btn.warning:hover {
  background: rgba(249, 226, 175, 0.2);
}
</style>
