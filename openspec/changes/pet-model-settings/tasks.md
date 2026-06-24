## 1. 后端：宠物种子数据

- [x] 1.1 在 `src-tauri/src/infrastructure/storage/mod.rs` 的 `initialize_mock_data` 中添加 models 表的种子数据插入逻辑（检查 models 表是否为空，为空则插入 7 条宠物记录：haru/hiyori/mao/natori/pixel-cat/arisa/panda）
- [x] 1.2 添加 `get_active_model_id` 命令到 `src-tauri/src/commands/settings.rs`，查询 status='active' 的模型 ID 返回给前端
- [x] 1.3 在 `set_active_model` 命令中增加 Tauri event 发射：`app.emit("pet-model-changed", serde_json::json!({ "modelId": id }))`
- [x] 1.4 注册新命令 `get_active_model_id` 到 Tauri invoke handler

## 2. 前端：设置窗口标签重命名

- [x] 2.1 修改 `src/components/settings/SettingsSidebar.vue` 将 "模型" 标签改为 "宠物"
- [x] 2.2 修改 `src/components/settings/modules/ModelConfigModule.vue` 标题从 "模型配置" 改为 "宠物配置"
- [x] 2.3 修改 `src/components/settings/modules/ModelConfigModule.vue` 中的文案："当前模型"→"当前宠物"，"使用此模型"→"使用此宠物"，"暂无模型"→"暂无宠物"

## 3. 前端：宠物窗口启动同步 active 模型

- [x] 3.1 在 `src/App.vue` 的 `onMounted`（非 settings 路由）中添加：先 `listen("pet-model-changed")` 注册事件监听，再 `invoke("get_active_model_id")` 获取 active ID，通过 `modelRegistry.getById()` 查找配置并调用 `petStore.setCurrentModel()`

## 4. 验证

- [x] 4.1 验证设置窗口宠物列表正常显示所有 7 个内置宠物（4 Live2D + 3 Sprite）
- [x] 4.2 验证在设置窗口切换宠物后，桌面宠物窗口实时切换渲染
- [x] 4.3 验证重启应用后桌面宠物显示为上次选择的 active 宠物
