/**
 * Storage frontend bridge.
 * Provides typed access to SQLite storage via Tauri invoke.
 */

import { invoke } from "@tauri-apps/api/core";

/**
 * Get a config value by key.
 */
export async function storageGet(key: string): Promise<string | null> {
  return await invoke<string | null>("storage_get", { key });
}

/**
 * Set a config value by key.
 */
export async function storageSet(key: string, value: string): Promise<void> {
  await invoke("storage_set", { key, value });
}

/**
 * Store a chat message.
 * Returns the message ID.
 */
export async function chatStore(
  _role: string,
  _content: string,
  _metadata?: string
): Promise<number> {
  // Placeholder: will be implemented when chat module is built
  console.warn("chatStore not yet implemented as Tauri command");
  return 0;
}

/**
 * List chat messages.
 */
export interface ChatMessage {
  id: number;
  role: string;
  content: string;
  timestamp: number;
  metadata: string | null;
}

export async function chatList(
  _limit: number = 50,
  _offset: number = 0
): Promise<ChatMessage[]> {
  // Placeholder: will be implemented when chat module is built
  console.warn("chatList not yet implemented as Tauri command");
  return [];
}
