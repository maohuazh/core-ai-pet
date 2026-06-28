<template>
  <AppModal
    :open="visible"
    title="敬请期待"
    max-width="360px"
    @update:open="$emit('update:visible', $event)"
  >
    <p class="cs-message">{{ message }}</p>
    <template #footer>
      <button class="btn-ok" @click="$emit('update:visible', false)">好的</button>
    </template>
  </AppModal>
</template>

<script setup lang="ts">
import AppModal from '../../ui/AppModal.vue';

withDefaults(
  defineProps<{
    visible: boolean;
    message?: string;
  }>(),
  {
    message: '该功能即将推出',
  }
);

defineEmits<{
  'update:visible': [v: boolean];
}>();
</script>

<style scoped>
.cs-message {
  margin: 0;
  font-size: 13px;
  color: var(--text-muted);
  line-height: 1.55;
}

.btn-ok {
  padding: 6px 16px;
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

.btn-ok:hover {
  background: var(--accent-hover);
}
</style>
