/**
 * Plugin management frontend bridge.
 */

import { invoke } from "@tauri-apps/api/core";

export interface PluginInfo {
  id: string;
  name: string;
  version: string;
  enabled: boolean;
  description: string | null;
}

/**
 * Get list of all loaded plugins.
 */
export async function pluginList(): Promise<PluginInfo[]> {
  return await invoke<PluginInfo[]>("plugin_list");
}

/**
 * Enable a plugin by ID.
 */
export async function pluginEnable(pluginId: string): Promise<void> {
  await invoke("plugin_enable", { pluginId });
}

/**
 * Disable a plugin by ID.
 */
export async function pluginDisable(pluginId: string): Promise<void> {
  await invoke("plugin_disable", { pluginId });
}
