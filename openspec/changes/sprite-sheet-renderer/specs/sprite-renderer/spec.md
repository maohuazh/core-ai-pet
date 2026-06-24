## ADDED Requirements

### Requirement: SpriteSheetRenderer 实现 IRenderer 接口
系统 SHALL 提供 `SpriteSheetRenderer` 类，基于 HTML Canvas 2D API 实现逐帧渲染。

#### Scenario: init 方法
- **WHEN** 调用 `renderer.init()`
- **THEN** 系统 SHALL 创建 HTML Canvas 元素
- **THEN** Canvas 尺寸 MUST 设置为 `frameWidth * scale` × `frameHeight * scale`
- **THEN** Canvas MUST 被附加到 DOM 容器

#### Scenario: render 方法主帧绘制
- **WHEN** 调用 `renderer.render()`
- **THEN** 系统 SHALL 从动画引擎获取 `FrameInfo`
- **THEN** 系统 SHALL 使用 `ctx.drawImage()` 从精灵表裁剪当前帧并绘制到 Canvas
- **THEN** 绘制坐标 MUST 为 `(0, 0)`，尺寸 MUST 为 `(displayWidth, displayHeight)`

#### Scenario: render 方法表情覆盖层
- **WHEN** `FrameInfo.expressionOverlay` 存在且表情图片已加载
- **THEN** 系统 SHALL 在主帧之上绘制表情 PNG
- **THEN** 表情绘制坐标 MUST 与主帧一致
- **THEN** 绘制后 MUST 重置 `globalCompositeOperation` 为 `"source-over"`

#### Scenario: render 方法无表情
- **WHEN** `FrameInfo.expressionOverlay` 为 null 或 undefined
- **THEN** 系统 SHALL 仅绘制主帧，不绘制表情层

#### Scenario: resize 方法
- **WHEN** 调用 `renderer.resize(width, height)`
- **THEN** Canvas 尺寸 MUST 更新为新尺寸
- **THEN** 系统 SHALL 重新计算缩放比例

#### Scenario: destroy 方法
- **WHEN** 调用 `renderer.destroy()`
- **THEN** 系统 SHALL 从 DOM 中移除 Canvas 元素
- **THEN** 系统 SHALL 释放精灵表与表情图片的引用

### Requirement: 渲染循环集成
Renderer SHALL 使用 `requestAnimationFrame` 驱动渲染循环。

#### Scenario: 渲染循环启动
- **WHEN** `renderer.init()` 完成
- **THEN** 系统 SHALL 启动 `requestAnimationFrame` 循环
- **THEN** 每帧 MUST 调用 `engine.update(deltaMs)` 获取帧信息
- **THEN** 每帧 MUST 调用 `render()` 绘制当前帧

#### Scenario: 渲染循环停止
- **WHEN** 调用 `renderer.destroy()`
- **THEN** 系统 SHALL 取消 `requestAnimationFrame` 回调

#### Scenario: 帧率控制
- **WHEN** `deltaMs` 小于 `frameInterval`
- **THEN** 引擎 MUST 不推进帧，返回当前帧信息
- **THEN** Canvas MUST 仍绘制当前帧（保持显示）

### Requirement: 性能优化
Renderer SHALL 支持基本的性能优化策略。

#### Scenario: 离屏 Canvas 缓存
- **WHEN** 精灵表图片加载完成
- **THEN** 系统 SHALL 将精灵表绘制到离屏 Canvas 以提升裁剪性能

#### Scenario: 调试覆盖层（开发模式）
- **WHEN** 开发模式下启用调试覆盖层
- **THEN** 系统 SHALL 在 Canvas 上显示当前帧编号、状态名称、方向、FPS
