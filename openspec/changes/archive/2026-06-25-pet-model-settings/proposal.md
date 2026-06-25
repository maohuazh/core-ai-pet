## Why

配置窗口中"模型"标签名不符合产品定位（桌面宠物应用），且宠物模型数据在前后端完全断开——前端 `ModelRegistry` 注册了 7 个宠物模型（Live2D + Sprite），但后端 SQLite `models` 表为空（无种子数据），导致设置窗口的宠物列表显示为空，切换模型也无法生效到桌面宠物窗口。需要统一数据源并打通跨窗口通信。

## What Changes

- 将设置窗口侧边栏的"模型"标签重命名为"宠物"
- 将 `ModelConfigModule` 标题从"模型配置"改为"宠物配置"
- 在 Rust 后端 `initialize_mock_data` 中插入所有已注册宠物模型的种子数据（haru、hiyori、mao、natori 为 Live2D；pixel-cat、arisa、panda 为 Sprite），使设置窗口能正常展示宠物列表
- 在 `set_active_model` 命令中发射 Tauri 事件通知桌面宠物窗口，宠物窗口监听事件后调用 `petStore.setCurrentModel()` 切换模型渲染
- 宠物窗口启动时从数据库读取当前 active 模型，同步到 PetStore（而非仅依赖 ModelRegistry 硬编码的默认值）

## Capabilities

### New Capabilities
- `pet-model-seeding`: 后端宠物模型种子数据初始化，确保 models 表包含所有内置宠物
- `pet-model-switching`: 跨窗口宠物模型切换——设置窗口切换后通过 Tauri 事件通知宠物窗口实时切换渲染

### Modified Capabilities
- `settings-model-config`: 标签名改为"宠物"；宠物列表从数据库正常加载并展示；切换操作触发跨窗口事件

## Impact

- **后端 (Rust)**: `src-tauri/src/infrastructure/storage/mod.rs` 添加宠物种子数据；`src-tauri/src/commands/settings.rs` 的 `set_active_model` 增加事件发射
- **前端 (Vue)**: `SettingsSidebar.vue` 标签文案；`ModelConfigModule.vue` 标题文案；`App.vue` 或 `Live2DCanvas.vue` 监听 Tauri 事件并更新 PetStore
- **数据流**: 新增 Tauri event `model-changed` 从设置窗口→宠物窗口；PetStore 启动时从 DB 同步 active 模型
- **DB schema**: 无变更，仅新增种子数据
