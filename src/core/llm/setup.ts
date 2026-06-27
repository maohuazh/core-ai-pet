import { llmRegistry } from './registry';
import { anthropicProvider } from './providers';
import type { LLMRole } from './role';

/**
 * Initialize LLM subsystem
 *
 * Registers all available providers and sets up default role mappings
 */
export function initializeLLM(): void {
  // Register Anthropic provider
  llmRegistry.register('anthropic', anthropicProvider);

  // Map default roles to providers
  llmRegistry.mapRoleToProvider('chat_assistant', 'anthropic');

  console.log('LLM subsystem initialized');
}

/**
 * Get provider for a role
 *
 * @param role - Role identifier
 * @returns Provider instance
 * @throws Error if provider not found for role
 */
export function getProviderForRole(role: LLMRole) {
  const provider = llmRegistry.getProviderForRole(role);
  if (!provider) {
    throw new Error(`No provider registered for role: ${role}`);
  }
  return provider;
}
