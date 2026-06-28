<template>
  <Teleport to="body">
    <Transition name="pop">
      <div
        v-if="open"
        ref="floatingEl"
        class="app-popover"
        role="dialog"
        :style="floatStyle"
        @click.stop
      >
        <slot />
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, ref, watch, onUnmounted } from "vue";
import { useFloating, type Anchor, type Placement } from "./useFloating";

const props = withDefaults(
  defineProps<{
    open: boolean;
    anchor: Anchor;
    placement?: Placement;
    offset?: number;
    /** 点击外部是否关闭，默认 true */
    closeOnOutside?: boolean;
    /** ESC 是否关闭，默认 true */
    closeOnEscape?: boolean;
  }>(),
  {
    placement: "bottom-start",
    offset: 6,
    closeOnOutside: true,
    closeOnEscape: true,
  }
);

const emit = defineEmits<{
  "update:open": [value: boolean];
}>();

const floatingEl = ref<HTMLElement | null>(null);
const anchorRef = computed(() => props.anchor);
const openRef = computed(() => props.open);

const { x, y, ready } = useFloating({
  anchor: anchorRef,
  floating: floatingEl,
  open: openRef,
  placement: computed(() => props.placement) as any,
  offset: props.offset,
});

const floatStyle = computed(() => ({
  position: "fixed" as const,
  left: `${x.value}px`,
  top: `${y.value}px`,
  visibility: ready.value ? ("visible" as const) : ("hidden" as const),
  zIndex: "var(--z-popover)",
}));

// ---- Outside click / ESC ----
function onDocPointer(e: PointerEvent) {
  if (!props.open || !props.closeOnOutside) return;
  const t = e.target as Node | null;
  if (!t) return;
  if (floatingEl.value && floatingEl.value.contains(t)) return;
  // 点击锚点本身不关（让锚点 toggle 自己控制）
  if (props.anchor instanceof HTMLElement && props.anchor.contains(t)) return;
  emit("update:open", false);
}

function onKey(e: KeyboardEvent) {
  if (!props.open || !props.closeOnEscape) return;
  if (e.key === "Escape") {
    e.stopPropagation();
    emit("update:open", false);
  }
}

watch(
  () => props.open,
  (v) => {
    if (v) {
      // 用 pointerdown 而非 click，避免点击产生的 click 冒泡先被锚点 toggle 关又开
      document.addEventListener("pointerdown", onDocPointer, true);
      document.addEventListener("keydown", onKey);
    } else {
      document.removeEventListener("pointerdown", onDocPointer, true);
      document.removeEventListener("keydown", onKey);
    }
  },
  { immediate: false }
);

onUnmounted(() => {
  document.removeEventListener("pointerdown", onDocPointer, true);
  document.removeEventListener("keydown", onKey);
});
</script>

<style scoped>
.app-popover {
  background: var(--bg-surface);
  color: var(--text);
  border: 1px solid var(--border-strong);
  border-radius: var(--r-lg);
  box-shadow: var(--shadow-popover);
  font-family: var(--font-sans);
  font-size: 13px;
  min-width: 140px;
  max-width: 360px;
  padding: 4px;
  user-select: none;
}

.pop-enter-active,
.pop-leave-active {
  transition: opacity var(--t-fast) ease, transform var(--t-fast) ease;
}
.pop-enter-from,
.pop-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
