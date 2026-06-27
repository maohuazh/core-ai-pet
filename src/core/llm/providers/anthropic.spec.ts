import { describe, it, expect, vi, beforeEach } from 'vitest';
import { AnthropicProvider } from '@/core/llm/providers/anthropic';
import type { LLMConfig, UnifiedDelta, LLMDeltaEvent, LLMDoneEvent } from '@/core/llm/types';

// Mock Tauri APIs
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn()
}));

describe('AnthropicProvider', () => {
  let provider: AnthropicProvider;
  let mockConfig: LLMConfig;

  beforeEach(() => {
    provider = new AnthropicProvider();
    mockConfig = {
      provider: 'anthropic',
      model: 'claude-3-5-sonnet-20241022',
      base_url: 'https://api.anthropic.com',
      secret_ref: 'test-secret-ref',
      role: 'chat_assistant',
      params: {
        temperature: 0.7,
        max_tokens: 4096
      }
    };

    vi.clearAllMocks();
  });

  describe('estimateCost', () => {
    it('should estimate cost based on token count', () => {
      const messages = [
        { role: 'user', content: 'Hello, how are you?' }
      ];

      const cost = provider.estimateCost(mockConfig, messages);

      expect(cost).toBeGreaterThan(0);
      expect(typeof cost).toBe('number');
    });

    it('should handle empty messages', () => {
      const cost = provider.estimateCost(mockConfig, []);

      expect(cost).toBeGreaterThan(0);
    });

    it('should handle multiple messages', () => {
      const messages = [
        { role: 'user', content: 'First message' },
        { role: 'assistant', content: 'Response' },
        { role: 'user', content: 'Another question' }
      ];

      const cost = provider.estimateCost(mockConfig, messages);

      expect(cost).toBeGreaterThan(0);
    });
  });

  describe('invoke', () => {
    it('should call Rust backend with correct parameters', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      const { listen } = await import('@tauri-apps/api/event');

      vi.mocked(invoke).mockResolvedValueOnce('mock-secret-key');
      vi.mocked(invoke).mockResolvedValueOnce('test-turn-id');
      vi.mocked(listen).mockResolvedValue(vi.fn());

      const request = {
        messages: [{ role: 'user' as const, content: 'Hello' }],
        system: 'You are a helpful assistant',
        stream: true
      };

      const onDelta = vi.fn();

      await provider.invoke(mockConfig, request, onDelta);

      expect(invoke).toHaveBeenCalledWith('llm_get_secret', {
        secretRef: 'test-secret-ref'
      });

      expect(invoke).toHaveBeenCalledWith('llm_invoke', {
        role: 'chat_assistant',
        request: {
          messages: request.messages,
          system: request.system,
          stream: request.stream
        }
      });

      expect(listen).toHaveBeenCalledTimes(2);
    });

    it('should handle delta events with matching turn_id', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      const { listen } = await import('@tauri-apps/api/event');

      vi.mocked(invoke).mockResolvedValueOnce('mock-secret-key');
      vi.mocked(invoke).mockResolvedValueOnce('test-turn-id');

      let deltaCallback: ((event: { payload: LLMDeltaEvent }) => void) | null = null;
      vi.mocked(listen).mockImplementation(async (event: string, callback: any) => {
        if (event === 'llm_delta') {
          deltaCallback = callback;
        }
        return vi.fn();
      });

      const request = {
        messages: [{ role: 'user' as const, content: 'Hello' }],
        stream: true
      };

      const onDelta = vi.fn();

      await provider.invoke(mockConfig, request, onDelta);

      // Simulate delta event
      if (deltaCallback) {
        deltaCallback({
          payload: {
            turn_id: 'test-turn-id',
            delta: {
              type: 'text',
              delta: 'Hello from Claude'
            }
          }
        });
      }

      expect(onDelta).toHaveBeenCalledWith({
        type: 'text',
        delta: 'Hello from Claude'
      });
    });

    it('should ignore delta events with different turn_id', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      const { listen } = await import('@tauri-apps/api/event');

      vi.mocked(invoke).mockResolvedValueOnce('mock-secret-key');
      vi.mocked(invoke).mockResolvedValueOnce('test-turn-id');

      let deltaCallback: ((event: { payload: LLMDeltaEvent }) => void) | null = null;
      vi.mocked(listen).mockImplementation(async (event: string, callback: any) => {
        if (event === 'llm_delta') {
          deltaCallback = callback;
        }
        return vi.fn();
      });

      const request = {
        messages: [{ role: 'user' as const, content: 'Hello' }],
        stream: true
      };

      const onDelta = vi.fn();

      await provider.invoke(mockConfig, request, onDelta);

      // Simulate delta event with different turn_id
      if (deltaCallback) {
        deltaCallback({
          payload: {
            turn_id: 'different-turn-id',
            delta: {
              type: 'text',
              delta: 'Should be ignored'
            }
          }
        });
      }

      expect(onDelta).not.toHaveBeenCalled();
    });

    it('should cleanup listeners on done event', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      const { listen } = await import('@tauri-apps/api/event');

      vi.mocked(invoke).mockResolvedValueOnce('mock-secret-key');
      vi.mocked(invoke).mockResolvedValueOnce('test-turn-id');

      const unlistenDelta = vi.fn();
      const unlistenDone = vi.fn();

      let deltaCallback: ((event: { payload: LLMDeltaEvent }) => void) | null = null;
      let doneCallback: ((event: { payload: LLMDoneEvent }) => void) | null = null;

      vi.mocked(listen).mockImplementation(async (event: string, callback: any) => {
        if (event === 'llm_delta') {
          deltaCallback = callback;
          return unlistenDelta;
        } else if (event === 'llm_done') {
          doneCallback = callback;
          return unlistenDone;
        }
        return vi.fn();
      });

      const request = {
        messages: [{ role: 'user' as const, content: 'Hello' }],
        stream: true
      };

      const onDelta = vi.fn();

      await provider.invoke(mockConfig, request, onDelta);

      // Simulate done event
      if (doneCallback) {
        doneCallback({
          payload: {
            turn_id: 'test-turn-id'
          }
        });
      }

      expect(unlistenDelta).toHaveBeenCalled();
      expect(unlistenDone).toHaveBeenCalled();
    });

    it('should handle all delta types', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      const { listen } = await import('@tauri-apps/api/event');

      vi.mocked(invoke).mockResolvedValueOnce('mock-secret-key');
      vi.mocked(invoke).mockResolvedValueOnce('test-turn-id');

      let deltaCallback: ((event: { payload: LLMDeltaEvent }) => void) | null = null;
      vi.mocked(listen).mockImplementation(async (event: string, callback: any) => {
        if (event === 'llm_delta') {
          deltaCallback = callback;
        }
        return vi.fn();
      });

      const request = {
        messages: [{ role: 'user' as const, content: 'Hello' }],
        stream: true
      };

      const onDelta = vi.fn();

      await provider.invoke(mockConfig, request, onDelta);

      // Test all delta types
      const deltas: UnifiedDelta[] = [
        { type: 'text', delta: 'Hello' },
        { type: 'thinking', delta: 'Processing...' },
        { type: 'tool_use_start', id: 'tool_1', name: 'search' },
        { type: 'tool_use_delta', id: 'tool_1', args_delta: '{"query":' },
        { type: 'tool_use_end', id: 'tool_1' },
        { type: 'usage', input_tokens: 100, output_tokens: 50 },
        { type: 'stop', reason: 'end_turn' },
        { type: 'error', recoverable: false, code: 'err', message: 'Error' }
      ];

      if (deltaCallback) {
        deltas.forEach(delta => {
          deltaCallback({
            payload: {
              turn_id: 'test-turn-id',
              delta
            }
          });
        });
      }

      expect(onDelta).toHaveBeenCalledTimes(8);
      deltas.forEach((delta, index) => {
        expect(onDelta).toHaveBeenNthCalledWith(index + 1, delta);
      });
    });
  });

  describe('ping', () => {
    it('should call llm_test_connection', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValueOnce(undefined);

      await provider.ping(mockConfig, 'test-api-key');

      expect(invoke).toHaveBeenCalledWith('llm_test_connection', {
        role: 'chat_assistant'
      });
    });
  });
});
