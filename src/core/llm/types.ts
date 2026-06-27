/**
 * LLM Types
 *
 * Core type definitions for LLM integration
 */

import type { LLMRole } from './role';

/**
 * LLM Provider types
 */
export type LLMProvider = 'anthropic' | 'openai' | 'mock';

/**
 * Message role in a conversation
 */
export type MessageRole = 'user' | 'assistant' | 'system';

/**
 * A single message in a conversation
 */
export interface Message {
  role: MessageRole;
  content: string;
}

/**
 * LLM Configuration
 */
export interface LLMConfig {
  provider: LLMProvider;
  model: string;
  base_url: string;
  secret_ref: string;
  role: LLMRole;
  params: LLMParams;
}

/**
 * LLM Parameters
 */
export interface LLMParams {
  temperature: number;
  max_tokens: number;
}

/**
 * Unified request format for LLM invocations
 */
export interface UnifiedRequest {
  messages: Message[];
  system?: string;
  stream: boolean;
}

/**
 * Unified delta events from LLM streaming responses
 */
export type UnifiedDelta =
  | TextDelta
  | ThinkingDelta
  | ToolUseStartDelta
  | ToolUseDeltaDelta
  | ToolUseEndDelta
  | UsageDelta
  | StopDelta
  | ErrorDelta;

/**
 * Text content delta
 */
export interface TextDelta {
  type: 'text';
  delta: string;
}

/**
 * Thinking process delta (for models with thinking capabilities)
 */
export interface ThinkingDelta {
  type: 'thinking';
  delta: string;
}

/**
 * Tool use start delta
 */
export interface ToolUseStartDelta {
  type: 'tool_use_start';
  id: string;
  name: string;
}

/**
 * Tool use arguments delta
 */
export interface ToolUseDeltaDelta {
  type: 'tool_use_delta';
  id: string;
  args_delta: string;
}

/**
 * Tool use end delta
 */
export interface ToolUseEndDelta {
  type: 'tool_use_end';
  id: string;
}

/**
 * Token usage delta
 */
export interface UsageDelta {
  type: 'usage';
  input_tokens?: number;
  output_tokens?: number;
  cached?: number;
}

/**
 * Stream stop delta
 */
export interface StopDelta {
  type: 'stop';
  reason: string;
}

/**
 * Error delta
 */
export interface ErrorDelta {
  type: 'error';
  recoverable: boolean;
  code: string;
  message: string;
}

/**
 * Provider interface for LLM implementations
 */
export interface ILLMProvider {
  /**
   * Invoke the LLM with a streaming request
   */
  invoke(
    config: LLMConfig,
    request: UnifiedRequest,
    onDelta: (delta: UnifiedDelta) => void
  ): Promise<void>;

  /**
   * Test connection to the provider
   */
  ping(config: LLMConfig, apiKey: string): Promise<void>;

  /**
   * Estimate cost for a request
   */
  estimateCost(config: LLMConfig, messages: Message[]): number;
}

/**
 * LLM Delta event from Rust backend
 */
export interface LLMDeltaEvent {
  turn_id: string;
  delta: UnifiedDelta;
}

/**
 * LLM Done event from Rust backend
 */
export interface LLMDoneEvent {
  turn_id: string;
}
