<template>
  <div class="mapping-row">
    <div class="mapping-header" @click="expanded = !expanded">
      <div class="trigger-info">
        <span class="trigger-icon">{{ triggerInfo.icon }}</span>
        <span class="trigger-label">{{ triggerInfo.label }}</span>
        <span v-if="triggerInfo.required" class="required-badge">⭐必填</span>
      </div>
      <button class="preview-btn" @click.stop="onPreview" title="预览">▶</button>
      <span class="expand-icon">{{ expanded ? "▼" : "▶" }}</span>
    </div>

    <div v-if="expanded" class="mapping-content">
      <div class="use-default-row">
        <label>
          <input type="checkbox" v-model="localData.useDefault" :disabled="disabled" />
          使用模型默认值
        </label>
      </div>

      <div class="selectors-grid" :class="{ disabled: localData.useDefault }">
        <!-- Motion Section -->
        <div class="selector-section">
          <div class="section-header">
            <label>
              <input
                type="checkbox"
                v-model="localData.motion.enabled"
                :disabled="localData.useDefault || disabled"
              />
              动作
            </label>
          </div>
          <MotionSelector
            v-if="localData.motion.enabled"
            :motions="motions"
            v-model="localData.motion"
            :disabled="localData.useDefault || disabled"
          />
        </div>

        <!-- Expression Section -->
        <div class="selector-section">
          <div class="section-header">
            <label>
              <input
                type="checkbox"
                v-model="localData.expression.enabled"
                :disabled="localData.useDefault || disabled"
              />
              表情
            </label>
          </div>
          <ExpressionSelector
            v-if="localData.expression.enabled"
            :expressions="expressions"
            v-model="localData.expression.name"
            :disabled="localData.useDefault || disabled"
          />
        </div>

        <!-- Effect Section -->
        <div class="selector-section">
          <div class="section-header">
            <label>
              <input
                type="checkbox"
                v-model="localData.effect.enabled"
                :disabled="localData.useDefault || disabled"
              />
              特效
            </label>
          </div>
          <EffectSelector
            v-if="localData.effect.enabled"
            :effects="AVAILABLE_EFFECTS"
            v-model="localData.effect"
            :disabled="localData.useDefault || disabled"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import type { MappingFormData, MotionInfo, ExpressionInfo, TriggerKey } from "../../core/action/types";
import { AVAILABLE_EFFECTS, TRIGGER_INFO } from "../../core/action/effects";
import MotionSelector from "./MotionSelector.vue";
import ExpressionSelector from "./ExpressionSelector.vue";
import EffectSelector from "./EffectSelector.vue";

const props = defineProps<{
  modelValue: MappingFormData;
  motions: MotionInfo[];
  expressions: ExpressionInfo[];
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: MappingFormData): void;
  (e: "preview", value: MappingFormData): void;
}>();

const expanded = ref(false);
const localData = ref<MappingFormData>(JSON.parse(JSON.stringify(props.modelValue)));

// Get trigger info
const triggerInfo = TRIGGER_INFO.find((t) => t.key === props.modelValue.triggerKey) || {
  key: props.modelValue.triggerKey as TriggerKey,
  label: props.modelValue.triggerKey,
  icon: "📌",
  required: false,
  description: "",
};

function onPreview() {
  emit("preview", localData.value);
}

// Watch for changes and emit to parent
watch(
  localData,
  (newVal) => {
    emit("update:modelValue", newVal);
  },
  { deep: true }
);

// Sync with parent value
watch(
  () => props.modelValue,
  (newVal) => {
    localData.value = JSON.parse(JSON.stringify(newVal));
  },
  { deep: true }
);
</script>

<style scoped>
.mapping-row {
  border: 1px solid var(--border-strong);
  border-radius: var(--r-lg);
  margin-bottom: 10px;
  background: var(--bg-elevated);
  overflow: hidden;
}

.mapping-header {
  display: flex;
  align-items: center;
  padding: 10px 14px;
  background: var(--bg-elevated);
  cursor: pointer;
  user-select: none;
  transition: background var(--t-fast);
}

.mapping-header:hover {
  background: var(--bg-hover);
}

.trigger-info {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 8px;
}

.trigger-icon {
  font-size: 16px;
}

.trigger-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text);
}

.required-badge {
  font-size: 10px;
  padding: 1px 6px;
  background: rgba(249, 226, 175, 0.15);
  color: var(--warning);
  border-radius: var(--r-sm);
}

.preview-btn {
  padding: 3px 10px;
  border: 1px solid var(--border-strong);
  border-radius: var(--r-sm);
  background: transparent;
  color: var(--text-muted);
  font-family: inherit;
  cursor: pointer;
  font-size: 11px;
  transition: background var(--t-fast), color var(--t-fast), border-color var(--t-fast);
}

.preview-btn:hover {
  background: var(--accent);
  color: var(--bg-base);
  border-color: var(--accent);
}

.expand-icon {
  margin-left: 8px;
  font-size: 11px;
  color: var(--text-dim);
}

.mapping-content {
  padding: 14px;
  border-top: 1px solid var(--border);
  background: var(--bg-base);
}

.use-default-row {
  margin-bottom: 14px;
}

.use-default-row label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 12px;
  color: var(--text-muted);
}

.use-default-row input[type="checkbox"] {
  cursor: pointer;
  accent-color: var(--accent);
}

.selectors-grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 12px;
}

.selectors-grid.disabled {
  opacity: 0.4;
  pointer-events: none;
}

.selector-section {
  border: 1px solid var(--border-strong);
  border-radius: var(--r-md);
  padding: 10px;
  background: var(--bg-surface);
}

.section-header {
  margin-bottom: 8px;
}

.section-header label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  font-weight: 500;
  color: var(--text);
  cursor: pointer;
}

.section-header input[type="checkbox"] {
  cursor: pointer;
  accent-color: var(--accent);
}

@media (max-width: 768px) {
  .selectors-grid {
    grid-template-columns: 1fr;
  }
}
</style>
