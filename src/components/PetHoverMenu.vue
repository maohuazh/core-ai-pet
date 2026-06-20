<template>
  <div class="hover-menu">
    <div
      v-for="(item, index) in menuItems"
      :key="item.action"
      class="menu-item"
      :style="getItemStyle(index)"
      @click="handleClick(item.action)"
      @mouseenter="hoveredButton = item.action"
      @mouseleave="hoveredButton = null"
    >
      <button class="menu-button" :style="{ animationDelay: `${index * 0.05}s` }">
        <span class="icon">{{ item.icon }}</span>
      </button>
      <span v-if="hoveredButton === item.action" class="tooltip">
        {{ item.label }}
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

interface MenuItem {
  action: string;
  icon: string;
  label: string;
}

const props = defineProps<{
  onAction: (action: string) => void | Promise<void>;
}>();

const menuItems: MenuItem[] = [
  { action: "task", icon: "📋", label: "任务" },
  { action: "message", icon: "💬", label: "消息" },
  { action: "jira", icon: "🔗", label: "Jira" },
  { action: "email", icon: "📧", label: "邮件" },
  { action: "agent", icon: "🤖", label: "Agent" },
  { action: "settings", icon: "⚙️", label: "设置" },
];

const hoveredButton = ref<string | null>(null);

const getItemStyle = (index: number) => {
  const radius = 80;
  const angle = (index / menuItems.length) * 2 * Math.PI - Math.PI / 2;
  const x = Math.cos(angle) * radius;
  const y = Math.sin(angle) * radius;

  return {
    transform: `translate(${x}px, ${y}px)`,
  };
};

const handleClick = async (action: string) => {
  await props.onAction(action);
};
</script>

<style scoped>
.hover-menu {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 0;
  height: 0;
  pointer-events: none;
  z-index: 10;
}

.menu-item {
  position: absolute;
  cursor: pointer;
  pointer-events: auto;
  margin-left: -17px;
  margin-top: -17px;
}

.menu-button {
  width: 35px;
  height: 35px;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.7);
  border: 1px solid rgba(255, 255, 255, 0.3);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: transform 0.2s ease, background 0.2s ease, border-color 0.2s ease;
  animation: popIn 0.3s ease-out forwards;
  animation-delay: 0s;
}

.menu-button:hover {
  transform: scale(1.25);
  background: rgba(80, 130, 255, 0.85);
  border-color: rgba(255, 255, 255, 0.6);
}

@keyframes popIn {
  from {
    opacity: 0;
    transform: scale(0);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.icon {
  font-size: 16px;
  line-height: 1;
}

.tooltip {
  position: absolute;
  bottom: -28px;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(0, 0, 0, 0.85);
  color: white;
  padding: 3px 8px;
  border-radius: 4px;
  font-size: 12px;
  white-space: nowrap;
  pointer-events: none;
  z-index: 11;
}
</style>
