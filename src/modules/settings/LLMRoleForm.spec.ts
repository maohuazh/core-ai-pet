import { describe, it, expect, vi, beforeEach } from 'vitest';
import { mount } from '@vue/test-utils';
import LLMRoleForm from './LLMRoleForm.vue';
import type { LLMConfig } from '@/core/llm/types';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

describe('LLMRoleForm', () => {
  const mockConfig: LLMConfig = {
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

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders form with config values', () => {
    const wrapper = mount(LLMRoleForm, {
      props: {
        role: 'chat_assistant',
        config: mockConfig
      }
    });

    expect(wrapper.find('select').element.value).toBe('anthropic');
    expect(wrapper.find('input[type="text"]').element.value).toBe('claude-3-5-sonnet-20241022');
  });

  it('validates required fields', async () => {
    const wrapper = mount(LLMRoleForm, {
      props: {
        role: 'chat_assistant',
        config: null
      }
    });

    // Clear all fields
    await wrapper.find('input[type="text"]').setValue('');

    // Try to save
    await wrapper.findAll('button')[1].trigger('click');

    // Should show validation errors
    expect(wrapper.text()).toContain('不能为空');
  });

  it('validates URL format', async () => {
    const wrapper = mount(LLMRoleForm, {
      props: {
        role: 'chat_assistant',
        config: { ...mockConfig, base_url: '' }
      }
    });

    // Set invalid URL
    const inputs = wrapper.findAll('input[type="text"]');
    await inputs[1].setValue('not-a-url');

    // Try to save
    await wrapper.findAll('button')[1].trigger('click');

    // Should show URL validation error
    expect(wrapper.text()).toContain('有效的 URL');
  });

  it('emits save event with config on successful save', async () => {
    const wrapper = mount(LLMRoleForm, {
      props: {
        role: 'chat_assistant',
        config: mockConfig
      }
    });

    await wrapper.findAll('button')[1].trigger('click');

    expect(wrapper.emitted('save')).toBeTruthy();
    expect(wrapper.emitted('save')![0][0]).toMatchObject({
      provider: 'anthropic',
      model: 'claude-3-5-sonnet-20241022'
    });
  });

  it('calls test connection on button click', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValueOnce(undefined);

    const wrapper = mount(LLMRoleForm, {
      props: {
        role: 'chat_assistant',
        config: mockConfig
      }
    });

    await wrapper.findAll('button')[0].trigger('click');

    expect(invoke).toHaveBeenCalledWith('llm_test_connection', expect.any(Object));
  });

  it('shows success message on successful connection test', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValueOnce(undefined);

    const wrapper = mount(LLMRoleForm, {
      props: {
        role: 'chat_assistant',
        config: mockConfig
      }
    });

    await wrapper.findAll('button')[0].trigger('click');
    await vi.waitFor(() => {
      expect(wrapper.text()).toContain('连接成功');
    });
  });

  it('shows error message on failed connection test', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockRejectedValueOnce(new Error('Connection failed'));

    const wrapper = mount(LLMRoleForm, {
      props: {
        role: 'chat_assistant',
        config: mockConfig
      }
    });

    await wrapper.findAll('button')[0].trigger('click');
    await vi.waitFor(() => {
      expect(wrapper.text()).toContain('连接失败');
    });
  });
});
