<template>
  <div class="effect-selector">
    <div class="selector-row">
      <label>特效:</label>
      <AppSelect
        v-model="selectedEffect"
        :options="effectOptions"
        :disabled="disabled"
        placeholder="(无)"
        @change="onEffectChange"
      />
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
        class="num-input"
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
import { ref, computed, watch } from "vue";
import AppSelect, { type SelectOption } from "../ui/AppSelect.vue";
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

const effectOptions = computed<SelectOption[]>(() =>
  props.effects.map((e) => ({
    value: e.id,
    label: `${e.icon} ${e.name} (${e.defaultDuration / 1000}s)`,
  }))
);

function onEffectChange() {
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
  min-width: 44px;
  font-size: 12px;
  color: var(--text-dim);
  flex-shrink: 0;
}

.num-input {
  flex: 1;
  padding: 5px 8px;
  border: 1px solid var(--border-strong);
  border-radius: var(--r-md);
  font-size: 12px;
  font-family: inherit;
  background: var(--bg-base);
  color: var(--text);
}

.num-input:disabled {
  background: var(--bg-surface);
  cursor: not-allowed;
  opacity: 0.5;
}

.num-input:hover:not(:disabled) {
  border-color: var(--accent);
}

.num-input:focus {
  outline: none;
  border-color: var(--accent);
}

.selector-row span {
  font-size: 12px;
  color: var(--text-dim);
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
  color: var(--text-muted);
}

.radio-group input[type="radio"] {
  cursor: pointer;
  accent-color: var(--accent);
}

.radio-group input[type="radio"]:disabled {
  cursor: not-allowed;
}
</style>
