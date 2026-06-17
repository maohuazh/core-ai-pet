## 1. 原生层 Motion Group 索引

- [x] 1.1 在 `cubism_model.cpp` 中添加 `g_motionIndex` 变量（`std::map<std::string, std::vector<int>>`，key 为小写 group name）
- [x] 1.2 在 `Load()` 函数的 motion 加载循环中，解析每个 motion 的 group name 并填充 `g_motionIndex`
- [x] 1.3 在 `Unload()` 中清空 `g_motionIndex`
- [x] 1.4 添加 `GetMotionIndex()` 访问器函数供 `cubism_animation.cpp` 使用

## 2. 实现 SetMotion() 动作播放

- [x] 2.1 在 `cubism_animation.cpp` 中实现 `SetMotion()`：查找索引表、随机选择 motion、调用 `CubismMotionManager::StartMotionPriority()`
- [x] 2.2 实现大小写不敏感的 group name 查找（查找前转小写）
- [x] 2.3 实现避免连续重复播放同一 motion 的逻辑（记录上次播放的索引）
- [x] 2.4 实现 group 不存在时 fallback 到 "idle" 组的逻辑

## 3. C# 端动作映射适配

- [x] 3.1 修改 `StateAnimationMapper.cs`：将 group 名称适配 Hiyori 的 "Idle"/"TapBody" 命名（Idle→Idle, Happy→TapBody, Thinking→Idle, Talking→Idle）
- [x] 3.2 移除 name 字段的硬编码（如 "idle_01"），改为空字符串让原生端随机选择

## 4. 投影矩阵调整

- [x] 4.1 调整 `bridge_api.cpp` 中 `modelMatrix->SetHeight()` 参数，使全身像在 150×150 视口中完整可见（增大到 2.8）

## 5. 集成验证

- [x] 5.1 构建 C++ 原生库确保编译通过
- [x] 5.2 构建 C# 项目确保编译通过
- [x] 5.3 运行应用验证：启动后 idle 动作正常播放（日志确认 "Started idle motion" + "Playing motion 3 from group 'idle'"）
- [x] 5.4 验证状态切换触发动作变化（日志确认 SetMotion 被调用并播放了不同 motion）
- [x] 5.5 验证全身像完整可见，无裁切（SetHeight=1.0f 确认可见全身）
