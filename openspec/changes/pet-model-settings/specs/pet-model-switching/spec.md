## ADDED Requirements

### Requirement: 跨窗口宠物模型切换
系统 SHALL 在设置窗口切换宠物模型后，通过 Tauri 事件通知宠物窗口实时切换渲染。

#### Scenario: 设置窗口切换宠物触发事件
- **WHEN** 用户在设置窗口点击"使用此模型"切换到另一个宠物
- **THEN** 系统 MUST 更新 DB 中该宠物 status 为 'active'，其余为 'inactive'
- **THEN** 系统 MUST 发射 Tauri 事件 `pet-model-changed`，payload 包含 `{ modelId: string }`
- **THEN** 设置窗口卡片列表 MUST 立即更新（active 标记切换）

#### Scenario: 宠物窗口接收事件并切换渲染
- **WHEN** 宠物窗口接收到 `pet-model-changed` 事件
- **THEN** 系统 MUST 通过 `modelRegistry.getById(modelId)` 获取目标宠物的 `PetModelConfig`
- **THEN** 系统 MUST 调用 `petStore.setCurrentModel(config)` 更新响应式状态
- **THEN** `Live2DCanvas` MUST 销毁旧渲染器并根据新模型 type 创建对应渲染器（Live2D 或 Sprite）
- **THEN** 新宠物 MUST 在新渲染器中正常显示

#### Scenario: 宠物窗口启动时同步 active 模型
- **WHEN** 宠物窗口启动（App.vue onMounted，非 settings 路由）
- **THEN** 系统 MUST 先注册 `pet-model-changed` 事件监听
- **THEN** 系统 MUST 调用 `invoke('get_active_model_id')` 获取 DB 中的 active 模型 ID
- **THEN** 系统 MUST 通过 ModelRegistry 查找对应配置并调用 `petStore.setCurrentModel()`
- **THEN** 宠物窗口 MUST 渲染 DB 中记录的 active 模型（而非硬编码默认值）

#### Scenario: 重启后恢复用户选择
- **WHEN** 用户切换宠物模型后关闭应用再重新打开
- **THEN** 宠物窗口 MUST 显示上次切换后的 active 宠物
