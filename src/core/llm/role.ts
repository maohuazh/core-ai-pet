/**
 * LLM Role definitions
 *
 * Roles define the purpose and context for LLM interactions.
 */

/**
 * Available LLM roles in the system
 */
export type LLMRole = 'chat_assistant';

/**
 * Role metadata
 */
export interface RoleMetadata {
  name: string;
  description: string;
  defaultModel: string;
}

/**
 * Role registry mapping
 */
export const ROLE_METADATA: Record<LLMRole, RoleMetadata> = {
  chat_assistant: {
    name: 'Chat Assistant',
    description: 'General purpose chat assistant for user interactions',
    defaultModel: 'claude-3-5-sonnet-20241022'
  }
};
