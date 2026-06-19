## Context

本项目是桌面宠物的首个实现阶段。基于已完成的架构设计文档（04-方案四-Tauri+Live2D-推荐方案.md），需要搭建基础框架：

- **技术栈**: Tauri 2.x + Vue 3 + TypeScript + PixiJS + pixi-live2d-display
- **目标功能**: 透明窗口中的 Live2D 宠物渲染 + 鼠标悬停交互菜单
- **约束**: 
  - 必须使用 Cubism 5 SDK（需要 Live2D 账号授权）
  - 需要支持 Windows 10/11 平台
  - 窗口必须始终置顶、透明背景、支持拖拽

**利益相关者**: 最终用户（桌面宠物使用者）、开发团队

## Goals / Non-Goals

**Goals:**
- 实现透明无边框窗口，支持始终置顶和鼠标穿透
- 加载并渲染 Live2D 模型，支持基础动画播放
- 实现宠物窗口拖拽功能（仅在模型不透明区域）
- 实现鼠标悬停菜单，显示 5 个操作按钮（聊天、设置、菜单、最小化、关闭）
- 建立完整的 Tauri + Vue 3 项目结构和构建流程

**Non-Goals:**
- 不包含 AI 对话功能（后续阶段）
- 不包含行为状态机（后续阶段）
- 不包含 TTS/语音合成（后续阶段）
- 不包含多宠物支持（仅单一模型）
- 不包含数据持久化（后续阶段）

## Decisions

### 1. 窗口管理方案

**决策**: 使用 Tauri 原生窗口 API + CSS pointer-events 控制鼠标穿透

**理由**: 
- Tauri 提供跨平台的窗口控制 API（decorations: false, transparent: true, alwaysOnTop: true）
- CSS pointer-events: none 实现窗口区域的鼠标穿透
- 通过 JavaScript 动态切换 pointer-events 值，实现模型区域的鼠标交互

**备选方案**:
- 方案 A: 使用 Rust 侧的鼠标事件监听 + SetWindowLongPtr 修改窗口样式 → 过于复杂，跨平台兼容性差
- 方案 B: 使用 Electron → 包体积过大（150MB vs 15MB），内存占用高（200MB vs 80MB）

### 2. Live2D 渲染方案

**决策**: 使用 pixi-live2d-display + PixiJS 7.x

**理由**:
- pixi-live2d-display 是成熟的 Live2D Web 渲染库，支持 Cubism 4/5
- 与 PixiJS 深度集成，性能好，社区活跃
- 提供完整的模型加载、动画控制、参数访问接口

**备选方案**:
- 方案 A: 使用 live2d.js 官方 Web SDK → 需要手动集成渲染循环，开发效率低
- 方案 B: 使用精灵图渲染 → 动画质量差，不支持复杂表情和物理效果

### 3. 拖拽实现方案

**决策**: 在 Vue 组件中监听 mousedown 事件，通过 Tauri invoke 调用 Rust 侧的 start_dragging API

**理由**:
- Tauri 提供 window.startDragging() API，可触发原生拖拽
- 通过检测鼠标事件的目标元素，仅在模型不透明区域触发拖拽
- 实现简单，跨平台兼容

**备选方案**:
- 方案 A: 使用 CSS -webkit-app-region: drag → 无法精确控制拖拽区域
- 方案 B: 在 Rust 侧监听全局鼠标事件 → 过于复杂，需要处理窗口焦点问题

### 4. 悬停菜单实现方案

**决策**: 使用 Vue 组件 + CSS 动画，鼠标进入宠物区域时显示径向布局的 5 个按钮

**理由**:
- 纯前端实现，无需 Tauri 调用
- CSS transition 和 transform 实现流畅的 popIn 动画
- 径向布局（圆形排列）视觉效果好，符合设计文档要求
- 每个按钮带 hover tooltip，用户体验友好

**备选方案**:
- 方案 A: 使用 Tauri 创建独立的弹出窗口 → 过于复杂，需要处理窗口对齐和焦点问题
- 方案 B: 使用固定位置的侧边栏菜单 → 占用空间大，不够灵活

### 5. 项目结构方案

**决策**: 采用分层架构
- **Rust 后端**: Commands → Services → Core → Infrastructure
- **Vue 前端**: Views → Components → Modules → Core → Services

**理由**:
- 与架构设计文档保持一致
- 职责清晰，便于维护和扩展
- 支持后续功能（AI、行为系统、TTS）的模块化集成

## Risks / Trade-offs

**[风险] Live2D 模型版权**: 需要 Live2D Cubism SDK 授权 → **缓解**: 使用官方提供的示例模型，或在生产环境中要求用户提供自己的模型

**[风险] 性能问题**: PixiJS + Live2D 渲染可能占用较多 GPU 资源 → **缓解**: 使用 requestAnimationFrame 控制渲染频率，闲置时降低帧率

**[风险] 鼠标穿透精度**: CSS pointer-events 可能无法精确控制透明区域的穿透 → **缓解**: 在 Live2D 模型周围设置一个小的交互区域，而非完全透明的窗口

**[权衡] 首屏加载时间**: 加载 Live2D 模型需要一定时间 → **接受**: 显示加载进度提示，优先保证功能完整性

**[权衡] 窗口拖拽体验**: 仅在模型不透明区域可拖拽可能不够直观 → **缓解**: 在悬停菜单中提供"移动"按钮，允许用户进入拖拽模式
