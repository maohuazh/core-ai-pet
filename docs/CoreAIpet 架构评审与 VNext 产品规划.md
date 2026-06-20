# CoreAIpet 架构评审与 VNext 产品规划

Version: 1.0
Author: ChatGPT Architecture Review
Date: 2026-06-20

---

# 1. 项目定位重新定义

当前项目定位：

```text
桌面宠物（Desktop Pet）
```

建议升级定位：

```text
AI Desktop Assistant Platform
（AI桌面工作助手平台）
```

桌宠只是表现层（Avatar Layer）。

真正的核心价值应该来自：

* AI Agent
* Work Memory
* Tool Calling
* 工作流自动化
* Jira/Slack/Calendar集成
* 多Agent协作

最终形态：

```text
AI Assistant Platform
        ↓
Avatar Layer
        ↓
Live2D / Pixel Agent
```

而不是：

```text
Live2D
    ↓
附带一些AI能力
```

---

# 2. 当前架构评估

## 技术选型

| 模块   | 技术         |
| ---- | ---------- |
| 桌面框架 | Tauri 2    |
| 后端   | Rust       |
| 前端   | Vue3       |
| 渲染   | PixiJS     |
| 角色   | Live2D     |
| 构建   | Vite       |
| 类型系统 | TypeScript |

总体评价：

⭐⭐⭐⭐⭐

属于目前桌面AI助手领域非常先进的技术栈。

---

# 3. 当前架构优点

## 3.1 Tauri替代Electron

优势：

* 内存占用低
* 启动速度快
* Rust安全
* 安装包更小

适合长期驻留桌面。

---

## 3.2 PixiJS + Live2D

优势：

* 动画性能优秀
* Live2D生态成熟
* 易于实现表情、动作、眼球跟踪

适合作为AI助手形象层。

---

## 3.3 模块划分较清晰

目前已经存在：

```text
core/
components/
services/
modules/
plugins/
```

具备进一步扩展能力。

---

# 4. 当前架构核心问题

## 问题1：系统是渲染驱动而非Agent驱动

当前：

```text
Vue
 ↓
PetStore
 ↓
Live2DRenderer
 ↓
PixiJS
```

这是：

```text
Avatar First
```

架构。

未来应该变成：

```text
Events
 ↓
Agent Runtime
 ↓
State Machine
 ↓
Avatar
```

即：

```text
Agent First
```

架构。

---

## 问题2：缺少Agent Runtime

当前没有：

```text
Agent
Planner
Tool Executor
Scheduler
```

未来：

* Jira Agent
* Slack Agent
* Email Agent
* Calendar Agent

都无法统一管理。

---

## 问题3：缺少Memory系统

目前状态仅包含：

```text
当前模型
```

未来需要：

```text
Conversation Memory
Work Memory
Long-Term Memory
Vector Memory
```

否则无法形成真正的个人AI助手。

---

## 问题4：缺少事件总线

未来大量事件来源：

```text
Slack消息
Jira变更
邮件到达
会议提醒
系统通知
```

如果继续使用Vue组件通信：

```text
复杂度会快速失控
```

必须引入统一EventBus。

---

## 问题5：状态机缺失

未来角色状态：

```text
Idle
Thinking
Talking
Working
Meeting
Sleeping
Alert
```

建议使用状态机统一管理。

---

## 问题6：插件系统只有目录没有运行时

当前：

```text
plugins/
    plugin.json
```

只是配置。

缺少：

```text
PluginManager
PluginRuntime
PluginLifecycle
PluginPermission
```

---

# 5. VNext架构设计

## 总体架构

```text
┌──────────────────────────────┐
│          Avatar Layer        │
│ Live2D / Pixel / Future 3D   │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│        State Machine         │
│ Idle / Talk / Think / Work   │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│        Agent Runtime         │
│ Planner / Tools / Memory     │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│          Event Bus           │
└──────────────┬───────────────┘
               │
 ┌─────────────┼──────────────┐
 ▼             ▼              ▼

Jira        Slack        Calendar

Email       MCP          System
```

---

# 6. 推荐新增目录

## Rust

```text
src-tauri/src/

core/
├ agent/
├ memory/
├ scheduler/
├ plugin/
├ eventbus/
└ workflow/
```

---

## Frontend

```text
src/

core/
├ avatar/
├ state/
├ events/
├ memory/
└ agent/
```

---

# 7. 数据存储设计

## SQLite

存储：

```text
用户配置
聊天记录
插件配置
Agent状态
窗口位置
```

---

## 向量存储

推荐：

LanceDB

存储：

```text
长期记忆
知识库
工作记忆
```

---

# 8. Avatar抽象层

新增：

```typescript
interface Avatar {
    speak()
    think()
    work()
    playMotion()
    playExpression()
}
```

实现：

```text
Live2DAvatar
PixelAvatar
ThreeDAvatar
```

未来可无缝切换表现形式。

---

# 9. EventBus设计

统一事件模型：

```typescript
interface Event {
    id:string
    type:string
    source:string
    payload:any
}
```

事件来源：

```text
JiraUpdated
EmailReceived
SlackMessage
MeetingStarted
TaskCompleted
```

统一进入Agent Runtime。

---

# 10. Plugin Framework设计

```typescript
interface Plugin {

    onLoad()

    onUnload()

    getTools()

    getEvents()

    handleEvent()
}
```

插件统一接入：

```text
Plugin Manager
        ↓
Event Bus
        ↓
Agent Runtime
```

---

# 11. 状态机设计

状态：

```text
Idle
Walking
Thinking
Talking
Working
Meeting
Sleeping
Alert
```

转换：

```text
EmailReceived
        ↓
Alert

UserChat
        ↓
Thinking

LLMResponse
        ↓
Talking

MeetingStart
        ↓
Meeting
```

统一驱动：

```text
Live2D Motion
Expression
Voice
```

---

# 12. 技术债优先级

P0

* 状态机
* SQLite
* 系统托盘
* 窗口位置持久化

P1

* EventBus
* PluginManager
* Agent Runtime

P2

* Chat
* Memory
* LLM Router

P3

* Jira Plugin
* Slack Plugin
* Calendar Plugin

P4

* Multi Agent
* Pixel Office

---

# 13. 最终愿景

目标不是：

```text
桌面宠物
```

而是：

```text
个人AI工作助理
```

最终能力：

* 管理邮件
* 管理Jira
* 管理会议
* 自动晨报
* 自动周报
* Work Memory
* 多Agent协作
* MCP工具生态

Live2D只是入口。

Agent Runtime才是产品核心。
