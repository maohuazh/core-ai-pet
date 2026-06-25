## RENAMED Requirements

### Requirement: 宠物标签重命名
FROM: 设置窗口侧边栏标签 "模型"
TO: 设置窗口侧边栏标签 "宠物"

## MODIFIED Requirements

### Requirement: Model list display
系统 SHALL 显示所有已注册的宠物列表（Live2D + SpriteSheet），数据来源于 SQLite 数据库。

#### Scenario: Display all models
- **WHEN** 用户打开设置面板并切换到宠物配置模块
- **THEN** 系统 MUST 显示所有已注册的宠物
- **THEN** 每个宠物 MUST 显示为卡片形式
- **THEN** 卡片 MUST 包含：宠物名称、类型（Live2D/SpriteSheet）、来源（内置/自定义）、缩略图

#### Scenario: Active model indicator
- **WHEN** 宠物为当前活跃模型
- **THEN** 卡片左侧 MUST 显示主色调指示条（#6366F1）
- **THEN** 卡片按钮 MUST 显示为"✓ 当前宠物"（禁用状态）

#### Scenario: Inactive model
- **WHEN** 宠物不是当前活跃模型
- **THEN** 卡片按钮 MUST 显示为"▶ 使用此宠物"
- **THEN** 卡片 MUST 显示"⚙ 动作映射"和"🗑 删除"按钮

### Requirement: Model switching
用户 SHALL 能够切换当前使用的宠物模型，切换后桌面宠物窗口实时生效。

#### Scenario: Switch to inactive model
- **WHEN** 用户点击非活跃宠物的"▶ 使用此宠物"按钮
- **THEN** 系统 MUST 将该宠物设为当前活跃宠物
- **THEN** 系统 MUST 更新 PetStore 状态（通过 Tauri 事件跨窗口通知）
- **THEN** 桌面宠物 MUST 切换为新宠物
- **THEN** 卡片按钮 MUST 更新为"✓ 当前宠物"

#### Scenario: Switch model persistence
- **WHEN** 用户切换宠物后重启应用
- **THEN** 系统 MUST 恢复上次选择的宠物为活跃宠物
