/**
 * EventBus frontend bridge.
 * Uses Tauri's event system for frontend-backend communication.
 */

import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import type { AppEvent, EventCallback } from "./types";

export type { AppEvent, EventCallback };
export { EventTypes } from "./types";

/**
 * Subscribe to an event type from the backend event bus.
 * Returns an unlisten function to stop listening.
 */
export async function subscribeEvent(
  eventType: string,
  callback: EventCallback
): Promise<UnlistenFn> {
  return await listen<AppEvent>(eventType, (event) => {
    callback(event.payload);
  });
}

/**
 * Publish an event to the backend event bus.
 * The event will be delivered to all subscribers (both backend and frontend).
 */
export async function publishEvent(
  eventType: string,
  source: string,
  payload: Record<string, unknown> = {}
): Promise<void> {
  // Emit via Tauri event system (frontend listeners)
  await emit(eventType, { event_type: eventType, source, timestamp: Date.now(), payload });

  // Also publish to backend event bus
  await invoke("emit_event", { eventType, source, payload });
}

/**
 * Subscribe to a backend-only event (via invoke).
 * For backend subscriptions that don't need frontend delivery.
 */
export async function subscribeBackendEvent(eventType: string): Promise<string> {
  return await invoke<string>("subscribe_event", { eventType });
}
