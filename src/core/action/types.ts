/**
 * Action & Expression Mapping Types
 */

// Database record type
export interface ActionMappingRecord {
  id: string;
  model_id: string;
  trigger_key: TriggerKey;
  motion_group: string | null;
  motion_name: string | null;
  expression_name: string | null;
  effect_name: string | null;
  effect_duration: number | null;
  effect_position: "center" | "above" | "below" | null;
  use_default: number; // 0 or 1
  created_at: string;
  updated_at: string;
}

// Trigger key types
export type TriggerKey =
  | "daily_1"
  | "daily_2"
  | "daily_3"
  | "new_message"
  | "new_task"
  | "new_email"
  | "task_in_progress"
  | "task_completed"
  | "task_approaching_deadline"
  | "task_overdue"
  | "llm.message"
  | "llm.invoke";

// Frontend form state
export interface MappingFormData {
  triggerKey: TriggerKey;
  useDefault: boolean;

  motion: {
    enabled: boolean;
    group: string;
    name: string;
  };

  expression: {
    enabled: boolean;
    name: string;
  };

  effect: {
    enabled: boolean;
    name: string;
    duration: number;
    position: "center" | "above" | "below";
  };
}

// Motion info from backend
export interface MotionInfo {
  group: string;
  name: string;
  display_name: string;
}

// Expression info from backend
export interface ExpressionInfo {
  name: string;
  display_name: string;
  file: string | null;
}

// Available effect
export interface AvailableEffect {
  id: string;
  name: string;
  icon: string;
  description: string;
  defaultDuration: number;
}

// Trigger display info
export interface TriggerInfo {
  key: TriggerKey;
  label: string;
  icon: string;
  required: boolean;
  description: string;
}
