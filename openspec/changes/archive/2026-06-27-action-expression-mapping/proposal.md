## Why

桌面宠物需要根据外部事件（Jira任务、邮件、聊天消息）做出恰当的视觉反馈，但目前缺少动作与表情映射配置系统。用户无法自定义宠物在不同业务场景下的表现，导致宠物缺乏智能化交互能力。

## What Changes

- 新增动作与表情映射配置系统，支持10种预定义触发场景（日常1-3、新消息、新任务、新邮件、任务进行中、任务完成、任务将到期、任务超时）
- 每个场景可独立配置Motion（动作）、Expression（表情）和Effect（特效）组合
- 从Live2D模型的.model3.json和SpriteSheet模型的manifest.json中提取可用动作/表情资源列表
- 实现12种系统预定义特效（闪光、爱心、汗滴等）
- 在设置面板模型列表中添加「⚙ 动作映射」入口按钮
- 新增动作映射配置面板，提供可视化配置界面
- 配置持久化到SQLite的model_action_mappings表
- 实现实时预览功能，修改配置后桌面宠物立即展示对应表现
- 运行时集成事件触发机制，根据外部事件查找映射配置并执行

## Capabilities

### New Capabilities

- `action-mapping-config`: 动作映射配置的数据模型、CRUD操作、前端服务和配置面板UI
- `motion-expression-extraction`: 从Live2D和SpriteSheet模型中提取可用动作/表情资源列表
- `effect-system`: 系统预定义特效列表和特效播放管理
- `trigger-handler`: 运行时事件触发机制，根据外部事件执行映射的动作/表情/特效

### Modified Capabilities

- `settings-model-config`: 在模型列表中添加「⚙ 动作映射」入口按钮，点击打开配置面板

## Impact

- **数据库**: 新增model_action_mappings表，包含trigger_key、motion_group、motion_name、expression_name、effect_name、effect_duration、effect_position、use_default等字段
- **Rust后端**: 新增get_action_mappings、save_action_mapping、delete_action_mapping等Tauri命令；新增Live2D/SpriteSheet资源提取函数
- **前端**: 新增ActionMappingPanel.vue配置面板、MappingRow.vue行组件、MotionSelector/ExpressionSelector/EffectSelector选择器组件；新增actionMappingService.ts服务层
- **事件系统**: 集成trigger-handler，监听external-event并映射到对应trigger_key
- **宠物窗口**: 支持实时预览，通过事件系统触发动作/表情/特效播放
- **依赖**: 需要Live2DRenderer和SpriteSheetRenderer提供getMotionGroups()和getExpressions()接口（已实现）
