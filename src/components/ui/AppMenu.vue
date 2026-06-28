<template>
  <AppPopover
    :open="open"
    :anchor="anchor"
    :placement="placement"
    :offset="offset"
    @update:open="emit('update:open', $event)"
  >
    <div
      ref="listEl"
      class="menu-list"
      role="menu"
      tabindex="-1"
      @keydown="onKeydown"
    >
      <template v-for="(it, idx) in items" :key="getKey(it, idx)">
        <div v-if="isDivider(it)" class="menu-divider" />
        <button
          v-else
          :class="['menu-item', { danger: it.danger, disabled: it.disabled, focused: focusedIdx === idx }]"
          :disabled="it.disabled"
          role="menuitem"
          @click="select(it)"
          @mouseenter="focusedIdx = idx"
        >
          <span v-if="it.icon" class="menu-icon">{{ it.icon }}</span>
          <span class="menu-label">{{ it.label }}</span>
          <span v-if="it.hint" class="menu-hint">{{ it.hint }}</span>
        </button>
      </template>
    </div>
  </AppPopover>
</template>

<script setup lang="ts">
import { nextTick, ref, watch } from "vue";
import AppPopover from "./AppPopover.vue";
import type { Anchor, Placement } from "./useFloating";

export interface MenuAction {
  id: string;
  label: string;
  icon?: string;
  hint?: string;
  danger?: boolean;
  disabled?: boolean;
  onSelect?: () => void;
}
export interface MenuDivider {
  kind: "divider";
}
export type MenuItem = MenuAction | MenuDivider;

const props = withDefaults(
  defineProps<{
    open: boolean;
    anchor: Anchor;
    items: MenuItem[];
    placement?: Placement;
    offset?: number;
  }>(),
  {
    placement: "bottom-start",
    offset: 6,
  }
);

const emit = defineEmits<{
  "update:open": [value: boolean];
  select: [item: MenuAction];
}>();

const listEl = ref<HTMLElement | null>(null);
const focusedIdx = ref<number>(-1);

function isDivider(it: MenuItem): it is MenuDivider {
  return (it as MenuDivider).kind === "divider";
}
function getKey(it: MenuItem, idx: number): string {
  return isDivider(it) ? `d${idx}` : it.id;
}
function firstSelectable(from = 0, dir: 1 | -1 = 1): number {
  const n = props.items.length;
  for (let i = 0; i < n; i++) {
    const idx = (from + i * dir + n) % n;
    const it = props.items[idx];
    if (!isDivider(it) && !it.disabled) return idx;
  }
  return -1;
}

function select(it: MenuAction) {
  if (it.disabled) return;
  emit("select", it);
  it.onSelect?.();
  emit("update:open", false);
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === "ArrowDown") {
    e.preventDefault();
    focusedIdx.value = firstSelectable(focusedIdx.value + 1, 1);
  } else if (e.key === "ArrowUp") {
    e.preventDefault();
    focusedIdx.value =
      focusedIdx.value <= 0
        ? firstSelectable(props.items.length - 1, -1)
        : firstSelectable(focusedIdx.value - 1, -1);
  } else if (e.key === "Enter" || e.key === " ") {
    e.preventDefault();
    const it = props.items[focusedIdx.value];
    if (it && !isDivider(it)) select(it);
  }
}

watch(
  () => props.open,
  async (v) => {
    if (v) {
      focusedIdx.value = firstSelectable(0, 1);
      await nextTick();
      listEl.value?.focus();
    } else {
      focusedIdx.value = -1;
    }
  }
);
</script>

<style scoped>
.menu-list {
  display: flex;
  flex-direction: column;
  gap: 1px;
  outline: none;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 7px 10px;
  border: none;
  background: transparent;
  color: var(--text);
  font-size: 13px;
  font-family: inherit;
  border-radius: var(--r-md);
  cursor: pointer;
  text-align: left;
  transition: background var(--t-fast);
}
.menu-item:hover:not(.disabled),
.menu-item.focused:not(.disabled) {
  background: var(--bg-elevated);
}
.menu-item.danger {
  color: var(--danger);
}
.menu-item.danger:hover:not(.disabled),
.menu-item.danger.focused:not(.disabled) {
  background: rgba(243, 139, 168, 0.12);
}
.menu-item.disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

.menu-icon {
  flex: 0 0 18px;
  text-align: center;
  font-size: 14px;
}
.menu-label {
  flex: 1;
  white-space: nowrap;
}
.menu-hint {
  color: var(--text-dim);
  font-size: 11px;
  margin-left: 16px;
}

.menu-divider {
  height: 1px;
  background: var(--border);
  margin: 4px 6px;
}
</style>
