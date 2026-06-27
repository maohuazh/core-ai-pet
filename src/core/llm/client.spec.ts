import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke, RoleNotFoundError, ConfigNotFoundError, SecretNotFoundError, llm } from './client';
import { llmRegistry } from './registry';
import type { ILLMProvider, LLMConfig, UnifiedRequest, UnifiedDelta } from './types';

// Mock provider
class MockProvider implements ILLMProvider {
  invoke = vi.fn(async (config: LLMConfig, request: UnifiedRequest, onDelta: (delta: UnifiedDelta) => void) => {
    // Add a small delay to allow abort signal to be checked
    await new Promise(resolve => setTimeout(resolve, 10));
    onDelta({ type: 'text', delta: 'Hello' });
    onDelta({ type: 'stop', reason: 'end_turn' });
  });

  ping = vi.fn(async () => {});
  estimateCost = vi.fn(() => 0.01);
}

describe('client', () => {
  let mockProvider: MockProvider;

  beforeEach(() => {
    llmRegistry.clear();
    mockProvider = new MockProvider();
  });

  describe('invoke', () => {
    it('should throw RoleNotFoundError when no provider is mapped to role', async () => {
      const request: UnifiedRequest = {
        messages: [{ role: 'user', content: 'Hello' }],
        stream: true
      };

      await expect(
        invoke('chat_assistant', request, vi.fn())
      ).rejects.toThrow(RoleNotFoundError);
    });

    it('should remove role mappings when provider is unregistered', async () => {
      // Register provider, map role, then unregister provider
      llmRegistry.register('anthropic', mockProvider);
      llmRegistry.mapRoleToProvider('chat_assistant', 'anthropic');
      llmRegistry.unregister('anthropic');

      const request: UnifiedRequest = {
        messages: [{ role: 'user', content: 'Hello' }],
        stream: true
      };

      // After unregistering provider, role mapping is also removed
      // So it should throw RoleNotFoundError, not ConfigNotFoundError
      await expect(
        invoke('chat_assistant', request, vi.fn())
      ).rejects.toThrow(RoleNotFoundError);
    });

    it('should invoke provider with correct parameters', async () => {
      llmRegistry.register('anthropic', mockProvider);
      llmRegistry.mapRoleToProvider('chat_assistant', 'anthropic');

      const request: UnifiedRequest = {
        messages: [{ role: 'user', content: 'Hello' }],
        stream: true
      };

      const onDelta = vi.fn();
      await invoke('chat_assistant', request, onDelta);

      expect(mockProvider.invoke).toHaveBeenCalledTimes(1);
      expect(onDelta).toHaveBeenCalledTimes(2);
      expect(onDelta).toHaveBeenCalledWith({ type: 'text', delta: 'Hello' });
      expect(onDelta).toHaveBeenCalledWith({ type: 'stop', reason: 'end_turn' });
    });

    it('should handle abort signal', async () => {
      llmRegistry.register('anthropic', mockProvider);
      llmRegistry.mapRoleToProvider('chat_assistant', 'anthropic');

      const request: UnifiedRequest = {
        messages: [{ role: 'user', content: 'Hello' }],
        stream: true
      };

      const abortController = new AbortController();
      abortController.abort();

      await expect(
        invoke('chat_assistant', request, vi.fn(), { abort: abortController.signal })
      ).rejects.toThrow('Invocation aborted');
    });
  });

  describe('llm namespace', () => {
    it('should export invoke function', () => {
      expect(llm.invoke).toBe(invoke);
    });
  });

  describe('error types', () => {
    it('should create RoleNotFoundError with correct message', () => {
      const error = new RoleNotFoundError('chat_assistant');
      expect(error.message).toBe('No provider registered for role: chat_assistant');
      expect(error.name).toBe('RoleNotFoundError');
    });

    it('should create ConfigNotFoundError with correct message', () => {
      const error = new ConfigNotFoundError('chat_assistant');
      expect(error.message).toBe('Configuration not found for role: chat_assistant');
      expect(error.name).toBe('ConfigNotFoundError');
    });

    it('should create SecretNotFoundError with correct message', () => {
      const error = new SecretNotFoundError('chat_assistant');
      expect(error.message).toBe('Secret not found for role: chat_assistant');
      expect(error.name).toBe('SecretNotFoundError');
    });
  });
});
