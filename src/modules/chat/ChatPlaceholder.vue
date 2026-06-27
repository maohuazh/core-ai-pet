<template>
  <div v-if="visible" class="chat-placeholder">
    <div class="chat-header">
      <span class="chat-title">💬 Chat</span>
      <button class="close-btn" @click="close">×</button>
    </div>
    <div ref="messagesContainer" class="chat-messages">
      <div v-for="(message, index) in messages" :key="index" class="message">
        <div class="message-role">{{ message.role }}</div>
        <div class="message-content">{{ message.content }}</div>
      </div>
      <div v-if="isLoading" class="loading-indicator">
        <span class="loading-dot"></span>
        <span class="loading-dot"></span>
        <span class="loading-dot"></span>
      </div>
    </div>
    <div class="chat-input-area">
      <textarea
        v-model="inputMessage"
        placeholder="输入消息... (Ctrl+Enter 发送)"
        @keydown.ctrl.enter="sendMessage"
        @keydown.meta.enter="sendMessage"
        class="message-input"
        rows="3"
      ></textarea>
      <button @click="sendMessage" :disabled="isLoading || !inputMessage.trim()" class="send-btn">
        发送
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, watch } from 'vue';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

interface Message {
  role: 'user' | 'assistant';
  content: string;
}

interface DeltaEvent {
  turn_id: string;
  delta: {
    type: string;
    text?: string;
    message?: string;
    [key: string]: any;
  };
}

interface DoneEvent {
  turn_id: string;
}

const visible = ref(false);
const messages = ref<Message[]>([]);
const inputMessage = ref('');
const isLoading = ref(false);
const currentTurnId = ref<string | null>(null);
const messagesContainer = ref<HTMLElement | null>(null);

let unlistenStream: UnlistenFn | null = null;
let unlistenDone: UnlistenFn | null = null;

onMounted(async () => {
  // Listen for llm_delta events (emitted by Rust backend)
  unlistenStream = await listen<DeltaEvent>('llm_delta', (event) => {
    const { turn_id, delta } = event.payload;

    if (turn_id !== currentTurnId.value) {
      return;
    }

    if (delta.type === 'Text' && delta.text) {
      // Append text to the last assistant message or create a new one
      const lastMessage = messages.value[messages.value.length - 1];
      if (lastMessage && lastMessage.role === 'assistant') {
        lastMessage.content += delta.text;
      } else {
        messages.value.push({
          role: 'assistant',
          content: delta.text
        });
      }
      scrollToBottom();
    } else if (delta.type === 'Error') {
      messages.value.push({
        role: 'assistant',
        content: `[错误: ${delta.message || '未知错误'}]`
      });
      scrollToBottom();
    }
  });

  // Listen for llm_done events (emitted by Rust backend)
  unlistenDone = await listen<DoneEvent>('llm_done', (event) => {
    const { turn_id } = event.payload;

    if (turn_id !== currentTurnId.value) {
      return;
    }

    isLoading.value = false;
    currentTurnId.value = null;
  });
});

onUnmounted(() => {
  if (unlistenStream) {
    unlistenStream();
  }
  if (unlistenDone) {
    unlistenDone();
  }
});

async function sendMessage() {
  const message = inputMessage.value.trim();
  if (!message || isLoading.value) {
    return;
  }

  // Add user message
  messages.value.push({
    role: 'user',
    content: message
  });

  // Clear input
  inputMessage.value = '';
  isLoading.value = true;

  // Show the chat window
  visible.value = true;

  try {
    // Generate turn ID
    currentTurnId.value = crypto.randomUUID();

    // Call LLM via Rust backend
    await invoke('llm_invoke', {
      role: 'chat_assistant',
      request: {
        messages: [{ role: 'user', content: message }],
        stream: true
      }
    });
  } catch (error) {
    console.error('Failed to invoke LLM:', error);
    messages.value.push({
      role: 'assistant',
      content: `[调用失败: ${error}]`
    });
    isLoading.value = false;
    currentTurnId.value = null;
  }

  scrollToBottom();
}

function scrollToBottom() {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
    }
  });
}

function close() {
  visible.value = false;
}

// Auto-scroll when messages change
watch(messages, () => {
  scrollToBottom();
}, { deep: true });

// Expose methods for parent component
defineExpose({
  show: () => { visible.value = true; },
  hide: close
});
</script>

<style scoped>
.chat-placeholder {
  position: fixed;
  top: 20px;
  right: 20px;
  width: 400px;
  height: 500px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
  display: flex;
  flex-direction: column;
  z-index: 9999;
}

.chat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  background: rgba(99, 102, 241, 0.1);
  border-radius: 12px 12px 0 0;
}

.chat-title {
  font-weight: 600;
  font-size: 14px;
  color: #333;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  color: #666;
  cursor: pointer;
  padding: 0;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: background 0.2s;
}

.close-btn:hover {
  background: rgba(0, 0, 0, 0.05);
}

.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.message {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.message-role {
  font-size: 11px;
  font-weight: 600;
  color: #666;
  text-transform: uppercase;
}

.message-content {
  padding: 8px 12px;
  background: rgba(0, 0, 0, 0.03);
  border-radius: 8px;
  font-size: 13px;
  line-height: 1.5;
  color: #333;
  white-space: pre-wrap;
  word-break: break-word;
}

.loading-indicator {
  display: flex;
  gap: 4px;
  padding: 8px 12px;
}

.loading-dot {
  width: 6px;
  height: 6px;
  background: #6366f1;
  border-radius: 50%;
  animation: bounce 1.4s infinite ease-in-out both;
}

.loading-dot:nth-child(1) {
  animation-delay: -0.32s;
}

.loading-dot:nth-child(2) {
  animation-delay: -0.16s;
}

@keyframes bounce {
  0%, 80%, 100% {
    transform: scale(0);
  }
  40% {
    transform: scale(1);
  }
}

.chat-input-area {
  display: flex;
  gap: 8px;
  padding: 12px;
  border-top: 1px solid rgba(0, 0, 0, 0.1);
  background: rgba(255, 255, 255, 0.5);
  border-radius: 0 0 12px 12px;
}

.message-input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 6px;
  font-size: 13px;
  font-family: inherit;
  resize: none;
  background: white;
}

.message-input:focus {
  outline: none;
  border-color: #6366f1;
}

.send-btn {
  padding: 8px 16px;
  background: #6366f1;
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.send-btn:hover:not(:disabled) {
  background: #5568d3;
}

.send-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Scrollbar styling */
.chat-messages::-webkit-scrollbar {
  width: 6px;
}

.chat-messages::-webkit-scrollbar-track {
  background: transparent;
}

.chat-messages::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.1);
  border-radius: 3px;
}

.chat-messages::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.2);
}
</style>
