## Why

CoreAIpet 当前仅支持 Live2D 渲染器，用户无法使用像素画、手绘逐帧动画等形式的桌面宠物。SpriteSheet 帧动画渲染器作为一种轻量级、易于制作的替代方案，可降低用户创作门槛，丰富宠物类型。

## What Changes

- 新增 `SpriteSheetRenderer`，基于 HTML Canvas 2D API 实现逐帧动画渲染
- 新增 `SpriteSheetAvatar`，实现现有 `IAvatar` 接口以支持 8 方向动画
- 定义 `manifest.json` 规范，描述精灵表帧布局、方向映射、状态配置
- 实现帧调度引擎，支持状态切换、循环播放、表情覆盖层
- 扩展 `AvatarFactory` 支持 `type: "sprite"` 创建 SpriteSheetAvatar
- 扩展 `ModelRegistry` 支持 sprite 类型模型注册与切换
- 数据库 `models` 表新增 `type` 和 `manifest_path` 字段

## Capabilities

### New Capabilities

- `sprite-manifest-spec`: 精灵表清单文件格式规范，定义 manifest.json 的字段、校验规则与示例
- `sprite-animation-engine`: 帧动画调度引擎，负责状态机、帧推进、方向切换、循环控制
- `sprite-avatar`: SpriteSheetAvatar 实现，适配 IAvatar 接口，封装精灵表加载与播放逻辑
- `sprite-renderer`: SpriteSheetRenderer 实现，基于 Canvas 2D 的逐帧渲染器
- `sprite-factory-integration`: AvatarFactory 与 ModelRegistry 集成，支持 sprite 类型分发与共存

### Modified Capabilities

- `avatar-abstraction`: IAvatar 接口需补充 `setDirection(degree: number)` 和 `setScale(scale: number)` 方法以支持 8 方向精灵
- `model-registry`: ModelRegistry 需支持 `type: "sprite" | "live2d"` 字段区分模型类型
- `model-rendering`: 渲染管线需支持 SpriteSheet 渲染器作为第二渲染后端

## Impact

- **新增文件**: `src/core/sprite/` 目录（types.ts, manifestParser.ts, animationEngine.ts, directionMapper.ts）、`src/core/avatar/SpriteSheetAvatar.ts`、`src/core/renderer/sprite/SpriteSheetRenderer.ts`
- **修改文件**: `src/core/avatar/factory.ts`、`src/core/model/ModelRegistry.ts`、`src/core/avatar/types.ts`
- **数据库变更**: `models` 表增加 `type` 和 `manifest_path` 列（需 migration）
- **依赖**: 无新增外部依赖（Canvas 2D 为浏览器原生 API）
- **向后兼容**: Live2D 渲染器不受影响，两种模型类型可共存
