import { describe, it, expect, vi, beforeEach } from 'vitest';
import { triggerHandler } from './triggerHandler';
import { actionMappingService } from '../action/actionMappingService';
import { petStore } from '../model/PetStore';

vi.mock('../action/actionMappingService', () => ({
  actionMappingService: {
    loadMappings: vi.fn(),
    recordToFormData: vi.fn()
  }
}));

vi.mock('../model/PetStore', () => ({
  petStore: {
    currentModel: {
      value: { id: 'test-model-id' }
    }
  }
}));

vi.mock('../action/triggerExecutor', () => ({
  executeTriggerAction: vi.fn()
}));

describe('triggerHandler', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('event mapping', () => {
    it('should map llm.message to llm.message trigger key', () => {
      const eventMapping = (triggerHandler as any).eventMapping;
      expect(eventMapping['llm.message']).toBe('llm.message');
    });

    it('should map llm.invoke to llm.invoke trigger key', () => {
      const eventMapping = (triggerHandler as any).eventMapping;
      expect(eventMapping['llm.invoke']).toBe('llm.invoke');
    });
  });

  describe('fireTrigger', () => {
    it('should handle llm.message trigger without executing pet actions', async () => {
      const handleLlmTriggerSpy = vi.spyOn(triggerHandler as any, 'handleLLmTrigger');

      await triggerHandler.fireTrigger('llm.message');

      expect(handleLlmTriggerSpy).toHaveBeenCalledWith('llm.message');
    });

    it('should handle llm.invoke trigger without executing pet actions', async () => {
      const handleLlmTriggerSpy = vi.spyOn(triggerHandler as any, 'handleLLmTrigger');

      await triggerHandler.fireTrigger('llm.invoke');

      expect(handleLlmTriggerSpy).toHaveBeenCalledWith('llm.invoke');
    });

    it('should not load action mappings for llm triggers', async () => {
      await triggerHandler.fireTrigger('llm.message');

      expect(actionMappingService.loadMappings).not.toHaveBeenCalled();
    });
  });
});
