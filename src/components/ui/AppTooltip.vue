<template>
  <Teleport to="body">
    <Transition name="tt">
      <div
        v-if="visible"
        ref="floatingEl"
        class="app-tooltip"
        role="tooltip"
        :style="floatStyle"
      >
        <slot>{{ text }}</slot>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useFloating, type Anchor, type Placement } from "./useFloating";

const props = withDefaults(
  defineProps<{
    /** 可控显隐；不传时由调用方包裹 mouseenter/leave 自行控制 */
    visible: boolean;
    anchor: Anchor;
    text?: string;
    placement?: Placement;
    offset?: number;
  }>(),
  {
    placement: "bottom-start",
    offset: 6,
  }
);

const floatingEl = ref<HTMLElement | null>(null);
const anchorRef = computed(() => props.anchor);
const visibleRef = computed(() => props.visible);

const { x, y, ready } = useFloating({
  anchor: anchorRef,
  floating: floatingEl,
  open: visibleRef,
  placement: computed(() => props.placement) as any,
  offset: props.offset,
});

const floatStyle = computed(() => ({
  position: "fixed" as const,
  left: `${x.value}px`,
  top: `${y.value}px`,
  visibility: ready.value ? ("visible" as const) : ("hidden" as const),
  zIndex: "var(--z-tooltip)",
}));
</script>

<style scoped>
.app-tooltip {
  background: var(--bg-surface);
  color: var(--text);
  border: 1px solid var(--border-strong);
  border-radius: var(--r-md);
  font-family: var(--font-sans);
  font-size: 11px;
  line-height: 1.4;
  padding: 4px 8px;
  pointer-events: none;
  box-shadow: var(--shadow-popover);
  max-width: 240px;
  white-space: nowrap;
}

.tt-enter-active,
.tt-leave-active {
  transition: opacity var(--t-fast) ease;
}
.tt-enter-from,
.tt-leave-to {
  opacity: 0;
}
</style>
