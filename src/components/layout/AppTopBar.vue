<template>
  <div class="app-top-bar">
    <div class="top-bar-left">
      <span class="top-bar-icon">🐾</span>
      <span class="top-bar-title">{{ title }}</span>
    </div>
    <div class="drag-region" @mousedown="startDrag" @dblclick="toggleMaximize"></div>
    <div class="top-bar-right">
      <button class="top-bar-action-btn" title="新建" @click="emit('new-click')">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.5" />
        </svg>
      </button>
      <button class="top-bar-action-btn" title="搜索" @click="emit('search-click')">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <circle cx="7" cy="7" r="5.5" stroke="currentColor" stroke-width="1.5" />
          <path d="M11 11L14 14" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
        </svg>
      </button>
      <button class="top-bar-action-btn" title="通知" @click="emit('notification-click')">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          <path d="M4 6a4 4 0 018 0c0 4 1.5 5.5 2 6H2c.5-.5 2-2 2-6z" stroke="currentColor" stroke-width="1.5" stroke-linejoin="round" />
          <path d="M6.5 12a1.5 1.5 0 003 0" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
        </svg>
      </button>
      <button class="top-bar-action-btn win-ctrl-btn" :title="maximized ? '还原' : '最大化'" @click="toggleMaximize">
        <svg v-if="!maximized" width="14" height="14" viewBox="0 0 14 14" fill="none">
          <rect x="1.5" y="1.5" width="11" height="11" rx="1" stroke="currentColor" stroke-width="1.5" />
        </svg>
        <svg v-else width="14" height="14" viewBox="0 0 14 14" fill="none">
          <rect x="3.5" y="0.5" width="9" height="9" rx="1" stroke="currentColor" stroke-width="1.2" />
          <rect x="1.5" y="4.5" width="9" height="9" rx="1" stroke="currentColor" stroke-width="1.2" fill="var(--bg-surface)" />
        </svg>
      </button>
      <div
        ref="avatarBtn"
        class="avatar-area"
        :class="{ 'avatar-logged-in': isLoggedIn }"
        @click="onAvatarClick"
      >
        <div class="avatar-circle">
          <template v-if="isLoggedIn && userName">
            <span class="avatar-initials">{{ userName.charAt(0).toUpperCase() }}</span>
          </template>
          <template v-else>
            <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
              <circle cx="8" cy="5.5" r="3" stroke="currentColor" stroke-width="1.5" />
              <path d="M2.5 14c0-3 2.5-5 5.5-5s5.5 2 5.5 5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" />
            </svg>
          </template>
        </div>
        <span v-if="isLoggedIn && userName" class="avatar-name">{{ userName }}</span>
        <span v-if="isLoggedIn" class="avatar-chevron" :class="{ open: menuOpen }">▾</span>
      </div>

      <AppMenu
        v-if="isLoggedIn"
        :open="menuOpen"
        :anchor="avatarBtn"
        :items="avatarMenuItems"
        placement="bottom-end"
        @update:open="menuOpen = $event"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import AppMenu, { type MenuItem } from "@/components/ui/AppMenu.vue";

const props = withDefaults(
  defineProps<{
    title: string;
    isLoggedIn?: boolean;
    userName?: string;
  }>(),
  {
    isLoggedIn: false,
    userName: "",
  }
);

const emit = defineEmits<{
  "avatar-click": [];
  "search-click": [];
  "notification-click": [];
  "new-click": [];
  "logout": [];
  "profile": [];
  "settings": [];
}>();

const avatarBtn = ref<HTMLElement | null>(null);
const menuOpen = ref(false);
const maximized = ref(false);

let unlistenMax: UnlistenFn | null = null;
let unlistenUnmax: UnlistenFn | null = null;

const avatarMenuItems: MenuItem[] = [
  { id: "profile", label: "个人信息", icon: "👤", onSelect: () => { menuOpen.value = false; emit("profile"); } },
  { id: "settings", label: "设置", icon: "⚙️", onSelect: () => { menuOpen.value = false; emit("settings"); } },
  { kind: "divider" },
  { id: "logout", label: "登出", icon: "🚪", danger: true, onSelect: () => { menuOpen.value = false; emit("logout"); } },
];

async function startDrag(event: MouseEvent) {
  if (event.button !== 0) return;
  const win = getCurrentWindow();
  // Try native drag first
  try {
    await win.startDragging();
    return;
  } catch {
    // Fall through to manual drag
  }
  // Manual drag fallback: track mouse delta and move window
  const startPos = await win.innerPosition();
  let curX = startPos.x;
  let curY = startPos.y;
  let dragging = true;
  let moving = false;
  const onMouseMove = async (e: MouseEvent) => {
    if (!dragging || moving) return;
    curX += e.movementX;
    curY += e.movementY;
    moving = true;
    await win.setPosition({ x: curX, y: curY });
    moving = false;
  };
  const onMouseUp = () => {
    dragging = false;
    document.removeEventListener("mousemove", onMouseMove);
    document.removeEventListener("mouseup", onMouseUp);
  };
  document.addEventListener("mousemove", onMouseMove);
  document.addEventListener("mouseup", onMouseUp);
}

async function toggleMaximize() {
  const win = getCurrentWindow();
  if (maximized.value) {
    await win.unmaximize();
  } else {
    await win.maximize();
  }
}

function onAvatarClick() {
  if (!props.isLoggedIn) {
    emit("avatar-click");
  } else {
    menuOpen.value = !menuOpen.value;
  }
}

onMounted(async () => {
  const win = getCurrentWindow();
  maximized.value = await win.isMaximized();
  unlistenMax = await listen("tauri://maximize", () => { maximized.value = true; });
  unlistenUnmax = await listen("tauri://unmaximize", () => { maximized.value = false; });
});

onUnmounted(() => {
  unlistenMax?.();
  unlistenUnmax?.();
});
</script>

<style scoped>
.app-top-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 48px;
  padding: 0 16px;
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border);
  flex-shrink: 0;
}

.top-bar-left {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: default;
  user-select: none;
  flex-shrink: 0;
}

.drag-region {
  flex: 1;
  cursor: move;
  min-width: 40px;
}

.top-bar-icon {
  font-size: 18px;
}

.top-bar-title {
  font-weight: 600;
  font-size: 15px;
  color: var(--text);
}

.top-bar-right {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.top-bar-action-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: var(--r-lg);
  background: transparent;
  color: var(--text-dim);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background var(--t-fast), color var(--t-fast);
}

.top-bar-action-btn:hover {
  background: var(--bg-elevated);
  color: var(--text);
}

.avatar-area {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: var(--r-lg);
  transition: background var(--t-fast);
}

.avatar-area:hover {
  background: var(--bg-elevated);
}

.avatar-circle {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: var(--bg-elevated);
  color: var(--text-muted);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 13px;
  font-weight: 600;
}

.avatar-logged-in .avatar-circle {
  background: var(--accent);
  color: var(--bg-base);
}

.avatar-initials {
  font-size: 14px;
}

.avatar-name {
  font-size: 13px;
  color: var(--text);
  font-weight: 500;
}

.avatar-chevron {
  font-size: 10px;
  color: var(--text-dim);
  transition: transform var(--t-fast);
}

.avatar-chevron.open {
  transform: rotate(180deg);
}
</style>
