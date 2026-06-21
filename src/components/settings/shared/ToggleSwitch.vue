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
  width: 44px;
  height: 24px;
  border: none;
  border-radius: 12px;
  background: rgba(0, 0, 0, 0.1);
  cursor: pointer;
  padding: 0;
  transition: all 0.2s ease;
}

.toggle-switch:hover {
  background: rgba(0, 0, 0, 0.15);
}

.toggle-switch.active {
  background: #6366f1;
}

.toggle-switch.active:hover {
  background: #818cf8;
}

.toggle-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: white;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  transition: transform 0.2s ease;
}

.toggle-switch.active .toggle-thumb {
  transform: translateX(20px);
}
</style>
