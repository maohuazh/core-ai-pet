/**
 * LLM Provider Registry
 *
 * Manages registration and resolution of LLM providers
 */

import type { LLMRole } from './role';
import type { LLMProvider, ILLMProvider } from './types';

/**
 * Registry entry for a provider
 */
interface ProviderEntry {
  provider: ILLMProvider;
  registeredAt: Date;
}

/**
 * LLM Provider Registry
 *
 * Singleton that manages provider registration and resolution
 */
class LLMRegistry {
  private providers: Map<LLMProvider, ProviderEntry> = new Map();
  private roleToProvider: Map<LLMRole, LLMProvider> = new Map();

  /**
   * Register a provider
   *
   * @param id - Provider identifier
   * @param provider - Provider implementation
   * @throws Error if provider with same id already exists
   */
  register(id: LLMProvider, provider: ILLMProvider): void {
    if (this.providers.has(id)) {
      console.warn(`Provider ${id} is already registered. Overwriting.`);
    }

    this.providers.set(id, {
      provider,
      registeredAt: new Date()
    });

    console.log(`Provider ${id} registered successfully`);
  }

  /**
   * Resolve a provider by id
   *
   * @param id - Provider identifier
   * @returns Provider implementation or undefined if not found
   */
  resolve(id: LLMProvider): ILLMProvider | undefined {
    const entry = this.providers.get(id);
    return entry?.provider;
  }

  /**
   * Map a role to a provider
   *
   * @param role - Role identifier
   * @param providerId - Provider identifier
   */
  mapRoleToProvider(role: LLMRole, providerId: LLMProvider): void {
    if (!this.providers.has(providerId)) {
      throw new Error(`Provider ${providerId} is not registered`);
    }

    this.roleToProvider.set(role, providerId);
  }

  /**
   * Get provider for a role
   *
   * @param role - Role identifier
   * @returns Provider implementation or undefined if not mapped
   */
  getProviderForRole(role: LLMRole): ILLMProvider | undefined {
    const providerId = this.roleToProvider.get(role);
    if (!providerId) {
      return undefined;
    }

    return this.resolve(providerId);
  }

  /**
   * List all registered providers
   *
   * @returns Array of provider ids
   */
  listProviders(): LLMProvider[] {
    return Array.from(this.providers.keys());
  }

  /**
   * List all role to provider mappings
   *
   * @returns Array of [role, providerId] tuples
   */
  listRoleMappings(): Array<[LLMRole, LLMProvider]> {
    return Array.from(this.roleToProvider.entries());
  }

  /**
   * Check if a provider is registered
   *
   * @param id - Provider identifier
   * @returns true if provider is registered
   */
  hasProvider(id: LLMProvider): boolean {
    return this.providers.has(id);
  }

  /**
   * Unregister a provider
   *
   * @param id - Provider identifier
   */
  unregister(id: LLMProvider): void {
    if (this.providers.delete(id)) {
      // Remove any role mappings for this provider
      for (const [role, providerId] of this.roleToProvider.entries()) {
        if (providerId === id) {
          this.roleToProvider.delete(role);
        }
      }

      console.log(`Provider ${id} unregistered`);
    }
  }

  /**
   * Clear all registrations
   */
  clear(): void {
    this.providers.clear();
    this.roleToProvider.clear();
  }
}

/**
 * Singleton instance of the LLM Registry
 */
export const llmRegistry = new LLMRegistry();
