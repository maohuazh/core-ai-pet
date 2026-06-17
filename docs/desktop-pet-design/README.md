# 桌面宠物架构方案设计文档

## 文档索引

| 文档 | 说明 |
|------|------|
| [方案对比](./00-方案对比.md) | 现有项目分析 + 方案对比矩阵 |
| [方案一：Electron + Live2D](./01-方案一-Electron+Live2D.md) | 快速开发方案 |
| [方案二：Tauri + 精灵图](./02-方案二-Tauri+精灵图.md) | 轻量级方案 |
| [方案三：PySide6 + Live2D](./03-方案三-PySide6+Live2D.md) | Python 原生方案 |
| [方案四：Tauri + Live2D](./04-方案四-Tauri+Live2D-推荐方案.md) | **推荐方案** |

---

## 项目背景

本文档基于对以下开源桌面宠物项目的分析：

| 项目 | 技术栈 | 特点 |
|------|--------|------|
| **BANDORI-PET-REV** | PySide6 + LuaJIT Live2D | 多进程、高性能、功能最全 |
| **Pet-GPT** | PyQt5 + GIF | 简单轻量、GIF 动画 |
| **chatgpt-desktopPet** | Electron + pixi-live2d | Live2D 渲染、壁纸模式 |
| **yuns-desktop-pet** | Electron + CSS 动画 | 多 AI 提供商、MCP 支持 |
| **awesome-codex-pet** | 精灵图集 + CSS | 标准化格式、社区画廊 |

---

## 方案选择建议

### 快速决策树

```
你的需求是什么？
│
├── 快速原型 / 团队熟悉 Web 技术
│   └── 方案一：Electron + Live2D
│
├── 极致性能 / 复杂多角色
│   └── 方案三：PySide6 + Live2D
│
├── 追求轻量 / 性能平衡
│   ├── 需要 Live2D 动画 → 方案四：Tauri + Live2D ⭐
│   └── 可用精灵图 → 方案二：Tauri + 精灵图
│
└── 不确定
    └── 方案四：Tauri + Live2D ⭐（最均衡）
```

### 方案对比一览

| 特性 | 方案一<br>Electron+Live2D | 方案二<br>Tauri+精灵图 | 方案三<br>PySide6+Live2D | 方案四<br>Tauri+Live2D |
|------|:-:|:-:|:-:|:-:|
| **包体积** | ~150MB | ~10MB | ~80MB | ~15MB |
| **内存占用** | ~200MB | ~50MB | ~150MB | ~80MB |
| **渲染质量** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **开发效率** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| **跨平台** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **安全性** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Live2D** | ✅ 原生 | ⚠️ 需适配 | ✅ 需适配 | ✅ 原生 |
| **精灵图** | ⚠️ 需实现 | ✅ 原生 | ⚠️ 需实现 | ✅ 支持 |
| **推荐指数** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |

---

## 核心架构要素

无论选择哪个方案，一个完整的桌面宠物系统都包含以下核心模块：

### 1. 窗口管理

```
┌─────────────────────────────┐
│  透明无边框窗口               │
│  - 透明背景                   │
│  - 始终置顶                   │
│  - 跳过任务栏                 │
│  - 鼠标穿透（透明区域）        │
│  - 可拖拽（不透明区域）        │
└─────────────────────────────┘
```

### 2. 渲染引擎

```
┌─────────────────────────────┐
│  渲染器接口                   │
│  ├── Live2D 渲染器            │
│  ├── 精灵图渲染器             │
│  └── 混合渲染器               │
└─────────────────────────────┘
```

### 3. 行为状态机

```
┌─────────────────────────────┐
│  状态机                       │
│  ├── 空闲 (Idle)             │
│  ├── 行走 (Walk)             │
│  ├── 说话 (Talk)             │
│  ├── 睡眠 (Sleep)            │
│  └── 反应 (React)            │
│                               │
│  触发器                       │
│  ├── 时间触发                 │
│  ├── 交互触发                 │
│  └── 系统事件触发             │
└─────────────────────────────┘
```

### 4. AI 集成

```
┌─────────────────────────────┐
│  AI 提供商                    │
│  ├── OpenAI                  │
│  ├── Claude                  │
│  ├── Gemini                  │
│  └── 自定义提供商             │
│                               │
│  功能                         │
│  ├── 流式聊天                 │
│  ├── 工具调用 (MCP)          │
│  ├── 视觉理解                 │
│  └── 语音合成 (TTS)          │
└─────────────────────────────┘
```

### 5. 数据持久化

```
┌─────────────────────────────┐
│  存储层                       │
│  ├── 配置存储                 │
│  ├── 聊天历史                 │
│  ├── 宠物状态                 │
│  └── 用户偏好                 │
└─────────────────────────────┘
```

---

## 通用扩展性设计

### 1. 宠物配置化

每个宠物可以通过配置文件定义：

```json
{
  "name": "my-pet",
  "renderer": "live2d",
  "modelPath": "models/my-pet/model.json",
  "behavior": {
    "states": { ... },
    "transitions": { ... },
    "schedules": { ... }
  },
  "personality": {
    "systemPrompt": "...",
    "greeting": "你好！"
  },
  "interaction": {
    "clickHead": { "motion": "tap_head", "message": "不要摸头！" }
  }
}
```

### 2. 插件系统

```typescript
// 宠物插件接口
interface PetPlugin {
  name: string
  version: string
  
  onInit(): void
  onDestroy(): void
  
  // 事件钩子
  onStateChange?(from: string, to: string): void
  onInteraction?(type: string, x: number, y: number): void
  onChat?(message: string): string | void
}
```

### 3. 主题系统

```typescript
interface Theme {
  name: string
  colors: {
    primary: string
    secondary: string
    background: string
  }
  fonts: {
    family: string
    size: number
  }
  animations: {
    speed: number
  }
}
```

---

## 开发路线图建议

### Phase 1: 基础框架 (1-2 周)
- [ ] 项目脚手架搭建
- [ ] 透明窗口实现
- [ ] 基础渲染引擎
- [ ] 简单拖拽交互

### Phase 2: 行为系统 (1-2 周)
- [ ] 状态机实现
- [ ] 基础状态（idle、walk）
- [ ] 时间触发器
- [ ] 交互触发器

### Phase 3: AI 集成 (2-3 周)
- [ ] AI 提供商接口
- [ ] 聊天窗口
- [ ] 流式响应
- [ ] 配置持久化

### Phase 4: 高级功能 (2-4 周)
- [ ] Live2D 口型同步
- [ ] MCP 工具调用
- [ ] TTS 语音合成
- [ ] 多角色支持

### Phase 5: 打磨优化 (1-2 周)
- [ ] 性能优化
- [ ] 打包发布
- [ ] 自动更新
- [ ] 文档完善

---

## 技术资源

### Live2D
- [Live2D 官方文档](https://docs.live2d.com/en/)
- [pixi-live2d-display](https://github.com/guansss/pixi-live2d-display)
- [免费 Live2D 模型](https://www.live2d.com/en/learn/sample/)

### Tauri
- [Tauri 官方文档](https://tauri.app/)
- [Tauri API](https://tauri.app/v1/api/js/)

### Electron
- [Electron 官方文档](https://www.electronjs.org/docs)

### PySide6
- [PySide6 文档](https://doc.qt.io/qtforpython/)
- [PyQt-Fluent-Widgets](https://github.com/zhiyiYo/PyQt-Fluent-Widgets)

---

## 许可证

本文档及方案设计遵循 MIT 许可证。
