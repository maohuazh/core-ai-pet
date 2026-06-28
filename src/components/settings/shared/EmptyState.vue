<template>
  <div class="empty-state">
    <div class="empty-icon">
      <slot name="icon">📭</slot>
    </div>
    <h3 class="empty-title">{{ title }}</h3>
    <p class="empty-description">{{ description }}</p>
    <button v-if="actionLabel" class="empty-action" @click="$emit('action')">
      {{ actionLabel }}
    </button>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  title: string;
  description?: string;
  actionLabel?: string;
}>();

defineEmits<{
  action: [];
}>();
</script>

<style scoped>
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  text-align: center;
}

.empty-icon {
  font-size: 44px;
  margin-bottom: 14px;
  opacity: 0.55;
}

.empty-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-muted);
  margin: 0 0 6px 0;
}

.empty-description {
  font-size: 12px;
  color: var(--text-dim);
  margin: 0 0 22px 0;
  max-width: 320px;
  line-height: 1.55;
}

.empty-action {
  padding: 8px 18px;
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

.empty-action:hover {
  background: var(--accent-hover);
}
</style>
