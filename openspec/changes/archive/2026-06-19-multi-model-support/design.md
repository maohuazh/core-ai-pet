## Context

当前 `core-ai-pet` 项目使用 PixiJS + pixi-live2d-display 渲染 Live2D 桌面宠物。模型加载逻辑硬编码在 `Live2DCanvas.vue` 中（CDN Haru 模型），`Live2DRenderer.ts` 作为唯一的渲染层封装了初始化和模型加载。

项目现状：
- `public/models/` 已有 4 个本地 Live2D 模型（Hiyori、Mao、Natori、shizuku）
- 渲染器在 `loadModel` 完成后用 `setTimeout` 硬编码启动 `motion("Idle", 0)`
- 没有模型元数据管理、没有切换 UI、没有事件驱动的动画/表情触发接口
- 菜单 `PetHoverMenu.vue` 只有 5 个固定功能项

约束：
- 保持现有 PixiJS + pixi-live2d-display 技术栈不变
- 保持 Vue 3 + Tauri 框架不变
- 不引入新的外部依赖（使用 Vue 原生响应式）

## Goals / Non-Goals

**Goals:**
- 支持运行时热切换 Live2D 宠物模型，无需重启应用
- 统一的模型注册表，支持声明式注册模型及其元数据
- Renderer 暴露通用的 motion/expression 触发接口
- 切换模型时自动发现模型能力（动画组、表情列表）
- 通过菜单 UI 触发模型切换

**Non-Goals:**
- 不支持 spritesheet 格式宠物（这是另一个渲染体系）
- 不做模型的在线下载/安装功能
- 不做用户自定义模型导入
- 不做模型编辑器或动画编辑器
- 不涉及 AI 对话功能的变更

## Decisions

### D1: 使用静态模型注册表而非动态扫描

**选择**: 在 `ModelRegistry.ts` 中用静态数组声明所有可用模型。

**理由**: 模型文件是随应用打包的本地资源，不需要运行时发现。静态声明简单、可预测、类型安全。

**替代方案**: 扫描 `public/models/` 目录自动发现模型 — 但 Vite 打包后无法做文件系统扫描，且缺少元数据（动画组、表情）。

### D2: 使用 Vue reactive 做状态管理而非引入 Pinia

**选择**: 用 `vue/reactivity` 的 `reactive()` + `ref()` 创建轻量的 `PetStore`。

**理由**: 当前应用状态简单（当前模型、模型列表），引入 Pinia 过重。Vue 原生响应式足以满足需求，零依赖。

### D3: 模型能力自动发现而非手动声明

**选择**: 模型加载后，通过 `model.internalModel.motionManager` 自动解析可用动画组和表情列表。

**理由**: 手动声明每个模型的动画组容易出错且维护成本高。pixi-live2d-display 已暴露了模型的内部结构，自动解析更可靠。

**替代方案**: 在注册表中手动列出每个模型的所有动画组和表情 — 维护成本高，新增模型时需要重复劳动。

### D4: Renderer 保持单例，模型热替换

**选择**: `Live2DRenderer` 实例不变，`loadModel` 时销毁旧模型、加载新模型到同一个 PixiJS stage。

**理由**: 重建 PixiJS Application 会导致 canvas 闪烁和性能开销。只替换 stage 上的模型对象更平滑。当前 `loadModel` 已有旧模型销毁逻辑，扩展即可。

### D5: 模型切换采用循环切换（ next model ）

**选择**: 菜单"切换模型"按钮每次点击切换到注册表中的下一个模型。

**理由**: 实现最简单，UI 只需一个按钮。模型数量少（3-4 个），循环切换够用。后续如需列表选择可轻松扩展。

## Risks / Trade-offs

- **[模型加载耗时]** → 大纹理模型（如 Mao 3MB 纹理）加载可能较慢。缓解：加载时显示简单的控制台日志，后续可加 loading 指示器。
- **[模型能力解析不稳定]** → pixi-live2d-display 内部 API 可能随版本变化。缓解：用 try-catch 包裹解析逻辑，失败时回退为空数组，不影响基本功能。
- **[Cubism 版本兼容]** → Hiyori/Mao/Natori 均为 Cubism 4，但 shizuku 可能是 Cubism 2。缓解：注册表中标注 Cubism 版本，加载前检查运行时可用性。
