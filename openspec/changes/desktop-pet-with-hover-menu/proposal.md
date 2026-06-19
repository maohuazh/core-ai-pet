## Why

桌面宠物项目已完成架构设计文档，现在需要进入实际实现阶段。第一步需要搭建基础框架：实现透明窗口中的 Live2D 宠物渲染，以及鼠标悬停时显示悬浮交互菜单。这是后续所有功能（AI 对话、行为系统、TTS 等）的基础，必须优先完成。

## What Changes

- 创建 Tauri 2.x + Vue 3 + TypeScript 项目脚手架
- 实现透明无边框窗口配置（始终置顶、鼠标穿透、跳过任务栏）
- 集成 pixi-live2d-display 渲染引擎，加载并显示 Live2D 模型
- 实现宠物拖拽功能（在模型不透明区域可拖动窗口）
- 实现鼠标悬停菜单（PetHoverMenu）：鼠标移到宠物上方时，显示 5 个悬浮按钮（聊天、设置、菜单、最小化、关闭）
- 悬停菜单采用径向布局，带 CSS popIn 动画和 hover tooltip

## Capabilities

### New Capabilities

- `transparent-window`: 透明窗口管理，包括窗口创建、拖拽、位置控制
- `live2d-renderer`: Live2D 模型渲染，使用 pixi-live2d-display 加载和显示模型
- `pet-drag`: 宠物窗口拖拽功能，在模型不透明区域可拖动
- `hover-menu`: 鼠标悬停交互菜单，显示悬浮操作按钮

### Modified Capabilities

（无 - 这是首个功能实现）

## Impact

- **项目结构**: 创建完整的 Tauri + Vue 3 项目结构，包括 Rust 后端和 Vue 前端
- **依赖**: 引入 Tauri 2.x、Vue 3、TypeScript、pixi.js、pixi-live2d-display、Cubism SDK
- **配置**: 需要配置 tauri.conf.json 的透明窗口、始终置顶、鼠标穿透等属性
- **构建**: 配置 vite.config.ts 和 Rust Cargo.toml 的依赖
- **API**: 定义 Tauri Commands 接口，供前端调用后端功能
