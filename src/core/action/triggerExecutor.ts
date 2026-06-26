/**
 * Bridge module for executing trigger actions in the pet window.
 * Used by triggerHandler to call avatar methods directly (same window),
 * avoiding the Tauri event system which only works cross-window.
 */

export interface TriggerActionPayload {
  motionGroup?: string | null;
  motionName?: string | null;
  expressionName?: string | null;
  effectName?: string | null;
  effectDuration?: number | null;
  effectPosition?: string | null;
}

type TriggerHandler = (payload: TriggerActionPayload) => Promise<void>;

let handler: TriggerHandler | null = null;

export function registerTriggerExecutor(fn: TriggerHandler): void {
  handler = fn;
}

export function unregisterTriggerExecutor(): void {
  handler = null;
}

export async function executeTriggerAction(payload: TriggerActionPayload): Promise<void> {
  if (!handler) {
    console.warn("No trigger executor registered, skipping trigger action");
    return;
  }
  await handler(payload);
}
