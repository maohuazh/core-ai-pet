<template>
  <div class="expression-selector">
    <label>表情:</label>
    <AppSelect
      v-model="selectedExpression"
      :options="expressionOptions"
      :disabled="disabled"
      placeholder="选择表情"
      @change="onChange"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import AppSelect, { type SelectOption } from "../ui/AppSelect.vue";
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

const expressionOptions = computed<SelectOption[]>(() =>
  props.expressions.map((e) => ({
    value: e.name,
    label: e.display_name || e.name,
  }))
);

function onChange() {
  emit("update:modelValue", selectedExpression.value);
}

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
  min-width: 44px;
  font-size: 12px;
  color: var(--text-dim);
  flex-shrink: 0;
}
</style>
