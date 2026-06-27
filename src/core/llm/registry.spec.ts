import { describe, it, expect, beforeEach, vi } from 'vitest';
import { llmRegistry } from '@/core/llm/registry';
import type { ILLMProvider, LLMConfig, UnifiedRequest, UnifiedDelta, Message } from '@/core/llm/types';
import type { LLMRole } from '@/core/llm/role';

// Mock provider for testing
class MockProvider implements ILLMProvider {
  async invoke(
    config: LLMConfig,
    request: UnifiedRequest,
    onDelta: (delta: UnifiedDelta) => void
  ): Promise<void> {
    // Mock implementation
    onDelta({
      type: 'text',
      delta: 'Mock response'
    });
  }

  async ping(config: LLMConfig, apiKey: string): Promise<void> {
    // Mock implementation
    if (apiKey === 'invalid') {
      throw new Error('Invalid API key');
    }
  }

  estimateCost(config: LLMConfig, messages: Message[]): number {
    return 0.01; // Mock cost
  }
}

describe('LLMRegistry', () => {
  beforeEach(() => {
    llmRegistry.clear();
  });

  describe('register', () => {
    it('should register a new provider', () => {
      const provider = new MockProvider();
      llmRegistry.register('mock', provider);

      expect(llmRegistry.hasProvider('mock')).toBe(true);
    });

    it('should overwrite existing provider with warning', () => {
      const consoleSpy = vi.spyOn(console, 'warn');
      const provider1 = new MockProvider();
      const provider2 = new MockProvider();

      llmRegistry.register('mock', provider1);
      llmRegistry.register('mock', provider2);

      expect(consoleSpy).toHaveBeenCalledWith(
        'Provider mock is already registered. Overwriting.'
      );
      expect(llmRegistry.hasProvider('mock')).toBe(true);
    });

    it('should log successful registration', () => {
      const consoleSpy = vi.spyOn(console, 'log');
      const provider = new MockProvider();

      llmRegistry.register('mock', provider);

      expect(consoleSpy).toHaveBeenCalledWith(
        'Provider mock registered successfully'
      );
    });
  });

  describe('resolve', () => {
    it('should resolve registered provider', () => {
      const provider = new MockProvider();
      llmRegistry.register('mock', provider);

      const resolved = llmRegistry.resolve('mock');
      expect(resolved).toBe(provider);
    });

    it('should return undefined for unregistered provider', () => {
      const resolved = llmRegistry.resolve('nonexistent');
      expect(resolved).toBeUndefined();
    });
  });

  describe('hasProvider', () => {
    it('should return true for registered provider', () => {
      const provider = new MockProvider();
      llmRegistry.register('mock', provider);

      expect(llmRegistry.hasProvider('mock')).toBe(true);
    });

    it('should return false for unregistered provider', () => {
      expect(llmRegistry.hasProvider('nonexistent')).toBe(false);
    });
  });

  describe('listProviders', () => {
    it('should return empty array when no providers registered', () => {
      const providers = llmRegistry.listProviders();
      expect(providers).toEqual([]);
    });

    it('should return all registered provider ids', () => {
      const provider1 = new MockProvider();
      const provider2 = new MockProvider();

      llmRegistry.register('mock1', provider1);
      llmRegistry.register('mock2', provider2);

      const providers = llmRegistry.listProviders();
      expect(providers).toContain('mock1');
      expect(providers).toContain('mock2');
      expect(providers).toHaveLength(2);
    });
  });

  describe('mapRoleToProvider', () => {
    it('should map role to provider', () => {
      const provider = new MockProvider();
      llmRegistry.register('mock', provider);

      llmRegistry.mapRoleToProvider('chat_assistant', 'mock');

      const resolved = llmRegistry.getProviderForRole('chat_assistant');
      expect(resolved).toBe(provider);
    });

    it('should throw error if provider not registered', () => {
      expect(() => {
        llmRegistry.mapRoleToProvider('chat_assistant', 'nonexistent');
      }).toThrow('Provider nonexistent is not registered');
    });
  });

  describe('getProviderForRole', () => {
    it('should return provider for mapped role', () => {
      const provider = new MockProvider();
      llmRegistry.register('mock', provider);
      llmRegistry.mapRoleToProvider('chat_assistant', 'mock');

      const resolved = llmRegistry.getProviderForRole('chat_assistant');
      expect(resolved).toBe(provider);
    });

    it('should return undefined for unmapped role', () => {
      const resolved = llmRegistry.getProviderForRole('chat_assistant');
      expect(resolved).toBeUndefined();
    });
  });

  describe('listRoleMappings', () => {
    it('should return empty array when no mappings exist', () => {
      const mappings = llmRegistry.listRoleMappings();
      expect(mappings).toEqual([]);
    });

    it('should return all role to provider mappings', () => {
      const provider = new MockProvider();
      llmRegistry.register('mock', provider);

      llmRegistry.mapRoleToProvider('chat_assistant', 'mock');

      const mappings = llmRegistry.listRoleMappings();
      expect(mappings).toEqual([['chat_assistant', 'mock']]);
    });
  });

  describe('unregister', () => {
    it('should unregister provider', () => {
      const provider = new MockProvider();
      llmRegistry.register('mock', provider);

      llmRegistry.unregister('mock');

      expect(llmRegistry.hasProvider('mock')).toBe(false);
    });

    it('should remove role mappings when unregistering provider', () => {
      const provider = new MockProvider();
      llmRegistry.register('mock', provider);
      llmRegistry.mapRoleToProvider('chat_assistant', 'mock');

      llmRegistry.unregister('mock');

      const resolved = llmRegistry.getProviderForRole('chat_assistant');
      expect(resolved).toBeUndefined();
    });

    it('should log successful unregistration', () => {
      const consoleSpy = vi.spyOn(console, 'log');
      const provider = new MockProvider();

      llmRegistry.register('mock', provider);
      llmRegistry.unregister('mock');

      expect(consoleSpy).toHaveBeenCalledWith('Provider mock unregistered');
    });
  });

  describe('clear', () => {
    it('should clear all providers and mappings', () => {
      const provider = new MockProvider();
      llmRegistry.register('mock', provider);
      llmRegistry.mapRoleToProvider('chat_assistant', 'mock');

      llmRegistry.clear();

      expect(llmRegistry.listProviders()).toEqual([]);
      expect(llmRegistry.listRoleMappings()).toEqual([]);
    });
  });
});
