/**
 * Settings Panel TypeScript Interfaces
 */

// === Jira Connection ===
export interface JiraConnection {
  id: string;
  name: string;
  url: string;
  email: string;
  status: 'connected' | 'expired' | 'error';
  enabled: boolean;
  created_at: string;
  updated_at: string;
  last_sync_at: string | null;
}

// === Email Account ===
export interface EmailAccount {
  id: string;
  name: string;
  email: string;
  provider: 'gmail' | 'outlook' | 'imap' | 'other';
  status: 'connected' | 'expired' | 'error';
  enabled: boolean;
  created_at: string;
  updated_at: string;
  last_sync_at: string | null;
}

// === Chat Platform ===
export interface ChatPlatform {
  id: string;
  name: string;
  icon: string | null;
  status: 'connected' | 'disconnected' | 'error';
  enabled: boolean;
  account_name: string | null;
  connected_at: string | null;
  created_at: string;
  updated_at: string;
}

// === Model ===
export interface Model {
  id: string;
  name: string;
  model_type: 'live2d' | 'sprite';
  path: string;
  manifest_path: string | null;
  model3_path: string | null;
  thumbnail: string | null;
  source: 'builtin' | 'cdn' | 'custom';
  status: 'active' | 'inactive';
  author: string | null;
  version: string | null;
  description: string | null;
  license: string | null;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

// === Action Mapping ===
export interface ActionMapping {
  id: string;
  model_id: string;
  trigger_key:
    | 'daily_1'
    | 'daily_2'
    | 'daily_3'
    | 'new_message'
    | 'new_task'
    | 'new_email'
    | 'task_in_progress'
    | 'task_completed'
    | 'task_approaching_deadline'
    | 'task_overdue';
  motion_group: string | null;
  motion_name: string | null;
  expression_name: string | null;
  effect_name: string | null;
  use_default: boolean;
  created_at: string;
  updated_at: string;
}

// === Settings Module Types ===
export type SettingsModule = 'jira' | 'email' | 'chat' | 'model' | 'llm';

export interface SettingsState {
  activeModule: SettingsModule;
  isSettingsOpen: boolean;
  jiraConnections: JiraConnection[];
  emailAccounts: EmailAccount[];
  chatPlatforms: ChatPlatform[];
  models: Model[];
}
