## ADDED Requirements

### Requirement: Model list display
系统 SHALL 显示所有已注册的模型列表（Live2D + SpriteSheet）。

#### Scenario: Display all models
- **WHEN** 用户打开设置面板并切换到模型配置模块
- **THEN** 系统 MUST 显示所有已注册的模型
- **THEN** 每个模型 MUST 显示为卡片形式
- **THEN** 卡片 MUST 包含：模型名称、类型（Live2D/SpriteSheet）、来源（内置/自定义）、缩略图

#### Scenario: Active model indicator
- **WHEN** 模型为当前活跃模型
- **THEN** 卡片左侧 MUST 显示主色调指示条（#6366F1）
- **THEN** 卡片按钮 MUST 显示为"✓ 当前模型"（禁用状态）

#### Scenario: Inactive model
- **WHEN** 模型不是当前活跃模型
- **THEN** 卡片按钮 MUST 显示为"▶ 使用此模型"
- **THEN** 卡片 MUST 显示"⚙ 动作映射"和"🗑 删除"按钮

### Requirement: Model switching
用户 SHALL 能够切换当前使用的宠物模型。

#### Scenario: Switch to inactive model
- **WHEN** 用户点击非活跃模型的"▶ 使用此模型"按钮
- **THEN** 系统 MUST 将该模型设为当前活跃模型
- **THEN** 系统 MUST 更新 PetStore 状态
- **THEN** 桌面宠物 MUST 切换为新模型
- **THEN** 卡片按钮 MUST 更新为"✓ 当前模型"

#### Scenario: Switch model persistence
- **WHEN** 用户切换模型后重启应用
- **THEN** 系统 MUST 恢复上次选择的模型为活跃模型

### Requirement: Model action mapping entry
用户 SHALL 能够从模型卡片跳转到动作映射配置。

#### Scenario: Open action mapping
- **WHEN** 用户点击模型卡片的"⚙ 动作映射"按钮
- **THEN** 系统 MUST 打开动作映射配置面板
- **THEN** 面板 MUST 显示该模型的所有可配置动作项

### Requirement: Model import entry
用户 SHALL 能够从模型配置模块跳转到模型导入流程。

#### Scenario: Open model import
- **WHEN** 用户点击"+ 导入"按钮
- **THEN** 系统 MUST 打开文件选择对话框
- **THEN** 用户选择模型文件后 MUST 启动导入流程

### Requirement: Model deletion
用户 SHALL 能够删除自定义模型。

#### Scenario: Delete custom model
- **WHEN** 用户点击自定义模型的"🗑 删除"按钮
- **THEN** 系统 MUST 弹出确认对话框
- **THEN** 用户确认后 MUST 删除模型记录
- **THEN** 系统 MUST 删除模型文件（如果是自定义导入的）
- **THEN** 卡片 MUST 从列表中移除

#### Scenario: Cannot delete active model
- **WHEN** 模型为当前活跃模型
- **THEN** "🗑 删除"按钮 MUST 禁用或隐藏

#### Scenario: Cannot delete builtin model
- **WHEN** 模型来源为 "builtin"
- **THEN** "🗑 删除"按钮 MUST 禁用或隐藏
