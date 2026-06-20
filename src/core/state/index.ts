/**
 * State machine frontend bridge.
 * Communicates with Rust backend state machine via Tauri invoke.
 */

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { PetState, StateChanged } from "./types";

export type { PetState, StateChanged };

/**
 * Get the current state from the backend state machine.
 */
export async function getState(): Promise<PetState> {
  const result = await invoke<string>("get_state");
  return JSON.parse(result) as PetState;
}

/**
 * Force set the state on the backend state machine.
 */
export async function setState(state: PetState): Promise<string> {
  return await invoke<string>("set_state", { state });
}

/**
 * Subscribe to state change events from the backend.
 * Returns an unlisten function to stop listening.
 */
export async function onStateChanged(
  callback: (event: StateChanged) => void
): Promise<UnlistenFn> {
  return await listen<StateChanged>("state_changed", (event) => {
    callback(event.payload);
  });
}
