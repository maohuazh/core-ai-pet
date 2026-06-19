## Why

当前 Live2D 桌宠渲染系统的模型 URL 硬编码在 `Live2DCanvas.vue` 中（CDN Haru 模型），无法切换或扩展。项目 `public/models/` 目录已有 Hiyori、Mao、Natori 三个本地 Live2D 模型，但代码无法引用它们。需要重构为可配置的多模型架构，支持运行时切换宠物模型。

## What Changes

- 新增**模型注册表**（ModelRegistry），统一管理所有可用模型的元数据（URL、名称、动画组、表情列表）
- 将 Live2DCanvas 中硬编码的模型 URL 替换为从 ModelRegistry 读取的当前选中模型
- Live2DRenderer 新增通用 motion/expression 触发接口，支持按名称动态调用任意动画组和表情
- Live2DRenderer 在模型加载完成后自动解析模型能力（可用动画组、表情），暴露给上层
- PetHoverMenu 新增"切换模型"菜单项，循环或列表选择已注册模型
- 新增 PetStore（响应式状态管理），保存当前选中模型、模型列表，驱动 UI 与渲染器同步
- 切换模型时，旧模型从 stage 移除并销毁，新模型加载后自动播放默认 idle 动画

## Capabilities

### New Capabilities
- `model-registry`: 模型注册表 — 定义模型元数据结构，管理已注册模型的增删查
- `model-rendering`: 模型渲染增强 — Renderer 通用 motion/expression 接口，模型能力自动发现
- `model-switching`: 模型切换 — UI 入口 + 状态管理 + 运行时热切换逻辑

### Modified Capabilities

（无已有 spec 需要修改）

## Impact

- **前端组件**: `Live2DCanvas.vue`（移除硬编码 URL）、`PetHoverMenu.vue`（新增切换菜单项）、`App.vue`（集成 PetStore）
- **核心模块**: `Live2DRenderer.ts`（新增 motion/expression 接口、模型能力解析）
- **新增模块**: `src/core/model/ModelRegistry.ts`、`src/core/model/PetStore.ts`
- **数据**: `public/models/` 下已有的 Hiyori、Mao、Natori、shizuku 模型将被注册
- **依赖**: 无新增外部依赖，使用现有 pixi-live2d-display + Vue 响应式系统
- **API**: 无外部 API 变更，纯内部重构
