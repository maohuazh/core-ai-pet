<template>
  <div class="expression-selector">
    <label>表情:</label>
    <select v-model="selectedExpression" :disabled="disabled" @change="onChange">
      <option value="">选择表情</option>
      <option v-for="expr in expressions" :key="expr.name" :value="expr.name">
        {{ expr.display_name || expr.name }}
      </option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import type { ExpressionInfo } from "../../core/action/types";

const props = defineProps<{
  expressions: ExpressionInfo[];
  modelValue: string;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
}>();

const selectedExpression = ref(props.modelValue);

function onChange() {
  emit("update:modelValue", selectedExpression.value);
}

// Sync with parent value
watch(
  () => props.modelValue,
  (newVal) => {
    selectedExpression.value = newVal;
  }
);
</script>

<style scoped>
.expression-selector {
  display: flex;
  align-items: center;
  gap: 8px;
}

.expression-selector label {
  min-width: 50px;
  font-size: 13px;
  color: #666;
}

.expression-selector select {
  flex: 1;
  padding: 6px 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 13px;
  background: white;
  cursor: pointer;
}

.expression-selector select:disabled {
  background: #f5f5f5;
  cursor: not-allowed;
  opacity: 0.6;
}

.expression-selector select:hover:not(:disabled) {
  border-color: #4a90e2;
}

.expression-selector select:focus {
  outline: none;
  border-color: #4a90e2;
  box-shadow: 0 0 0 2px rgba(74, 144, 226, 0.1);
}
</style>
