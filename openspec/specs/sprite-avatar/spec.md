# sprite-avatar Specification

## Purpose

提供 `SpriteSheetAvatar`，作为 `IAvatar` 接口的精灵表实现，将 PetState 与 Avatar API 映射到底层动画引擎（`SpriteSheetAnimationEngine`）与渲染器（`SpriteSheetRenderer`）。

## Requirements

### Requirement: SpriteSheetAvatar 实现 IAvatar 接口
系统 SHALL 提供 `SpriteSheetAvatar` 类，实现 `IAvatar` 接口的所有方法。

#### Scenario: 构造与初始化
- **WHEN** 创建 `SpriteSheetAvatar` 实例
- **THEN** 构造函数 MUST 接收 `SpriteSheetRenderer` 实例作为参数

#### Scenario: load 方法
- **WHEN** 调用 `avatar.load(config)`
- **THEN** 系统 SHALL 从 config 中读取 manifest.json 路径
- **THEN** 系统 SHALL 使用 `manifestParser` 解析并校验 manifest
- **THEN** 系统 SHALL 预加载精灵表图片（`spritesheet.image`）
- **THEN** 若图片加载失败，MUST 抛出 `Error`

#### Scenario: playMotion 方法
- **WHEN** 调用 `avatar.playMotion("walk")`
- **THEN** 系统 SHALL 从 manifest.motions 中查找对应状态（如 "Walking"）
- **THEN** 系统 SHALL 调用 `engine.setState("Walking")` 切换动画
- **THEN** 返回 Promise 在动画开始时 resolve

#### Scenario: playMotion 无效动作
- **WHEN** 调用 `avatar.playMotion("nonexistent")`
- **THEN** 系统 MUST 静默失败（打印警告日志）

#### Scenario: setExpression 方法
- **WHEN** 调用 `avatar.setExpression("happy")`
- **THEN** 系统 SHALL 从 manifest.expressions 中查找 "happy" 对应的覆盖层路径
- **THEN** 系统 SHALL 预加载表情 PNG 并设置为当前表情

#### Scenario: setExpression 无表情
- **WHEN** manifest 中 `expressions` 字段不存在
- **THEN** 系统 MUST 静默失败(打印警告日志)

#### Scenario: setDirection 方法
- **WHEN** 调用 `avatar.setDirection(90)`
- **THEN** 系统 SHALL 调用 `directionMapper.degreeToDirection(90)` 获取方向（如 "E"）
- **THEN** 系统 SHALL 调用 `engine.setDirection("E")` 切换方向

#### Scenario: getState 方法
- **WHEN** 调用 `avatar.getState()`
- **THEN** 系统 SHALL 返回当前动画引擎的状态名称（如 "Idle"）

#### Scenario: dispose 方法
- **WHEN** 调用 `avatar.dispose()`
- **THEN** 系统 SHALL 释放精灵表图片与表情图片的内存
- **THEN** 系统 SHALL 销毁动画引擎实例

### Requirement: SpriteSheetAvatar 状态映射
Avatar SHALL 将 PetState 映射到 manifest 中定义的状态名称。

#### Scenario: 状态映射
- **WHEN** 调用 `avatar.setState(PetState.Walking)`
- **THEN** 系统 SHALL 将 PetState 转换为字符串（如 "Walking"）
- **THEN** 系统 SHALL 调用 `engine.setState("Walking")`

#### Scenario: 未知状态降级
- **WHEN** 调用 `avatar.setState(PetState.Alert)` 但 manifest 中无 "Alert" 状态
- **THEN** 系统 MUST 降级到 "Idle" 状态
- **THEN** 系统 MUST 打印警告日志
