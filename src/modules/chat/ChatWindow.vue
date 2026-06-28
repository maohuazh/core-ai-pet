<template>
  <div class="chat-window">
    <!-- Left: Session List -->
    <aside class="session-sidebar">
      <div class="session-header">
        <span class="session-title">会话</span>
        <button class="new-chat-btn" @click="createNewSession" title="新对话">+</button>
      </div>
      <div class="session-list">
        <div
          v-for="session in sessions"
          :key="session.id"
          class="session-item"
          :class="{ active: currentSessionId === session.id }"
          @click="selectSession(session.id)"
        >
          <div class="session-info">
            <span class="session-name">{{ session.title }}</span>
            <span class="session-workspace">{{ session.workspace || 'default' }}</span>
          </div>
          <button class="session-delete" @click.stop="deleteSession(session.id)" title="删除">×</button>
        </div>
        <div v-if="sessions.length === 0" class="session-empty">暂无对话</div>
      </div>
    </aside>

    <!-- Center: Messages -->
    <main class="message-area">
      <div class="message-header">
        <span class="message-header-title">{{ currentSessionTitle }}</span>
      </div>

      <div ref="messagesContainer" class="messages-scroll">
        <div v-if="messages.length === 0 && !isLoading" class="empty-state">
          <div class="empty-icon">💬</div>
          <div class="empty-text">开始对话吧！</div>
          <div class="empty-hint">输入消息并按 Enter 发送</div>
        </div>

        <div v-for="(msg, index) in messages" :key="index" class="message-row" :class="msg.role">
          <!-- User message: right side -->
          <div v-if="msg.role === 'user'" class="message-bubble user-bubble">
            <div class="bubble-content">{{ msg.content }}</div>
          </div>

          <!-- Assistant message: left side -->
          <div v-else class="assistant-block">
            <!-- Thinking block -->
            <div v-if="msg.thinking" class="thinking-block">
              <div class="thinking-header" @click="msg.thinkingExpanded = !msg.thinkingExpanded">
                <span class="thinking-toggle">{{ msg.thinkingExpanded ? '▼' : '▶' }}</span>
                <span class="thinking-label">💭 思考过程</span>
              </div>
              <div v-show="msg.thinkingExpanded" class="thinking-content">{{ msg.thinking }}</div>
            </div>

            <!-- Tool call block -->
            <div v-if="msg.toolCalls && msg.toolCalls.length > 0" class="tool-block">
              <div v-for="tool in msg.toolCalls" :key="tool.id" class="tool-call">
                <div class="tool-header">🔧 {{ tool.name }}</div>
                <pre class="tool-args">{{ tool.args }}</pre>
              </div>
            </div>

            <!-- Response text -->
            <div v-if="msg.content" class="message-bubble assistant-bubble">
              <div class="bubble-content">{{ msg.content }}</div>
            </div>
          </div>
        </div>

        <!-- Loading indicator -->
        <div v-if="isLoading" class="message-row assistant">
          <div class="assistant-block">
            <div v-if="streamingThinking" class="thinking-block">
              <div class="thinking-header" @click="streamingThinkingExpanded = !streamingThinkingExpanded">
                <span class="thinking-toggle">{{ streamingThinkingExpanded ? '▼' : '▶' }}</span>
                <span class="thinking-label">💭 思考中...</span>
              </div>
              <div v-show="streamingThinkingExpanded" class="thinking-content">{{ streamingThinking }}</div>
            </div>
            <div v-if="!streamingThinking && !streamingContent" class="loading-indicator">
              <span class="loading-dot"></span>
              <span class="loading-dot"></span>
              <span class="loading-dot"></span>
            </div>
            <div v-if="streamingContent" class="message-bubble assistant-bubble">
              <div class="bubble-content">{{ streamingContent }}<span class="cursor-blink">▊</span></div>
            </div>
          </div>
        </div>
      </div>

      <!-- Input area -->
      <div class="input-area">
        <textarea
          ref="inputRef"
          v-model="inputMessage"
          placeholder="输入消息... (Enter 发送, Shift+Enter 换行)"
          @keydown.enter.exact.prevent="sendMessage"
          class="message-input"
          rows="2"
        ></textarea>
        <button
          @click="sendMessage"
          :disabled="isLoading || !inputMessage.trim() || !currentSessionId"
          class="send-btn"
        >
          发送
        </button>
      </div>
    </main>

    <!-- Bottom bar: Workspace selector + git branch -->
    <footer class="bottom-bar">
      <label class="workspace-label">工作区:</label>
      <select v-model="currentWorkspace" class="workspace-select" @change="onWorkspaceChange">
        <option value="">默认</option>
        <option v-for="ws in workspaces" :key="ws" :value="ws">{{ formatWorkspaceLabel(ws) }}</option>
        <option value="__add_dir__">📁 添加目录...</option>
      </select>
      <span v-if="gitBranch" class="git-branch" :title="currentWorkspace || '当前项目'">
        <svg class="git-icon" viewBox="0 0 16 16" width="12" height="12" fill="currentColor">
          <path d="M11.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122V6A2.5 2.5 0 0110 8.5H6a1 1 0 00-1 1v1.128a2.251 2.251 0 11-1.5 0V5.372a2.25 2.25 0 111.5 0v1.836A2.492 2.492 0 016 7h4a1 1 0 001-1v-.628A2.25 2.25 0 019.5 3.25zM4.25 12a.75.75 0 100 1.5.75.75 0 000-1.5zM3.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0z"/>
        </svg>
        {{ gitBranch }}
      </span>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, computed, watch } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

// ---- Types ----
interface Session {
  id: string;
  title: string;
  workspace: string | null;
  created_at: number;
  updated_at: number;
}

interface ToolCall {
  id: string;
  name: string;
  args: string;
}

interface DisplayMessage {
  role: 'user' | 'assistant';
  content: string;
  thinking?: string;
  thinkingExpanded?: boolean;
  toolCalls?: ToolCall[];
}

interface DeltaEvent {
  turn_id: string;
  delta: {
    type: string;
    delta?: string;
    message?: string;
    id?: string;
    name?: string;
    args_delta?: string;
    [key: string]: any;
  };
}

interface DoneEvent {
  turn_id: string;
}

// ---- State ----
const sessions = ref<Session[]>([]);
const currentSessionId = ref<string | null>(null);
const messages = ref<DisplayMessage[]>([]);
const inputMessage = ref('');
const isLoading = ref(false);
const currentTurnId = ref<string | null>(null);
const messagesContainer = ref<HTMLElement | null>(null);
const inputRef = ref<HTMLTextAreaElement | null>(null);

// Streaming state (shown while loading)
const streamingContent = ref('');
const streamingThinking = ref('');
const streamingThinkingExpanded = ref(false);
const streamingToolCalls = ref<ToolCall[]>([]);

// Workspace
const currentWorkspace = ref('');
const workspaces = ref<string[]>([]);
const gitBranch = ref<string | null>(null);

let unlistenStream: UnlistenFn | null = null;
let unlistenDone: UnlistenFn | null = null;

const currentSessionTitle = computed(() => {
  const s = sessions.value.find(s => s.id === currentSessionId.value);
  return s?.title ?? '新对话';
});

// ---- Lifecycle ----
onMounted(async () => {
  inputRef.value?.focus();
  await loadSessions();

  // Auto-create first session if none exist
  if (sessions.value.length === 0) {
    await createNewSession();
  } else {
    await selectSession(sessions.value[0].id);
  }

  // Load workspaces from app_settings
  await loadWorkspaces();

  // Listen for LLM streaming events
  unlistenStream = await listen<DeltaEvent>('llm_delta', (event) => {
    const { turn_id, delta } = event.payload;
    if (turn_id !== currentTurnId.value) return;

    switch (delta.type) {
      case 'text':
        if (delta.delta) streamingContent.value += delta.delta;
        scrollToBottom();
        break;
      case 'thinking':
        if (delta.delta) streamingThinking.value += delta.delta;
        break;
      case 'tool_use_start':
        if (delta.id && delta.name) {
          streamingToolCalls.value.push({ id: delta.id, name: delta.name, args: '' });
        }
        break;
      case 'tool_use_delta':
        if (delta.args_delta) {
          const last = streamingToolCalls.value[streamingToolCalls.value.length - 1];
          if (last) last.args += delta.args_delta;
        }
        break;
      case 'error':
        const errCode = delta.code ? `[${delta.code}] ` : '';
        const errMsg = delta.message?.trim() || '未知错误';
        streamingContent.value += `\n[错误: ${errCode}${errMsg}]`;
        break;
    }
  });

  unlistenDone = await listen<DoneEvent>('llm_done', async (event) => {
    const { turn_id } = event.payload;
    if (turn_id !== currentTurnId.value) return;

    // Finalize: move streaming state into a display message
    const assistantMsg: DisplayMessage = {
      role: 'assistant',
      content: streamingContent.value,
      thinking: streamingThinking.value || undefined,
      thinkingExpanded: false,
      toolCalls: streamingToolCalls.value.length > 0 ? [...streamingToolCalls.value] : undefined,
    };
    messages.value.push(assistantMsg);

    // Persist assistant message to DB
    if (currentSessionId.value && streamingContent.value) {
      await invoke('chat_store_message', {
        sessionId: currentSessionId.value,
        turnId: currentTurnId.value,
        role: 'assistant',
        content: streamingContent.value,
        messageType: 'text',
      }).catch(e => console.error('Failed to store assistant message:', e));
    }

    // Reset streaming state
    streamingContent.value = '';
    streamingThinking.value = '';
    streamingToolCalls.value = [];
    isLoading.value = false;
    currentTurnId.value = null;
    scrollToBottom();
  });
});

onUnmounted(() => {
  unlistenStream?.();
  unlistenDone?.();
});

// ---- Session Management ----
async function loadSessions() {
  try {
    sessions.value = await invoke<Session[]>('chat_list_sessions');
  } catch (e) {
    console.error('Failed to load sessions:', e);
  }
}

async function createNewSession() {
  try {
    const session = await invoke<Session>('chat_create_session', {
      title: null,
      workspace: currentWorkspace.value || null,
    });
    sessions.value.unshift(session);
    await selectSession(session.id);
  } catch (e) {
    console.error('Failed to create session:', e);
  }
}

async function selectSession(sessionId: string) {
  currentSessionId.value = sessionId;
  // Sync workspace selector with session's workspace
  const session = sessions.value.find(s => s.id === sessionId);
  currentWorkspace.value = session?.workspace || '';
  await loadGitBranch();
  try {
    const stored = await invoke<any[]>('chat_get_messages', { sessionId });
    messages.value = stored.map(m => ({
      role: m.role as 'user' | 'assistant',
      content: m.content,
      thinkingExpanded: false,
    }));
  } catch (e) {
    console.error('Failed to load messages:', e);
    messages.value = [];
  }
  scrollToBottom();
}

async function deleteSession(sessionId: string) {
  try {
    await invoke('chat_delete_session', { sessionId });
    sessions.value = sessions.value.filter(s => s.id !== sessionId);
    if (currentSessionId.value === sessionId) {
      if (sessions.value.length > 0) {
        await selectSession(sessions.value[0].id);
      } else {
        currentSessionId.value = null;
        messages.value = [];
        await createNewSession();
      }
    }
  } catch (e) {
    console.error('Failed to delete session:', e);
  }
}

// ---- Send Message ----
async function sendMessage() {
  const content = inputMessage.value.trim();
  if (!content || isLoading.value || !currentSessionId.value) return;

  // Add user message to display
  messages.value.push({ role: 'user', content });
  inputMessage.value = '';
  isLoading.value = true;

  // Update session title from first message
  const session = sessions.value.find(s => s.id === currentSessionId.value);
  if (session && session.title === '新对话') {
    session.title = content.slice(0, 30) + (content.length > 30 ? '...' : '');
  }

  // Persist user message
  await invoke('chat_store_message', {
    sessionId: currentSessionId.value,
    turnId: null,
    role: 'user',
    content,
    messageType: 'text',
  }).catch(e => console.error('Failed to store user message:', e));

  scrollToBottom();

  try {
    // Build message history for context
    const history = messages.value
      .filter(m => m.role === 'user' || (m.role === 'assistant' && m.content))
      .map(m => ({ role: m.role, content: m.content }));

    // Invoke LLM and use backend-generated turn_id so streaming events match
    const result = await invoke<{ turn_id: string }>('llm_invoke', {
      role: 'chat_assistant',
      request: {
        messages: history,
        stream: true,
      },
    });
    currentTurnId.value = result.turn_id;
  } catch (error: any) {
    console.error('Failed to invoke LLM:', error);
    const errMsg = typeof error === 'string' ? error : (error?.message || String(error));
    messages.value.push({
      role: 'assistant',
      content: `[调用失败: ${errMsg}]\n\n请检查 AI 模型配置是否已保存。`,
    });
    isLoading.value = false;
    currentTurnId.value = null;
  }
}

// ---- Workspace ----
async function loadWorkspaces() {
  try {
    const stored = await invoke<string | null>('storage_get', { key: 'workspaces' });
    if (stored) {
      workspaces.value = JSON.parse(stored);
    }
    // Restore last workspace selection
    const savedWs = await invoke<string | null>('storage_get', { key: 'current_workspace' });
    if (savedWs) {
      currentWorkspace.value = savedWs;
    }
  } catch {
    workspaces.value = [];
  }
  await loadGitBranch();
}

async function onWorkspaceChange() {
  // Handle "添加目录..." option
  if (currentWorkspace.value === '__add_dir__') {
    currentWorkspace.value = '';
    await addWorkspace();
    return;
  }
  // Persist workspace preference
  invoke('storage_set', { key: 'current_workspace', value: currentWorkspace.value }).catch(() => {});
  // Update session in DB
  if (currentSessionId.value) {
    await invoke('chat_update_session', {
      sessionId: currentSessionId.value,
      title: null,
      workspace: currentWorkspace.value || null,
    }).catch(e => console.error('Failed to update session workspace:', e));
    // Update sidebar display
    const session = sessions.value.find(s => s.id === currentSessionId.value);
    if (session) session.workspace = currentWorkspace.value || null;
  }
  // Load git branch for new workspace
  await loadGitBranch();
}

async function addWorkspace() {
  try {
    const selected = await open({ directory: true, multiple: false, title: '选择工作目录' });
    if (selected && typeof selected === 'string') {
      if (!workspaces.value.includes(selected)) {
        workspaces.value.push(selected);
        await invoke('storage_set', { key: 'workspaces', value: JSON.stringify(workspaces.value) });
      }
      currentWorkspace.value = selected;
      await onWorkspaceChange();
    }
  } catch (e) {
    console.error('Failed to pick workspace folder:', e);
  }
}

async function loadGitBranch() {
  // Use currentWorkspace if set, otherwise default to app's CWD (project root)
  const wsPath = currentWorkspace.value || '.';
  try {
    gitBranch.value = await invoke<string | null>('get_git_branch', { workspace: wsPath });
  } catch {
    gitBranch.value = null;
  }
}

function formatWorkspaceLabel(ws: string): string {
  // Show only the last path component for readability
  const parts = ws.replace(/[\\/]+$/, '').split(/[\\/]/);
  return parts[parts.length - 1] || ws;
}

// ---- Helpers ----
function scrollToBottom() {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
    }
  });
}

watch(messages, () => { scrollToBottom(); }, { deep: true });
</script>

<style scoped>
.chat-window {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: #1e1e2e;
  color: #cdd6f4;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

/* ---- Layout ---- */
.chat-window {
  display: grid;
  grid-template-columns: 220px 1fr;
  grid-template-rows: 1fr auto;
  grid-template-areas:
    "sidebar main"
    "sidebar bottom";
}

/* ---- Session Sidebar ---- */
.session-sidebar {
  grid-area: sidebar;
  background: #181825;
  border-right: 1px solid #313244;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.session-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 14px;
  border-bottom: 1px solid #313244;
}

.session-title {
  font-size: 13px;
  font-weight: 600;
  color: #a6adc8;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.new-chat-btn {
  background: #45475a;
  border: none;
  color: #cdd6f4;
  width: 26px;
  height: 26px;
  border-radius: 6px;
  font-size: 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.15s;
}
.new-chat-btn:hover { background: #585b70; }

.session-list {
  flex: 1;
  overflow-y: auto;
  padding: 6px;
}

.session-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 10px;
  border-radius: 6px;
  cursor: pointer;
  margin-bottom: 2px;
  transition: background 0.15s;
}
.session-item:hover { background: #313244; }
.session-item.active { background: #45475a; }

.session-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
  min-width: 0;
}

.session-name {
  font-size: 13px;
  color: #cdd6f4;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.session-workspace {
  font-size: 10px;
  color: #6c7086;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.session-delete {
  background: none;
  border: none;
  color: #6c7086;
  font-size: 14px;
  cursor: pointer;
  padding: 0 4px;
  flex-shrink: 0;
  transition: color 0.15s;
}
.session-delete:hover { color: #f38ba8; }

.session-empty {
  text-align: center;
  color: #6c7086;
  font-size: 12px;
  padding: 20px 10px;
}

/* ---- Message Area ---- */
.message-area {
  grid-area: main;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.message-header {
  padding: 10px 16px;
  border-bottom: 1px solid #313244;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.message-header-title {
  font-size: 14px;
  font-weight: 600;
  color: #cdd6f4;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.git-branch {
  font-size: 11px;
  color: #a6e3a1;
  background: #1e1e2e;
  border: 1px solid #45475a;
  border-radius: 10px;
  padding: 2px 8px;
  white-space: nowrap;
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  gap: 4px;
}
.git-icon {
  flex-shrink: 0;
}

.messages-scroll {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  gap: 8px;
  flex: 1;
}
.empty-icon { font-size: 48px; opacity: 0.3; }
.empty-text { font-size: 16px; color: #a6adc8; }
.empty-hint { font-size: 12px; color: #6c7086; }

/* ---- Message Rows ---- */
.message-row {
  display: flex;
}
.message-row.user {
  justify-content: flex-end;
}
.message-row.assistant {
  justify-content: flex-start;
}

/* ---- User Bubble ---- */
.user-bubble {
  max-width: 70%;
  background: #89b4fa;
  color: #1e1e2e;
  border-radius: 14px 14px 4px 14px;
  padding: 8px 14px;
}
.user-bubble .bubble-content {
  font-size: 13px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-word;
}

/* ---- Assistant Block ---- */
.assistant-block {
  max-width: 80%;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.assistant-bubble {
  background: #313244;
  color: #cdd6f4;
  border-radius: 14px 14px 14px 4px;
  padding: 8px 14px;
}
.assistant-bubble .bubble-content {
  font-size: 13px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-word;
}

.cursor-blink {
  animation: blink 1s step-end infinite;
  color: #89b4fa;
}
@keyframes blink {
  50% { opacity: 0; }
}

/* ---- Thinking Block ---- */
.thinking-block {
  background: #1e1e2e;
  border: 1px solid #45475a;
  border-radius: 8px;
  overflow: hidden;
}
.thinking-header {
  padding: 6px 10px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: #a6adc8;
  user-select: none;
}
.thinking-header:hover { background: #313244; }
.thinking-toggle { font-size: 10px; }
.thinking-label { font-weight: 500; }
.thinking-content {
  padding: 8px 10px;
  font-size: 12px;
  color: #7f849c;
  line-height: 1.5;
  white-space: pre-wrap;
  border-top: 1px solid #45475a;
  max-height: 200px;
  overflow-y: auto;
}

/* ---- Tool Block ---- */
.tool-block {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.tool-call {
  background: #1e1e2e;
  border: 1px solid #45475a;
  border-radius: 8px;
  overflow: hidden;
}
.tool-header {
  padding: 6px 10px;
  font-size: 12px;
  font-weight: 500;
  color: #a6e3a1;
  background: #1e1e2e;
}
.tool-args {
  padding: 6px 10px;
  font-size: 11px;
  color: #7f849c;
  font-family: 'Cascadia Code', 'Fira Code', monospace;
  white-space: pre-wrap;
  word-break: break-all;
  max-height: 120px;
  overflow-y: auto;
  margin: 0;
  border-top: 1px solid #45475a;
}

/* ---- Loading ---- */
.loading-indicator {
  display: flex;
  gap: 4px;
  padding: 8px 12px;
}
.loading-dot {
  width: 6px;
  height: 6px;
  background: #89b4fa;
  border-radius: 50%;
  animation: dotBounce 1.4s infinite ease-in-out both;
}
.loading-dot:nth-child(1) { animation-delay: -0.32s; }
.loading-dot:nth-child(2) { animation-delay: -0.16s; }
@keyframes dotBounce {
  0%, 80%, 100% { transform: scale(0); }
  40% { transform: scale(1); }
}

/* ---- Input Area ---- */
.input-area {
  display: flex;
  gap: 8px;
  padding: 10px 14px;
  border-top: 1px solid #313244;
  background: #181825;
  flex-shrink: 0;
}
.message-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid #45475a;
  border-radius: 8px;
  font-size: 13px;
  font-family: inherit;
  resize: none;
  background: #1e1e2e;
  color: #cdd6f4;
}
.message-input:focus {
  outline: none;
  border-color: #89b4fa;
}
.message-input::placeholder { color: #6c7086; }

.send-btn {
  padding: 8px 18px;
  background: #89b4fa;
  color: #1e1e2e;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.15s;
  align-self: flex-end;
}
.send-btn:hover:not(:disabled) { background: #74c7ec; }
.send-btn:disabled { opacity: 0.4; cursor: not-allowed; }

/* ---- Bottom Bar ---- */
.bottom-bar {
  grid-area: bottom;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 14px;
  border-top: 1px solid #313244;
  background: #181825;
  flex-shrink: 0;
}
.workspace-label {
  font-size: 12px;
  color: #6c7086;
}
.workspace-select {
  padding: 3px 8px;
  border: 1px solid #45475a;
  border-radius: 4px;
  font-size: 12px;
  background: #1e1e2e;
  color: #cdd6f4;
}
.workspace-select:focus { outline: none; border-color: #89b4fa; }

/* ---- Scrollbar ---- */
.messages-scroll::-webkit-scrollbar,
.session-list::-webkit-scrollbar { width: 5px; }
.messages-scroll::-webkit-scrollbar-track,
.session-list::-webkit-scrollbar-track { background: transparent; }
.messages-scroll::-webkit-scrollbar-thumb,
.session-list::-webkit-scrollbar-thumb { background: #45475a; border-radius: 3px; }
</style>
