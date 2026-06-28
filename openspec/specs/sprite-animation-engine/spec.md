# sprite-animation-engine Specification

## Purpose

提供逐帧精灵动画的调度引擎（`SpriteSheetAnimationEngine`），负责帧推进、状态切换、方向切换以及精灵表内裁剪坐标的计算，为 `SpriteSheetRenderer` 与 `SpriteSheetAvatar` 提供底层动画驱动。

## Requirements

### Requirement: 帧调度引擎核心循环
系统 SHALL 提供 `SpriteSheetAnimationEngine` 类，负责逐帧动画的调度与状态管理。

#### Scenario: 帧推进
- **WHEN** 调用 `engine.update(deltaMs)` 且 elapsed 累积超过 `frameInterval`（= 1000 / fps）
- **THEN** 引擎 SHALL 推进 `currentFrame` 到下一帧
- **THEN** 若 `loop: true`，帧到末尾后 MUST 回绕到起始帧
- **THEN** 若 `loop: false`，帧到末尾后 MUST 设置 `isComplete: true` 并触发 `onComplete` 回调

#### Scenario: 帧间隔计算
- **WHEN** 状态定义 `fps = 12`
- **THEN** `frameInterval` MUST 等于 `83.33ms`（1000 / 12）

#### Scenario: 状态切换重置帧
- **WHEN** 调用 `engine.setState(newStateName)`
- **THEN** `currentFrame` MUST 重置为 0
- **THEN** `elapsed` MUST 重置为 0
- **THEN** `frameInterval` MUST 更新为新状态的 fps

#### Scenario: 返回当前帧信息
- **WHEN** 调用 `engine.update(deltaMs)`
- **THEN** 引擎 SHALL 返回 `FrameInfo` 对象，含 `sourceRect`（裁剪坐标）、`displayWidth`、`displayHeight`

### Requirement: 方向切换
引擎 SHALL 支持 8 方向动画切换，方向变化时仅切换行偏移，不重置帧索引。

#### Scenario: 方向切换保持帧
- **WHEN** 当前帧为 `currentFrame = 2`，调用 `engine.setDirection("NE")`
- **THEN** `currentFrame` MUST 保持为 2
- **THEN** 源图裁剪坐标 MUST 根据新方向的行偏移重新计算

#### Scenario: 无效方向
- **WHEN** 调用 `engine.setDirection(invalid)` 且精灵表无 `directions` 配置
- **THEN** 引擎 MUST 静默失败（打印警告日志）

### Requirement: 源图裁剪坐标计算
引擎 SHALL 根据当前状态、方向、帧索引计算精灵表中的裁剪坐标。

#### Scenario: 无方向精灵裁剪
- **WHEN** `directions.enabled = false`，`absFrame = 5`，`columns = 8`
- **THEN** `col = absFrame % columns = 5`
- **THEN** `row = Math.floor(absFrame / columns) = 0`
- **THEN** `sx = col * (frameWidth + padding)`
- **THEN** `sy = row * (frameHeight + padding)`

#### Scenario: 有方向精灵裁剪
- **WHEN** `directions.enabled = true`，方向为 "S"（对应行 4），`absFrame = 3`，`columns = 8`
- **THEN** `col = absFrame % columns = 3`
- **THEN** `row = dirRow + Math.floor(absFrame / columns) = 4 + 0 = 4`
- **THEN** `sx = col * (frameWidth + padding)`
- **THEN** `sy = row * (frameHeight + padding)`

#### Scenario: 跨行帧
- **WHEN** `absFrame = 10`，`columns = 8`，方向行 = 0
- **THEN** `col = 10 % 8 = 2`
- **THEN** `row = 0 + Math.floor(10 / 8) = 1`

### Requirement: 动画完成回调
引擎 SHALL 在 `loop: false` 的状态播放完毕后触发回调。

#### Scenario: 单次播放完成
- **WHEN** 状态 `Alert` 定义为 `loop: false`，帧数 = 4
- **THEN** 播放完第 4 帧后，`isComplete` MUST 为 `true`
- **THEN** `onComplete` 回调 MUST 被调用

#### Scenario: 循环播放无完成
- **WHEN** 状态 `Idle` 定义为 `loop: true`
- **THEN** `isComplete` MUST 始终为 `false`
- **THEN** `onComplete` 回调 MUST 不被调用
