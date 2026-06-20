/**
 * Event types for the EventBus system.
 */

export interface AppEvent {
  event_type: string;
  source: string;
  timestamp: number;
  payload: Record<string, unknown>;
}

export type EventCallback = (event: AppEvent) => void;

/** Standard event types matching Rust backend */
export const EventTypes = {
  UserChat: "UserChat",
  LLMResponse: "LLMResponse",
  EmailReceived: "EmailReceived",
  SlackMessage: "SlackMessage",
  JiraUpdated: "JiraUpdated",
  MeetingStarted: "MeetingStarted",
  TaskCompleted: "TaskCompleted",
  StateChanged: "StateChanged",
} as const;
