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

    <ConfirmDialog
      v-model:visible="showUnsavedDialog"
      title="未保存的修改"
      message="有未保存的修改，确定要返回吗？"
      confirm-text="确定返回"
      confirm-class="primary"
      @confirm="confirmBack"
    />

    <AppModal
      :open="showSaveStatus"
      :title="saveOk ? '保存成功' : '保存失败'"
      max-width="380px"
      @update:open="showSaveStatus = $event"
    >
      <p class="status-msg" :class="{ err: !saveOk }">
        {{ saveOk ? '配置已保存' : `保存失败: ${error}` }}
      </p>
      <template #footer>
        <button class="btn-ok" @click="showSaveStatus = false">好的</button>
      </template>
    </AppModal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import type { MappingFormData, TriggerKey, MotionInfo, ExpressionInfo } from "../../core/action/types";
import { actionMappingService } from "../../core/action/actionMappingService";
import MappingRow from "./MappingRow.vue";
import ConfirmDialog from "../settings/shared/ConfirmDialog.vue";
import AppModal from "../ui/AppModal.vue";

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
const showSaveStatus = ref(false);
const saveOk = ref(true);

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
    saveOk.value = true;
    showSaveStatus.value = true;
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
    saveOk.value = false;
    showSaveStatus.value = true;
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
  background: var(--bg-base);
  color: var(--text);
  margin: -24px;
}

.panel-header {
  display: flex;
  align-items: center;
  padding: 12px 20px;
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border);
  gap: 12px;
}

.back-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-strong);
  border-radius: var(--r-md);
  background: transparent;
  color: var(--text-muted);
  font-family: inherit;
  cursor: pointer;
  font-size: 12px;
  transition: background var(--t-fast), color var(--t-fast);
}

.back-btn:hover {
  background: var(--bg-elevated);
  color: var(--text);
}

.panel-title {
  flex: 1;
  font-size: 14px;
  font-weight: 600;
  color: var(--text);
}

.panel-actions {
  display: flex;
  gap: 8px;
}

.btn {
  padding: 6px 16px;
  border: none;
  border-radius: var(--r-md);
  font-size: 12px;
  font-weight: 500;
  font-family: inherit;
  cursor: pointer;
  transition: background var(--t-fast);
}

.btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--accent);
  color: var(--bg-base);
  font-weight: 600;
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn-secondary {
  background: var(--bg-hover);
  color: var(--text);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--bg-hover-2);
}

.loading-state,
.error-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px;
  color: var(--text-dim);
  font-size: 13px;
}

.error-state {
  color: var(--danger);
}

.error-state button {
  margin-top: 14px;
  padding: 6px 14px;
  border: 1px solid var(--danger);
  border-radius: var(--r-md);
  background: transparent;
  color: var(--danger);
  font-family: inherit;
  cursor: pointer;
}

.mapping-list {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.mapping-list::-webkit-scrollbar {
  width: 5px;
}
.mapping-list::-webkit-scrollbar-track {
  background: transparent;
}
.mapping-list::-webkit-scrollbar-thumb {
  background: var(--border-strong);
  border-radius: 3px;
}

.section-divider {
  margin: 20px 0 12px;
  padding-bottom: 6px;
  border-bottom: 1px solid var(--accent);
  font-size: 12px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.section-divider:first-child {
  margin-top: 0;
}

.status-msg {
  margin: 0;
  font-size: 13px;
  color: var(--text-muted);
  line-height: 1.55;
}
.status-msg.err {
  color: var(--danger);
}

.btn-ok {
  padding: 6px 16px;
  border: none;
  border-radius: var(--r-lg);
  background: var(--accent);
  color: var(--bg-base);
  font-size: 13px;
  font-weight: 600;
  font-family: inherit;
  cursor: pointer;
  transition: background var(--t-fast);
}
.btn-ok:hover {
  background: var(--accent-hover);
}
</style>
