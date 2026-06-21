# PRD: 设置面板 UI (Settings Panel)

> 版本: 1.0
> 日期: 2026-06-20
> 状态: 草案

---

## 1. 概述 (Overview)

为 CoreAIpet 桌面宠物应用实现一个**全局设置面板**，提供 Jira 连接、邮箱连接、聊天工具、模型配置四大功能模块。面板采用半透明浅色毛玻璃风格，具备高端现代感的视觉体验。

面板作为独立窗口打开（非悬浮菜单内嵌），支持从 PetHoverMenu（`src/components/PetHoverMenu.vue`）的设置按钮触发。所有配置数据持久化到 SQLite 数据库（通过 Tauri SQLite 插件 `src/core/storage/index.ts`）。

---

## 2. 目标 (Goals)

| # | 目标                      | 验证标准                                    |
|---|-------------------------|-----------------------------------------|
| G1 | 实现半透明毛玻璃风格设置面板窗口        | 面板窗口背景可透视桌面，UI 元素清晰可读                   |
| G2 | 实现 Jira 连接模块（含 Mock 数据） | 显示授权按钮、2 条示例连接、可切换启用状态                  |
| G3 | 实现邮箱连接模块（含 Mock 数据）     | 同 Jira 结构，2 条示例账户                       |
| G4 | 实现聊天工具连接模块（纯 Mock）      | 桶Jira结构，2 条示例账户                         |
| G5 | 实现模型配置模块                | 列出所有 Live2D + SpriteSheet 模型，支持切换/导入/删除 |
| G6 | 所有配置持久化到 SQLite         | 重启应用后配置保留                               |
| G7 | 流畅的交互动效                 | hover 高亮、页面切换过渡、操作反馈                    |

---

## 3. 非目标 (Non-Goals)

| # | 非目标 | 原因 |
|---|--------|------|
| NG1 | 不实现 Jira/邮箱的实际连接逻辑 | 本阶段仅需 UI 壳 + Mock 数据，实际 API 对接后续 PRD |
| NG2 | 不实现聊天工具的实际消息收发 | 仅展示 Mock 会话数据 |
| NG3 | 不实现模型的导入/删除功能 | 由 [PRD-External-Model-Import.md](./PRD-External-Model-Import.md) 覆盖 |
| NG4 | 不实现 Action/Expression 映射配置 UI | 由 [PRD-Action-Expression-Mapping.md](./PRD-Action-Expression-Mapping.md) 覆盖 |
| NG5 | 不实现多语言国际化 | 初期仅中文 |

---

## 4. 功能需求 (Functional Requirements)

### 4.1 面板窗口基础

| 属性 | 规格 |
|------|------|
| 窗口类型 | Tauri 独立窗口（非主窗口） |
| 窗口尺寸 | 宽 680px × 高 720px（可调整） |
| 最小尺寸 | 宽 560px × 高 480px |
| 背景 | 半透明毛玻璃效果（`backdrop-filter: blur(20px)`） |
| 背景色 | `rgba(255, 255, 255, 0.75)` |
| 边框 | 圆角 16px，无边框窗口（`decorations: false`） |
| 阴影 | 外阴影 `0 8px 32px rgba(0, 0, 0, 0.12)` |
| 置顶 | 可选跟随主窗口（默认不置顶） |
| 关闭行为 | 点击关闭按钮隐藏窗口（不销毁） |

### 4.2 导航结构

```
┌──────────────────────────────────────────────────────────┐
│  ⚙️ 设置                                         ─ □ ✕  │
├────────────┬─────────────────────────────────────────────┤
│            │                                             │
│  📋 Jira   │                                             │
│  📧 邮箱   │          [ 当前模块内容区 ]                  │
│  💬 聊天   │                                             │
│  🎭 模型   │                                             │
│            │                                             │
│            │                                             │
│            │                                             │
│            │                                             │
│            │                                             │
├────────────┴─────────────────────────────────────────────┤
│  CoreAIpet v1.0.0                                       │
└──────────────────────────────────────────────────────────┘
```

- 左侧导航栏宽度 160px，半透明背景 `rgba(255, 255, 255, 0.4)`
- 导航项高度 44px，hover 时背景变为 `rgba(99, 102, 241, 0.08)`
- 选中项左侧有 3px 宽的主色调指示条（`#6366F1`）
- 右侧内容区带淡入过渡动画（`transition: opacity 0.2s ease`）

### 4.3 模块 A：Jira 连接

#### 4.3.1 功能描述

展示已连接的 Jira 实例列表，支持授权新连接、启用/禁用、删除操作。本阶段仅使用 Mock 数据，实际授权逻辑留空。

#### 4.3.2 Mock 数据

```typescript
interface MockJiraConnection {
  id: string;
  name: string;              // 显示名称
  url: string;               // Jira 实例 URL
  email: string;             // 登录邮箱
  status: "connected" | "expired" | "error";
  enabled: boolean;          // 是否启用
  createdAt: string;         // ISO 时间戳
  lastSyncAt?: string;       // 最后同步时间
}

const MOCK_JIRA_CONNECTIONS: MockJiraConnection[] = [
  {
    id: "jira-mock-001",
    name: "公司项目管理",
    url: "https://mycompany.atlassian.net",
    email: "zhangsan@mycompany.com",
    status: "connected",
    enabled: true,
    createdAt: "2026-05-10T08:00:00Z",
    lastSyncAt: "2026-06-20T09:30:00Z"
  },
  {
    id: "jira-mock-002",
    name: "开源项目追踪",
    url: "https://opensource.atlassian.net",
    email: "zhangsan@gmail.com",
    status: "expired",
    enabled: false,
    createdAt: "2026-03-15T10:00:00Z",
    lastSyncAt: "2026-05-01T14:00:00Z"
  }
];
```

#### 4.3.3 UI 布局

```
┌─────────────────────────────────────────────────────┐
│  Jira 连接                                    [+ 添加] │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │ 🟢 公司项目管理                          ⋮   │  │
│  │ mycompany.atlassian.net                       │  │
│  │ zhangsan@mycompany.com                        │  │
│  │ 上次同步: 2026-06-20 09:30                    │  │
│  │                                               │  │
│  │ [████████ 启用开关 ████████]    [撤销授权]    │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │ 🔴 开源项目追踪                          ⋮   │  │
│  │ opensource.atlassian.net                      │  │
│  │ zhangsan@gmail.com                            │  │
│  │ 授权已过期                                    │  │
│  │                                               │  │
│  │ [░░░░░░░░ 启用开关 ░░░░░░░░]    [重新授权]   │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
└─────────────────────────────────────────────────────┘
```

#### 4.3.4 交互说明

| 操作 | 行为 |
|------|------|
| 点击「+ 添加」| 弹出授权对话框（本阶段显示 "功能开发中" Toast）|
| 切换启用开关 | 更新 `enabled` 状态，写入 SQLite |
| 点击「撤销授权」| 弹出确认对话框 → 删除连接记录 |
| 点击「重新授权」| 显示 "功能开发中" Toast |
| 点击卡片右上角 `⋮` | 下拉菜单：编辑名称 / 删除 |

### 4.4 模块 B：邮箱连接

#### 4.4.1 功能描述

结构与 Jira 模块完全一致，仅数据模型不同。

#### 4.4.2 Mock 数据

```typescript
interface MockEmailAccount {
  id: string;
  name: string;
  email: string;
  provider: "gmail" | "outlook" | "imap" | "other";
  status: "connected" | "expired" | "error";
  enabled: boolean;
  createdAt: string;
  lastSyncAt?: string;
}

const MOCK_EMAIL_ACCOUNTS: MockEmailAccount[] = [
  {
    id: "email-mock-001",
    name: "工作邮箱",
    email: "zhangsan@mycompany.com",
    provider: "outlook",
    status: "connected",
    enabled: true,
    createdAt: "2026-04-01T08:00:00Z",
    lastSyncAt: "2026-06-20T10:00:00Z"
  },
  {
    id: "email-mock-002",
    name: "个人邮箱",
    email: "zhangsan@gmail.com",
    provider: "gmail",
    status: "connected",
    enabled: true,
    createdAt: "2026-01-20T08:00:00Z",
    lastSyncAt: "2026-06-20T09:45:00Z"
  }
];
```

#### 4.4.3 UI 布局

与 Jira 卡片结构一致，额外显示 `provider` 图标（Gmail/Outlook/IMAP 等）。

### 4.5 模块 C：聊天工具

#### 4.5.1 功能描述

展示已连接的聊天工具列表，支持授权新连接、启用/禁用、断开连接操作。本阶段仅使用 Mock 数据，实际授权逻辑留空。

#### 4.5.2 Mock 数据

```typescript
interface MockChatPlatform {
  id: string;
  name: string;                    // "WeChat" / "Slack" / "Teams" / "Discord"
  icon: string;                    // 平台图标（emoji 或图片路径）
  status: "connected" | "disconnected" | "error";
  enabled: boolean;
  connectedAt?: string;            // 连接时间（ISO 时间戳）
  accountName?: string;            // 连接的账号名称
}

const MOCK_CHAT_PLATFORMS: MockChatPlatform[] = [
  {
    id: "chat-wechat",
    name: "WeChat",
    icon: "💬",
    status: "connected",
    enabled: true,
    connectedAt: "2026-05-15T10:30:00Z",
    accountName: "张三"
  },
  {
    id: "chat-slack",
    name: "Slack",
    icon: "💼",
    status: "connected",
    enabled: true,
    connectedAt: "2026-04-20T14:00:00Z",
    accountName: "zhangsan@company.com"
  },
  {
    id: "chat-teams",
    name: "Microsoft Teams",
    icon: "👥",
    status: "disconnected",
    enabled: false
  },
  {
    id: "chat-discord",
    name: "Discord",
    icon: "🎮",
    status: "disconnected",
    enabled: false
  }
];
```

#### 4.5.3 UI 布局

```
┌──────────────────────────────────────────────────────┐
│  聊天工具连接                                  [+ 添加] │
├──────────────────────────────────────────────────────┤
│                                                      │
│  ┌────────────────────────────────────────────────┐  │
│  │ 💬 WeChat                                ⋮    │  │
│  │ 已连接: 张三                                   │  │
│  │ 连接时间: 2026-05-15                           │  │
│  │                                                │  │
│  │ [████████ 启用开关 ████████]    [断开连接]     │  │
│  └────────────────────────────────────────────────┘  │
│                                                      │
│  ┌────────────────────────────────────────────────┐  │
│  │ 💼 Slack                                 ⋮    │  │
│  │ 已连接: zhangsan@company.com                   │  │
│  │ 连接时间: 2026-04-20                           │  │
│  │                                                │  │
│  │ [████████ 启用开关 ████████]    [断开连接]     │  │
│  └────────────────────────────────────────────────┘  │
│                                                      │
│  ┌────────────────────────────────────────────────┐  │
│  │ 👥 Microsoft Teams                       ⋮    │  │
│  │ 未连接                                         │  │
│  │                                                │  │
│  │ [░░░░░░░░ 启用开关 ░░░░░░░░]    [授权连接]     │  │
│  └────────────────────────────────────────────────┘  │
│                                                      │
│  ┌────────────────────────────────────────────────┐  │
│  │ 🎮 Discord                               ⋮    │  │
│  │ 未连接                                         │  │
│  │                                                │  │
│  │ [░░░░░░░░ 启用开关 ░░░░░░░░]    [授权连接]     │  │
│  └────────────────────────────────────────────────┘  │
│                                                      │
└──────────────────────────────────────────────────────┘
```

#### 4.5.4 交互说明

| 操作 | 行为 |
|------|------|
| 点击「+ 添加」| 弹出聊天工具选择对话框（本阶段显示 "功能开发中" Toast）|
| 切换启用开关 | 更新 `enabled` 状态，写入 SQLite |
| 点击「断开连接」| 弹出确认对话框 → 更新状态为 disconnected |
| 点击「授权连接」| 显示 "功能开发中" Toast |
| 点击卡片右上角 `⋮` | 下拉菜单：编辑名称 / 删除 |

### 4.6 模块 D：模型配置

#### 4.6.1 功能描述

列出所有已注册的模型（Live2D + SpriteSheet），支持：
- 切换当前活跃模型
- 查看模型详情
- 跳转到导入 / 删除操作
- 跳转到 Action/Expression 映射配置

#### 4.6.2 UI 布局

```
┌──────────────────────────────────────────────────────┐
│  模型配置                                    [+ 导入] │
├──────────────────────────────────────────────────────┤
│                                                      │
│  ┌────────────────────────────────────────────────┐  │
│  │  🎭                   Hiyori                   │  │
│  │  类型: Live2D  |  来源: 内置                    │  │
│  │                                                │  │
│  │  [▶ 使用此模型]   [⚙ 动作映射]   [🗑 删除]    │  │
│  └────────────────────────────────────────────────┘  │
│                                                      │
│  ┌────────────────────────────────────────────────┐  │
│  │  🖼️                   PixelCat                 │  │
│  │  类型: SpriteSheet  |  来源: 自定义             │  │
│  │                                                │  │
│  │  [▶ 使用此模型]   [⚙ 动作映射]   [🗑 删除]    │  │
│  └────────────────────────────────────────────────┘  │
│                                                      │
└──────────────────────────────────────────────────────┘
```

#### 4.6.3 交互说明

| 操作 | 行为 |
|------|------|
| 点击「+ 导入」| 打开文件选择对话框，选择模型目录（详见 PRD-External-Model-Import）|
| 点击「▶ 使用此模型」| 调用 `PetStore.setActiveModel(modelId)`，切换渲染器 |
| 点击「⚙ 动作映射」| 打开动作映射配置面板（详见 PRD-Action-Expression-Mapping）|
| 点击「🗑 删除」| 确认对话框 → 调用 `ModelRegistry.removeModel(modelId)` |
| 当前活跃模型 | 卡片左边框显示主色调指示条，按钮变为「✓ 当前模型」|

---

## 5. 技术设计 (Technical Design)

### 5.1 窗口管理

新增 Tauri 窗口，在 Rust 端配置：

```rust
// src-tauri/src/main.rs 或 windows 模块

use tauri::{Manager, WindowBuilder, WindowUrl};

fn create_settings_window(app: &tauri::AppHandle) -> tauri::Result<tauri::Window> {
    // 如果窗口已存在，直接聚焦
    if let Some(w) = app.get_window("settings") {
        w.show()?;
        w.set_focus()?;
        return Ok(w);
    }

    WindowBuilder::new(
        app,
        "settings",
        WindowUrl::App("/settings".into())
    )
    .title("CoreAIpet - 设置")
    .inner_size(680.0, 720.0)
    .min_inner_size(560.0, 480.0)
    .decorations(false)          // 自定义标题栏
    .transparent(true)           // 支持透明背景
    .always_on_top(false)
    .resizable(true)
    .build()
}
```

前端路由：

```typescript
// src/router.ts (或简单的条件渲染)
// 当 URL path 为 /settings 时渲染 SettingsPanel
```

### 5.2 前端组件结构

```
src/components/settings/
  ├── SettingsPanel.vue              ← 设置面板根组件
  ├── SettingsSidebar.vue            ← 左侧导航栏
  ├── SettingsTitleBar.vue           ← 自定义标题栏（拖拽 + 关闭按钮）
  ├── modules/
  │   ├── JiraModule.vue             ← Jira 连接模块
  │   ├── EmailModule.vue            ← 邮箱连接模块
  │   ├── ChatModule.vue             ← 聊天工具模块
  │   └── ModelConfigModule.vue      ← 模型配置模块
  ├── shared/
  │   ├── ConnectionCard.vue         ← 通用连接卡片（Jira/Email 复用）
  │   ├── ToggleSwitch.vue           ← 启用/禁用开关
  │   ├── ConfirmDialog.vue          ← 确认对话框
  │   └── EmptyState.vue             ← 空状态占位
  └── types.ts                       ← 设置面板类型定义
```

### 5.3 状态管理

```typescript
// src/core/settings/settingsStore.ts

import { defineStore } from "pinia";  // 或使用现有状态管理方案

interface SettingsState {
  // UI 状态
  activeModule: "jira" | "email" | "chat" | "model";
  isSettingsOpen: boolean;

  // Jira 数据
  jiraConnections: JiraConnection[];

  // Email 数据
  emailAccounts: EmailAccount[];

  // Chat 数据
  chatPlatforms: ChatPlatform[];
}

export const useSettingsStore = defineStore("settings", {
  state: (): SettingsState => ({
    activeModule: "jira",
    isSettingsOpen: false,
    jiraConnections: [],
    emailAccounts: [],
    chatPlatforms: [],
  }),

  actions: {
    async loadAllData() { /* 从 SQLite 加载 */ },
    async toggleJiraEnabled(id: string, enabled: boolean) { /* ... */ },
    async toggleEmailEnabled(id: string, enabled: boolean) { /* ... */ },
    async toggleChatEnabled(id: string, enabled: boolean) { /* ... */ },
    async removeJiraConnection(id: string) { /* ... */ },
    async removeEmailAccount(id: string) { /* ... */ },
    async disconnectChatPlatform(id: string) { /* ... */ },
    setActiveModule(module: SettingsState["activeModule"]) { /* ... */ },
    openSettings() { /* 创建/显示窗口 */ },
    closeSettings() { /* 隐藏窗口 */ },
  }
});
```

### 5.4 Tauri IPC 接口

```rust
// src-tauri/src/settings_commands.rs

#[tauri::command]
async fn get_jira_connections(db: State<'_, Database>) -> Result<Vec<JiraConnection>, String> {
    db.get_jira_connections().map_err(|e| e.to_string())
}

#[tauri::command]
async fn toggle_jira_connection(db: State<'_, Database>, id: String, enabled: bool) -> Result<(), String> {
    db.toggle_jira_connection(&id, enabled).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_jira_connection(db: State<'_, Database>, id: String) -> Result<(), String> {
    db.delete_jira_connection(&id).map_err(|e| e.to_string())
}

// Email 同理
#[tauri::command]
async fn get_email_accounts(db: State<'_, Database>) -> Result<Vec<EmailAccount>, String> { ... }

#[tauri::command]
async fn toggle_email_account(db: State<'_, Database>, id: String, enabled: bool) -> Result<(), String> { ... }

#[tauri::command]
async fn delete_email_account(db: State<'_, Database>, id: String) -> Result<(), String> { ... }

// Chat 平台连接
#[tauri::command]
async fn get_chat_platforms(db: State<'_, Database>) -> Result<Vec<ChatPlatform>, String> { ... }

#[tauri::command]
async fn toggle_chat_platform(db: State<'_, Database>, id: String, enabled: bool) -> Result<(), String> { ... }

#[tauri::command]
async fn disconnect_chat_platform(db: State<'_, Database>, id: String) -> Result<(), String> { ... }
```

### 5.5 从 PetHoverMenu 触发

修改 `src/components/PetHoverMenu.vue`，在菜单项中新增设置入口：

```typescript
const menuItems = [
  { icon: "⚙️", label: "设置", action: "openSettings" },
  // ... 现有菜单项
];

function handleMenuAction(action: string) {
  switch (action) {
    case "openSettings":
      invoke("open_settings_window");
      break;
    // ...
  }
}
```

---

## 6. 数据库设计 (Database Schema)

### 6.1 完整表结构

```sql
-- =====================================================
-- Jira 连接表
-- =====================================================
CREATE TABLE IF NOT EXISTS jira_connections (
    id              TEXT PRIMARY KEY,          -- UUID
    name            TEXT NOT NULL,             -- 用户自定义显示名
    url             TEXT NOT NULL,             -- Jira 实例 URL
    email           TEXT NOT NULL,             -- 登录邮箱
    api_token       TEXT,                      -- 加密存储的 API Token（本阶段为空）
    status          TEXT NOT NULL DEFAULT 'connected'
                        CHECK (status IN ('connected', 'expired', 'error')),
    enabled         INTEGER NOT NULL DEFAULT 1, -- 0=禁用, 1=启用
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),
    last_sync_at    TEXT                       -- 最后成功同步时间
);

CREATE INDEX idx_jira_enabled ON jira_connections(enabled);

-- =====================================================
-- 邮箱连接表
-- =====================================================
CREATE TABLE IF NOT EXISTS email_accounts (
    id              TEXT PRIMARY KEY,          -- UUID
    name            TEXT NOT NULL,             -- 用户自定义显示名
    email           TEXT NOT NULL,             -- 邮箱地址
    provider        TEXT NOT NULL DEFAULT 'imap'
                        CHECK (provider IN ('gmail', 'outlook', 'imap', 'other')),
    auth_type       TEXT NOT NULL DEFAULT 'oauth2'
                        CHECK (auth_type IN ('oauth2', 'app_password', 'imap_password')),
    auth_token      TEXT,                      -- 加密存储的认证凭据（本阶段为空）
    imap_host       TEXT,                      -- IMAP 服务器地址
    imap_port       INTEGER,                   -- IMAP 端口
    smtp_host       TEXT,                      -- SMTP 服务器地址
    smtp_port       INTEGER,                   -- SMTP 端口
    status          TEXT NOT NULL DEFAULT 'connected'
                        CHECK (status IN ('connected', 'expired', 'error')),
    enabled         INTEGER NOT NULL DEFAULT 1,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),
    last_sync_at    TEXT
);

CREATE INDEX idx_email_enabled ON email_accounts(enabled);

-- =====================================================
-- 聊天工具连接表
-- =====================================================
CREATE TABLE IF NOT EXISTS chat_platforms (
    id              TEXT PRIMARY KEY,          -- UUID
    name            TEXT NOT NULL,             -- "WeChat" / "Slack" / "Teams" / "Discord"
    icon            TEXT,                      -- 平台图标（emoji 或图片路径）
    status          TEXT NOT NULL DEFAULT 'disconnected'
                        CHECK (status IN ('connected', 'disconnected', 'error')),
    enabled         INTEGER NOT NULL DEFAULT 0,
    account_name    TEXT,                      -- 连接的账号名称
    auth_token      TEXT,                      -- 加密存储的认证凭据（本阶段为空）
    connected_at    TEXT,                      -- 连接时间
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_chat_platform_enabled ON chat_platforms(enabled);
CREATE INDEX idx_chat_platform_status ON chat_platforms(status);

-- =====================================================
-- 模型注册表
-- =====================================================
CREATE TABLE IF NOT EXISTS models (
    id              TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    type            TEXT NOT NULL DEFAULT 'live2d'
                        CHECK (type IN ('live2d', 'sprite')),
    path            TEXT NOT NULL,             -- 模型目录路径（相对或绝对）
    manifest_path   TEXT,                      -- SpriteSheet: manifest.json 相对路径
    model3_path     TEXT,                      -- Live2D: .model3.json 相对路径
    thumbnail       TEXT,                      -- 缩略图路径
    source          TEXT NOT NULL DEFAULT 'builtin'
                        CHECK (source IN ('builtin', 'cdn', 'custom')),
    status          TEXT NOT NULL DEFAULT 'active'
                        CHECK (status IN ('active', 'inactive')),
    -- 元数据
    author          TEXT,
    version         TEXT,
    description     TEXT,
    license         TEXT,
    -- 排序
    sort_order      INTEGER NOT NULL DEFAULT 0,
    -- 时间戳
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_models_type ON models(type);
CREATE INDEX idx_models_status ON models(status);
CREATE INDEX idx_models_source ON models(source);

-- =====================================================
-- 模型动作/表情映射表
-- =====================================================
CREATE TABLE IF NOT EXISTS model_action_mappings (
    id              TEXT PRIMARY KEY,
    model_id        TEXT NOT NULL REFERENCES models(id) ON DELETE CASCADE,

    -- 触发场景
    trigger_key     TEXT NOT NULL
                        CHECK (trigger_key IN (
                            'daily_1', 'daily_2', 'daily_3',
                            'new_message', 'new_task', 'new_email',
                            'task_in_progress', 'task_completed',
                            'task_approaching_deadline', 'task_overdue'
                        )),

    -- 动作配置
    motion_group    TEXT,                      -- 动作分组名
    motion_name     TEXT,                      -- 具体动作名
    expression_name TEXT,                      -- 表情名
    effect_name     TEXT,                      -- 特效名（可选）

    -- 是否使用模型默认值（仅 daily_1 默认 true）
    use_default     INTEGER NOT NULL DEFAULT 0,

    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),

    -- 每个模型每个 trigger 只能有一条映射
    UNIQUE(model_id, trigger_key)
);

CREATE INDEX idx_action_mapping_model ON model_action_mappings(model_id);
CREATE INDEX idx_action_mapping_trigger ON model_action_mappings(trigger_key);

-- =====================================================
-- 应用全局设置表（KV 存储）
-- =====================================================
CREATE TABLE IF NOT EXISTS app_settings (
    key             TEXT PRIMARY KEY,
    value           TEXT NOT NULL,             -- JSON 序列化的值
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### 6.2 Mock 数据初始化

```sql
-- 插入 Jira Mock 数据
INSERT INTO jira_connections (id, name, url, email, status, enabled, last_sync_at) VALUES
('jira-mock-001', '公司项目管理', 'https://mycompany.atlassian.net', 'zhangsan@mycompany.com', 'connected', 1, '2026-06-20T09:30:00Z'),
('jira-mock-002', '开源项目追踪', 'https://opensource.atlassian.net', 'zhangsan@gmail.com', 'expired', 0, '2026-05-01T14:00:00Z');

-- 插入 Email Mock 数据
INSERT INTO email_accounts (id, name, email, provider, status, enabled, last_sync_at) VALUES
('email-mock-001', '工作邮箱', 'zhangsan@mycompany.com', 'outlook', 'connected', 1, '2026-06-20T10:00:00Z'),
('email-mock-002', '个人邮箱', 'zhangsan@gmail.com', 'gmail', 'connected', 1, '2026-06-20T09:45:00Z');

-- 插入 Chat Mock 数据
INSERT INTO chat_platforms (id, name, icon, status, enabled, account_name, connected_at) VALUES
('chat-wechat', 'WeChat', '💬', 'connected', 1, '张三', '2026-05-15T10:30:00Z'),
('chat-slack', 'Slack', '💼', 'connected', 1, 'zhangsan@company.com', '2026-04-20T14:00:00Z'),
('chat-teams', 'Microsoft Teams', '👥', 'disconnected', 0, NULL, NULL),
('chat-discord', 'Discord', '🎮', 'disconnected', 0, NULL, NULL);
```

### 6.3 ER 图

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ jira_connections │    │  email_accounts  │    │  chat_platforms │
├─────────────────┤    ├─────────────────┤    ├─────────────────┤
│ id (PK)         │    │ id (PK)         │    │ id (PK)         │
│ name            │    │ name            │    │ name            │
│ url             │    │ email           │    │ icon            │
│ email           │    │ provider        │    │ status          │
│ api_token       │    │ auth_type       │    │ enabled         │
│ status          │    │ auth_token      │    │ account_name    │
│ enabled         │    │ imap_host/port  │    │ auth_token      │
│ created_at      │    │ smtp_host/port  │    │ connected_at    │
│ updated_at      │    │ status          │    │ created_at      │
│ last_sync_at    │    │ enabled         │    │ updated_at      │
└─────────────────┘    │ created_at      │    └─────────────────┘
                       │ updated_at      │
                       │ last_sync_at    │
                       └─────────────────┘

┌─────────────────┐         ┌─────────────────────────┐
│     models       │         │  model_action_mappings   │
├─────────────────┤         ├─────────────────────────┤
│ id (PK)         │◄────────│ model_id (FK)           │
│ name            │    ┌───│ trigger_key             │
│ type            │    │   │ motion_group            │
│ path            │    │   │ motion_name             │
│ manifest_path   │    │   │ expression_name         │
│ model3_path     │    │   │ effect_name             │
│ thumbnail       │    │   │ use_default             │
│ source          │    │   │ created_at              │
│ status          │    │   │ updated_at              │
│ author          │    │   └─────────────────────────┘
│ version         │    │
│ description     │    │   ┌─────────────────────────┐
│ license         │    │   │     app_settings        │
│ sort_order      │    │   ├─────────────────────────┤
│ created_at      │    │   │ key (PK)               │
│ updated_at      │    │   │ value (JSON)            │
└─────────────────┘    │   │ updated_at              │
                       │   └─────────────────────────┘
```

---

## 7. UI/UX 设计

### 7.1 视觉规范

#### 色彩系统

| 用途 | 色值 | 说明 |
|------|------|------|
| 主色调 | `#6366F1` | Indigo 500，用于选中态、按钮、指示条 |
| 主色调 hover | `#818CF8` | Indigo 400 |
| 主色调 active | `#4F46E5` | Indigo 600 |
| 成功色 | `#10B981` | Emerald 500，用于已连接状态 |
| 警告色 | `#F59E0B` | Amber 500，用于过期状态 |
| 错误色 | `#EF4444` | Red 500，用于错误状态/删除按钮 |
| 文字主色 | `#1F2937` | Gray 800 |
| 文字次色 | `#6B7280` | Gray 500 |
| 文字辅助色 | `#9CA3AF` | Gray 400 |
| 背景色 | `rgba(255, 255, 255, 0.75)` | 半透明白 |
| 卡片背景 | `rgba(255, 255, 255, 0.6)` | 更浅的半透明 |
| 分割线 | `rgba(0, 0, 0, 0.06)` | 极淡的黑色 |

#### 排版

| 层级 | 字号 | 字重 | 行高 |
|------|------|------|------|
| 面板标题 | 18px | 600 | 28px |
| 模块标题 | 16px | 600 | 24px |
| 卡片标题 | 14px | 500 | 22px |
| 正文 | 13px | 400 | 20px |
| 辅助文字 | 12px | 400 | 18px |

#### 间距系统

- 基础单位: 4px
- 小间距: 8px
- 中间距: 12px
- 大间距: 16px
- 模块间距: 24px

#### 动效规范

| 动效类型 | 时长 | 缓动函数 | 用途 |
|----------|------|----------|------|
| 快速反馈 | 100ms | `ease-out` | 按钮按下、开关切换 |
| 标准过渡 | 200ms | `ease` | hover 效果、颜色变化 |
| 页面切换 | 250ms | `ease-in-out` | 模块切换淡入淡出 |
| 卡片展开 | 300ms | `cubic-bezier(0.4, 0, 0.2, 1)` | 对话框弹出 |

### 7.2 交互状态

#### 按钮

```
正常态:  bg=#6366F1  text=white  shadow=0 1px 2px rgba(0,0,0,0.05)
Hover:   bg=#818CF8  shadow=0 2px 4px rgba(99,102,241,0.2)  transform=translateY(-1px)
Active:  bg=#4F46E5  transform=translateY(0)  shadow=none
Disabled: bg=#E5E7EB  text=#9CA3AF  cursor=not-allowed
```

#### 卡片

```
正常态:  bg=rgba(255,255,255,0.6)  border=1px solid rgba(0,0,0,0.04)
Hover:   bg=rgba(255,255,255,0.8)  border=1px solid rgba(99,102,241,0.15)
         shadow=0 4px 12px rgba(0,0,0,0.06)
```

#### 导航项

```
正常态:  bg=transparent  text=#6B7280
Hover:   bg=rgba(99,102,241,0.08)  text=#1F2937
Active:  bg=rgba(99,102,241,0.12)  text=#6366F1
         border-left=3px solid #6366F1
```

### 7.3 用户流程

#### 流程 1：打开设置面板

```
用户右键桌面宠物 / 点击 HoverMenu 设置图标
  → invoke("open_settings_window")
  → Tauri 创建/显示设置窗口
  → 前端路由到 /settings
  → SettingsPanel 挂载
  → 从 SQLite 加载所有配置数据
  → 显示默认模块（Jira）
```

#### 流程 2：切换模块

```
用户点击左侧导航项
  → activeModule 更新
  → 内容区淡出（250ms）
  → 新模块内容淡入（250ms）
  → 导航项高亮切换
```

#### 流程 3：切换模型

```
用户点击模型卡片「▶ 使用此模型」
  → 调用 PetStore.setActiveModel(modelId)
  → 当前活跃模型标记更新
  → AvatarFactory 创建新 Avatar 实例
  → 当前渲染器销毁，新渲染器初始化
  → 桌面宠物切换为新模型
  → 卡片按钮变为「✓ 当前模型」
```

---

## 8. 实现计划 (Implementation Plan)

### 阶段一：基础框架（预计 3 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 创建 Tauri 设置窗口（Rust 端）| 无 | 窗口管理代码 |
| 前端路由配置 `/settings` | 无 | 路由 |
| `SettingsPanel.vue` 根组件 | 路由 | 面板骨架 |
| `SettingsTitleBar.vue` 自定义标题栏 | 面板 | 可拖拽标题栏 + 关闭按钮 |
| `SettingsSidebar.vue` 导航栏 | 面板 | 左侧导航 + 模块切换 |
| 全局样式变量与毛玻璃效果 | 面板 | CSS tokens |

**验证点**：能从 HoverMenu 打开设置窗口，左侧导航可切换模块。

### 阶段二：Jira + Email 模块（预计 3 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 数据库迁移：jira_connections + email_accounts 表 | 无 | migration SQL |
| Mock 数据初始化逻辑 | 表结构 | 启动时插入 Mock 数据 |
| `ConnectionCard.vue` 通用卡片组件 | 阶段一 | 可复用卡片 |
| `ToggleSwitch.vue` 开关组件 | 阶段一 | 开关组件 |
| `JiraModule.vue` 实现 | 卡片 + 开关 | Jira 模块 UI |
| `EmailModule.vue` 实现 | 卡片 + 开关 | Email 模块 UI |
| Tauri IPC 命令实现 | 数据库 | 增删改查接口 |

**验证点**：Mock 数据正确显示，开关/删除操作持久化到 SQLite。

### 阶段三：Chat + Model 模块（预计 3 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 数据库迁移：chat_platforms 表 | 无 | migration SQL |
| `ChatModule.vue` 实现 | 阶段一 | 聊天工具连接列表 UI |
| `ModelConfigModule.vue` 实现 | 阶段一 + ModelRegistry | 模型列表 UI |
| 模型切换功能 | PetStore | 实时切换渲染器 |
| `ConfirmDialog.vue` 确认对话框 | 阶段一 | 对话框组件 |

**验证点**：Chat 平台连接列表显示（WeChat/Slack/Teams/Discord），启用/禁用操作正常；模型列表中 Live2D 与 SpriteSheet 混合显示，切换功能正常。

### 阶段四：打磨（预计 2 天）

| 任务 | 依赖 | 产出 |
|------|------|------|
| 动效细节打磨 | 全部 | hover/transition/动画 |
| 响应式布局适配 | 全部 | 最小尺寸 560×480 下不溢出 |
| 暗色模式预留 | 样式 | CSS 变量支持未来扩展 |
| 错误处理与空状态 | 全部 | 网络错误/空列表提示 |

**总计：约 11 个工作日**

---

## 9. 风险与约束 (Risks & Constraints)

| # | 风险 | 影响 | 缓解措施 |
|---|------|------|----------|
| R1 | Tauri 多窗口内存开销 | 额外 ~30MB RAM | 设置窗口关闭后隐藏而非销毁，避免重复创建 |
| R2 | 半透明窗口在 Linux 下兼容性 | 毛玻璃效果可能失效 | CSS fallback 为纯色背景 |
| R3 | SQLite 并发写入冲突 | 多窗口同时操作数据库 | 使用写锁；前端防抖 |
| R4 | 模型切换时闪烁 | 用户体验差 | 新模型预加载完成后再销毁旧模型 |
| R5 | Mock 数据与真实数据迁移 | 后续需重构 | Mock 数据结构与真实表结构保持一致 |

### 约束

- **窗口管理**：使用 Tauri 原生多窗口 API
- **样式方案**：CSS Variables + Scoped CSS（不使用 CSS-in-JS）
- **状态管理**：与项目现有方案一致（Pinia 或直接 reactive）
- **数据库**：使用现有 `src/core/storage/index.ts` 中的 SQLite 封装
- **组件规范**：Vue 3 Composition API + `<script setup>` 语法
