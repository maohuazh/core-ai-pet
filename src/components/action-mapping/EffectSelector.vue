<template>
  <div class="effect-selector">
    <div class="selector-row">
      <label>特效:</label>
      <select v-model="selectedEffect" :disabled="disabled" @change="onEffectChange">
        <option value="">(无)</option>
        <option v-for="effect in effects" :key="effect.id" :value="effect.id">
          {{ effect.icon }} {{ effect.name }} ({{ effect.defaultDuration / 1000 }}s)
        </option>
      </select>
    </div>
    <div v-if="selectedEffect" class="selector-row">
      <label>持续:</label>
      <input
        type="number"
        v-model="duration"
        :disabled="disabled"
        min="100"
        step="100"
        @change="onChange"
      />
      <span>ms</span>
    </div>
    <div v-if="selectedEffect" class="selector-row">
      <label>位置:</label>
      <div class="radio-group">
        <label>
          <input type="radio" v-model="position" value="center" :disabled="disabled" @change="onChange" />
          居中
        </label>
        <label>
          <input type="radio" v-model="position" value="above" :disabled="disabled" @change="onChange" />
          上方
        </label>
        <label>
          <input type="radio" v-model="position" value="below" :disabled="disabled" @change="onChange" />
          下方
        </label>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import type { AvailableEffect } from "../../core/action/types";

const props = defineProps<{
  effects: AvailableEffect[];
  modelValue: { name: string; duration: number; position: "center" | "above" | "below" };
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (
    e: "update:modelValue",
    value: { name: string; duration: number; position: "center" | "above" | "below" }
  ): void;
}>();

const selectedEffect = ref(props.modelValue.name);
const duration = ref(props.modelValue.duration);
const position = ref(props.modelValue.position);

function onEffectChange() {
  // Set default duration when selecting an effect
  if (selectedEffect.value) {
    const effect = props.effects.find((e) => e.id === selectedEffect.value);
    if (effect) {
      duration.value = effect.defaultDuration;
    }
  }
  onChange();
}

function onChange() {
  emit("update:modelValue", {
    name: selectedEffect.value,
    duration: duration.value,
    position: position.value,
  });
}

// Sync with parent value
watch(
  () => props.modelValue,
  (newVal) => {
    selectedEffect.value = newVal.name;
    duration.value = newVal.duration;
    position.value = newVal.position;
  }
);
</script>

<style scoped>
.effect-selector {
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

.selector-row select,
.selector-row input[type="number"] {
  flex: 1;
  padding: 6px 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 13px;
  background: white;
}

.selector-row select:disabled,
.selector-row input:disabled {
  background: #f5f5f5;
  cursor: not-allowed;
  opacity: 0.6;
}

.selector-row select:hover:not(:disabled),
.selector-row input:hover:not(:disabled) {
  border-color: #4a90e2;
}

.selector-row select:focus,
.selector-row input:focus {
  outline: none;
  border-color: #4a90e2;
  box-shadow: 0 0 0 2px rgba(74, 144, 226, 0.1);
}

.selector-row span {
  font-size: 13px;
  color: #666;
}

.radio-group {
  display: flex;
  gap: 12px;
}

.radio-group label {
  display: flex;
  align-items: center;
  gap: 4px;
  min-width: auto;
  cursor: pointer;
}

.radio-group input[type="radio"] {
  cursor: pointer;
}

.radio-group input[type="radio"]:disabled {
  cursor: not-allowed;
}
</style>
