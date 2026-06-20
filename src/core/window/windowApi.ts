import { invoke } from "@tauri-apps/api/core";

/**
 * Enable or disable click-through mode on the pet window.
 * When enabled, mouse events pass through to windows below.
 */
export async function setClickThrough(enabled: boolean): Promise<void> {
  await invoke("set_click_through", { enabled });
}
