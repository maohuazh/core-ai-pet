<template>
  <div class="app-layout">
    <AppTopBar :title="title" />
    <div class="app-layout-body">
      <AppSidebar
        :items="sidebarItems"
        :active="activeItem"
        :collapsed="false"
        @update:active="emit('update:activeItem', $event)"
      >
        <template #footer>
          <slot name="sidebar-footer" />
        </template>
      </AppSidebar>
      <AppContentArea>
        <slot />
      </AppContentArea>
    </div>
  </div>
</template>

<script setup lang="ts">
import AppTopBar from "./AppTopBar.vue";
import AppSidebar from "./AppSidebar.vue";
import AppContentArea from "./AppContentArea.vue";
import type { NavItem } from "./types";

defineProps<{
  title: string;
  sidebarItems: NavItem[];
  activeItem: string;
}>();

const emit = defineEmits<{
  "update:activeItem": [id: string];
}>();
</script>

<style scoped>
.app-layout {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
  background: var(--bg-base);
  color: var(--text);
  overflow: hidden;
}

.app-layout-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}
</style>
