# CoreAIpet — 项目级指南

## UI 规则（强制）

本项目所有窗口、弹窗、菜单、对话框必须遵循
[`docs/UI-style-rules.md`](./docs/UI-style-rules.md)。

- 基线风格：`src/modules/chat/ChatWindow.vue`（Catppuccin Mocha 深色）
- 公共浮层组件位于 `src/components/ui/`（`AppPopover` / `AppMenu` / `AppModal` / `AppTooltip`）
- **禁止**：`alert()` / `confirm()` / `prompt()` / 原生 `<select>` 下拉 / `backdrop-filter: blur()`
- 新增弹窗时在 PR 描述中粘贴 [UI-style-rules.md §3](./docs/UI-style-rules.md#3-强制审查清单写新弹窗时自查) 的自查清单

正在进行的重构详见 [`docs/UI-popup-redesign.md`](./docs/UI-popup-redesign.md)。
