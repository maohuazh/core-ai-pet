/**
 * Action Mapping Service - wraps IPC calls for action mapping CRUD operations
 */

import { invoke } from "@tauri-apps/api/core";
import type {
  ActionMappingRecord,
  MappingFormData,
  MotionInfo,
  ExpressionInfo,
  TriggerKey,
} from "./types";

class ActionMappingService {
  /**
   * Load all mappings for a model
   */
  async loadMappings(modelId: string): Promise<ActionMappingRecord[]> {
    return invoke<ActionMappingRecord[]>("get_action_mappings", {
      modelId,
    });
  }

  /**
   * Save a single mapping (upsert)
   */
  async saveMapping(
    modelId: string,
    formData: MappingFormData
  ): Promise<void> {
    // Validate daily_1 constraint
    if (formData.triggerKey === "daily_1") {
      if (
        !formData.useDefault &&
        !formData.motion.enabled &&
        !formData.expression.enabled
      ) {
        throw new Error("日常1 必须配置动作或表情，或选择使用默认值");
      }
    }

    const params = this.formDataToParams(formData, modelId);

    await invoke("save_action_mapping", params);
  }

  /**
   * Save multiple mappings in batch
   */
  async saveMappings(
    modelId: string,
    formDataList: MappingFormData[]
  ): Promise<void> {
    // Validate daily_1 constraint
    const daily1 = formDataList.find((f) => f.triggerKey === "daily_1");
    if (daily1) {
      if (
        !daily1.useDefault &&
        !daily1.motion.enabled &&
        !daily1.expression.enabled
      ) {
        throw new Error("日常1 必须配置动作或表情，或选择使用默认值");
      }
    }

    // Save each mapping
    for (const formData of formDataList) {
      await this.saveMapping(modelId, formData);
    }
  }

  /**
   * Delete a mapping by id
   */
  async deleteMapping(id: string): Promise<void> {
    await invoke("delete_action_mapping", { id });
  }

  /**
   * Get available motions for a model
   */
  async getAvailableMotions(modelId: string): Promise<MotionInfo[]> {
    return invoke<MotionInfo[]>("get_available_motions", { modelId });
  }

  /**
   * Get available expressions for a model
   */
  async getAvailableExpressions(modelId: string): Promise<ExpressionInfo[]> {
    return invoke<ExpressionInfo[]>("get_available_expressions", { modelId });
  }

  /**
   * Create default mappings for a model (10 trigger scenarios)
   */
  createDefaultMappings(): MappingFormData[] {
    const triggers: TriggerKey[] = [
      "daily_1",
      "daily_2",
      "daily_3",
      "new_message",
      "new_task",
      "new_email",
      "task_in_progress",
      "task_completed",
      "task_approaching_deadline",
      "task_overdue",
    ];

    return triggers.map((key) => ({
      triggerKey: key,
      useDefault: key === "daily_1", // Only daily_1 defaults to use_default
      motion: { enabled: false, group: "", name: "" },
      expression: { enabled: false, name: "" },
      effect: {
        enabled: false,
        name: "",
        duration: 2000,
        position: "center" as const,
      },
    }));
  }

  /**
   * Convert database record to form data
   */
  recordToFormData(record: ActionMappingRecord): MappingFormData {
    return {
      triggerKey: record.trigger_key,
      useDefault: record.use_default === 1,
      motion: {
        enabled: !!record.motion_name,
        group: record.motion_group || "",
        name: record.motion_name || "",
      },
      expression: {
        enabled: !!record.expression_name,
        name: record.expression_name || "",
      },
      effect: {
        enabled: !!record.effect_name,
        name: record.effect_name || "",
        duration: record.effect_duration || 2000,
        position: (record.effect_position as "center" | "above" | "below") || "center",
      },
    };
  }

  /**
   * Convert form data to IPC parameters
   */
  private formDataToParams(
    form: MappingFormData,
    modelId: string
  ): Record<string, any> {
    return {
      modelId,
      triggerKey: form.triggerKey,
      motionGroup: form.motion.enabled ? form.motion.group : null,
      motionName: form.motion.enabled ? form.motion.name : null,
      expressionName: form.expression.enabled ? form.expression.name : null,
      effectName: form.effect.enabled ? form.effect.name : null,
      effectDuration: form.effect.enabled ? form.effect.duration : null,
      effectPosition: form.effect.enabled ? form.effect.position : null,
      useDefault: form.useDefault,
    };
  }
}

export const actionMappingService = new ActionMappingService();
