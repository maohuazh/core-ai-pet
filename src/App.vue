<template>
  <ChatWindow v-if="isChatRoute" />
  <SettingsPanel v-else-if="isSettingsRoute" />
  <div
    v-else
    class="pet-container"
    @dblclick="openChat"
    @contextmenu.prevent="onContextMenu"
  >
    <Live2DCanvas ref="canvasRef" />
    <WindowCloseButton v-if="showMenu && !contextMenuOpen" />
    <PetHoverMenu v-if="showMenu && !contextMenuOpen" :on-action="handleMenuAction" />
  </div>

  <AppMenu
    :open="contextMenuOpen"
    :anchor="contextMenuAnchor"
    :items="contextMenuItems"
    @update:open="contextMenuOpen = $event"
  />

  <AppModal
    :open="showComingSoon"
    title="敬请期待"
    max-width="360px"
    @update:open="showComingSoon = $event"
  >
    <p class="coming-soon-msg">{{ comingSoonMsg }}</p>
    <template #footer>
      <button class="btn-ok" @click="showComingSoon = false">好的</button>
    </template>
  </AppModal>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import Live2DCanvas from "./components/Live2DCanvas.vue";
import WindowCloseButton from "./components/WindowCloseButton.vue";
import PetHoverMenu from "./components/PetHoverMenu.vue";
import ChatWindow from "./modules/chat/ChatWindow.vue";
import SettingsPanel from "./components/settings/SettingsPanel.vue";
import AppModal from "./components/ui/AppModal.vue";
import AppMenu, { type MenuItem } from "./components/ui/AppMenu.vue";
import { invoke } from "@tauri-apps/api/core";
import { modelRegistry } from "./core/model/ModelRegistry";
import { petStore } from "./core/model/PetStore";
import { registerTriggerExecutor, unregisterTriggerExecutor } from "./core/action/triggerExecutor";
import { AVAILABLE_EFFECTS } from "./core/action/effects";
import { petMenuItems } from "./core/pet/petMenu";

// Check current route
const isSettingsRoute = computed(() => window.location.pathname === "/settings");
const isChatRoute = computed(() => window.location.pathname === "/chat");

const canvasRef = ref<InstanceType<typeof Live2DCanvas> | null>(null);
const showMenu = ref(false);
const showComingSoon = ref(false);
const comingSoonMsg = ref("");

// Right-click context menu
const contextMenuAnchor = ref<{ x: number; y: number } | null>(null);
const contextMenuOpen = ref(false);

const contextMenuItems: MenuItem[] = petMenuItems.map((item) => ({
  id: item.action,
  label: item.label,
  icon: item.icon,
  onSelect: () => handleMenuAction(item.action),
}));

let unlistenStart: UnlistenFn | null = null;
let unlistenEnd: UnlistenFn | null = null;
let unlistenModelChanged: UnlistenFn | null = null;
let unlistenPreviewMapping: UnlistenFn | null = null;
let hideTimeout: ReturnType<typeof setTimeout> | null = null;

onMounted(async () => {
  // Only set up pet window listeners if not in settings or chat route
  if (!isSettingsRoute.value && !isChatRoute.value) {
    // Listen for model change events from settings window
    unlistenModelChanged = await listen<{ modelId: string }>("pet-model-changed", (event) => {
      const model = modelRegistry.getById(event.payload.modelId);
      if (model) {
        petStore.setCurrentModel(model);
      }
    });

    // Listen for preview action mapping events from settings window
    unlistenPreviewMapping = await listen<{
      modelId: string;
      motionGroup?: string | null;
      motionName?: string | null;
      expressionName?: string | null;
      effectName?: string | null;
      effectDuration?: number | null;
      effectPosition?: string | null;
    }>("preview-action-mapping", async (event) => {
      const avatar = canvasRef.value?.avatar;
      if (!avatar) {
        console.warn("Preview: avatar not available");
        return;
      }

      try {
        // Play motion if configured
        if (event.payload.motionGroup) {
          // Note: We use group name and let it pick a random motion from that group
          // In future, we could map motionName to index if needed
          await avatar.playMotion(event.payload.motionGroup);
        }

        // Play expression if configured
        if (event.payload.expressionName) {
          await avatar.playExpression(event.payload.expressionName);
        }

        // Play effect if configured
        if (event.payload.effectName) {
          showEffect(event.payload.effectName, event.payload.effectDuration || 2000, event.payload.effectPosition || "center");
        }
      } catch (e) {
        console.error("Preview failed:", e);
      }
    });

    // Register trigger executor so triggerHandler can call avatar directly (same window)
    registerTriggerExecutor(async (payload) => {
      const avatar = canvasRef.value?.avatar;
      if (!avatar) {
        console.warn("Trigger: avatar not available");
        return;
      }

      try {
        if (payload.motionGroup) {
          await avatar.playMotion(payload.motionGroup);
        }
        if (payload.expressionName) {
          await avatar.playExpression(payload.expressionName);
        }
        if (payload.effectName) {
          showEffect(payload.effectName, payload.effectDuration || 2000, payload.effectPosition || "center");
        }
      } catch (e) {
        console.error("Trigger failed:", e);
      }
    });

    // Sync active model from DB on startup
    try {
      const activeId = await invoke<string | null>("get_active_model_id");
      if (activeId) {
        const model = modelRegistry.getById(activeId);
        if (model) {
          petStore.setCurrentModel(model);
        }
      }
    } catch (e) {
      console.warn("Failed to sync active model from DB:", e);
    }

    // Listen for cursor monitor events from Rust backend
    unlistenStart = await listen("pet-hover-start", () => {
      if (hideTimeout) {
        clearTimeout(hideTimeout);
        hideTimeout = null;
      }
      showMenu.value = true;
    });

    unlistenEnd = await listen("pet-hover-end", () => {
      hideTimeout = setTimeout(() => {
        showMenu.value = false;
      }, 200);
    });
  }

  // TEMP: Press T to manually trigger daily_2 for testing (only in pet window)
  if (!isSettingsRoute.value && !isChatRoute.value) {
    const onTestKey = async (e: KeyboardEvent) => {
      if (e.key === "t" || e.key === "T") {
        console.log("Manual trigger: daily_2");
        const { triggerHandler } = await import("./core/events/triggerHandler");
        await triggerHandler.fireTrigger("daily_2");
      }
    };
    window.addEventListener("keydown", onTestKey);
    (window as any).__testKeyHandler = onTestKey;
  }

  // Keyboard shortcut: Ctrl+Alt+N to open chat window
  const onChatShortcut = (e: KeyboardEvent) => {
    if ((e.ctrlKey || e.metaKey) && e.altKey && e.key.toLowerCase() === 'n') {
      e.preventDefault();
      invoke("open_chat_window").catch((e) => console.error("Failed to open chat window:", e));
    }
  };
  window.addEventListener("keydown", onChatShortcut);
  (window as any).__chatShortcutHandler = onChatShortcut;
});

onUnmounted(() => {
  unlistenStart?.();
  unlistenEnd?.();
  unlistenModelChanged?.();
  unlistenPreviewMapping?.();
  unregisterTriggerExecutor();
  if ((window as any).__testKeyHandler) {
    window.removeEventListener("keydown", (window as any).__testKeyHandler);
  }
  if ((window as any).__chatShortcutHandler) {
    window.removeEventListener("keydown", (window as any).__chatShortcutHandler);
  }
  if (hideTimeout) clearTimeout(hideTimeout);
});

/** Open chat window (called on double-click) */
function openChat() {
  invoke("open_chat_window").catch((e) => console.error("Failed to open chat window:", e));
}

/** Show context menu at cursor position */
function onContextMenu(e: MouseEvent) {
  contextMenuAnchor.value = { x: e.clientX, y: e.clientY };
  contextMenuOpen.value = true;
}

/** Show a floating emoji effect above/near the pet */
function showEffect(effectName: string, duration: number, position: string) {
  const effect = AVAILABLE_EFFECTS.find((e) => e.id === effectName);
  if (!effect) {
    console.warn(`Unknown effect: ${effectName}`);
    return;
  }

  const el = document.createElement("div");
  el.textContent = effect.icon;
  el.style.cssText = `
    position: fixed;
    font-size: 32px;
    pointer-events: none;
    z-index: 9999;
    animation: effectFloat ${duration}ms ease-out forwards;
    left: 50%;
    ${position === "above" ? "top: 10%;" : position === "below" ? "bottom: 10%;" : "top: 30%;"}
    transform: translateX(-50%);
  `;

  // Add keyframe animation if not already added
  if (!document.getElementById("effect-float-style")) {
    const style = document.createElement("style");
    style.id = "effect-float-style";
    style.textContent = `
      @keyframes effectFloat {
        0% { opacity: 0; transform: translateX(-50%) translateY(10px) scale(0.5); }
        20% { opacity: 1; transform: translateX(-50%) translateY(0) scale(1.2); }
        40% { transform: translateX(-50%) translateY(-5px) scale(1); }
        80% { opacity: 1; }
        100% { opacity: 0; transform: translateX(-50%) translateY(-30px) scale(0.8); }
      }
    `;
    document.head.appendChild(style);
  }

  document.body.appendChild(el);
  setTimeout(() => el.remove(), duration);
}

const actionLabels: Record<string, string> = {
  task: "任务",
  message: "消息",
  jira: "Jira",
  email: "邮件",
  agent: "Agent",
  settings: "设置",
};

const handleMenuAction = async (action: string) => {
  console.log("Menu action:", action);

  if (action === "settings") {
    await invoke("open_settings_window");
    return;
  }

  if (action === "message") {
    await invoke("open_chat_window");
    return;
  }

  const label = actionLabels[action] ?? action;
  comingSoonMsg.value = `${label}功能即将推出`;
  showComingSoon.value = true;
};
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  width: 100%;
  height: 100%;
  overflow: hidden;
  background: transparent;
}

.pet-container {
  width: 100vw;
  height: 100vh;
  position: relative;
  background: transparent;
}
</style>

<style scoped>
.coming-soon-msg {
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
