<template>
  <div class="main-window">
    <AppTopBar
      :title="pageTitle"
      :is-logged-in="isLoggedIn"
      :user-name="userName"
      @avatar-click="openLoginModal"
      @logout="handleLogout"
    />
    <div class="main-body">
      <AppIconRail
        :items="railItems"
        :active="activeItem"
        @update:active="activeItem = $event"
      />
      <SessionListPanel
        v-if="showSessionPanel"
        :sessions="chatSessions"
        :active-id="activeSessionId"
        @select="activeSessionId = $event"
        @clear="chatSessions = []"
      />
      <div class="main-content">
        <Transition name="fade" mode="out-in">
          <component :is="currentPage" :key="activeItem" />
        </Transition>
      </div>
    </div>
  </div>

  <!-- 登录弹窗 -->
  <AppModal
    :open="showLoginModal"
    :title="isRegisterMode ? '注册' : '登录'"
    max-width="360px"
    @update:open="showLoginModal = $event"
  >
    <form class="login-form" @submit.prevent="handleSubmit">
      <div v-if="isRegisterMode" class="form-row">
        <label class="form-label">名</label>
        <input
          v-model="regFirstName"
          class="form-input"
          type="text"
          placeholder="First name"
          autocomplete="given-name"
        />
      </div>
      <div v-if="isRegisterMode" class="form-row">
        <label class="form-label">姓</label>
        <input
          v-model="regLastName"
          class="form-input"
          type="text"
          placeholder="Last name"
          autocomplete="family-name"
        />
      </div>
      <div class="form-row">
        <label class="form-label">邮箱</label>
        <input
          v-model="email"
          class="form-input"
          type="email"
          placeholder="your@email.com"
          autocomplete="email"
        />
      </div>
      <div class="form-row">
        <label class="form-label">密码</label>
        <input
          v-model="password"
          class="form-input"
          type="password"
          placeholder="••••••"
          autocomplete="current-password"
        />
      </div>
      <p v-if="authError" class="form-error">{{ authError }}</p>
    </form>
    <template #footer>
      <button class="btn-link" type="button" @click="toggleMode">
        {{ isRegisterMode ? '已有账号？登录' : '没有账号？注册' }}
      </button>
      <button
        class="btn-ok"
        :disabled="authLoading"
        @click="handleSubmit"
      >
        {{ authLoading ? '请稍候...' : (isRegisterMode ? '注册' : '登录') }}
      </button>
    </template>
  </AppModal>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import AppTopBar from "@/components/layout/AppTopBar.vue";
import AppIconRail from "@/components/layout/AppIconRail.vue";
import SessionListPanel from "@/components/layout/SessionListPanel.vue";
import AppModal from "@/components/ui/AppModal.vue";
import {
  HomePage,
  TasksPage,
  JiraPage,
  EmailPage,
  MessagePage,
} from "@/pages";
import type { NavItem } from "@/components/layout/types";
import type { SessionItem } from "@/components/layout/SessionListPanel.vue";
import {
  login,
  logout,
  register,
  getCurrentUser,
  type UserProfile,
} from "@/core/auth";

const activeItem = ref("home");
const showLoginModal = ref(false);
const isLoggedIn = ref(false);
const userName = ref("");
const currentUser = ref<UserProfile | null>(null);

// Auth form state
const isRegisterMode = ref(false);
const email = ref("");
const password = ref("");
const regFirstName = ref("");
const regLastName = ref("");
const authError = ref("");
const authLoading = ref(false);

const railItems: NavItem[] = [
  { id: "home", label: "主页", icon: "🏠" },
  { id: "chat", label: "聊天", icon: "💬" },
  { id: "tasks", label: "任务", icon: "📋" },
  { id: "jira", label: "Jira", icon: "📎" },
  { id: "email", label: "邮件", icon: "" },
  { id: "message", label: "消息", icon: "✉️" },
];

const pageTitles: Record<string, string> = {
  home: "主页",
  chat: "聊天",
  tasks: "任务",
  jira: "Jira",
  email: "邮件",
  message: "消息",
};

const showSessionPanel = computed(() => activeItem.value === "chat" || activeItem.value === "message");

const pageTitle = computed(() => pageTitles[activeItem.value] ?? "CoreAIpet");

const activeSessionId = ref("s1");
const chatSessions: SessionItem[] = [
  { id: "s1", title: "新对话", time: "14:30", preview: "你好，我是CoreAIpet，很高兴为你..." },
  { id: "s2", title: "宠物模型讨论", time: "10:22", preview: "帮我对比一下Haru和Mao模型的区别..." },
  { id: "s3", title: "Qwen3 系列对比", time: "昨天", preview: "qwen3-7b 和 qwen3-7-plus 有什么..." },
  { id: "s4", title: "项目进度跟进", time: "04/19", preview: "帮我总结一下最近的项目进展情况" },
];

const currentPage = computed(() => {
  switch (activeItem.value) {
    case "home":
      return HomePage;
    case "tasks":
      return TasksPage;
    case "jira":
      return JiraPage;
    case "email":
      return EmailPage;
    case "chat":
    case "message":
      return MessagePage;
    default:
      return HomePage;
  }
});

function applyUser(user: UserProfile) {
  currentUser.value = user;
  isLoggedIn.value = true;
  userName.value = user.first_name;
}

function openLoginModal() {
  authError.value = "";
  email.value = "";
  password.value = "";
  regFirstName.value = "";
  regLastName.value = "";
  isRegisterMode.value = false;
  showLoginModal.value = true;
}

function toggleMode() {
  isRegisterMode.value = !isRegisterMode.value;
  authError.value = "";
}

async function handleSubmit() {
  authError.value = "";
  authLoading.value = true;

  try {
    if (isRegisterMode.value) {
      if (!regFirstName.value || !regLastName.value || !email.value || !password.value) {
        authError.value = "请填写所有字段";
        return;
      }
      const user = await register(
        regFirstName.value,
        regLastName.value,
        email.value,
        password.value,
      );
      applyUser(user);
    } else {
      if (!email.value || !password.value) {
        authError.value = "请输入邮箱和密码";
        return;
      }
      const user = await login(email.value, password.value);
      applyUser(user);
    }
    showLoginModal.value = false;
  } catch (err) {
    authError.value = typeof err === "string" ? err : "登录失败，请重试";
  } finally {
    authLoading.value = false;
  }
}

async function handleLogout() {
  try {
    await logout();
  } catch {
    // ignore
  }
  currentUser.value = null;
  isLoggedIn.value = false;
  userName.value = "";
}

onMounted(async () => {
  // Restore session on startup
  try {
    const user = await getCurrentUser();
    if (user) applyUser(user);
  } catch {
    // ignore — user stays logged out
  }
});
</script>

<style scoped>
.main-window {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
  background: var(--bg-base);
  color: var(--text);
  overflow: hidden;
}

.main-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.main-content::-webkit-scrollbar {
  width: 5px;
}

.main-content::-webkit-scrollbar-track {
  background: transparent;
}

.main-content::-webkit-scrollbar-thumb {
  background: var(--border-strong);
  border-radius: 3px;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity var(--t-med) ease-in-out;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* ── Login form ── */

.login-form {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.form-row {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.form-label {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-dim);
}

.form-input {
  height: 34px;
  padding: 0 10px;
  border: 1px solid var(--border);
  border-radius: var(--r-lg);
  background: var(--bg-base);
  color: var(--text);
  font-size: 13px;
  font-family: inherit;
  outline: none;
  transition: border-color var(--t-fast);
}

.form-input:focus {
  border-color: var(--accent);
}

.form-input::placeholder {
  color: var(--text-muted);
}

.form-error {
  margin: 0;
  font-size: 12px;
  color: #f38ba8;
  line-height: 1.4;
}

.btn-link {
  padding: 6px 0;
  border: none;
  background: transparent;
  color: var(--text-dim);
  font-size: 12px;
  font-family: inherit;
  cursor: pointer;
  text-decoration: underline;
  text-underline-offset: 2px;
  transition: color var(--t-fast);
  margin-right: auto;
}

.btn-link:hover {
  color: var(--accent);
}

.btn-ok {
  padding: 6px 16px;
  border: none;
  border-radius: var(--r-lg);
  background: var(--accent);
  color: var(--bg-base);
  font-size: 13px;
  font-weight: 600;
  font-family: inherit;
  cursor: pointer;
  transition: background var(--t-fast), opacity var(--t-fast);
}

.btn-ok:hover {
  background: var(--accent-hover);
}

.btn-ok:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
