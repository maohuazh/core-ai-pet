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
  border: 1px solid #ddd;
  border-radius: 8px;
  margin-bottom: 12px;
  background: white;
  overflow: hidden;
}

.mapping-header {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  background: #f8f9fa;
  cursor: pointer;
  user-select: none;
}

.mapping-header:hover {
  background: #f0f2f5;
}

.trigger-info {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 8px;
}

.trigger-icon {
  font-size: 18px;
}

.trigger-label {
  font-size: 14px;
  font-weight: 500;
  color: #333;
}

.required-badge {
  font-size: 11px;
  padding: 2px 6px;
  background: #fff3cd;
  border-radius: 4px;
  color: #856404;
}

.preview-btn {
  padding: 4px 12px;
  border: 1px solid #ddd;
  border-radius: 4px;
  background: white;
  cursor: pointer;
  font-size: 12px;
  transition: all 0.2s;
}

.preview-btn:hover {
  background: #4a90e2;
  color: white;
  border-color: #4a90e2;
}

.expand-icon {
  margin-left: 8px;
  font-size: 12px;
  color: #666;
}

.mapping-content {
  padding: 16px;
  border-top: 1px solid #eee;
}

.use-default-row {
  margin-bottom: 16px;
}

.use-default-row label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-size: 13px;
}

.use-default-row input[type="checkbox"] {
  cursor: pointer;
}

.selectors-grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 16px;
}

.selectors-grid.disabled {
  opacity: 0.5;
  pointer-events: none;
}

.selector-section {
  border: 1px solid #eee;
  border-radius: 6px;
  padding: 12px;
  background: #fafbfc;
}

.section-header {
  margin-bottom: 8px;
}

.section-header label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  font-weight: 500;
  color: #333;
  cursor: pointer;
}

.section-header input[type="checkbox"] {
  cursor: pointer;
}

@media (max-width: 768px) {
  .selectors-grid {
    grid-template-columns: 1fr;
  }
}
</style>
