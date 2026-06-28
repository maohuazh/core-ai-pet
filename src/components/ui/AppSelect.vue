<template>
  <div class="app-select" :class="{ disabled }">
    <button
      ref="triggerEl"
      type="button"
      class="select-trigger"
      :class="{ open, placeholder: !hasValue }"
      :disabled="disabled"
      @click="toggle"
    >
      <span class="select-text">{{ displayText }}</span>
      <span class="select-arrow">▾</span>
    </button>
    <AppMenu
      :open="open"
      :anchor="triggerEl"
      :items="menuItems"
      placement="bottom-start"
      @update:open="open = $event"
      @select="onSelect"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import AppMenu, { type MenuItem } from "./AppMenu.vue";

export interface SelectOption<T = string | number> {
  value: T;
  label: string;
  icon?: string;
  disabled?: boolean;
}

const props = withDefaults(
  defineProps<{
    modelValue: string | number | null | undefined;
    options: SelectOption[];
    placeholder?: string;
    disabled?: boolean;
  }>(),
  {
    placeholder: "请选择",
    disabled: false,
  }
);

const emit = defineEmits<{
  "update:modelValue": [value: string | number];
  change: [value: string | number];
}>();

const triggerEl = ref<HTMLButtonElement | null>(null);
const open = ref(false);

const hasValue = computed(() => {
  return props.modelValue !== null && props.modelValue !== undefined && props.modelValue !== "";
});

const displayText = computed(() => {
  if (!hasValue.value) return props.placeholder;
  const opt = props.options.find((o) => o.value === props.modelValue);
  return opt?.label ?? String(props.modelValue);
});

const menuItems = computed<MenuItem[]>(() =>
  props.options.map((o) => ({
    id: String(o.value),
    label: o.label,
    icon: o.icon,
    disabled: o.disabled,
  }))
);

function toggle() {
  if (!props.disabled) open.value = !open.value;
}

function onSelect(item: { id: string }) {
  const opt = props.options.find((o) => String(o.value) === item.id);
  if (!opt || opt.disabled) return;
  emit("update:modelValue", opt.value);
  emit("change", opt.value);
}
</script>

<style scoped>
.app-select {
  display: inline-flex;
  width: 100%;
}

.select-trigger {
  flex: 1;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 6px 10px;
  background: var(--bg-base);
  color: var(--text);
  border: 1px solid var(--border-strong);
  border-radius: var(--r-lg);
  font-family: inherit;
  font-size: 13px;
  text-align: left;
  cursor: pointer;
  transition: border-color var(--t-fast);
}

.select-trigger:hover:not(:disabled) {
  border-color: var(--accent);
}

.select-trigger:focus,
.select-trigger.open {
  outline: none;
  border-color: var(--accent);
}

.select-trigger:disabled {
  background: var(--bg-surface);
  color: var(--text-dim);
  cursor: not-allowed;
  opacity: 0.6;
}

.select-trigger.placeholder .select-text {
  color: var(--text-dim);
}

.select-text {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.select-arrow {
  color: var(--text-dim);
  font-size: 10px;
  flex-shrink: 0;
}
</style>
