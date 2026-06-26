<template>
  <div class="motion-selector">
    <div class="selector-row">
      <label>分组:</label>
      <select v-model="selectedGroup" :disabled="disabled" @change="onGroupChange">
        <option value="">选择分组</option>
        <option v-for="group in groups" :key="group" :value="group">{{ group }}</option>
      </select>
    </div>
    <div class="selector-row">
      <label>动作:</label>
      <select v-model="selectedMotion" :disabled="disabled || !selectedGroup" @change="onMotionChange">
        <option value="">选择动作</option>
        <option v-for="motion in filteredMotions" :key="motion.name" :value="motion.name">
          {{ motion.name }}
        </option>
      </select>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import type { MotionInfo } from "../../core/action/types";

const props = defineProps<{
  motions: MotionInfo[];
  modelValue: { group: string; name: string };
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: { group: string; name: string }): void;
}>();

const selectedGroup = ref(props.modelValue.group);
const selectedMotion = ref(props.modelValue.name);

// Get unique groups
const groups = computed(() => {
  const groupSet = new Set(props.motions.map((m) => m.group));
  return Array.from(groupSet).sort();
});

// Filter motions by selected group
const filteredMotions = computed(() => {
  if (!selectedGroup.value) return [];
  return props.motions.filter((m) => m.group === selectedGroup.value);
});

function onGroupChange() {
  selectedMotion.value = "";
  emit("update:modelValue", { group: selectedGroup.value, name: "" });
}

function onMotionChange() {
  emit("update:modelValue", {
    group: selectedGroup.value,
    name: selectedMotion.value,
  });
}

// Sync with parent value
watch(
  () => props.modelValue,
  (newVal) => {
    selectedGroup.value = newVal.group;
    selectedMotion.value = newVal.name;
  }
);
</script>

<style scoped>
.motion-selector {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.selector-row {
  display: flex;
  align-items: center;
  gap: 8px;
}

.selector-row label {
  min-width: 50px;
  font-size: 13px;
  color: #666;
}

.selector-row select {
  flex: 1;
  padding: 6px 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 13px;
  background: white;
  cursor: pointer;
}

.selector-row select:disabled {
  background: #f5f5f5;
  cursor: not-allowed;
  opacity: 0.6;
}

.selector-row select:hover:not(:disabled) {
  border-color: #4a90e2;
}

.selector-row select:focus {
  outline: none;
  border-color: #4a90e2;
  box-shadow: 0 0 0 2px rgba(74, 144, 226, 0.1);
}
</style>
