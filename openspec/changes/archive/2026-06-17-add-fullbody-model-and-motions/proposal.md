## Why

当前 Live2D 模型加载后只播放一个固定的 idle 动作，无法根据角色状态（Idle、Happy、Thinking、Talking）切换动画。原生层 `SetMotion()` 函数是 TODO 未实现，C# 端的 `CharacterController` 虽然已在状态变化时调用 `SetMotion()`，但实际没有任何效果。需要完成动作系统的端到端打通，让角色在不同状态下展现不同的全身动画。

## What Changes

- **实现原生层动作播放**：完成 `cubism_animation.cpp` 中 `SetMotion()` 的 TODO，建立 motion group+name 到已加载 `ACubismMotion*` 的映射，通过 `CubismMotionManager::StartMotion()` 触发动画播放
- **重构动作映射层**：修改 `StateAnimationMapper` 适配实际模型的 motion group 名称（Hiyori 只有 Idle 和 TapBody 两组），使用随机选择 + 优先级回退策略
- **动作分组索引**：在模型加载时建立 `group → [motion_index_list]` 的索引表，支持按组名快速查找和随机选择
- **保持 Hiyori 作为默认模型**：Hiyori 是全身版角色（10 个动作），满足需求。无需替换模型
- **窗口尺寸适配**：当前 150×150 窗口显示全身像可能裁切，需调整投影矩阵确保全身可见（可能需要增加窗口高度或缩放模型）

## Capabilities

### New Capabilities
- `motion-playback`: 实现 Live2D 动作播放系统 — 包含动作索引构建、按组名查找、随机选择、淡入淡出切换

### Modified Capabilities
- `live2d-model-loading`: 模型加载时构建 motion group 索引表，供动作播放系统使用
- `model-display-resize`: 可能需要调整窗口尺寸或投影参数以适配全身像显示（当前 150×150 可能不足）

## Impact

- **原生端**：`cubism_animation.cpp`（实现 SetMotion TODO）、`cubism_model.cpp`（构建 group 索引）、`bridge_api.cpp`（传递索引数据）
- **C# 端**：`StateAnimationMapper.cs`（适配实际 motion group 名称）、可能 `DisplayConstants.cs`（窗口高度调整）
- **模型资源**：无变更，继续使用 Hiyori（10 个全身动作）
- **依赖**：无新增
