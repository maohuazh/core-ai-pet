/**
 * Pet state types matching Rust backend PetState enum.
 */

export type PetState =
  | "Idle"
  | "Walking"
  | "Thinking"
  | "Talking"
  | "Working"
  | "Meeting"
  | "Sleeping"
  | "Alert";

export interface StateChanged {
  old_state: PetState;
  new_state: PetState;
  event: string;
  timestamp: number;
}

export const ALL_STATES: PetState[] = [
  "Idle",
  "Walking",
  "Thinking",
  "Talking",
  "Working",
  "Meeting",
  "Sleeping",
  "Alert",
];
