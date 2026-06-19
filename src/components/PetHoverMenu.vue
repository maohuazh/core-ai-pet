<template>
  <div class="hover-menu">
    <button
      v-for="(item, index) in menuItems"
      :key="item.action"
      class="menu-button"
      :style="getButtonStyle(index)"
      @click="handleClick(item.action)"
      @mouseenter="hoveredButton = item.action"
      @mouseleave="hoveredButton = null"
    >
      <span class="icon">{{ item.icon }}</span>
      <span v-if="hoveredButton === item.action" class="tooltip">
        {{ item.label }}
      </span>
    </button>
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
  { action: "chat", icon: "💬", label: "聊天" },
  { action: "settings", icon: "⚙️", label: "设置" },
  { action: "switchModel", icon: "🔄", label: "切换模型" },
  { action: "menu", icon: "☰", label: "菜单" },
  { action: "minimize", icon: "🔽", label: "最小化" },
  { action: "close", icon: "❌", label: "关闭" },
];

const hoveredButton = ref<string | null>(null);

const getButtonStyle = (index: number) => {
  const radius = 80;
  const angle = (index / menuItems.length) * 2 * Math.PI - Math.PI / 2;
  const x = Math.cos(angle) * radius;
  const y = Math.sin(angle) * radius;

  return {
    transform: `translate(${x}px, ${y}px)`,
    transitionDelay: `${index * 0.05}s`,
  };
};

const handleClick = async (action: string) => {
  console.log("Button clicked:", action);
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

.menu-button {
  position: absolute;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.6);
  border: 1px solid white;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  pointer-events: auto;
  transition: all 0.3s cubic-bezier(0.68, -0.55, 0.265, 1.55);
  animation: popIn 0.3s ease-out forwards;
  transform-origin: center;
  margin-left: -20px;
  margin-top: -20px;
}

.menu-button:hover {
  transform: scale(1.2) translate(var(--x, 0), var(--y, 0));
  background: rgba(100, 150, 255, 0.8);
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
  font-size: 20px;
  color: white;
}

.tooltip {
  position: absolute;
  bottom: -30px;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(0, 0, 0, 0.8);
  color: white;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  white-space: nowrap;
  pointer-events: none;
}
</style>
