<template>
  <div class="app-content-area">
    <slot />
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.app-content-area {
  flex: 1;
  padding: 24px;
  overflow-y: auto;
}

/* Custom scrollbar */
.app-content-area::-webkit-scrollbar {
  width: 5px;
}

.app-content-area::-webkit-scrollbar-track {
  background: transparent;
}

.app-content-area::-webkit-scrollbar-thumb {
  background: var(--border-strong);
  border-radius: var(--r-sm);
}

.app-content-area::-webkit-scrollbar-thumb:hover {
  background: var(--text-dim);
}
</style>
