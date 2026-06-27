import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type {
  ILLMProvider,
  LLMConfig,
  UnifiedRequest,
  UnifiedDelta,
  LLMDeltaEvent,
  LLMDoneEvent
} from '../types';
import type { LLMRole } from '../role';

/**
 * Anthropic Provider Adapter
 *
 * Bridges frontend ILLMProvider interface with Rust backend implementation
 */
export class AnthropicProvider implements ILLMProvider {
  /**
   * Invoke Anthropic API via Rust backend
   *
   * @param config - LLM configuration
   * @param request - Unified request format
   * @param onDelta - Callback for streaming deltas
   */
  async invoke(
    config: LLMConfig,
    request: UnifiedRequest,
    onDelta: (delta: UnifiedDelta) => void
  ): Promise<void> {
    // Get the API key from Rust backend
    const apiKey = await this.getApiKey(config.secret_ref);

    // Start the streaming invocation
    const turnId = await invoke<string>('llm_invoke', {
      role: config.role,
      request: {
        messages: request.messages,
        system: request.system,
        stream: request.stream
      }
    });

    // Listen for delta events
    const unlistenDelta = await listen<LLMDeltaEvent>('llm_delta', (event) => {
      if (event.payload.turn_id === turnId) {
        onDelta(event.payload.delta);
      }
    });

    // Listen for done event
    const unlistenDone = await listen<LLMDoneEvent>('llm_done', (event) => {
      if (event.payload.turn_id === turnId) {
        unlistenDelta();
        unlistenDone();
      }
    });
  }

  /**
   * Test connection to Anthropic API
   *
   * @param config - LLM configuration
   * @param apiKey - API key to test
   */
  async ping(config: LLMConfig, apiKey: string): Promise<void> {
    await invoke('llm_test_connection', {
      role: config.role
    });
  }

  /**
   * Estimate cost for a request
   *
   * @param config - LLM configuration
   * @param messages - Messages to estimate cost for
   * @returns Estimated cost in USD
   */
  estimateCost(config: LLMConfig, messages: Array<{ role: string; content: string }>): number {
    // Simple cost estimation based on token count
    // This is a rough estimate - actual implementation would use tiktoken
    const inputTokens = messages.reduce((acc, msg) => acc + msg.content.length / 4, 0);
    const outputTokens = config.params.max_tokens;

    // Claude pricing (approximate):
    // Input: $0.003 / 1K tokens
    // Output: $0.015 / 1K tokens
    const inputCost = (inputTokens / 1000) * 0.003;
    const outputCost = (outputTokens / 1000) * 0.015;

    return inputCost + outputCost;
  }

  /**
   * Get API key from Rust backend
   *
   * @param secretRef - Secret reference
   * @returns API key
   */
  private async getApiKey(secretRef: string): Promise<string> {
    const result = await invoke<{ plaintext: string }>('llm_get_secret', {
      secretRef
    });
    return result.plaintext;
  }
}

/**
 * Singleton instance of Anthropic provider
 */
export const anthropicProvider = new AnthropicProvider();
