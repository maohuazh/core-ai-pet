<template>
  <div class="motion-selector">
    <div class="selector-row">
      <label>分组:</label>
      <AppSelect
        v-model="selectedGroup"
        :options="groupOptions"
        :disabled="disabled"
        placeholder="选择分组"
        @change="onGroupChange"
      />
    </div>
    <div class="selector-row">
      <label>动作:</label>
      <AppSelect
        v-model="selectedMotion"
        :options="motionOptions"
        :disabled="disabled || !selectedGroup"
        placeholder="选择动作"
        @change="onMotionChange"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import AppSelect, { type SelectOption } from "../ui/AppSelect.vue";
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

const groupOptions = computed<SelectOption[]>(() => {
  const groupSet = new Set(props.motions.map((m) => m.group));
  return Array.from(groupSet).sort().map((g) => ({ value: g, label: g }));
});

const motionOptions = computed<SelectOption[]>(() => {
  if (!selectedGroup.value) return [];
  return props.motions
    .filter((m) => m.group === selectedGroup.value)
    .map((m) => ({ value: m.name, label: m.name }));
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
  min-width: 44px;
  font-size: 12px;
  color: var(--text-dim);
  flex-shrink: 0;
}
</style>
