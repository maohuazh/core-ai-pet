<template>
  <AppModal
    :open="visible"
    :title="title"
    max-width="380px"
    @update:open="onOpenChange"
  >
    <div class="rename-body">
      <label v-if="label" class="rename-label">{{ label }}</label>
      <input
        ref="inputEl"
        v-model="localValue"
        class="rename-input"
        :placeholder="placeholder"
        @keydown.enter="submit"
      />
    </div>
    <template #footer>
      <button class="btn btn-cancel" @click="cancel">取消</button>
      <button class="btn btn-confirm" :disabled="!canSubmit" @click="submit">
        确定
      </button>
    </template>
  </AppModal>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue';
import AppModal from '../../ui/AppModal.vue';

const props = withDefaults(
  defineProps<{
    visible: boolean;
    initialValue?: string;
    title?: string;
    label?: string;
    placeholder?: string;
  }>(),
  {
    title: '编辑名称',
    placeholder: '请输入',
    initialValue: '',
  }
);

const emit = defineEmits<{
  'update:visible': [v: boolean];
  submit: [value: string];
}>();

const localValue = ref(props.initialValue);
const inputEl = ref<HTMLInputElement | null>(null);

const canSubmit = computed(() => {
  const trimmed = localValue.value.trim();
  return trimmed.length > 0 && trimmed !== props.initialValue;
});

watch(
  () => props.visible,
  async (v) => {
    if (v) {
      localValue.value = props.initialValue;
      await nextTick();
      inputEl.value?.focus();
      inputEl.value?.select();
    }
  }
);

function onOpenChange(v: boolean) {
  if (!v) cancel();
}
function cancel() {
  emit('update:visible', false);
}
function submit() {
  if (!canSubmit.value) return;
  emit('submit', localValue.value.trim());
  emit('update:visible', false);
}
</script>

<style scoped>
.rename-body {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.rename-label {
  font-size: 12px;
  color: var(--text-muted);
}

.rename-input {
  width: 100%;
  padding: 7px 10px;
  background: var(--bg-base);
  color: var(--text);
  border: 1px solid var(--border-strong);
  border-radius: var(--r-lg);
  font-family: inherit;
  font-size: 13px;
  transition: border-color var(--t-fast);
}

.rename-input::placeholder {
  color: var(--text-dim);
}

.rename-input:focus {
  outline: none;
  border-color: var(--accent);
}

.btn {
  padding: 6px 16px;
  border: none;
  border-radius: var(--r-lg);
  font-size: 13px;
  font-weight: 500;
  font-family: inherit;
  cursor: pointer;
  transition: background var(--t-fast), color var(--t-fast);
}

.btn-cancel {
  background: transparent;
  color: var(--text-muted);
}
.btn-cancel:hover {
  background: var(--bg-elevated);
  color: var(--text);
}

.btn-confirm {
  background: var(--accent);
  color: var(--bg-base);
  font-weight: 600;
}
.btn-confirm:hover:not(:disabled) {
  background: var(--accent-hover);
}
.btn-confirm:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
</style>
