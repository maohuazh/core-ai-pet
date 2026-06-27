import { getProviderForRole } from './setup';
import type { LLMRole } from './role';
import type { UnifiedRequest, UnifiedDelta } from './types';
import { llmRegistry } from './registry';

/**
 * Custom error types for LLM operations
 */
export class RoleNotFoundError extends Error {
  constructor(role: string) {
    super(`No provider registered for role: ${role}`);
    this.name = 'RoleNotFoundError';
  }
}

export class ConfigNotFoundError extends Error {
  constructor(role: string) {
    super(`Configuration not found for role: ${role}`);
    this.name = 'ConfigNotFoundError';
  }
}

export class SecretNotFoundError extends Error {
  constructor(role: string) {
    super(`Secret not found for role: ${role}`);
    this.name = 'SecretNotFoundError';
  }
}

/**
 * Options for LLM invocation
 */
export interface InvokeOptions {
  abort?: AbortSignal;
}

/**
 * Invoke LLM for a specific role
 *
 * @param role - Role identifier (e.g., 'chat_assistant')
 * @param request - Unified request containing messages and parameters
 * @param onDelta - Callback for streaming deltas
 * @param opts - Optional invocation options (abort signal, etc.)
 * @returns Promise that resolves when streaming completes
 * @throws RoleNotFoundError if no provider is registered for the role
 * @throws ConfigNotFoundError if configuration is missing
 * @throws SecretNotFoundError if API key is not configured
 */
export async function invoke(
  role: LLMRole,
  request: UnifiedRequest,
  onDelta: (delta: UnifiedDelta) => void,
  opts?: InvokeOptions
): Promise<void> {
  // Check if role has a registered provider
  const providerId = llmRegistry.listRoleMappings().find(([r]) => r === role)?.[1];
  if (!providerId) {
    throw new RoleNotFoundError(role);
  }

  // Check if provider is registered
  if (!llmRegistry.hasProvider(providerId)) {
    throw new ConfigNotFoundError(role);
  }

  // Get provider instance
  const provider = getProviderForRole(role);

  // Check abort signal
  if (opts?.abort?.aborted) {
    throw new Error('Invocation aborted');
  }

  // Set up abort handling if signal provided
  if (opts?.abort) {
    const abortHandler = () => {
      // Note: Actual abort implementation would need to be handled in the provider
      // For now, we just throw if already aborted
      throw new Error('Invocation aborted');
    };
    opts.abort.addEventListener('abort', abortHandler, { once: true });
  }

  // Invoke the provider
  try {
    await provider.invoke(
      {
        provider: providerId,
        model: '', // Model will be loaded from config in Rust backend
        base_url: '', // Base URL will be loaded from config in Rust backend
        secret_ref: '', // Secret ref will be loaded from config in Rust backend
        role,
        params: {
          temperature: 0.7,
          max_tokens: 4096
        }
      },
      request,
      onDelta
    );
  } catch (error) {
    // Check if it's an abort error
    if (opts?.abort?.aborted) {
      throw new Error('Invocation aborted');
    }
    throw error;
  }
}

/**
 * LLM client namespace
 */
export const llm = {
  invoke
};
