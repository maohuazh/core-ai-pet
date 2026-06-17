## Context

当前 Live2D 动作系统的完整链路已搭建但关键节点未实现：

- **C# 端**：`CharacterController.SetState()` → `StateAnimationMapper.GetAnimation()` → `Live2DRenderHost.SetMotion(group, name)` → `Bridge_SetMotionGroup()` ✓ 已连通
- **原生端**：`Bridge_SetMotionGroup()` → `Animation::SetMotion()` → **TODO: 未实现实际播放** ✗ 断点
- **模型加载**：所有 motion 文件在 `Load()` 时已加载到 `g_motions` 数组，但只有索引没有按 group 的查找表
- **模型资源**：Hiyori 有 10 个动作，分为 Idle（9 个）和 TapBody（1 个）两组

问题：`StateAnimationMapper` 期望的 motion group 名称（idle, happy, thinking, talking）与 Hiyori 实际的 group 名称（Idle, TapBody）不匹配。

## Goals / Non-Goals

**Goals:**
- 实现原生层 `SetMotion()` 的实际动作播放功能
- 建立 motion group 索引，支持按组名查找和随机选择
- 适配 Hiyori 的 motion group 名称，实现状态→动作的合理映射
- 确保动作切换时有淡入淡出效果（利用 CubismMotion 的 FadeIn/FadeOut）
- 全身像在 150×150 窗口中完整可见（可能需要调整投影）

**Non-Goals:**
- 不替换模型（继续使用 Hiyori）
- 不实现动作编辑器或自定义动作序列
- 不实现口型同步（LipSync）— 当前 Talking 状态只需触发对应动作
- 不添加新的 CharacterState 枚举值

## Decisions

### Decision 1: 在模型加载时构建 group 索引表

**选择**: 在 `cubism_model.cpp` 的 `Load()` 函数中，加载完所有 motion 后构建 `std::map<std::string, std::vector<int>>` 索引表，key 为 group name，value 为该组内 motion 在 `g_motions` 数组中的索引列表。

**理由**:
- 查找效率 O(1)，不需要每次 SetMotion 时遍历所有 motion
- 支持按组随机选择（从 vector 中随机取一个索引）
- 与现有 `g_motions` 数组兼容

**替代方案**:
- 每次 SetMotion 时遍历 `model3.json` 的 motion 列表重新查找 → 性能差，逻辑重复
- 用 `std::unordered_map<std::string, std::vector<ACubismMotion*>>` 直接存指针 → 需要处理所有权，不如存索引简单

### Decision 2: 实现 SetMotion() 通过索引表查找并触发播放

**选择**: `SetMotion(group, name)` 实现：
1. 在索引表中查找 group（不区分大小写）
2. 如果找到，从该组的 motion 列表中随机选一个（如果 name 为空）或按 name 匹配
3. 通过 `CubismMotionManager::StartMotion()` 触发播放，优先级设为 3

**理由**:
- 随机选择避免重复播放同一动作，增加自然感
- 优先级 3 允许被更高优先级的系统动作打断
- FadeIn/FadeOut 由 CubismMotionManager 自动处理

### Decision 3: StateAnimationMapper 适配 Hiyori 的 group 名称

**选择**: 修改映射表：
- Idle → "Idle" 组（随机 9 选 1）
- Happy → "TapBody" 组（唯一的 tap 反应动作）
- Thinking → "Idle" 组（不同随机 variant）
- Talking → "Idle" 组（配合 LipSync 参数组，但 LipSync 暂不实现）

**理由**:
- Hiyori 只有 Idle 和 TapBody 两组，需要创造性映射
- TapBody 作为 Happy 状态的反应动作很合适（身体被点击的反应 = 开心）
- Thinking 和 Talking 复用 Idle 组的不同 variant，通过随机选择避免单调

**替代方案**:
- 硬编码 motion 索引 → 脆弱，换模型就失效
- 在 model3.json 中添加自定义 group → 需要修改模型文件，不现实

### Decision 4: 投影矩阵调整确保全身可见

**选择**: 将 `modelMatrix->SetHeight(2.0f)` 改为根据视口宽高比动态计算，确保全身不被裁切。对于 150×150 正方形视口，可能需要 `SetHeight(2.8f)` 或更大值来缩小模型以显示全身。

**理由**:
- Hiyori 的全身像在高为 2.0 单位时可能只填满视口高度的 80%，头部可能被裁切
- 需要根据实际模型渲染效果微调
- 简单的做法是增大 height 值使模型在视口中缩小

## Risks / Trade-offs

**[Risk] Hiyori 的 TapBody 动作可能只适合"被点击"场景** → Mitigation: 作为 Happy 状态的临时映射是可接受的。后续添加更多模型/动作组时可改进。

**[Risk] 投影调整可能导致模型过小** → Mitigation: 通过运行时测试微调 SetHeight 参数值。如果 150×150 不够，可将窗口高度增加到 150×200。

**[Risk] Motion group 名称大小写不一致** → Mitigation: 索引表查找时统一转小写比较。Hiyori 用 "Idle"/"TapBody"，C# 端发送 "idle"/"happy" 等。

**[Trade-off] 随机选择 vs 顺序播放**: 随机选择更自然但可能连续重复。Mitigation: 可以记录上次播放的索引，避免连续重复。
