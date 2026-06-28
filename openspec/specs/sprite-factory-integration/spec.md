# sprite-factory-integration Specification

## Purpose

定义 `AvatarFactory`、`ModelRegistry`、数据库表结构以及模型切换流程在精灵表（sprite）类型加入后的扩展规则，确保 Live2D 与 Sprite 模型可以共存与无缝切换。

## Requirements

### Requirement: AvatarFactory 支持 sprite 类型
`AvatarFactory` SHALL 扩展以支持创建 SpriteSheetAvatar 实例。

#### Scenario: 创建 sprite 类型 Avatar
- **WHEN** 调用 `createAvatar("sprite", renderer)`
- **THEN** 系统 SHALL 返回 `SpriteSheetAvatar` 实例
- **THEN** SpriteSheetAvatar MUST 接收 `SpriteSheetRenderer` 实例

#### Scenario: 创建 live2d 类型 Avatar
- **WHEN** 调用 `createAvatar("live2d", renderer)`
- **THEN** 系统 SHALL 返回 `Live2DAvatar` 实例（行为不变）

#### Scenario: 创建未知类型 Avatar
- **WHEN** 调用 `createAvatar("unknown", renderer)`
- **THEN** 系统 SHALL 抛出 `Error`，消息为 `Unknown avatar type: unknown`

### Requirement: ModelRegistry 支持 sprite 类型
`ModelRegistry` SHALL 扩展 `ModelInfo` 接口以支持 sprite 类型模型。

#### Scenario: 注册 sprite 类型模型
- **WHEN** 调用 `registry.register({ type: "sprite", name: "PixelCat", manifestPath: "/models/pixel-cat/manifest.json", ... })`
- **THEN** 模型 MUST 被成功注册
- **THEN** `registry.getById(id)` MUST 返回包含 `type: "sprite"` 的模型信息

#### Scenario: 模型列表中混合类型
- **WHEN** 注册表中有 Live2D 和 Sprite 模型
- **THEN** `registry.getAll()` MUST 返回混合类型的模型数组
- **THEN** 每个模型 MUST 包含 `type` 字段标识其类型

### Requirement: 数据库 models 表扩展
`models` 表 SHALL 增加 `type` 和 `manifest_path` 字段以支持 SpriteSheet 模型。

#### Scenario: 表结构变更
- **WHEN** 应用启动时执行 migration
- **THEN** `models` 表 MUST 新增 `type` 列（TEXT, NOT NULL, DEFAULT 'live2d'）
- **THEN** `models` 表 MUST 新增 `manifest_path` 列（TEXT, 可选）

#### Scenario: 已有数据兼容
- **WHEN** migration 执行前已有 Live2D 模型记录
- **THEN** migration 后这些记录的 `type` MUST 自动设置为 'live2d'
- **THEN** 原有字段（`modelUrl` 等）MUST 保持不变

#### Scenario: 插入 sprite 模型
- **WHEN** 插入一条 sprite 类型模型记录
- **THEN** `type` MUST 为 'sprite'
- **THEN** `manifest_path` MUST 为有效的相对路径

### Requirement: 模型切换兼容
模型切换逻辑 SHALL 支持在 Live2D 和 Sprite 模型之间无缝切换。

#### Scenario: 从 Live2D 切换到 Sprite
- **WHEN** 当前模型为 Live2D，用户选择 Sprite 模型
- **THEN** 系统 SHALL 销毁 Live2DAvatar 与 Live2DRenderer
- **THEN** 系统 SHALL 创建 SpriteSheetAvatar 与 SpriteSheetRenderer
- **THEN** 新模型 MUST 正常加载并播放动画

#### Scenario: 从 Sprite 切换到 Live2D
- **WHEN** 当前模型为 Sprite，用户选择 Live2D 模型
- **THEN** 系统 SHALL 销毁 SpriteSheetAvatar 与 SpriteSheetRenderer
- **THEN** 系统 SHALL 创建 Live2DAvatar 与 Live2DRenderer
- **THEN** 新模型 MUST 正常加载并播放动画
