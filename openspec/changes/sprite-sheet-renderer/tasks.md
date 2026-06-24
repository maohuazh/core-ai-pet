## 1. 基础设施与类型定义

- [x] 1.1 创建 `src/core/sprite/types.ts`，定义 `SpriteSheetManifest`、`ManifestMeta`、`SpritesheetConfig`、`DirectionsConfig`、`StateDefinition`、`MotionDefinition`、`ExpressionDefinition`、`FrameInfo`、`SourceRect` 等 TypeScript 接口
- [x] 1.2 创建 `src/core/sprite/index.ts`，导出所有 sprite 模块的类型和类
- [x] 1.3 创建 `src/core/renderer/sprite/index.ts`，导出 SpriteSheetRenderer

## 2. manifest.json 解析与校验

- [x] 2.1 安装 zod 依赖：`npm install zod`
- [x] 2.2 创建 `src/core/sprite/manifestParser.ts`，使用 Zod 定义 manifest schema 并实现 `parseManifest(raw: unknown): SpriteSheetManifest` 函数
- [x] 2.3 编写 manifestParser 单元测试：覆盖有效 manifest、缺失字段、非法数值、无方向精灵、无表情精灵等场景

## 3. 方向映射

- [x] 3.1 创建 `src/core/sprite/directionMapper.ts`，实现 `degreeToDirection(degree: number): Direction` 函数，将 0-360° 映射到 8 方向
- [x] 3.2 编写 directionMapper 单元测试：覆盖 8 个扇区边界值（0°, 45°, 90°, ...）与负角度

## 4. 帧动画引擎

- [x] 4.1 创建 `src/core/sprite/animationEngine.ts`，实现 `SpriteSheetAnimationEngine` 类，包含 `update(deltaMs)`、`setState(name)`、`setDirection(dir)`、`computeSourceRect()` 方法
- [x] 4.2 实现帧推进逻辑：`elapsed` 累加、`frameInterval` 判断、循环回绕与单次播放完成回调
- [x] 4.3 实现方向切换：保持 currentFrame 不变，仅更新行偏移
- [x] 4.4 实现源图裁剪坐标计算：支持有方向/无方向两种模式，支持 padding
- [x] 4.5 编写 animationEngine 单元测试：帧推进、状态切换重置、循环/单次播放、方向切换、裁剪坐标计算

## 5. SpriteSheetRenderer 渲染器

- [x] 5.1 创建 `src/core/renderer/sprite/SpriteSheetRenderer.ts`，实现 `IRenderer` 接口
- [x] 5.2 实现 `init()` 方法：创建 Canvas 元素，附加到 DOM 容器，启动 requestAnimationFrame 循环
- [x] 5.3 实现 `render()` 方法：从引擎获取 FrameInfo，使用 drawImage 裁剪绘制精灵帧
- [x] 5.4 实现表情覆盖层绘制：当 FrameInfo 含 expressionOverlay 时叠加绘制表情 PNG
- [x] 5.5 实现 `resize()`、`destroy()` 方法
- [x] 5.6 实现调试覆盖层（可选）：显示帧编号、状态、方向、FPS

## 6. SpriteSheetAvatar 适配器

- [x] 6.1 创建 `src/core/avatar/SpriteSheetAvatar.ts`，实现 `IAvatar` 接口
- [x] 6.2 实现 `load(config)` 方法：解析 manifest.json，预加载精灵表图片
- [x] 6.3 实现 `playMotion()` 方法：从 manifest.motions 查找对应状态，调用 engine.setState()
- [x] 6.4 实现 `setExpression()` 方法：加载表情 PNG 并设置到引擎
- [x] 6.5 实现 `setDirection()` 方法：调用 degreeToDirection 映射后设置引擎方向
- [x] 6.6 实现 `setState()` 方法：将 PetState 映射为字符串后调用引擎
- [x] 6.7 实现 `dispose()` 方法：释放图片内存，销毁引擎

## 7. AvatarFactory 集成

- [x] 7.1 修改 `src/core/avatar/types.ts`，在 AvatarType 中添加 `"sprite"` 类型
- [x] 7.2 修改 `src/core/avatar/factory.ts`，在 createAvatar 中添加 `"sprite"` 分支，返回 SpriteSheetAvatar 实例
- [x] 7.3 更新 AvatarFactory 相关测试

## 8. ModelRegistry 扩展

- [x] 8.1 修改 `src/core/model/ModelRegistry.ts`，在 ModelInfo 接口中添加 `type` 和 `manifestPath` 字段
- [x] 8.2 注册一个示例 Sprite 模型（PixelCat）到 ModelRegistry
- [x] 8.3 更新 ModelRegistry 相关测试

## 9. 数据库 Migration

- [x] 9.1 修改 `src-tauri/src/infrastructure/storage/mod.rs`，在初始化时执行 migration：`ALTER TABLE models ADD COLUMN type TEXT NOT NULL DEFAULT 'live2d'`
- [x] 9.2 添加 `manifest_path` 列：`ALTER TABLE models ADD COLUMN manifest_path TEXT`
- [x] 9.3 更新 Rust 侧的模型查询 SQL，包含新字段（已有 type 和 manifest_path）
- [x] 9.4 验证 migration 向后兼容：已有 Live2D 模型记录的 type 自动为 'live2d'

## 10. 端到端集成测试

- [x] 10.1 创建示例精灵表图片（512x1024, 8x32 frames of 64x64 pixel cats）
- [x] 10.2 创建示例 manifest.json（含 8 方向、8 状态、4 表情）
- [x] 10.3 编写端到端测试：从 manifest 解析 → 创建 Avatar → 创建 Renderer → 屏幕显示动画
- [x] 10.4 测试状态切换：Idle → Walking → Sleeping → Alert，验证动画切换正确
- [x] 10.5 测试方向切换：8 个方向各切换一次，验证帧裁剪正确
- [x] 10.6 测试模型切换：Live2D → Sprite → Live2D，验证渲染器正确切换

## 11. 打磨与错误处理

- [x] 11.1 实现精灵表图片加载失败的降级处理（显示占位图 + 错误日志）
- [x] 11.2 实现 manifest.json 解析失败的用户提示（设置面板中显示错误详情）
- [x] 11.3 实现表情覆盖层加载失败的降级（忽略表情层，继续渲染主帧）
- [x] 11.4 添加性能监控日志：帧率、精灵表尺寸、内存占用
