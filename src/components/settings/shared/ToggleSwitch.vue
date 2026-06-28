<template>
  <button
    :class="['toggle-switch', { active: enabled }]"
    @click="toggle"
    role="switch"
    :aria-checked="enabled"
  >
    <span class="toggle-thumb"></span>
  </button>
</template>

<script setup lang="ts">
const props = defineProps<{
  enabled: boolean;
}>();

const emit = defineEmits<{
  'update:enabled': [enabled: boolean];
}>();

const toggle = () => {
  emit('update:enabled', !props.enabled);
};
</script>

<style scoped>
.toggle-switch {
  position: relative;
  width: 40px;
  height: 22px;
  border: none;
  border-radius: 11px;
  background: var(--bg-hover);
  cursor: pointer;
  padding: 0;
  transition: background var(--t-fast);
}

.toggle-switch:hover {
  background: var(--bg-hover-2);
}

.toggle-switch.active {
  background: var(--accent);
}

.toggle-switch.active:hover {
  background: var(--accent-hover);
}

.toggle-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: var(--text);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
  transition: transform var(--t-fast);
}

.toggle-switch.active .toggle-thumb {
  background: var(--bg-base);
  transform: translateX(18px);
}
</style>
