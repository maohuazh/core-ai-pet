## ADDED Requirements

### Requirement: 内置宠物种子数据
系统 SHALL 在数据库初始化时插入所有内置宠物模型记录，确保设置窗口能展示完整宠物列表。

#### Scenario: 首次启动插入种子数据
- **WHEN** 数据库 `models` 表为空
- **THEN** 系统 MUST 插入以下宠物记录：haru（Live2D, CDN）、hiyori（Live2D, builtin）、mao（Live2D, builtin）、natori（Live2D, builtin）、pixel-cat（Sprite, builtin）、arisa（Sprite, builtin）、panda（Sprite, builtin）
- **THEN** 每条记录 MUST 包含正确的 type、path、source、author、description 字段
- **THEN** haru 的 status MUST 为 'active'（默认宠物）
- **THEN** 其余宠物的 status MUST 为 'inactive'

#### Scenario: 种子数据与 ModelRegistry 一致
- **WHEN** 种子数据插入完成
- **THEN** DB 中每条记录的 id MUST 与前端 `ModelRegistry` 中注册的 id 完全一致
- **THEN** DB 中 type 字段 MUST 与 ModelRegistry 中的 type 一致（live2d 或 sprite）
- **THEN** DB 中 path 字段 MUST 与 ModelRegistry 中的 modelUrl 一致

#### Scenario: 非空表不重复插入
- **WHEN** 数据库 `models` 表已有记录
- **THEN** 系统 MUST NOT 重复插入种子数据
