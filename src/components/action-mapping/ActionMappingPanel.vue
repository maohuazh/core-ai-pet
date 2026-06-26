<template>
  <div class="action-mapping-panel">
    <div class="panel-header">
      <button class="back-btn" @click="onBack">← 返回模型列表</button>
      <div class="panel-title">动作映射: {{ modelName }}</div>
      <div class="panel-actions">
        <button class="btn btn-secondary" @click="onReset" :disabled="!hasChanges">重置</button>
        <button class="btn btn-primary" @click="onSave" :disabled="!hasChanges || saving">
          {{ saving ? "保存中..." : "保存" }}
        </button>
      </div>
    </div>

    <div v-if="loading" class="loading-state">加载中...</div>

    <div v-else-if="error" class="error-state">
      <p>{{ error }}</p>
      <button @click="loadData">重试</button>
    </div>

    <div v-else class="mapping-list">
      <div class="section-divider">
        <span>日常状态</span>
      </div>
      <MappingRow
        v-for="triggerKey in dailyTriggers"
        :key="triggerKey"
        :model-value="getMappingByTrigger(triggerKey)"
        :motions="motions"
        :expressions="expressions"
        @update:model-value="(val) => updateMapping(triggerKey, val)"
        @preview="onPreview"
      />

      <div class="section-divider">
        <span>事件触发</span>
      </div>
      <MappingRow
        v-for="triggerKey in eventTriggers"
        :key="triggerKey"
        :model-value="getMappingByTrigger(triggerKey)"
        :motions="motions"
        :expressions="expressions"
        @update:model-value="(val) => updateMapping(triggerKey, val)"
        @preview="onPreview"
      />
    </div>

    <div v-if="showUnsavedDialog" class="dialog-overlay" @click="showUnsavedDialog = false">
      <div class="dialog" @click.stop>
        <h3>未保存的修改</h3>
        <p>有未保存的修改，确定要返回吗？</p>
        <div class="dialog-actions">
          <button class="btn btn-secondary" @click="showUnsavedDialog = false">取消</button>
          <button class="btn btn-primary" @click="confirmBack">确定</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import type { MappingFormData, TriggerKey, MotionInfo, ExpressionInfo } from "../../core/action/types";
import { actionMappingService } from "../../core/action/actionMappingService";
import MappingRow from "./MappingRow.vue";

const props = defineProps<{
  modelId: string;
  modelName: string;
}>();

const emit = defineEmits<{
  (e: "back"): void;
}>();

const loading = ref(true);
const saving = ref(false);
const error = ref<string | null>(null);
const showUnsavedDialog = ref(false);

const mappings = ref<MappingFormData[]>([]);
const originalMappings = ref<MappingFormData[]>([]);
const motions = ref<MotionInfo[]>([]);
const expressions = ref<ExpressionInfo[]>([]);

const dailyTriggers: TriggerKey[] = ["daily_1", "daily_2", "daily_3"];
const eventTriggers: TriggerKey[] = [
  "new_message",
  "new_task",
  "new_email",
  "task_in_progress",
  "task_completed",
  "task_approaching_deadline",
  "task_overdue",
];

const hasChanges = computed(() => {
  return JSON.stringify(mappings.value) !== JSON.stringify(originalMappings.value);
});

function getMappingByTrigger(triggerKey: TriggerKey): MappingFormData {
  return mappings.value.find((m) => m.triggerKey === triggerKey)!;
}

function updateMapping(triggerKey: TriggerKey, value: MappingFormData) {
  const index = mappings.value.findIndex((m) => m.triggerKey === triggerKey);
  if (index !== -1) {
    mappings.value[index] = value;
  }
}

async function loadData() {
  loading.value = true;
  error.value = null;

  try {
    // Load mappings
    const records = await actionMappingService.loadMappings(props.modelId);

    if (records.length === 0) {
      // No mappings yet, create defaults
      mappings.value = actionMappingService.createDefaultMappings();
    } else {
      // Convert records to form data
      mappings.value = records.map((r) => actionMappingService.recordToFormData(r));
    }

    // Store original for change detection
    originalMappings.value = JSON.parse(JSON.stringify(mappings.value));

    // Load available motions and expressions
    motions.value = await actionMappingService.getAvailableMotions(props.modelId);
    expressions.value = await actionMappingService.getAvailableExpressions(props.modelId);
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
  } finally {
    loading.value = false;
  }
}

async function onSave() {
  saving.value = true;
  error.value = null;

  try {
    await actionMappingService.saveMappings(props.modelId, mappings.value);
    originalMappings.value = JSON.parse(JSON.stringify(mappings.value));
    alert("配置已保存");
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
    alert(`保存失败: ${error.value}`);
  } finally {
    saving.value = false;
  }
}

function onReset() {
  mappings.value = JSON.parse(JSON.stringify(originalMappings.value));
}

function onBack() {
  if (hasChanges.value) {
    showUnsavedDialog.value = true;
  } else {
    emit("back");
  }
}

function confirmBack() {
  showUnsavedDialog.value = false;
  emit("back");
}

async function onPreview(formData: MappingFormData) {
  try {
    // Emit preview event to pet window
    const { emit } = await import("@tauri-apps/api/event");
    await emit("preview-action-mapping", {
      modelId: props.modelId,
      motionGroup: formData.motion.enabled ? formData.motion.group : null,
      motionName: formData.motion.enabled ? formData.motion.name : null,
      expressionName: formData.expression.enabled ? formData.expression.name : null,
      effectName: formData.effect.enabled ? formData.effect.name : null,
      effectDuration: formData.effect.enabled ? formData.effect.duration : null,
      effectPosition: formData.effect.enabled ? formData.effect.position : null,
    });
  } catch (e) {
    console.error("Preview failed:", e);
  }
}

onMounted(() => {
  loadData();
});
</script>

<style scoped>
.action-mapping-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #f5f6f7;
}

.panel-header {
  display: flex;
  align-items: center;
  padding: 16px 24px;
  background: white;
  border-bottom: 1px solid #e1e4e8;
  gap: 16px;
}

.back-btn {
  padding: 8px 16px;
  border: 1px solid #ddd;
  border-radius: 6px;
  background: white;
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}

.back-btn:hover {
  background: #f0f2f5;
  border-color: #c0c4c8;
}

.panel-title {
  flex: 1;
  font-size: 16px;
  font-weight: 600;
  color: #333;
}

.panel-actions {
  display: flex;
  gap: 8px;
}

.btn {
  padding: 8px 20px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: #4a90e2;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #357abd;
}

.btn-secondary {
  background: white;
  color: #666;
  border: 1px solid #ddd;
}

.btn-secondary:hover:not(:disabled) {
  background: #f0f2f5;
}

.loading-state,
.error-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px;
  color: #666;
}

.error-state {
  color: #d32f2f;
}

.error-state button {
  margin-top: 16px;
  padding: 8px 16px;
  border: 1px solid #d32f2f;
  border-radius: 4px;
  background: white;
  color: #d32f2f;
  cursor: pointer;
}

.mapping-list {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.section-divider {
  margin: 24px 0 16px;
  padding-bottom: 8px;
  border-bottom: 2px solid #4a90e2;
  font-size: 14px;
  font-weight: 600;
  color: #333;
}

.section-divider:first-child {
  margin-top: 0;
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: white;
  border-radius: 8px;
  padding: 24px;
  min-width: 400px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.dialog h3 {
  margin: 0 0 12px;
  font-size: 16px;
  color: #333;
}

.dialog p {
  margin: 0 0 20px;
  font-size: 14px;
  color: #666;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
</style>
