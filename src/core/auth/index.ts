/**
 * Auth frontend bridge.
 * Provides login / logout / register / getCurrentUser via Tauri invoke.
 */

import { invoke } from "@tauri-apps/api/core";

export interface UserProfile {
  id: string;
  first_name: string;
  last_name: string;
  email: string;
  avatar: string | null;
}

export async function login(email: string, password: string): Promise<UserProfile> {
  return await invoke<UserProfile>("login", { email, password });
}

export async function logout(): Promise<void> {
  await invoke("logout");
}

export async function getCurrentUser(): Promise<UserProfile | null> {
  return await invoke<UserProfile | null>("get_current_user");
}

export async function register(
  firstName: string,
  lastName: string,
  email: string,
  password: string,
): Promise<UserProfile> {
  return await invoke<UserProfile>("register", {
    firstName,
    lastName,
    email,
    password,
  });
}
