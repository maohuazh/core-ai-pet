/**
 * Trigger Handler - listens for external events and triggers configured actions
 */

import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { actionMappingService } from "../action/actionMappingService";
import { executeTriggerAction } from "../action/triggerExecutor";
import { petStore } from "../model/PetStore";
import type { TriggerKey } from "../action/types";

class TriggerHandler {
  private unlistenFns: UnlistenFn[] = [];
  private dailyTickInterval: ReturnType<typeof setInterval> | null = null;

  // Event type to trigger key mapping
  private eventMapping: Record<string, TriggerKey> = {
    "chat-message-received": "new_message",
    "jira-task-assigned": "new_task",
    "email-received": "new_email",
    "jira-task-status-changed": "task_in_progress",
    "jira-task-completed": "task_completed",
    "jira-task-deadline-approaching": "task_approaching_deadline",
    "jira-task-overdue": "task_overdue",
    "llm.message": "llm.message",
    "llm.invoke": "llm.invoke",
  };

  /**
   * Initialize the trigger handler and start listening for events
   */
  async init(): Promise<void> {
    // Listen for external events from backend
    for (const eventType of Object.keys(this.eventMapping)) {
      const unlisten = await listen(eventType, async () => {
        const triggerKey = this.eventMapping[eventType];
        if (triggerKey) {
          await this.fireTrigger(triggerKey);
        }
      });
      this.unlistenFns.push(unlisten);
    }

    // Start daily tick timer (every 5 minutes)
    this.startDailyTick();

    console.log("TriggerHandler initialized");
  }

  /**
   * Fire a trigger and execute the configured action (public for testing)
   */
  async fireTrigger(triggerKey: TriggerKey): Promise<void> {
    // Handle LLM triggers separately
    if (triggerKey === "llm.message" || triggerKey === "llm.invoke") {
      await this.handleLLmTrigger(triggerKey);
      return;
    }

    const activeModelId = petStore.currentModel.value.id;
    if (!activeModelId) {
      console.warn("No active model, skipping trigger");
      return;
    }

    try {
      // Load mappings for the active model
      const mappings = await actionMappingService.loadMappings(activeModelId);

      // Find the mapping for this trigger
      const record = mappings.find((m) => m.trigger_key === triggerKey);
      if (!record) {
        console.log(`No mapping found for trigger: ${triggerKey}`);
        return;
      }

      // Convert to form data for easier handling
      const formData = actionMappingService.recordToFormData(record);

      // If use_default is true, just play default idle motion
      if (formData.useDefault) {
        console.log(`Trigger ${triggerKey}: using default motion`);
        await executeTriggerAction({
          motionGroup: "Idle",
          motionName: null,
          expressionName: null,
          effectName: null,
          effectDuration: null,
          effectPosition: null,
        });
        return;
      }

      // Build payload from configured actions
      const payload: Record<string, any> = {
        modelId: activeModelId,
        motionGroup: null,
        motionName: null,
        expressionName: null,
        effectName: null,
        effectDuration: null,
        effectPosition: null,
      };

      if (formData.motion.enabled && formData.motion.group) {
        console.log(`Trigger ${triggerKey}: playing motion ${formData.motion.group}/${formData.motion.name}`);
        payload.motionGroup = formData.motion.group;
        payload.motionName = formData.motion.name;
      }

      if (formData.expression.enabled && formData.expression.name) {
        console.log(`Trigger ${triggerKey}: playing expression ${formData.expression.name}`);
        payload.expressionName = formData.expression.name;
      }

      if (formData.effect.enabled && formData.effect.name) {
        payload.effectName = formData.effect.name;
        payload.effectDuration = formData.effect.duration;
        payload.effectPosition = formData.effect.position;
      }

      // Execute via the bridge (direct call, same window)
      await executeTriggerAction(payload);
    } catch (e) {
      console.error(`Failed to fire trigger ${triggerKey}:`, e);
    }
  }

  /**
   * Handle LLM-related triggers
   */
  private async handleLLmTrigger(triggerKey: TriggerKey): Promise<void> {
    console.log(`LLM trigger fired: ${triggerKey}`);
    // LLM triggers don't execute pet actions, they're handled by the chat UI
    // This is just a placeholder for potential future pet reactions to LLM events
  }

  /**
   * Start the daily tick timer
   */
  private startDailyTick(): void {
    // Fire every 5 minutes
    this.dailyTickInterval = setInterval(async () => {
      // TEMP: always daily_2 for testing, restore weights after
      const triggerKey: TriggerKey = "daily_2";

      console.log(`Daily tick: triggering ${triggerKey}`);
      await this.fireTrigger(triggerKey);
    }, 10 * 1000); // 10 seconds (TEMP: for testing, change back to 5 * 60 * 1000)

    console.log("Daily tick timer started");
  }

  /**
   * Stop the trigger handler
   */
  destroy(): void {
    // Unlisten all events
    for (const unlisten of this.unlistenFns) {
      unlisten();
    }
    this.unlistenFns = [];

    // Stop daily tick timer
    if (this.dailyTickInterval) {
      clearInterval(this.dailyTickInterval);
      this.dailyTickInterval = null;
    }

    console.log("TriggerHandler destroyed");
  }
}

export const triggerHandler = new TriggerHandler();
