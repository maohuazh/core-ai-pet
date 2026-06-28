## ADDED Requirements

### Requirement: Chat window new sidebar layout
聊天窗口 SHALL 采用新的左右布局设计。

#### Scenario: Chat sidebar structure
- **WHEN** 聊天窗口渲染
- **THEN** 左侧 MUST 显示会话列表区域（宽度约 260px）
- **THEN** 会话列表顶部 MUST 有搜索框
- **THEN** 会话列表 MUST 有"新对话"按钮
- **THEN** 会话列表底部 MUST 有"清空记录"按钮

#### Scenario: Chat sidebar session items
- **WHEN** 会话列表渲染
- **THEN** 每个会话项 MUST 显示：会话标题、最后消息摘要（截断）、时间戳
- **THEN** 选中会话 MUST 有高亮背景
- **THEN** 会话 MUST 支持右键菜单（删除等）

#### Scenario: Chat message area preserved
- **WHEN** 用户选择会话
- **THEN** 右侧 MUST 显示消息区域，包含以下现有功能：
  - 消息列表（用户消息 + AI 回复）
  - 流式 LLM 响应（thinking blocks + tool calls）
  - Workspace 选择器
  - Git 分支显示
  - 底部输入框（Enter 发送、Shift+Enter 换行）
  - 宠物模型选择器
  - LLM 模型选择器

#### Scenario: Chat window existing sessions
- **WHEN** 聊天窗口打开
- **THEN** MUST 从 SQLite 加载已有会话
- **THEN** MUST 保持会话的创建、选择、删除功能
