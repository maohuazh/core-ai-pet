## ADDED Requirements

### Requirement: Show hover menu on mouse enter
系统 SHALL 在鼠标进入宠物区域时显示悬浮操作菜单。

#### Scenario: Mouse enters pet area
- **WHEN** 鼠标光标进入 Live2D 模型所在的区域
- **THEN** 系统 SHALL 显示悬浮菜单
- **THEN** 菜单 SHALL 以径向布局显示在宠物周围
- **THEN** 菜单显示 SHALL 带有 popIn 动画效果（持续 0.3s）

#### Scenario: Menu contains 5 buttons
- **WHEN** 悬浮菜单显示
- **THEN** 菜单 SHALL 包含 5 个操作按钮
- **THEN** 按钮 SHALL 按圆形径向排列（聊天、设置、菜单、最小化、关闭）

### Requirement: Hover menu button design
悬浮菜单的每个按钮 SHALL 具有统一的视觉设计和交互反馈。

#### Scenario: Button appearance
- **WHEN** 悬浮菜单显示
- **THEN** 每个按钮 SHALL 为圆形（直径 40px）
- **THEN** 按钮 SHALL 带有图标（使用 emoji 或 SVG）
- **THEN** 按钮 SHALL 有半透明背景（rgba(0,0,0,0.6)）
- **THEN** 按钮 SHALL 有白色边框（1px solid white）

#### Scenario: Button hover effect
- **WHEN** 鼠标悬停在按钮上
- **THEN** 按钮 SHALL 放大至 1.2 倍（transform: scale(1.2)）
- **THEN** 按钮 SHALL 显示 tooltip 文字（如"聊天"、"设置"等）
- **THEN** 按钮背景 SHALL 变为高亮色（rgba(100,150,255,0.8)）

### Requirement: Hide hover menu on mouse leave
系统 SHALL 在鼠标离开宠物区域时隐藏悬浮菜单。

#### Scenario: Mouse leaves pet area
- **WHEN** 鼠标光标离开 Live2D 模型所在的区域
- **THEN** 系统 SHALL 隐藏悬浮菜单
- **THEN** 菜单隐藏 SHALL 带有淡出动画效果（持续 0.2s）

#### Scenario: Mouse moves to menu
- **WHEN** 鼠标从宠物区域移动到悬浮菜单上
- **THEN** 菜单 SHALL 保持显示（不隐藏）

### Requirement: Hover menu button actions
每个悬浮菜单按钮 SHALL 执行对应的操作。

#### Scenario: Click chat button
- **WHEN** 用户点击"聊天"按钮
- **THEN** 系统 SHALL 打开聊天窗口（当前阶段为占位提示）

#### Scenario: Click settings button
- **WHEN** 用户点击"设置"按钮
- **THEN** 系统 SHALL 打开设置窗口（当前阶段为占位提示）

#### Scenario: Click menu button
- **WHEN** 用户点击"菜单"按钮
- **THEN** 系统 SHALL 显示更多选项（当前阶段为占位提示）

#### Scenario: Click minimize button
- **WHEN** 用户点击"最小化"按钮
- **THEN** 系统 SHALL 隐藏宠物窗口（保留在系统托盘）

#### Scenario: Click close button
- **WHEN** 用户点击"关闭"按钮
- **THEN** 系统 SHALL 退出应用程序

### Requirement: Hover menu CSS animation
悬浮菜单 SHALL 使用 CSS 动画实现流畅的显示和隐藏效果。

#### Scenario: popIn animation on show
- **WHEN** 菜单从隐藏变为显示
- **THEN** 每个按钮 SHALL 按顺序执行 popIn 动画
- **THEN** 第一个按钮延迟 0s，后续每个按钮延迟 0.05s
- **THEN** 动画效果：从 scale(0) 放大到 scale(1)，带弹性效果

#### Scenario: FadeOut animation on hide
- **WHEN** 菜单从显示变为隐藏
- **THEN** 所有按钮 SHALL 同时执行淡出动画
- **THEN** 动画效果：opacity 从 1 变为 0，持续 0.2s
