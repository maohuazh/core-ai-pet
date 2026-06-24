## Context

Tauri 2 + Vue 3 + PixiJS v7 桌面宠物应用。当前存在两个阻塞性 UI 问题：

1. **Live2D 模型不可见**：窗口/容器区域已显示（用户能看到宠物区域），但 Live2D 模型本身未渲染。PixiJS Application 硬编码为 240x240，而 Tauri 主窗口为 200x200，存在尺寸不匹配。
2. **设置窗口 X 按钮无效**：`SettingsTitleBar.vue` 的关闭按钮调用 `appWindow.hide()`，Tauri 后端 `settings.rs` 的 `CloseRequested` 事件处理器调用 `prevent_close()` + `hide()`。用户反馈点击 X 按钮无法关闭/隐藏窗口。

相关代码文件：
- `src/core/renderer/live2d/Live2DRenderer.ts` — PixiJS 渲染器（硬编码 240x240）
- `src/components/Live2DCanvas.vue` — Canvas 组件（CSS 100%×100%）
- `src/components/settings/SettingsTitleBar.vue` — 设置窗口标题栏
- `src-tauri/src/commands/settings.rs` — 设置窗口后端逻辑
- `src-tauri/tauri.conf.json` — 主窗口配置（200x200）

## Goals / Non-Goals

**Goals:**
- Live2D 模型在 200x200 窗口中正确可见（模型居中、正确缩放）
- 设置窗口 X 按钮能正确关闭/隐藏窗口
- 修复不引入新功能，仅修复 bug

**Non-Goals:**
- 不修改模型加载逻辑或模型文件
- 不修改窗口透明度/点击穿透等已有行为
- 不调整设置窗口的 hide-vs-close 策略（保持 hide 行为，但确保其生效）

## Decisions

### Decision 1: Live2D 渲染器尺寸 — 动态匹配窗口尺寸

**选择**: 将 PixiJS Application 尺寸从硬编码 240x240 改为动态读取 canvas 元素实际尺寸。

**理由**:
- 主窗口为 200x200（tauri.conf.json），Canvas CSS 为 100%×100%，因此 canvas 实际像素应为 200x200
- 硬编码 240x240 导致渲染区域大于窗口，模型可能被裁剪或定位偏移
- 动态读取可适配未来窗口尺寸变化

**替代方案**:
- 改 tauri.conf.json 窗口为 240x240 → 但需同步修改 CSS、hover 菜单位置等，改动范围大
- 使用 CSS `transform: scale()` 缩放 canvas → 模糊且不可靠

**具体做法**:
1. 在 `Live2DCanvas.vue` 的 `onMounted` 中，在调用 `renderer.init()` 之前，读取 `canvasEl.value.clientWidth` 和 `canvasEl.value.clientHeight`
2. 将这两个值传给 `Live2DRenderer.init(width, height)`
3. `Live2DRenderer.init()` 使用传入的尺寸初始化 PixiJS Application
4. 同步更新 spec 中"240x240 渲染区域"的约束

### Decision 2: 设置窗口关闭 — 保持 hide 行为，修复实现

**选择**: 保持 `hide()` 语义（窗口隐藏而非销毁），但修复使其正确生效。

**理由**:
- 现有 spec 明确定义：设置窗口关闭时隐藏而非销毁，保留状态
- 问题很可能是 Tauri 2 API 路径问题或 close 事件处理冲突

**具体做法**:
1. 检查 `@tauri-apps/api/window` 的 `getCurrentWindow()` 在 Tauri 2 中是否返回正确的窗口引用
2. 前端：确保 `close()` 函数正确调用 `hide()`，如有必要改用 Tauri 2 正确的 API（可能需要 `@tauri-apps/api/webviewWindow`）
3. 后端：简化 `CloseRequested` 处理器，确保 `prevent_close()` + `hide()` 逻辑正确

## Risks / Trade-offs

- **[Risk] Tauri 2 API 变化** → 检查 `@tauri-apps/api` 版本和实际可用方法，用 `getCurrentWindow()` 确认
- **[Risk] Canvas 尺寸在 mount 时可能为 0** → 使用 `nextTick` 或 `requestAnimationFrame` 确保 DOM 布局完成
- **[Trade-off] 动态尺寸 vs 固定尺寸** → 动态更灵活但增加复杂度；对于固定 200x200 窗口，也可以用常量 200。选择动态以适配未来变化。
