import { describe, it, expect } from 'vitest';
import type {
  UnifiedDelta,
  TextDelta,
  ThinkingDelta,
  ToolUseStartDelta,
  ToolUseDeltaDelta,
  ToolUseEndDelta,
  UsageDelta,
  StopDelta,
  ErrorDelta
} from '@/core/llm/types';

describe('UnifiedDelta Type System', () => {
  describe('TextDelta', () => {
    it('should create valid text delta', () => {
      const delta: TextDelta = {
        type: 'text',
        delta: 'Hello, world!'
      };

      expect(delta.type).toBe('text');
      expect(delta.delta).toBe('Hello, world!');
    });

    it('should accept empty string', () => {
      const delta: TextDelta = {
        type: 'text',
        delta: ''
      };

      expect(delta.delta).toBe('');
    });

    it('should accept multiline text', () => {
      const delta: TextDelta = {
        type: 'text',
        delta: 'Line 1\nLine 2\nLine 3'
      };

      expect(delta.delta).toContain('\n');
    });
  });

  describe('ThinkingDelta', () => {
    it('should create valid thinking delta', () => {
      const delta: ThinkingDelta = {
        type: 'thinking',
        delta: 'Analyzing the request...'
      };

      expect(delta.type).toBe('thinking');
      expect(delta.delta).toBe('Analyzing the request...');
    });
  });

  describe('ToolUseStartDelta', () => {
    it('should create valid tool use start delta', () => {
      const delta: ToolUseStartDelta = {
        type: 'tool_use_start',
        id: 'tool_123',
        name: 'web_search'
      };

      expect(delta.type).toBe('tool_use_start');
      expect(delta.id).toBe('tool_123');
      expect(delta.name).toBe('web_search');
    });
  });

  describe('ToolUseDeltaDelta', () => {
    it('should create valid tool use delta', () => {
      const delta: ToolUseDeltaDelta = {
        type: 'tool_use_delta',
        id: 'tool_123',
        args_delta: '{"query": "test"}'
      };

      expect(delta.type).toBe('tool_use_delta');
      expect(delta.id).toBe('tool_123');
      expect(delta.args_delta).toContain('query');
    });
  });

  describe('ToolUseEndDelta', () => {
    it('should create valid tool use end delta', () => {
      const delta: ToolUseEndDelta = {
        type: 'tool_use_end',
        id: 'tool_123'
      };

      expect(delta.type).toBe('tool_use_end');
      expect(delta.id).toBe('tool_123');
    });
  });

  describe('UsageDelta', () => {
    it('should create valid usage delta with all fields', () => {
      const delta: UsageDelta = {
        type: 'usage',
        input_tokens: 100,
        output_tokens: 50,
        cached: 20
      };

      expect(delta.type).toBe('usage');
      expect(delta.input_tokens).toBe(100);
      expect(delta.output_tokens).toBe(50);
      expect(delta.cached).toBe(20);
    });

    it('should create valid usage delta with optional fields', () => {
      const delta: UsageDelta = {
        type: 'usage',
        input_tokens: 100
      };

      expect(delta.output_tokens).toBeUndefined();
      expect(delta.cached).toBeUndefined();
    });
  });

  describe('StopDelta', () => {
    it('should create valid stop delta', () => {
      const delta: StopDelta = {
        type: 'stop',
        reason: 'end_turn'
      };

      expect(delta.type).toBe('stop');
      expect(delta.reason).toBe('end_turn');
    });

    it('should accept different stop reasons', () => {
      const reasons = ['end_turn', 'max_tokens', 'stop_sequence'];

      reasons.forEach(reason => {
        const delta: StopDelta = {
          type: 'stop',
          reason
        };
        expect(delta.reason).toBe(reason);
      });
    });
  });

  describe('ErrorDelta', () => {
    it('should create valid error delta', () => {
      const delta: ErrorDelta = {
        type: 'error',
        recoverable: true,
        code: 'rate_limit',
        message: 'Rate limit exceeded'
      };

      expect(delta.type).toBe('error');
      expect(delta.recoverable).toBe(true);
      expect(delta.code).toBe('rate_limit');
      expect(delta.message).toBe('Rate limit exceeded');
    });

    it('should handle non-recoverable errors', () => {
      const delta: ErrorDelta = {
        type: 'error',
        recoverable: false,
        code: 'invalid_api_key',
        message: 'Invalid API key provided'
      };

      expect(delta.recoverable).toBe(false);
    });
  });

  describe('UnifiedDelta Union Type', () => {
    it('should accept any valid delta type', () => {
      const deltas: UnifiedDelta[] = [
        { type: 'text', delta: 'Hello' },
        { type: 'thinking', delta: 'Thinking...' },
        { type: 'tool_use_start', id: '1', name: 'test' },
        { type: 'tool_use_delta', id: '1', args_delta: '{}' },
        { type: 'tool_use_end', id: '1' },
        { type: 'usage', input_tokens: 10 },
        { type: 'stop', reason: 'end_turn' },
        { type: 'error', recoverable: false, code: 'err', message: 'Error' }
      ];

      expect(deltas).toHaveLength(8);
      deltas.forEach(delta => {
        expect(delta).toHaveProperty('type');
      });
    });

    it('should allow type narrowing via discriminated union', () => {
      const delta: UnifiedDelta = {
        type: 'text',
        delta: 'Hello'
      };

      if (delta.type === 'text') {
        // TypeScript should narrow the type here
        expect(delta.delta).toBe('Hello');
      }
    });

    it('should handle all 8 delta types', () => {
      const types = [
        'text',
        'thinking',
        'tool_use_start',
        'tool_use_delta',
        'tool_use_end',
        'usage',
        'stop',
        'error'
      ];

      types.forEach(type => {
        let delta: UnifiedDelta;

        switch (type) {
          case 'text':
            delta = { type: 'text', delta: 'test' };
            break;
          case 'thinking':
            delta = { type: 'thinking', delta: 'test' };
            break;
          case 'tool_use_start':
            delta = { type: 'tool_use_start', id: '1', name: 'test' };
            break;
          case 'tool_use_delta':
            delta = { type: 'tool_use_delta', id: '1', args_delta: '{}' };
            break;
          case 'tool_use_end':
            delta = { type: 'tool_use_end', id: '1' };
            break;
          case 'usage':
            delta = { type: 'usage' };
            break;
          case 'stop':
            delta = { type: 'stop', reason: 'test' };
            break;
          case 'error':
            delta = { type: 'error', recoverable: false, code: 'test', message: 'test' };
            break;
        }

        expect(delta.type).toBe(type);
      });
    });
  });
});
