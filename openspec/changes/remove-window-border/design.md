## Context

当前 Tauri 窗口已配置 `decorations: false` 和 `transparent: true`，PixiJS 设置了 `backgroundAlpha: 0`，CSS 设置了 `background: transparent`。但在 Windows 上 WebView2 可能仍显示微妙的窗口边框或默认背景色，导致宠物模型周围出现可见的矩形轮廓。

## Goals / Non-Goals

**Goals:**
- 确保窗口背景在所有平台上完全透明，无任何可见边框或底色
- 仅显示 Live2D 模型像素，其余区域完全不可见

**Non-Goals:**
- 不改变窗口尺寸或形状
- 不改变模型渲染逻辑
- 不实现不规则窗口形状（如仅围绕模型轮廓的窗口）

## Decisions

### D1: 在 Rust 后端显式设置窗口背景透明

**选择**: 在 `main.rs` 的 Tauri Builder setup 中，获取窗口句柄并调用 `set_background_color(Some(Color::rgba(0, 0, 0, 0)))` 强制透明。

**理由**: `tauri.conf.json` 中的 `transparent: true` 在某些平台（尤其 Windows WebView2）上可能不够充分，Rust 端显式设置可确保一致性。

### D2: CSS 层面添加防御性透明样式

**选择**: 在 `index.html`、`App.vue`、`Live2DCanvas.vue` 中显式添加 `border: none; outline: none; background: transparent` 等样式，覆盖任何默认浏览器样式。

**理由**: 防止 WebView 默认样式或用户代理样式引入可见边框。

## Risks / Trade-offs

- **[Windows 合成层限制]** → 某些 Windows 版本（尤其旧版）对透明窗口支持不完整。缓解：目标平台为 Windows 10/11，WCO 和 DWM 合成均支持透明。
- **[透明区域点击穿透已实现]** → 现有代码已处理 transparent area 的鼠标穿透，本次变更不会影响该行为。
