## Why

当前鼠标悬停菜单只显示一个 ❌ 关闭按钮，且位于模型中央遮挡宠物形象。需要替换为 6 个功能按钮（任务/消息/Jira/邮件/Agent/设置），以环形布局展示在模型周围，提供快捷入口。

## What Changes

- 重新设计 `PetHoverMenu.vue`，将单一关闭按钮替换为 6 个功能按钮
- 按钮以环形布局排列在模型周围（左 3 右 3 或全环形）
- 每个按钮有独立图标、标签和 action 标识
- 移除原有的 ❌ 关闭按钮（关闭功能可通过系统托盘或其他方式实现）
- `App.vue` 更新 `handleMenuAction` 处理新的 6 个动作

## Capabilities

### New Capabilities
- `hover-menu-layout`: 环形悬浮菜单布局 — 6 个功能按钮的定位、样式和交互

### Modified Capabilities

（无已有 spec 需要修改）

## Impact

- **前端组件**: `PetHoverMenu.vue`（完全重写菜单项和布局）、`App.vue`（新增动作处理）
- **无新增依赖**: 纯 CSS 布局变更
